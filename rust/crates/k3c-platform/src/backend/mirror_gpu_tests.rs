//! RU55.3: Der Spiegel-Shader auf einem Adapter (falls einer da ist, sonst übersprungen) gegen `mirror_fragment`.

use super::*;
use crate::mirror::{MirrorUniforms, mirror_fragment, pack_mirror};
use crate::{MirrorCall, Rect};

const W: u32 = 640;
const H: u32 = 360;

#[test]
fn shader_besteht_die_naga_validierung() {
    let m = validate_wgsl(MIRROR_SHADER).expect("mirror.wgsl");
    let names: Vec<_> = m.entry_points.iter().map(|e| e.name.as_str()).collect();
    assert_eq!(names, ["vs", "fs"]);
}

fn uni() -> MirrorUniforms {
    MirrorUniforms {
        src_top: 200.0,
        ground: 500.0,
        axis: 540.0,
        squash: 0.8,
        brightness: 0.65,
        wobble: 3.0,
        phase_wobble: 1.0,
        phase_glint: 0.0,
        emissive: 0.5,
        tint: 0.2,
        depth_fade: 0.3,
        glint_mix: 0.0,
        water: [0.2, 0.4, 0.6],
        glint: [1.0; 3],
    }
}

/// Farbe der Quelle an einer Stelle des Fensters (glatter Verlauf, damit Abtasten und Rechnung übereinstimmen).
fn gradient(x: f32, y: f32) -> [f32; 3] {
    [x / 1920.0, y / 1080.0, 0.5]
}

/// Die Zwischentextur mit dem Verlauf gefüllt (Wert der Pixelmitte).
fn strip_texture(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> wgpu::Texture {
    let strip = mirror_texture(device, (W, H), format);
    let mut data = Vec::with_capacity((W * H * 4) as usize);
    for y in 0..H {
        for x in 0..W {
            let c = gradient(
                (x as f32 + 0.5) * 1920.0 / W as f32,
                (y as f32 + 0.5) * 1080.0 / H as f32,
            );
            data.extend(c.iter().map(|v| (v * 255.0).round() as u8));
            data.push(255);
        }
    }
    super::super::setup::write(queue, &strip, (0, 0, W, H), &data);
    strip
}

/// Bind-Group mit Uniforms (ein Aufruf), Zwischentextur und Sampler.
fn bind(device: &wgpu::Device, bgl: &wgpu::BindGroupLayout, strip: &wgpu::Texture) -> wgpu::BindGroup {
    let call = MirrorCall {
        dst: Rect::new(0.0, 540.0, 1920.0, 540.0),
        u: uni(),
    };
    let mut bytes = vec![0u8; mirror_stride(device) as usize];
    pack_mirror(&call, &mut bytes);
    use wgpu::util::DeviceExt as _;
    let ubuf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: &bytes,
        usage: wgpu::BufferUsages::UNIFORM,
    });
    let sampler = super::super::setup::sampler(device, wgpu::FilterMode::Linear);
    let view = strip.create_view(&Default::default());
    let entry = |binding, resource| wgpu::BindGroupEntry { binding, resource };
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: bgl,
        entries: &[
            entry(
                0,
                wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &ubuf,
                    offset: 0,
                    size: wgpu::BufferSize::new(crate::mirror::MIRROR_BYTES as u64),
                }),
            ),
            entry(1, wgpu::BindingResource::TextureView(&view)),
            entry(2, wgpu::BindingResource::Sampler(&sampler)),
        ],
    })
}

/// Zeichnet den Spiegel in ein schwarzes Ziel und liest die Pixel (RGBA8) zurück.
fn render(device: &wgpu::Device, queue: &wgpu::Queue, pipe: &wgpu::RenderPipeline, bg: &wgpu::BindGroup) -> Vec<u8> {
    let size = wgpu::Extent3d {
        width: W,
        height: H,
        depth_or_array_layers: 1,
    };
    let target = device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let tview = target.create_view(&Default::default());
    let out = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: u64::from(W * H * 4),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    {
        let att = wgpu::RenderPassColorAttachment {
            view: &tview,
            depth_slice: None,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
        };
        let mut p = enc.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[Some(att)],
            ..Default::default()
        });
        p.set_pipeline(pipe);
        p.set_bind_group(0, Some(bg), &[0]);
        p.draw(0..6, 0..1);
    }
    let layout = wgpu::TexelCopyBufferLayout {
        offset: 0,
        bytes_per_row: Some(W * 4),
        rows_per_image: Some(H),
    };
    let src = wgpu::TexelCopyTextureInfo {
        texture: &target,
        mip_level: 0,
        origin: wgpu::Origin3d::ZERO,
        aspect: wgpu::TextureAspect::All,
    };
    enc.copy_texture_to_buffer(src, wgpu::TexelCopyBufferInfo { buffer: &out, layout }, size);
    queue.submit([enc.finish()]);
    out.slice(..).map_async(wgpu::MapMode::Read, |r| assert!(r.is_ok()));
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    out.slice(..).get_mapped_range().expect("Bereich").to_vec()
}

/// Rendert den Spiegel offscreen und vergleicht mit der Rust-Referenz.
#[test]
fn shader_gleich_referenz_auf_gpu() {
    let inst = wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
    let Ok(adapter) = pollster::block_on(inst.request_adapter(&Default::default())) else {
        eprintln!("kein GPU-Adapter: übersprungen");
        return;
    };
    let Ok((device, queue)) = pollster::block_on(adapter.request_device(&Default::default())) else {
        return;
    };
    let bgl = mirror_layout(&device);
    let scope = device.push_error_scope(wgpu::ErrorFilter::Validation);
    let pipe = mirror_pipeline(&device, &bgl, wgpu::TextureFormat::Rgba8Unorm).expect("Pipeline");
    let strip = strip_texture(&device, &queue, wgpu::TextureFormat::Rgba8Unorm);
    let px = render(&device, &queue, &pipe, &bind(&device, &bgl, &strip));
    let err = pollster::block_on(scope.pop());
    assert!(err.is_none(), "{err:?}");
    let rect = [0.0, 540.0, 1920.0, 540.0];
    let (mut worst, mut n) = (0.0_f32, 0);
    for ty in (H / 2 + 2)..H - 2 {
        for tx in (2..W - 2).step_by(5) {
            let p = [(tx as f32 + 0.5) * 3.0, (ty as f32 + 0.5) * 3.0];
            let want = mirror_fragment(&uni(), rect, p, &gradient);
            let i = ((ty * W + tx) * 4) as usize;
            for k in 0..3 {
                worst = worst.max((f32::from(px[i + k]) / 255.0 - want[k]).abs());
            }
            n += 1;
        }
    }
    assert!(n > 1000);
    assert!(worst < 0.04, "größte Abweichung GPU gegen Referenz: {worst}");
    // Oberhalb des Rechtecks bleibt das Ziel schwarz.
    assert_eq!(&px[..4], &[0, 0, 0, 255]);
}
