//! wgpu-Seite: Gerät, Surface (Fifo), Texturen, Zeichnen eines [`Frame`] (Instanzen und Dreiecke im Wechsel nach
//! Draw-Call-Reihenfolge). Aufbau der Pipelines und Puffer in `setup.rs`.

use std::sync::Arc;

use winit::window::Window;

use super::mirror_setup::mirror_stride;
use super::setup::{Grow, bind_layout, pipeline, river_layout, river_pipelines, river_stride, sampler, upload, write};
use crate::batch::Pending;
use crate::log;
use crate::river::{RIVER_BYTES, pack_river};
use crate::{Blend, Filter, Frame, INST_BYTES, Kind, LOGICAL, Rect, VERT_BYTES};

const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.118,
    g: 0.118,
    b: 0.235,
    a: 1.0,
};
const START_BYTES: u64 = 1 << 18;

#[path = "gpu_mirror.rs"]
mod mirror;
use mirror::init_mirror;

pub struct Gfx {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    /// Pipelines je (Art, Mischung): `[Quads, Tris][Over, Multiply]`.
    pipes: [[wgpu::RenderPipeline; 2]; 2],
    bgl: wgpu::BindGroupLayout,
    /// Fluss-Shader (B-1072): Pipelines je Mischung; `None`, wenn der Shader nicht übersetzt (Fluss-Aufrufe entfallen).
    river_pipes: Option<[wgpu::RenderPipeline; 2]>,
    river_bgl: wgpu::BindGroupLayout,
    river_stride: u64,
    river_u: Grow,
    /// Bind-Group über `river_u`, je Bild neu (der Puffer kann gewachsen sein).
    river_bg: Option<wgpu::BindGroup>,
    /// Spiegel (RU55.3, gpu_mirror.rs): Pipeline nur mit kopierbarer Fläche; Zwischentextur lazy in Flächengröße.
    mirror_pipe: Option<wgpu::RenderPipeline>,
    mirror_bgl: wgpu::BindGroupLayout,
    mirror_stride: u64,
    mirror_u: Grow,
    mirror_tex: Option<(wgpu::Texture, wgpu::TextureView)>,
    mirror_bg: Option<wgpu::BindGroup>,
    linear: wgpu::Sampler,
    nearest: wgpu::Sampler,
    white: wgpu::BindGroup,
    /// Je `TexId` Textur und Bind-Group; `None` = freigegeben oder nie hochgeladen.
    texs: Vec<Option<(wgpu::Texture, wgpu::BindGroup)>>,
    inst: Grow,
    verts: Grow,
    idx: Grow,
    scratch: Vec<u8>,
}

