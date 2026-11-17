//! wgpu-Seite: Gerät, Surface (Fifo), Texturen, Zeichnen eines [`Frame`] (Instanzen und Dreiecke im Wechsel nach
//! Draw-Call-Reihenfolge). Aufbau der Pipelines und Puffer in `setup.rs`.

use std::sync::Arc;

use winit::window::Window;

use super::setup::{Grow, bind_layout, pipeline, sampler, upload};
use crate::batch::Pending;
use crate::{Filter, Frame, INST_BYTES, Kind, LOGICAL, Rect, VERT_BYTES};

const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.118,
    g: 0.118,
    b: 0.235,
    a: 1.0,
};
const START_BYTES: u64 = 1 << 18;

pub struct Gfx {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    quads: wgpu::RenderPipeline,
    tris: wgpu::RenderPipeline,
    bgl: wgpu::BindGroupLayout,
    linear: wgpu::Sampler,
    nearest: wgpu::Sampler,
    white: wgpu::BindGroup,
    /// Je `TexId` die Bind-Group; `None` = freigegeben oder nie hochgeladen.
    texs: Vec<Option<wgpu::BindGroup>>,
    inst: Grow,
    verts: Grow,
    idx: Grow,
    scratch: Vec<u8>,
}

impl Gfx {
    /// Erzeugt Gerät und Pipelines für `win`; liefert zusätzlich den Namen des Backends (Vulkan, Dx12 …).
    pub fn new(win: Arc<Window>) -> Result<(Self, String), String> {
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
        config.present_mode = wgpu::PresentMode::Fifo;
        surface.configure(&device, &config);
        let bgl = bind_layout(&device);
        let linear = sampler(&device, wgpu::FilterMode::Linear);
        let nearest = sampler(&device, wgpu::FilterMode::Nearest);
        let white = upload(&device, &queue, &bgl, &linear, (1, 1), &[255; 4]);
        use wgpu::BufferUsages as U;
        let g = Self {
            quads: pipeline(&device, &bgl, config.format, false),
            tris: pipeline(&device, &bgl, config.format, true),
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
            self.draw(&mut enc, &view, frame);
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
            let bg = upload(&self.device, &self.queue, &self.bgl, s, (w, h), &rgba);
            let i = id.0 as usize;
            if self.texs.len() <= i {
                self.texs.resize_with(i + 1, || None);
            }
            self.texs[i] = Some(bg);
        }
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
        let idx = frame.indices();
        if !idx.is_empty() {
            let n = idx.len() * 4;
            self.idx.reserve(&self.device, n as u64);
            self.scratch.clear();
            self.scratch.extend(idx.iter().flat_map(|i| i.to_ne_bytes()));
            self.queue.write_buffer(&self.idx.buf, 0, &self.scratch);
        }
    }

    fn draw(&self, enc: &mut wgpu::CommandEncoder, view: &wgpu::TextureView, frame: &Frame) {
        let att = wgpu::RenderPassColorAttachment {
            view,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(CLEAR),
                store: wgpu::StoreOp::Store,
            },
        };
        let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(att)],
            ..Default::default()
        });
        let (sw, sh) = (self.config.width, self.config.height);
        let mut current = None;
        for r in frame.runs() {
            let Some((x, y, w, h)) = scissor(r.clip, sw, sh) else {
                continue;
            };
            let bg = match r.tex {
                None => &self.white,
                Some(t) => match self.texs.get(t.0 as usize) {
                    Some(Some(bg)) => bg,
                    _ => continue,
                },
            };
            if current != Some(r.kind) {
                current = Some(r.kind);
                match r.kind {
                    Kind::Quads => {
                        p.set_pipeline(&self.quads);
                        p.set_vertex_buffer(0, self.inst.buf.slice(..));
                    }
                    Kind::Tris => {
                        p.set_pipeline(&self.tris);
                        p.set_vertex_buffer(0, self.verts.buf.slice(..));
                        p.set_index_buffer(self.idx.buf.slice(..), wgpu::IndexFormat::Uint32);
                    }
                }
            }
            p.set_scissor_rect(x, y, w, h);
            p.set_bind_group(0, Some(bg), &[]);
            let range = r.first..r.first + r.count;
            match r.kind {
                Kind::Quads => p.draw(0..6, range),
                Kind::Tris => p.draw_indexed(range, 0, 0..1),
            }
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
mod tests {
    use super::*;

    #[test]
    fn ausschnitt_wird_auf_surface_gestreckt_und_begrenzt() {
        assert_eq!(scissor(None, 1280, 720), Some((0, 0, 1280, 720)));
        let right = Rect::new(960.0, 0.0, 960.0, 1080.0);
        assert_eq!(scissor(Some(right), 1280, 720), Some((640, 0, 640, 720)));
        assert_eq!(
            scissor(Some(Rect::new(-100.0, -5.0, 3000.0, 2000.0)), 640, 360),
            Some((0, 0, 640, 360))
        );
        assert_eq!(scissor(Some(Rect::new(2000.0, 0.0, 10.0, 10.0)), 640, 360), None);
    }
}
