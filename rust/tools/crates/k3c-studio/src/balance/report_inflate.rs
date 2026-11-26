//! zlib (RFC 1950/1951) für [`super::report_png`]: Entpacken nach dem Verfahren von `puff`, Packen nur in
//! unkomprimierten Blöcken. Jeder Fehler ist `None`; die Adler-32-Prüfsumme deckt falsch Entpacktes auf.

const LENGTH_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258,
];
const LENGTH_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145,
    8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13,
];
/// Reihenfolge der Längen des Längen-Codes im dynamischen Block.
const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

struct Bits<'a> {
    data: &'a [u8],
    /// Position in Bits.
    pos: usize,
}

impl Bits<'_> {
    fn take(&mut self, n: u8) -> Option<usize> {
        let mut value = 0;
        for i in 0..n {
            let byte = self.data.get(self.pos / 8)?;
            value |= usize::from((byte >> (self.pos % 8)) & 1) << i;
            self.pos += 1;
        }
        Some(value)
    }
}

/// Kanonischer Huffman-Code: Zahl der Codes je Länge und die Zeichen in Code-Reihenfolge.
struct Code {
    count: [usize; 16],
    symbols: Vec<usize>,
}

impl Code {
    fn new(lengths: &[u8]) -> Self {
        let mut count = [0; 16];
        let mut symbols = Vec::new();
        for len in 1..16u8 {
            for (symbol, _) in lengths.iter().enumerate().filter(|(_, l)| **l == len) {
                count[usize::from(len)] += 1;
                symbols.push(symbol);
            }
        }
        Code { count, symbols }
    }

    fn read(&self, bits: &mut Bits<'_>) -> Option<usize> {
        let (mut code, mut first, mut index) = (0, 0, 0);
        for &count in &self.count[1..] {
            code |= bits.take(1)?;
            if code < first + count {
                return self.symbols.get(index + code - first).copied();
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
        }
        None
    }
}

fn fixed_codes() -> (Code, Code) {
    let lengths: Vec<u8> = (0..288)
        .map(|i| match i {
            0..=143 | 280.. => 8,
            144..=255 => 9,
            _ => 7,
        })
        .collect();
    (Code::new(&lengths), Code::new(&[5; 30]))
}

fn dynamic_codes(bits: &mut Bits<'_>) -> Option<(Code, Code)> {
    let literals = bits.take(5)? + 257;
    let distances = bits.take(5)? + 1;
    let used = bits.take(4)? + 4;
    let mut meta = [0u8; 19];
    for &i in &ORDER[..used] {
        meta[i] = bits.take(3)? as u8;
    }
    let meta = Code::new(&meta);
    let mut lengths: Vec<u8> = Vec::new();
    while lengths.len() < literals + distances {
        let (value, repeat) = match meta.read(bits)? {
            symbol @ 0..=15 => (symbol as u8, 1),
            16 => (*lengths.last()?, 3 + bits.take(2)?),
            17 => (0, 3 + bits.take(3)?),
            _ => (0, 11 + bits.take(7)?),
        };
        lengths.extend(std::iter::repeat_n(value, repeat));
    }
    if lengths.len() != literals + distances {
        return None;
    }
    Some((Code::new(&lengths[..literals]), Code::new(&lengths[literals..])))
}

fn block(bits: &mut Bits<'_>, out: &mut Vec<u8>, size: usize, codes: &(Code, Code)) -> Option<()> {
    loop {
        let symbol = codes.0.read(bits)?;
        if symbol == 256 {
            return Some(());
        }
        if symbol < 256 {
            out.push(symbol as u8);
        } else {
            let i = symbol - 257;
            let len = usize::from(*LENGTH_BASE.get(i)?) + bits.take(LENGTH_EXTRA[i])?;
            let d = codes.1.read(bits)?;
            let dist = usize::from(*DIST_BASE.get(d)?) + bits.take(DIST_EXTRA[d])?;
            let start = out.len().checked_sub(dist)?;
            for k in 0..len {
                out.push(out[start + k]);
            }
        }
        if out.len() > size {
            return None;
        }
    }
}

fn stored(bits: &mut Bits<'_>, out: &mut Vec<u8>) -> Option<()> {
    let at = bits.pos.div_ceil(8);
    let head = bits.data.get(at..at + 4)?;
    let len = usize::from(u16::from_le_bytes([head[0], head[1]]));
    out.extend_from_slice(bits.data.get(at + 4..at + 4 + len)?);
    bits.pos = (at + 4 + len) * 8;
    Some(())
}

fn adler32(data: &[u8]) -> u32 {
    let (a, b) = data.iter().fold((1u32, 0u32), |(a, b), &byte| {
        let a = (a + u32::from(byte)) % 65521;
        (a, (b + a) % 65521)
    });
    (b << 16) | a
}

/// Entpackt einen zlib-Strom, der genau `size` Bytes ergeben muss.
pub(crate) fn zlib(data: &[u8], size: usize) -> Option<Vec<u8>> {
    let head = data.get(..2)?;
    // Deflate, Prüfbits, kein Wörterbuch
    if head[0] & 0x0f != 8 || u16::from_be_bytes([head[0], head[1]]) % 31 != 0 || head[1] & 0x20 != 0 {
        return None;
    }
    let mut bits = Bits {
        data: &data[2..],
        pos: 0,
    };
    let mut out = Vec::new();
    loop {
        let last = bits.take(1)? == 1;
        match bits.take(2)? {
            0 => stored(&mut bits, &mut out)?,
            1 => block(&mut bits, &mut out, size, &fixed_codes())?,
            2 => {
                let codes = dynamic_codes(&mut bits)?;
                block(&mut bits, &mut out, size, &codes)?;
            }
            _ => return None,
        }
        if out.len() > size {
            return None;
        }
        if last {
            break;
        }
    }
    let at = bits.pos.div_ceil(8);
    let sum = u32::from_be_bytes(bits.data.get(at..at + 4)?.try_into().ok()?);
    (out.len() == size && adler32(&out) == sum).then_some(out)
}

/// Verpackt `data` unkomprimiert als zlib-Strom.
pub(crate) fn zlib_stored(data: &[u8]) -> Vec<u8> {
    let mut out = vec![0x78, 0x01];
    let mut blocks = data.chunks(usize::from(u16::MAX)).peekable();
    if blocks.peek().is_none() {
        out.extend_from_slice(&[1, 0, 0, 0xff, 0xff]);
    }
    while let Some(part) = blocks.next() {
        let len = part.len() as u16;
        out.push(u8::from(blocks.peek().is_none()));
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(part);
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}