impl Gfx {
    /// Erzeugt Gerät und Pipelines für `win`; liefert zusätzlich den Namen des Backends (Vulkan, Dx12 …).
    pub fn new(win: Arc<Window>, novsync: bool) -> Result<(Self, String), String> {
        let desc = wgpu::InstanceDescriptor::new_without_display_handle_from_env();
        let inst = wgpu::Instance::new(desc);
        let surface = inst.create_surface(win.clone()).map_err(|e| e.to_string())?;
        let opts = wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        };
        let adapter = pollster::block_on(inst.request_adapter(&opts)).map_err(|e| e.to_string())?;
        let backend = format!("{:?}", adapter.get_info().backend);
        let (device, queue) =
            pollster::block_on(adapter.request_device(&Default::default())).map_err(|e| e.to_string())?;
        let size = win.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .ok_or("Surface nicht nutzbar")?;
        let copy_src = tune_config(&mut config, &surface.get_capabilities(&adapter), novsync);
        surface.configure(&device, &config);
        let bgl = bind_layout(&device);
        let linear = sampler(&device, wgpu::FilterMode::Linear);
        let nearest = sampler(&device, wgpu::FilterMode::Nearest);
        let (_, white) = upload(&device, &queue, &bgl, &linear, (1, 1), &[255; 4]);
        use wgpu::BufferUsages as U;
        let river_bgl = river_layout(&device);
        let river_pipes = river_pipelines(&device, &river_bgl, config.format);
        let (mirror_bgl, mirror_pipe) = init_mirror(&device, &config, copy_src);
        let g = Self {
            mirror_pipe,
            mirror_stride: mirror_stride(&device),
            mirror_u: Grow::new(&device, "spiegel", U::UNIFORM, START_BYTES),
            mirror_tex: None,
            mirror_bg: None,
            mirror_bgl,
            river_stride: river_stride(&device),
            river_u: Grow::new(&device, "fluss", U::UNIFORM, START_BYTES),
            river_bg: None,
            river_pipes,
            river_bgl,
            pipes: [false, true].map(|tris| {
                [Blend::Over, Blend::Multiply].map(|mode| pipeline(&device, &bgl, config.format, (tris, mode)))
            }),
            inst: Grow::new(&device, "instanzen", U::VERTEX, START_BYTES),
            verts: Grow::new(&device, "ecken", U::VERTEX, START_BYTES),
            idx: Grow::new(&device, "indizes", U::INDEX, START_BYTES),
            surface,
            device,
            queue,
            config,
            bgl,
            linear,
            nearest,
            white,
            texs: Vec::new(),
            scratch: Vec::new(),
        };
        Ok((g, backend))
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.config.width = w.max(1);
        self.config.height = h.max(1);
        self.surface.configure(&self.device, &self.config);
        self.mirror_tex = None;
    }

    /// Ob der Spiegel zeichnen kann (Pipeline steht, Fläche ist kopierbar).
    pub fn mirror_ok(&self) -> bool {
        self.mirror_pipe.is_some()
    }

    /// Lädt neue Texturen hoch, schreibt Instanzen und Dreiecke, zeichnet ein Bild und gibt danach
    /// freigegebene Texturen frei. `pre_present` läuft unmittelbar vor `present` (Messpunkt der CPU-Zeit).
    pub fn render(&mut self, frame: &mut Frame, pre_present: impl FnOnce()) {
        self.upload_pending(frame);
        self.fill_buffers(frame);
        let st = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => Some(t),
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                None
            }
            _ => None,
        };
        if let Some(st) = st {
            let view = st.texture.create_view(&Default::default());
            let mut enc = self.device.create_command_encoder(&Default::default());
            self.draw(&mut enc, (&view, &st.texture), frame);
            self.queue.submit([enc.finish()]);
            pre_present();
            self.queue.present(st);
        }
        for t in frame.take_freed() {
            if let Some(slot) = self.texs.get_mut(t.0 as usize) {
                *slot = None;
            }
        }
    }

    fn upload_pending(&mut self, frame: &mut Frame) {
        for Pending { id, w, h, filter, rgba } in frame.take_pending() {
            let s = match filter {
                Filter::Linear => &self.linear,
                Filter::Nearest => &self.nearest,
            };
            let t = upload(&self.device, &self.queue, &self.bgl, s, (w, h), &rgba);
            let i = id.0 as usize;
            if self.texs.len() <= i {
                self.texs.resize_with(i + 1, || None);
            }
            self.texs[i] = Some(t);
        }
        if let Some((id, ups)) = frame.take_atlas_updates()
            && let Some(Some((tex, _))) = self.texs.get(id.0 as usize)
        {
            for u in ups {
                write(&self.queue, tex, (u.x, u.y, u.w, u.h), &u.rgba);
            }
        }
    }

    /// Schreibt die Uniforms der Fluss-Aufrufe (ein Abstand `river_stride` je Aufruf) und bindet sie.
    fn fill_rivers(&mut self, frame: &Frame) {
        let calls = frame.river_calls();
        self.river_bg = None;
        if calls.is_empty() || self.river_pipes.is_none() {
            return;
        }
        let stride = self.river_stride as usize;
        self.river_u.reserve(&self.device, (calls.len() * stride) as u64);
        self.scratch.clear();
        self.scratch.resize(calls.len() * stride, 0);
        for (c, slot) in calls.iter().zip(self.scratch.chunks_mut(stride)) {
            pack_river(c, slot);
        }
        self.queue.write_buffer(&self.river_u.buf, 0, &self.scratch);
        self.river_bg = Some(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("fluss"),
            layout: &self.river_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &self.river_u.buf,
                    offset: 0,
                    size: wgpu::BufferSize::new(RIVER_BYTES as u64),
                }),
            }],
        }));
    }

    fn fill_buffers(&mut self, frame: &Frame) {
        let n = frame.instance_count() * INST_BYTES;
        if n > 0 {
            self.inst.reserve(&self.device, n as u64);
            self.scratch.resize(n, 0);
            frame.write_instances(&mut self.scratch);
            self.queue.write_buffer(&self.inst.buf, 0, &self.scratch);
        }
        let n = frame.vertices().len() * VERT_BYTES;
        if n > 0 {
            self.verts.reserve(&self.device, n as u64);
            self.scratch.resize(n, 0);
            frame.write_vertices(&mut self.scratch);
            self.queue.write_buffer(&self.verts.buf, 0, &self.scratch);
        }
        self.fill_rivers(frame);
        self.fill_mirrors(frame);
        let idx = frame.indices();
        if !idx.is_empty() {
            let n = idx.len() * 4;
            self.idx.reserve(&self.device, n as u64);
            self.scratch.clear();
            self.scratch.extend(idx.iter().flat_map(|i| i.to_ne_bytes()));
            self.queue.write_buffer(&self.idx.buf, 0, &self.scratch);
        }
    }

    fn draw(&self, enc: &mut wgpu::CommandEncoder, (view, tex): (&wgpu::TextureView, &wgpu::Texture), frame: &Frame) {
        let mut p = begin_pass(enc, view, wgpu::LoadOp::Clear(CLEAR));
        let (sw, sh) = (self.config.width, self.config.height);
        let mut current = None;
        for r in frame.runs() {
            let Some(sc) = scissor(r.clip, sw, sh) else {
                continue;
            };
            let (x, y, w, h) = sc;
            if r.kind == Kind::River {
                if self.draw_river(&mut p, r, sc) {
                    // Pipeline und Gruppe 0 gehören jetzt dem Fluss: der nächste Run setzt alles neu.
                    current = None;
                }
                continue;
            }
            if r.kind == Kind::Mirror {
                if let (Some(_), Some(_), Some(call)) = (
                    &self.mirror_pipe,
                    &self.mirror_bg,
                    frame.mirror_calls().get(r.first as usize),
                ) {
                    // Das bisher Gezeichnete über der Gehlinie wandert in die Zwischentextur: neuer Durchgang ohne Löschen.
                    drop(p);
                    self.copy_strip(enc, tex, call);
                    p = begin_pass(enc, view, wgpu::LoadOp::Load);
                    self.draw_mirror(&mut p, r.first, sc);
                    current = None;
                }
                continue;
            }
            let bg = match r.tex {
                None => &self.white,
                Some(t) => match self.texs.get(t.0 as usize) {
                    Some(Some((_, bg))) => bg,
                    _ => continue,
                },
            };
            if current != Some((r.kind, r.blend)) {
                if current.map(|c| c.0) != Some(r.kind) {
                    match r.kind {
                        Kind::Quads => p.set_vertex_buffer(0, self.inst.buf.slice(..)),
                        Kind::Tris => {
                            p.set_vertex_buffer(0, self.verts.buf.slice(..));
                            p.set_index_buffer(self.idx.buf.slice(..), wgpu::IndexFormat::Uint32);
                        }
                        Kind::River | Kind::Mirror => {}
                    }
                }
                current = Some((r.kind, r.blend));
                p.set_pipeline(&self.pipes[r.kind as usize][r.blend as usize]);
            }
            p.set_scissor_rect(x, y, w, h);
            p.set_bind_group(0, Some(bg), &[]);
            let range = r.first..r.first + r.count;
            match r.kind {
                Kind::Quads => p.draw(0..6, range),
                Kind::Tris => p.draw_indexed(range, 0, 0..1),
                Kind::River | Kind::Mirror => {}
            }
        }
    }

    /// Zeichnet einen Fluss-Aufruf; `false`, wenn der Shader fehlt.
    fn draw_river(&self, p: &mut wgpu::RenderPass<'_>, r: &crate::Run, (x, y, w, h): (u32, u32, u32, u32)) -> bool {
        let (Some(pipes), Some(bg)) = (&self.river_pipes, &self.river_bg) else {
            return false;
        };
        p.set_pipeline(&pipes[r.blend as usize]);
        p.set_scissor_rect(x, y, w, h);
        p.set_bind_group(0, Some(bg), &[(u64::from(r.first) * self.river_stride) as u32]);
        p.draw(0..6, 0..1);
        true
    }
}

