//! wgpu-Seite: Gerät, Surface (Fifo), Pipeline für Instanzen, Texturen, Zeichnen eines [`Frame`].

use std::sync::Arc;

use winit::window::Window;

use crate::batch::Pending;
use crate::{Frame, INST_BYTES, LOGICAL, Rect};

const SHADER: &str = include_str!("shader.wgsl");
const CLEAR: wgpu::Color = wgpu::Color {
    r: 0.118,
    g: 0.118,
    b: 0.235,
    a: 1.0,
};
const START_CAP: u64 = 4096;

pub struct Gfx {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipe: wgpu::RenderPipeline,
    bgl: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    white: wgpu::BindGroup,
    texs: Vec<wgpu::BindGroup>,
    ibuf: wgpu::Buffer,
    cap: u64,
    scratch: Vec<u8>,
}

impl Gfx {
    /// Erzeugt Gerät und Pipeline für `win`; liefert zusätzlich den Namen des Backends (Vulkan, Dx12 …).
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
        let pipe = pipeline(&device, &bgl, config.format);
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let white = upload(&device, &queue, &bgl, &sampler, 1, 1, &[255; 4]);
        let ibuf = instance_buffer(&device, START_CAP);
        let g = Self {
            surface,
            device,
            queue,
            config,
            pipe,
            bgl,
            sampler,
            white,
            texs: Vec::new(),
            ibuf,
            cap: START_CAP,
            scratch: Vec::new(),
        };
        Ok((g, backend))
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.config.width = w.max(1);
        self.config.height = h.max(1);
        self.surface.configure(&self.device, &self.config);
    }

    /// Lädt neue Texturen hoch, schreibt die Instanzen und zeichnet ein Bild.
    /// `pre_present` läuft unmittelbar vor `present` (Messpunkt der CPU-Zeit).
    pub fn render(&mut self, frame: &mut Frame, pre_present: impl FnOnce()) {
        self.upload_pending(frame);
        self.fill_instances(frame);
        let st = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(t) | wgpu::CurrentSurfaceTexture::Suboptimal(t) => t,
            wgpu::CurrentSurfaceTexture::Outdated | wgpu::CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
            _ => return,
        };
        let view = st.texture.create_view(&Default::default());
        let mut enc = self.device.create_command_encoder(&Default::default());
        self.draw(&mut enc, &view, frame);
        self.queue.submit([enc.finish()]);
        pre_present();
        self.queue.present(st);
    }

    fn upload_pending(&mut self, frame: &mut Frame) {
        for Pending { id, w, h, rgba } in frame.take_pending() {
            let bg = upload(&self.device, &self.queue, &self.bgl, &self.sampler, w, h, &rgba);
            debug_assert_eq!(id.0 as usize, self.texs.len());
            self.texs.push(bg);
        }
    }

    fn fill_instances(&mut self, frame: &Frame) {
        let n = frame.instance_count() as u64;
        if n == 0 {
            return;
        }
        if n > self.cap {
            self.cap = n.next_power_of_two();
            self.ibuf = instance_buffer(&self.device, self.cap);
        }
        self.scratch.resize(n as usize * INST_BYTES, 0);
        frame.write_instances(&mut self.scratch);
        self.queue.write_buffer(&self.ibuf, 0, &self.scratch);
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
        p.set_pipeline(&self.pipe);
        p.set_vertex_buffer(0, self.ibuf.slice(..));
        let (sw, sh) = (self.config.width, self.config.height);
        for r in frame.runs() {
            let Some((x, y, w, h)) = scissor(r.clip, sw, sh) else {
                continue;
            };
            p.set_scissor_rect(x, y, w, h);
            let bg = r.tex.and_then(|t| self.texs.get(t.0 as usize)).unwrap_or(&self.white);
            p.set_bind_group(0, Some(bg), &[]);
            p.draw(0..6, r.first..r.first + r.count);
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

fn instance_buffer(device: &wgpu::Device, cap: u64) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("instanzen"),
        size: cap * INST_BYTES as u64,
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn bind_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let tex = wgpu::BindingType::Texture {
        sample_type: wgpu::TextureSampleType::Float { filterable: true },
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: false,
    };
    let entry = |binding, ty| wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty,
        count: None,
    };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[
            entry(0, tex),
            entry(1, wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)),
        ],
    })
}

fn pipeline(device: &wgpu::Device, bgl: &wgpu::BindGroupLayout, format: wgpu::TextureFormat) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(bgl)],
        immediate_size: 0,
    });
    let attrs = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4];
    let buffers = [Some(wgpu::VertexBufferLayout {
        array_stride: INST_BYTES as u64,
        step_mode: wgpu::VertexStepMode::Instance,
        attributes: &attrs,
    })];
    let target = wgpu::ColorTargetState {
        format,
        blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
        write_mask: wgpu::ColorWrites::ALL,
    };
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs"),
            compilation_options: Default::default(),
            buffers: &buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs"),
            compilation_options: Default::default(),
            targets: &[Some(target)],
        }),
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        multiview_mask: None,
        cache: None,
    })
}

fn upload(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bgl: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    w: u32,
    h: u32,
    rgba: &[u8],
) -> wgpu::BindGroup {
    let ext = wgpu::Extent3d {
        width: w,
        height: h,
        depth_or_array_layers: 1,
    };
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: ext,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(w * 4),
            rows_per_image: Some(h),
        },
        ext,
    );
    let view = tex.create_view(&Default::default());
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(sampler),
            },
        ],
    })
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
