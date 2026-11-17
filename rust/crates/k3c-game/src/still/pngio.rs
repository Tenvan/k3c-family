//! PNG schreiben und lesen für das Standbild (8 Bit, RGB oder RGBA, gerade Farben).

use std::io::BufWriter;
use std::path::Path;

use super::raster::Image;

/// Schreibt das Bild als RGBA-PNG.
pub fn write_png(path: &Path, img: &Image) -> Result<(), String> {
    let fail = |e: &dyn std::fmt::Display| format!("{}: {e}", path.display());
    if let Some(dir) = path.parent().filter(|d| !d.as_os_str().is_empty()) {
        std::fs::create_dir_all(dir).map_err(|e| fail(&e))?;
    }
    let file = std::fs::File::create(path).map_err(|e| fail(&e))?;
    let mut enc = png::Encoder::new(BufWriter::new(file), img.width, img.height);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    let mut w = enc.write_header().map_err(|e| fail(&e))?;
    w.write_image_data(&img.pix).map_err(|e| fail(&e))?;
    w.finish().map_err(|e| fail(&e))
}

/// Liest ein PNG (RGB oder RGBA, 8 Bit) als gerades RGBA; ein RGB-Bild bekommt Alpha 255.
pub fn read_png(path: &Path) -> Result<Image, String> {
    let fail = |e: &dyn std::fmt::Display| format!("{}: {e}", path.display());
    let file = std::fs::File::open(path).map_err(|e| fail(&e))?;
    let mut r = png::Decoder::new(std::io::BufReader::new(file))
        .read_info()
        .map_err(|e| fail(&e))?;
    let mut buf = vec![0u8; r.output_buffer_size().ok_or_else(|| fail(&"Größe"))?];
    let info = r.next_frame(&mut buf).map_err(|e| fail(&e))?;
    buf.truncate(info.buffer_size());
    let pix = match (info.color_type, info.bit_depth) {
        (png::ColorType::Rgba, png::BitDepth::Eight) => buf,
        (png::ColorType::Rgb, png::BitDepth::Eight) => buf
            .as_chunks::<3>()
            .0
            .iter()
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
        (c, d) => return Err(fail(&format!("Farbtyp {c:?} {d:?} nicht unterstützt"))),
    };
    Ok(Image {
        width: info.width,
        height: info.height,
        pix,
    })
}