/// Beginnt einen Durchgang auf `view` mit der Anfangs-Operation `load` (Löschen im ersten, Laden nach einem Spiegel).
fn begin_pass<'e>(
    enc: &'e mut wgpu::CommandEncoder,
    view: &wgpu::TextureView,
    load: wgpu::LoadOp<wgpu::Color>,
) -> wgpu::RenderPass<'e> {
    let att = wgpu::RenderPassColorAttachment {
        view,
        depth_slice: None,
        resolve_target: None,
        ops: wgpu::Operations {
            load,
            store: wgpu::StoreOp::Store,
        },
    };
    enc.begin_render_pass(&wgpu::RenderPassDescriptor {
        color_attachments: &[Some(att)],
        ..Default::default()
    })
}

/// Stellt Präsentationsmodus, Format und Nutzung der Fläche ein; liefert, ob die Fläche kopierbar ist (Spiegel).
fn tune_config(config: &mut wgpu::SurfaceConfiguration, caps: &wgpu::SurfaceCapabilities, novsync: bool) -> bool {
    config.present_mode = pick_present_mode(novsync, &caps.present_modes);
    let copy_src = caps.usages.contains(wgpu::TextureUsages::COPY_SRC);
    if copy_src {
        config.usage |= wgpu::TextureUsages::COPY_SRC;
    }
    if let Some(f) = pick_surface_format(&caps.formats) {
        config.format = f;
    }
    if config.format.is_srgb() {
        log::warn(
            "🚦 Fläche nur als sRGB, Farben können blasser sein",
            &[("format", &format!("{:?}", config.format))],
        );
    }
    copy_src
}

