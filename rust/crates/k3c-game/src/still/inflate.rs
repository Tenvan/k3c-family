//! Kleiner gzip-/Inflate-Leser (RFC 1951/1952) für die Zustände des Standbilds: Die Zustände liegen als
//! `.state.json.gz` im Repo, und eine Abhängigkeit für das Entpacken gibt es im Workspace nicht (neue Crates nur mit
//! Freigabe). Nur Lesen, Prüfsumme und Länge aus dem Trailer werden nicht ausgewertet (das Lesen scheitert an
//! beschädigten Daten ohnehin).

/// Entpackt einen gzip-Strom (ein Mitglied).
pub fn gunzip(raw: &[u8]) -> Result<Vec<u8>, String> {
    if raw.len() < 18 || raw[0] != 0x1f || raw[1] != 0x8b || raw[2] != 8 {
        return Err("kein gzip".to_string());
    }
    let flags = raw[3];
    let mut p = 10;
    if flags & 4 != 0 {
        let n = usize::from(*raw.get(p).ok_or("gzip: abgeschnitten")?)
            | usize::from(*raw.get(p + 1).ok_or("gzip: abgeschnitten")?) << 8;
        p += 2 + n;
    }
    for bit in [8, 16] {
        if flags & bit != 0 {
            while *raw.get(p).ok_or("gzip: abgeschnitten")? != 0 {
                p += 1;
            }
            p += 1;
        }
    }
    if flags & 2 != 0 {
        p += 2;
    }
    inflate(raw.get(p..).ok_or("gzip: abgeschnitten")?)
}

struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
    bit: u32,
}

impl Bits<'_> {
    fn one(&mut self) -> Result<u32, String> {
        let b = *self.data.get(self.pos).ok_or("inflate: abgeschnitten")?;
        let v = u32::from(b >> self.bit) & 1;
        self.bit += 1;
        if self.bit == 8 {
            self.bit = 0;
            self.pos += 1;
        }
        Ok(v)
    }

    fn bits(&mut self, n: u32) -> Result<u32, String> {
        let mut v = 0;
        for i in 0..n {
            v |= self.one()? << i;
        }
        Ok(v)
    }
}

/// Huffman-Tabelle nach Längen (kanonisch): Anzahl je Länge und Symbole nach Code sortiert.
struct Huff {
    count: [u16; 16],
    symbol: Vec<u16>,
}

fn huff(lengths: &[u8]) -> Huff {
    let mut count = [0u16; 16];
    for &l in lengths {
        count[usize::from(l)] += 1;
    }
    count[0] = 0;
    let mut offs = [0u16; 16];
    for i in 1..16 {
        offs[i] = offs[i - 1] + count[i - 1];
    }
    let mut symbol = vec![0u16; lengths.len()];
    for (s, &l) in lengths.iter().enumerate() {
        if l != 0 {
            symbol[usize::from(offs[usize::from(l)])] = s as u16;
            offs[usize::from(l)] += 1;
        }
    }
    Huff { count, symbol }
}

fn decode(b: &mut Bits, h: &Huff) -> Result<usize, String> {
    let (mut code, mut first, mut index) = (0i32, 0i32, 0i32);
    for len in 1..16 {
        code |= b.one()? as i32;
        let n = i32::from(h.count[len]);
        if code - n < first {
            return Ok(usize::from(h.symbol[(index + (code - first)) as usize]));
        }
        index += n;
        first = (first + n) << 1;
        code <<= 1;
    }
    Err("inflate: ungültiger Code".to_string())
}

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145,
    8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];

