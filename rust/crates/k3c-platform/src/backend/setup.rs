//! wgpu-Aufbau: Bind-Layout, die zwei Pipelines (Rechteck-Instanzen, Dreiecke), Puffer, Textur-Upload.

use crate::{INST_BYTES, VERT_BYTES};

const SHADER: &str = include_str!("shader.wgsl");

/// Wachsender GPU-Puffer (Instanzen, Ecken oder Indizes); wird nur bei Bedarf vergrößert.
pub struct Grow {
    pub buf: wgpu::Buffer,
    cap: u64,
    usage: wgpu::BufferUsages,
    label: &'static str,
}

impl Grow {
    pub fn new(device: &wgpu::Device, label: &'static str, usage: wgpu::BufferUsages, cap: u64) -> Self {
        Self {
            buf: buffer(device, label, usage, cap),
            cap,
            usage,
            label,
        }
    }

    /// Sorgt für mindestens `bytes` Platz (Zweierpotenz).
    pub fn reserve(&mut self, device: &wgpu::Device, bytes: u64) {
        if bytes > self.cap {
            self.cap = bytes.next_power_of_two();
            self.buf = buffer(device, self.label, self.usage, self.cap);
        }
    }
}

fn buffer(device: &wgpu::Device, label: &str, usage: wgpu::BufferUsages, size: u64) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size,
        usage: usage | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

pub fn bind_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
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

pub fn sampler(device: &wgpu::Device, f: wgpu::FilterMode) -> wgpu::Sampler {
    device.create_sampler(&wgpu::SamplerDescriptor {
        mag_filter: f,
        min_filter: f,
        ..Default::default()
    })
}

/// Pipeline für Rechteck-Instanzen (`vs`/`fs`) oder Dreiecke (`vs_tri`/`fs_tri`).
pub fn pipeline(
    device: &wgpu::Device,
    bgl: &wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
    tris: bool,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(bgl)],
        immediate_size: 0,
    });
    let quad_attrs = wgpu::vertex_attr_array![0 => Float32x4, 1 => Float32x4, 2 => Float32x4, 3 => Float32x4];
    let tri_attrs = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Float32x4];
    let buffers = [Some(if tris {
        wgpu::VertexBufferLayout {
            array_stride: VERT_BYTES as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &tri_attrs,
        }
    } else {
        wgpu::VertexBufferLayout {
            array_stride: INST_BYTES as u64,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &quad_attrs,
        }
    })];
    let (vs, fs) = if tris { ("vs_tri", "fs_tri") } else { ("vs", "fs") };
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
            entry_point: Some(vs),
            compilation_options: Default::default(),
            buffers: &buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some(fs),
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

/// Lädt RGBA8 hoch und liefert Textur und Bind-Group mit dem Sampler `sampler`.
pub fn upload(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    bgl: &wgpu::BindGroupLayout,
    sampler: &wgpu::Sampler,
    (w, h): (u32, u32),
    rgba: &[u8],
) -> (wgpu::Texture, wgpu::BindGroup) {
    let ext = wgpu::Extent3d {
        width: w.max(1),
        height: h.max(1),
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
    write(queue, &tex, (0, 0, w, h), rgba);
    let view = tex.create_view(&Default::default());
    let bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
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
    });
    (tex, bg)
}

/// Schreibt RGBA8 in den Ausschnitt `(x, y, w, h)` der Textur (leer: nichts).
pub fn write(queue: &wgpu::Queue, tex: &wgpu::Texture, (x, y, w, h): (u32, u32, u32, u32), rgba: &[u8]) {
    if w == 0 || h == 0 {
        return;
    }
    queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: wgpu::Origin3d { x, y, z: 0 },
            aspect: wgpu::TextureAspect::All,
        },
        rgba,
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(w * 4),
            rows_per_image: Some(h),
        },
        wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
    );
}
