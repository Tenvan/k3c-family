//! wgpu-Aufbau des Spiegels (RU55.3): Bind-Layout, Pipeline, Zwischentextur. Der Shader rechnet `mirror_fragment`.

use super::setup::{blend_state, validate_wgsl};
use crate::Blend;
use crate::mirror::MIRROR_BYTES;

/// Spiegel-Shader (RU55.3), `mirror_fragment` als WGSL.
pub const MIRROR_SHADER: &str = include_str!("mirror.wgsl");

/// Bind-Layout des Spiegel-Shaders: Uniform-Puffer mit dynamischem Versatz (ein Aufruf je Versatz), die Zwischentextur
/// und ein Sampler.
pub fn mirror_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    let entry = |binding, visibility, ty| wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty,
        count: None,
    };
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("spiegel"),
        entries: &[
            entry(
                0,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: wgpu::BufferSize::new(MIRROR_BYTES as u64),
                },
            ),
            entry(
                1,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
            ),
            entry(
                2,
                wgpu::ShaderStages::FRAGMENT,
                wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            ),
        ],
    })
}

/// Abstand der Aufrufe im Uniform-Puffer des Spiegels (`MIRROR_BYTES` auf die Versatz-Ausrichtung aufgerundet).
pub fn mirror_stride(device: &wgpu::Device) -> u64 {
    let a = u64::from(device.limits().min_uniform_buffer_offset_alignment).max(1);
    (MIRROR_BYTES as u64).div_ceil(a) * a
}

/// Pipeline des Spiegels (deckend: Quelle über das Ziel, Alpha 1); `Err`, wenn der Shader nicht übersetzt.
pub fn mirror_pipeline(
    device: &wgpu::Device,
    bgl: &wgpu::BindGroupLayout,
    format: wgpu::TextureFormat,
) -> Result<wgpu::RenderPipeline, String> {
    validate_wgsl(MIRROR_SHADER)?;
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("spiegel"),
        source: wgpu::ShaderSource::Wgsl(MIRROR_SHADER.into()),
    });
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[Some(bgl)],
        immediate_size: 0,
    });
    let target = wgpu::ColorTargetState {
        format,
        blend: Some(blend_state(Blend::Over)),
        write_mask: wgpu::ColorWrites::ALL,
    };
    Ok(device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("spiegel"),
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

/// Zwischentextur des Spiegels in der Größe der Fläche und im Format der Fläche (Ziel der Kopie, Quelle des Shaders).
pub fn mirror_texture(device: &wgpu::Device, (w, h): (u32, u32), format: wgpu::TextureFormat) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("spiegel"),
        size: wgpu::Extent3d {
            width: w.max(1),
            height: h.max(1),
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}

#[cfg(test)]
#[path = "mirror_gpu_tests.rs"]
mod mirror_gpu_tests;