fn codes(b: &mut Bits, out: &mut Vec<u8>, lit: &Huff, dist: &Huff) -> Result<(), String> {
    loop {
        let s = decode(b, lit)?;
        match s {
            0..=255 => out.push(s as u8),
            256 => return Ok(()),
            _ => {
                let i = s - 257;
                if i >= 29 {
                    return Err("inflate: ungültige Länge".to_string());
                }
                let len = usize::from(LEN_BASE[i]) + b.bits(u32::from(LEN_EXTRA[i]))? as usize;
                let d = decode(b, dist)?;
                if d >= 30 {
                    return Err("inflate: ungültiger Abstand".to_string());
                }
                let back = usize::from(DIST_BASE[d]) + b.bits(u32::from(DIST_EXTRA[d]))? as usize;
                if back > out.len() {
                    return Err("inflate: Abstand zu groß".to_string());
                }
                for _ in 0..len {
                    out.push(out[out.len() - back]);
                }
            }
        }
    }
}

fn dynamic(b: &mut Bits) -> Result<(Huff, Huff), String> {
    const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];
    let nlen = b.bits(5)? as usize + 257;
    let ndist = b.bits(5)? as usize + 1;
    let ncode = b.bits(4)? as usize + 4;
    let mut cl = [0u8; 19];
    for &o in ORDER.iter().take(ncode) {
        cl[o] = b.bits(3)? as u8;
    }
    let clh = huff(&cl);
    let mut lengths = vec![0u8; nlen + ndist];
    let mut i = 0;
    while i < nlen + ndist {
        let s = decode(b, &clh)?;
        let (val, rep) = match s {
            0..=15 => (s as u8, 1),
            16 => {
                let prev = *lengths
                    .get(i.wrapping_sub(1))
                    .ok_or("inflate: Wiederholung ohne Wert")?;
                (prev, 3 + b.bits(2)? as usize)
            }
            17 => (0, 3 + b.bits(3)? as usize),
            _ => (0, 11 + b.bits(7)? as usize),
        };
        if i + rep > nlen + ndist {
            return Err("inflate: zu viele Längen".to_string());
        }
        lengths[i..i + rep].fill(val);
        i += rep;
    }
    Ok((huff(&lengths[..nlen]), huff(&lengths[nlen..])))
}

/// Entpackt einen rohen Deflate-Strom.
pub fn inflate(data: &[u8]) -> Result<Vec<u8>, String> {
    let mut b = Bits { data, pos: 0, bit: 0 };
    let mut out = Vec::new();
    loop {
        let last = b.one()?;
        match b.bits(2)? {
            0 => {
                if b.bit != 0 {
                    b.bit = 0;
                    b.pos += 1;
                }
                let h = b.data.get(b.pos..b.pos + 4).ok_or("inflate: abgeschnitten")?;
                let n = usize::from(h[0]) | usize::from(h[1]) << 8;
                b.pos += 4;
                out.extend_from_slice(b.data.get(b.pos..b.pos + n).ok_or("inflate: abgeschnitten")?);
                b.pos += n;
            }
            1 => {
                let mut l = [8u8; 288];
                l[144..256].fill(9);
                l[256..280].fill(7);
                codes(&mut b, &mut out, &huff(&l), &huff(&[5u8; 30]))?;
            }
            2 => {
                let (lit, dist) = dynamic(&mut b)?;
                codes(&mut b, &mut out, &lit, &dist)?;
            }
            _ => return Err("inflate: ungültiger Block".to_string()),
        }
        if last == 1 {
            return Ok(out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entpackt_gespeicherten_und_festen_block() {
        // gzip von "hallo" (Go, BestCompression): feste Huffman-Codes
        let gz = [
            0x1f, 0x8b, 8, 0, 0, 0, 0, 0, 2, 255, 0xcb, 0x48, 0xcc, 0xc9, 0xc9, 0x07, 0x00, 0x86, 0xa6, 0x10, 0x36, 5,
            0, 0, 0,
        ];
        assert_eq!(gunzip(&gz).unwrap(), b"hallo");
        // gespeicherter Block
        let raw = [1, 3, 0, 0xfc, 0xff, b'a', b'b', b'c'];
        assert_eq!(inflate(&raw).unwrap(), b"abc");
        assert!(gunzip(&[1, 2, 3]).is_err());
    }
}
