//! Zeilen-Writer (Port von `workkit/console/writer.go`).

use std::io;
use std::sync::{Arc, Mutex, PoisonError};

type Emit = Box<dyn FnMut(String) + Send>;

struct Inner {
    buf: Vec<u8>,
    emit: Emit,
}

/// Zerlegt einen Ausgabestrom in Zeilen und gibt jede an `emit` weiter; lange Zeilen bleiben ganz, weil nichts eine
/// feste Zeilenlänge annimmt. Klone teilen Puffer und Ziel (z. B. Konsolen-Senke und Besitzer, der am Ende
/// [`LineWriter::flush_line`] ruft).
#[derive(Clone)]
pub struct LineWriter {
    inner: Arc<Mutex<Inner>>,
}

impl LineWriter {
    /// Writer, der jede vollständige Zeile an `emit` gibt.
    pub fn new(emit: impl FnMut(String) + Send + 'static) -> LineWriter {
        LineWriter {
            inner: Arc::new(Mutex::new(Inner {
                buf: Vec::new(),
                emit: Box::new(emit),
            })),
        }
    }

    /// Gibt eine angefangene letzte Zeile weiter (Go `Flush`).
    pub fn flush_line(&self) {
        let mut g = self.inner.lock().unwrap_or_else(PoisonError::into_inner);
        if !g.buf.is_empty() {
            let line = clean(&g.buf);
            g.buf.clear();
            (g.emit)(line);
        }
    }
}

impl io::Write for LineWriter {
    /// Nimmt beliebige Stücke an; nur vollständige Zeilen gehen weiter.
    fn write(&mut self, p: &[u8]) -> io::Result<usize> {
        let mut g = self.inner.lock().unwrap_or_else(PoisonError::into_inner);
        let inner = &mut *g;
        inner.buf.extend_from_slice(p);
        let mut start = 0;
        while let Some(i) = inner.buf[start..].iter().position(|&b| b == b'\n') {
            let line = clean(&inner.buf[start..start + i]);
            (inner.emit)(line);
            start += i + 1;
        }
        inner.buf.drain(..start);
        Ok(p.len())
    }

    /// Hält angefangene Zeilen zurück; die letzte gibt erst [`LineWriter::flush_line`] weiter.
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Entfernt das Zeilenende und macht den Text zu gültigem UTF-8 (Windows-Werkzeuge schreiben teils Codepage); jede
/// Folge ungültiger Bytes wird zu einem `U+FFFD` wie Go `strings.ToValidUTF8`.
pub(crate) fn clean(b: &[u8]) -> String {
    let end = b.iter().rposition(|&c| c != b'\r').map_or(0, |i| i + 1);
    let mut out = String::with_capacity(end);
    let mut in_invalid = false;
    for chunk in b[..end].utf8_chunks() {
        if !chunk.valid().is_empty() {
            out.push_str(chunk.valid());
            in_invalid = false;
        }
        if !chunk.invalid().is_empty() {
            if !in_invalid {
                out.push('\u{FFFD}');
            }
            in_invalid = true;
        }
    }
    out
}