/// Format der Fläche ohne sRGB-Kodierung (B-1128): Shader und Texturen liefern schon sRGB-kodierte Werte (wie Go),
/// eine `*Srgb`-Fläche kodierte sie ein zweites Mal und die Farben würden blass. `Surface::get_default_config` nimmt
/// das erste Format und das ist unter Windows ein sRGB-Format. Ohne lineares Format bleibt das erste.
fn pick_surface_format(formats: &[wgpu::TextureFormat]) -> Option<wgpu::TextureFormat> {
    use wgpu::TextureFormat as F;
    let linear8 = |f: &&F| matches!(f, F::Bgra8Unorm | F::Rgba8Unorm);
    formats
        .iter()
        .find(linear8)
        .or_else(|| formats.iter().find(|f| !f.is_srgb()))
        .or(formats.first())
        .copied()
}

/// Präsentationsmodus: mit `novsync` ohne Warten (`Immediate`, sonst `Mailbox`), sonst oder ohne Treffer `Fifo`
/// (immer verfügbar).
fn pick_present_mode(novsync: bool, modes: &[wgpu::PresentMode]) -> wgpu::PresentMode {
    use wgpu::PresentMode as M;
    if !novsync {
        return M::Fifo;
    }
    match [M::Immediate, M::Mailbox].into_iter().find(|m| modes.contains(m)) {
        Some(m) => m,
        None => {
            log::info("🐢 novsync: kein Modus ohne Warten, Fifo", &[]);
            M::Fifo
        }
    }
}

/// Ausschnitt in logischen Einheiten → Scissor in Pixeln der Surface (gleiche Streckung wie der Shader);
/// `None` bei leerer Fläche.
fn scissor(clip: Option<Rect>, sw: u32, sh: u32) -> Option<(u32, u32, u32, u32)> {
    let Some(c) = clip else {
        return Some((0, 0, sw, sh));
    };
    let (kx, ky) = (sw as f32 / LOGICAL.0, sh as f32 / LOGICAL.1);
    let x0 = (c.x * kx).round().clamp(0.0, sw as f32) as u32;
    let y0 = (c.y * ky).round().clamp(0.0, sh as f32) as u32;
    let x1 = ((c.x + c.w) * kx).round().clamp(0.0, sw as f32) as u32;
    let y1 = ((c.y + c.h) * ky).round().clamp(0.0, sh as f32) as u32;
    (x1 > x0 && y1 > y0).then_some((x0, y0, x1 - x0, y1 - y0))
}

#[cfg(test)]
#[path = "gpu_tests.rs"]
mod tests;
