//! Tests des GPU-Backends ohne Adapter (Präsentationsmodus, Flächenformat, Ausschnitt).

use super::*;

#[test]
fn present_modus_nach_wunsch_und_angebot() {
    use wgpu::PresentMode as M;
    let all = [M::Fifo, M::Mailbox, M::Immediate];
    assert_eq!(pick_present_mode(false, &all), M::Fifo);
    assert_eq!(pick_present_mode(true, &all), M::Immediate);
    assert_eq!(pick_present_mode(true, &[M::Fifo, M::Mailbox]), M::Mailbox);
    assert_eq!(pick_present_mode(true, &[M::Fifo]), M::Fifo);
    assert_eq!(pick_present_mode(true, &[]), M::Fifo);
}

/// B-1128: nie eine sRGB-Fläche, wenn eine lineare angeboten wird (sonst blasse Farben); sonst das erste Format.
#[test]
fn flaeche_ohne_srgb_kodierung() {
    use wgpu::TextureFormat as F;
    assert_eq!(
        pick_surface_format(&[F::Bgra8UnormSrgb, F::Bgra8Unorm]),
        Some(F::Bgra8Unorm)
    );
    assert_eq!(
        pick_surface_format(&[F::Rgba8UnormSrgb, F::Rgba16Float]),
        Some(F::Rgba16Float)
    );
    assert_eq!(
        pick_surface_format(&[F::Bgra8Unorm, F::Bgra8UnormSrgb]),
        Some(F::Bgra8Unorm)
    );
    assert_eq!(pick_surface_format(&[F::Bgra8UnormSrgb]), Some(F::Bgra8UnormSrgb));
    assert_eq!(pick_surface_format(&[]), None);
}

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
