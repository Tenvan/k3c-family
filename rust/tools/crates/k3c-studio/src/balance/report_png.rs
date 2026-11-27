//! Zuschnitt der Grafiken für den HTML-Bericht (Go `image/png`). Das Studio hat keine PNG-Crate, darum liest dies nur,
//! was der Papierschnitt-Renderer schreibt (8 Bit RGBA ohne Interlacing); anderes ergibt `None`, also einen
//! Platzhalter im Bericht. Geschrieben wird unkomprimiert: gleiche Pixel wie Go, andere und größere Bytes.
//! Mit einer freigegebenen Abhängigkeit `png` entfallen diese Datei und `report_inflate.rs`.

use base64::Engine as _;

const SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
/// Obergrenze der entpackten Pixel (weit über jedem Sprite), damit eine kaputte Datei keinen Speicher frisst.
const MAX_BYTES: usize = 64 << 20;

pub(crate) struct Rgba {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

/// Das erste Feld der Größe `frame` (oder das ganze Bild), ohne transparenten Rand, als data-URI.
pub(crate) fn cropped_data_uri(raw: &[u8], frame: Option<(u32, u32)>) -> Option<String> {
    let img = decode(raw)?;
    let (w, h) = frame.map_or((img.width, img.height), |(w, h)| {
        (img.width.min(w as usize), img.height.min(h as usize))
    });
    let png = encode(&trimmed(&img, w, h));
    let text = base64::engine::general_purpose::STANDARD.encode(png);
    Some(format!("data:image/png;base64,{text}"))
}

/// Schneidet das Feld `w`×`h` oben links auf die nicht transparenten Pixel zu (ganz transparent: das Feld).
fn trimmed(img: &Rgba, w: usize, h: usize) -> Rgba {
    let opaque = |x: usize, y: usize| img.pixels[(y * img.width + x) * 4 + 3] > 0;
    let columns: Vec<usize> = (0..w).filter(|&x| (0..h).any(|y| opaque(x, y))).collect();
    let rows: Vec<usize> = (0..h).filter(|&y| (0..w).any(|x| opaque(x, y))).collect();
    let (x0, x1) = columns.first().zip(columns.last()).map_or((0, w), |(a, b)| (*a, b + 1));
    let (y0, y1) = rows.first().zip(rows.last()).map_or((0, h), |(a, b)| (*a, b + 1));
    let pixels = (y0..y1)
        .flat_map(|y| &img.pixels[(y * img.width + x0) * 4..(y * img.width + x1) * 4])
        .copied()
        .collect();
    Rgba {
        width: x1 - x0,
        height: y1 - y0,
        pixels,
    }
}

fn be32(b: &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes(b.get(..4)?.try_into().ok()?))
}

pub(crate) fn decode(raw: &[u8]) -> Option<Rgba> {
    let mut rest = raw.strip_prefix(SIGNATURE)?;
    let (mut width, mut height, mut data) = (0usize, 0usize, Vec::new());
    loop {
        let len = be32(rest)? as usize;
        let kind = rest.get(4..8)?;
        let body = rest.get(8..8usize.checked_add(len)?)?;
        match kind {
            b"IHDR" => {
                width = be32(body)? as usize;
                height = be32(body.get(4..)?)? as usize;
                // Tiefe 8, Farbtyp RGBA, Kompression, Filter und Interlacing 0
                if body.get(8..13)? != [8, 6, 0, 0, 0] {
                    return None;
                }
            }
            b"IDAT" => data.extend_from_slice(body),
            b"IEND" => break,
            _ => {}
        }
        rest = rest.get(12usize.checked_add(len)?..)?;
    }
    let stride = width.checked_mul(4)?;
    let size = stride.checked_add(1)?.checked_mul(height)?;
    if width == 0 || height == 0 || size > MAX_BYTES {
        return None;
    }
    let lines = super::report_inflate::zlib(&data, size)?;
    Some(Rgba {
        width,
        height,
        pixels: unfilter(&lines, stride)?,
    })
}

/// Macht die Zeilenfilter rückgängig; `lines` enthält je Zeile ein Filter-Byte und `stride` Bytes.
fn unfilter(lines: &[u8], stride: usize) -> Option<Vec<u8>> {
    let mut out: Vec<u8> = Vec::new();
    for line in lines.chunks_exact(stride + 1) {
        let start = out.len();
        for (i, &byte) in line[1..].iter().enumerate() {
            let left = if i >= 4 { out[start + i - 4] } else { 0 };
            let up = if start > 0 { out[start - stride + i] } else { 0 };
            let corner = if start > 0 && i >= 4 {
                out[start - stride + i - 4]
            } else {
                0
            };
            let guess = match line[0] {
                0 => 0,
                1 => left,
                2 => up,
                3 => u8::try_from((u16::from(left) + u16::from(up)) / 2).ok()?,
                4 => paeth(left, up, corner),
                _ => return None,
            };
            out.push(byte.wrapping_add(guess));
        }
    }
    Some(out)
}

fn paeth(a: u8, b: u8, c: u8) -> u8 {
    let p = i16::from(a) + i16::from(b) - i16::from(c);
    let distance = |v: u8| (p - i16::from(v)).abs();
    if distance(a) <= distance(b) && distance(a) <= distance(c) {
        a
    } else if distance(b) <= distance(c) {
        b
    } else {
        c
    }
}

fn crc32(data: &[u8]) -> u32 {
    !data.iter().fold(!0u32, |crc, &byte| {
        (0..8).fold(crc ^ u32::from(byte), |c, _| {
            (c >> 1) ^ (0xEDB8_8320 & (c & 1).wrapping_neg())
        })
    })
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], body: &[u8]) {
    out.extend_from_slice(&(body.len() as u32).to_be_bytes());
    let start = out.len();
    out.extend_from_slice(kind);
    out.extend_from_slice(body);
    let crc = crc32(&out[start..]);
    out.extend_from_slice(&crc.to_be_bytes());
}

/// PNG mit Filter 0 und unkomprimierten Deflate-Blöcken.
pub(crate) fn encode(img: &Rgba) -> Vec<u8> {
    let mut lines = Vec::new();
    for line in img.pixels.chunks_exact(img.width.max(1) * 4) {
        lines.push(0);
        lines.extend_from_slice(line);
    }
    let mut header = Vec::new();
    header.extend_from_slice(&(img.width as u32).to_be_bytes());
    header.extend_from_slice(&(img.height as u32).to_be_bytes());
    header.extend_from_slice(&[8, 6, 0, 0, 0]);
    let mut out = SIGNATURE.to_vec();
    chunk(&mut out, b"IHDR", &header);
    chunk(&mut out, b"IDAT", &super::report_inflate::zlib_stored(&lines));
    chunk(&mut out, b"IEND", &[]);
    out
}
