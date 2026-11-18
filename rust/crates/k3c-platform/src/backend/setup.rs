//! wgpu-Aufbau: Bind-Layout, die zwei Pipelines (Rechteck-Instanzen, Dreiecke), Puffer, Textur-Upload.

use crate::river::RIVER_BYTES;
use crate::{Blend, INST_BYTES, VERT_BYTES};

const SHADER: &str = include_str!("shader.wgsl");
/// Fluss-Shader (B-1072), `river.kage` als WGSL.
pub const RIVER_SHADER: &str = include_str!("river.wgsl");

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

/// Mischzustand je [`Blend`]: `Multiply` rechnet Ziel × Quelle (Farbe), die Deckkraft des Ziels bleibt (wie
/// `multiply` in `lightdraw.go`: Quelle-Faktor Ziel-Farbe bzw. 0, Ziel-Faktor 0 bzw. 1).
pub fn blend_state(mode: Blend) -> wgpu::BlendState {
    match mode {
        Blend::Over => wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
        Blend::Multiply => wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Dst,
                dst_factor: wgpu::BlendFactor::Zero,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Zero,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        },
    }
}

/// Pipeline für Rechteck-Instanzen (`vs`/`fs`) oder Dreiecke (`vs_tri`/`fs_tri`) mit der Mischung `mode`.
pub fn pipeline(
    device: &wgpu::Device,
    bgl: &wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
    (tris, mode): (bool, Blend),
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
        blend: Some(blend_state(mode)),
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

/// Prüft WGSL ohne GPU (Parser und Validator von `naga`); der Fehlertext nennt die Stelle.
pub fn validate_wgsl(src: &str) -> Result<wgpu::naga::Module, String> {
    use wgpu::naga::valid::{Capabilities, ValidationFlags, Validator};
    let module = wgpu::naga::front::wgsl::parse_str(src).map_err(|e| e.emit_to_string(src))?;
    Validator::new(ValidationFlags::all(), Capabilities::empty())
        .validate(&module)
        .map_err(|e| e.emit_to_string(src))?;
    Ok(module)
}

/// Bind-Layout des Fluss-Shaders: ein Uniform-Puffer mit dynamischem Versatz (ein Aufruf je Versatz).
pub fn river_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("fluss"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: true,
                min_binding_size: wgpu::BufferSize::new(RIVER_BYTES as u64),
            },
            count: None,
        }],
    })
}

/// Abstand der Aufrufe im Uniform-Puffer: `RIVER_BYTES` aufgerundet auf die Versatz-Ausrichtung des Geräts.
pub fn river_stride(device: &wgpu::Device) -> u64 {
    let a = u64::from(device.limits().min_uniform_buffer_offset_alignment).max(1);
    (RIVER_BYTES as u64).div_ceil(a) * a
}

/// Pipeline des Flusses mit der Mischung `mode`; `Err`, wenn der Shader nicht übersetzt (Aufrufer meldet es im Log und
/// überspringt Fluss-Aufrufe).
pub fn river_pipeline(
    device: &wgpu::Device,
    bgl: &wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
    mode: Blend,
) -> Result<wgpu::RenderPipeline, String> {
    validate_wgsl(RIVER_SHADER)?;
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("fluss"),
        source: wgpu::ShaderSource::Wgsl(RIVER_SHADER.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(bgl)],
        immediate_size: 0,
    });
    let target = wgpu::ColorTargetState {
        format,
        blend: Some(blend_state(mode)),
        write_mask: wgpu::ColorWrites::ALL,
    };
    Ok(device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("fluss"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs"),
            compilation_options: Default::default(),
            buffers: &[],
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
    }))
}

/// Pipelines des Flusses je Mischung (`Over`, `Multiply`); bei einem Fehler im Log gemeldet und `None`: Der Fluss
/// fällt über `Canvas::river` auf Kacheln zurück, das Backend überspringt Fluss-Aufrufe.
pub fn river_pipelines(
    device: &wgpu::Device,
    bgl: &wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
) -> Option<[wgpu::RenderPipeline; 2]> {
    let made = river_pipeline(device, bgl, format, Blend::Over)
        .and_then(|a| Ok([a, river_pipeline(device, bgl, format, Blend::Multiply)?]));
    made.map_err(|e| crate::log::error("❌ Fluss-Shader übersetzt nicht, Fluss als Kacheln", &[("err", &e)]))
        .ok()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::river::{SRC_H, TAU};
    use wgpu::naga::{Expression, Literal};

    /// Wert einer benannten `const` des Moduls als `f32`.
    fn constant(m: &wgpu::naga::Module, name: &str) -> f32 {
        let (_, c) = m
            .constants
            .iter()
            .find(|(_, c)| c.name.as_deref() == Some(name))
            .unwrap_or_else(|| panic!("const {name} fehlt"));
        match m.global_expressions[c.init] {
            Expression::Literal(Literal::F32(v)) => v,
            Expression::Literal(Literal::AbstractFloat(v)) => v as f32,
            ref e => panic!("const {name}: {e:?}"),
        }
    }

    /// B-1072 AC-01: beide Shader bestehen die Validierung ohne GPU.
    #[test]
    fn shader_bestehen_die_naga_validierung() {
        validate_wgsl(SHADER).expect("shader.wgsl");
        let m = validate_wgsl(RIVER_SHADER).expect("river.wgsl");
        let names: Vec<_> = m.entry_points.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(names, ["vs", "fs"]);
    }

    /// B-1072 AC-03: Die Konstanten des WGSL sind die der Rust-Referenz (`river_fragment`) und von `river.kage`.
    #[test]
    fn konstanten_gleich_der_referenz() {
        let m = validate_wgsl(RIVER_SHADER).unwrap();
        assert_eq!(constant(&m, "tau"), TAU);
        assert_eq!(constant(&m, "srcH"), SRC_H);
    }

    /// Eine kaputte Quelle meldet einen Fehler statt zu paniken.
    #[test]
    fn kaputtes_wgsl_gibt_fehler() {
        assert!(validate_wgsl("fn fs( {").is_err());
    }

    /// B-1072 AC-02 auf einem Adapter, falls einer da ist (sonst übersprungen wie die Standbild-Tests ohne Grafik):
    /// Bind-Layout, Stride und beide Pipelines entstehen ohne Validierungsfehler.
    #[test]
    fn fluss_pipeline_entsteht_auf_gpu() {
        let inst = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let Ok(adapter) = pollster::block_on(inst.request_adapter(&Default::default())) else {
            eprintln!("kein GPU-Adapter: übersprungen");
            return;
        };
        let Ok((device, _)) = pollster::block_on(adapter.request_device(&Default::default())) else {
            return;
        };
        let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
        let bgl = river_layout(&device);
        let pipes = river_pipelines(&device, &bgl, wgpu::TextureFormat::Bgra8Unorm);
        let err = pollster::block_on(scope.pop());
        assert!(err.is_none(), "{err:?}");
        assert!(pipes.is_some());
        assert!(river_stride(&device) >= RIVER_BYTES as u64);
    }
}
