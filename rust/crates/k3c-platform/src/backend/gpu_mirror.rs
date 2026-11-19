//! Spiegel im GPU-Backend (RU55.3): Uniforms und Bind-Group, Kopie des Streifens, Zeichnen. Teil von `gpu.rs` (Zugriff auf
//! die Felder von `Gfx`).

use super::super::mirror_setup::{mirror_layout, mirror_pipeline, mirror_texture};
use super::{Gfx, LOGICAL};
use crate::mirror::{MIRROR_BYTES, pack_mirror};
use crate::{Frame, log};

/// Layout und Pipeline des Spiegels; ohne kopierbare Fläche oder bei einem Shader-Fehler keine Pipeline (Wellen).
pub(super) fn init_mirror(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    copy_src: bool,
) -> (wgpu::BindGroupLayout, Option<wgpu::RenderPipeline>) {
    let bgl = mirror_layout(device);
    let pipe = if copy_src {
        mirror_pipeline(device, &bgl, config.format)
            .map_err(|e| log::error("❌ Spiegel-Shader übersetzt nicht, Fluss mit Wellen", &[("err", &e)]))
            .ok()
    } else {
        log::warn("🚦 Fläche nicht kopierbar, Fluss mit Wellen statt Spiegel", &[]);
        None
    };
    (bgl, pipe)
}

impl Gfx {
    /// Schreibt die Uniforms der Spiegel-Aufrufe (ein Abstand `mirror_stride` je Aufruf) und bindet sie mit der
    /// Zwischentextur (angelegt beim ersten Bedarf und nach jeder Größenänderung).
    pub(super) fn fill_mirrors(&mut self, frame: &Frame) {
        let calls = frame.mirror_calls();
        self.mirror_bg = None;
        if calls.is_empty() || self.mirror_pipe.is_none() {
            return;
        }
        let stride = self.mirror_stride as usize;
        self.mirror_u.reserve(&self.device, (calls.len() * stride) as u64);
        self.scratch.clear();
        self.scratch.resize(calls.len() * stride, 0);
        for (c, slot) in calls.iter().zip(self.scratch.chunks_mut(stride)) {
            pack_mirror(c, slot);
        }
        self.queue.write_buffer(&self.mirror_u.buf, 0, &self.scratch);
        let size = (self.config.width, self.config.height);
        let format = self.config.format;
        let device = &self.device;
        let (_, view) = self.mirror_tex.get_or_insert_with(|| {
            let t = mirror_texture(device, size, format);
            let v = t.create_view(&Default::default());
            (t, v)
        });
        self.mirror_bg = Some(self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("spiegel"),
            layout: &self.mirror_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &self.mirror_u.buf,
                        offset: 0,
                        size: wgpu::BufferSize::new(MIRROR_BYTES as u64),
                    }),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&self.linear),
                },
            ],
        }));
    }

    /// Kopiert den Streifen `[src_top, ground]` über die Breite des Aufrufs von der Fläche in die Zwischentextur (an
    /// denselben Platz, damit der Shader mit Anteilen des Fensters abtastet).
    pub(super) fn copy_strip(&self, enc: &mut wgpu::CommandEncoder, surface: &wgpu::Texture, call: &crate::MirrorCall) {
        let Some((tex, _)) = &self.mirror_tex else {
            return;
        };
        // Größe der geholten Fläche (kann nach einem Fensterwechsel von `config` abweichen): nie darüber hinaus kopieren
        let (sw, sh) = (
            self.config.width.min(surface.width()).min(tex.width()),
            self.config.height.min(surface.height()).min(tex.height()),
        );
        let (kx, ky) = (sw as f32 / LOGICAL.0, sh as f32 / LOGICAL.1);
        let (d, u) = (call.dst, &call.u);
        let x0 = (d.x * kx).floor().clamp(0.0, sw as f32) as u32;
        let x1 = ((d.x + d.w) * kx).ceil().clamp(0.0, sw as f32) as u32;
        let y0 = (u.src_top * ky).floor().clamp(0.0, sh as f32) as u32;
        let y1 = (u.ground * ky).ceil().clamp(0.0, sh as f32) as u32;
        if x1 <= x0 || y1 <= y0 {
            return;
        }
        let origin = wgpu::Origin3d { x: x0, y: y0, z: 0 };
        let info = |texture| wgpu::TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin,
            aspect: wgpu::TextureAspect::All,
        };
        enc.copy_texture_to_texture(
            info(surface),
            info(tex),
            wgpu::Extent3d {
                width: x1 - x0,
                height: y1 - y0,
                depth_or_array_layers: 1,
            },
        );
    }

    /// Zeichnet einen Spiegel-Aufruf (Pipeline, Zwischentextur und Uniforms sind vorhanden).
    pub(super) fn draw_mirror(&self, p: &mut wgpu::RenderPass<'_>, first: u32, (x, y, w, h): (u32, u32, u32, u32)) {
        let (Some(pipe), Some(bg)) = (&self.mirror_pipe, &self.mirror_bg) else {
            return;
        };
        p.set_pipeline(pipe);
        p.set_scissor_rect(x, y, w, h);
        p.set_bind_group(0, Some(bg), &[(u64::from(first) * self.mirror_stride) as u32]);
        p.draw(0..6, 0..1);
    }
}
