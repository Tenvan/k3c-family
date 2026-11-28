//! `Range` wie Go `http.parseRange`, beschränkt auf eine Spanne.

/// Ausschnitt einer Datei.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Span {
    pub start: u64,
    pub len: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum RangeError {
    /// Nicht lesbar.
    Invalid,
    /// Beginnt hinter dem Ende der Datei.
    NoOverlap,
}

impl std::fmt::Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            RangeError::Invalid => "invalid range",
            RangeError::NoOverlap => "invalid range: failed to overlap",
        })
    }
}

fn number(text: &str) -> Result<u64, RangeError> {
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err(RangeError::Invalid);
    }
    text.parse().map_err(|_| RangeError::Invalid)
}

/// Eine Angabe `a-b`, `a-` oder `-n`; `None`, wenn sie hinter dem Ende beginnt.
fn one(spec: &str, size: u64) -> Result<Option<Span>, RangeError> {
    let (first, last) = spec.split_once('-').ok_or(RangeError::Invalid)?;
    let (first, last) = (first.trim(), last.trim());
    if first.is_empty() {
        let len = number(last)?.min(size);
        return Ok(Some(Span { start: size - len, len }));
    }
    let start = number(first)?;
    if start >= size {
        return Ok(None);
    }
    let end = if last.is_empty() {
        size - 1
    } else {
        let end = number(last)?;
        if end < start {
            return Err(RangeError::Invalid);
        }
        end.min(size - 1)
    };
    Ok(Some(Span {
        start,
        len: end - start + 1,
    }))
}

/// Die Spanne aus dem Wert von `Range`; `None` heißt die ganze Datei (kein Wert, mehrere Spannen, leere Datei).
pub(super) fn parse(header: &str, size: u64) -> Result<Option<Span>, RangeError> {
    if header.is_empty() {
        return Ok(None);
    }
    let specs = header.strip_prefix("bytes=").ok_or(RangeError::Invalid)?;
    let mut spans = Vec::new();
    let mut missed = false;
    for spec in specs.split(',').map(str::trim).filter(|s| !s.is_empty()) {
        match one(spec, size)? {
            Some(span) => spans.push(span),
            None => missed = true,
        }
    }
    match spans.as_slice() {
        // Wie Go: eine leere Datei kommt trotz Bereich ganz.
        [] if missed && size == 0 => Ok(None),
        [] if missed => Err(RangeError::NoOverlap),
        [span] => Ok(Some(*span)),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn span(start: u64, len: u64) -> Result<Option<Span>, RangeError> {
        Ok(Some(Span { start, len }))
    }

    #[test]
    fn formen_wie_go() {
        assert_eq!(parse("", 10), Ok(None));
        assert_eq!(parse("bytes=2-4", 10), span(2, 3));
        assert_eq!(parse("bytes=2-", 10), span(2, 8));
        assert_eq!(parse("bytes=-3", 10), span(7, 3));
        assert_eq!(parse("bytes=-30", 10), span(0, 10));
        assert_eq!(parse("bytes=8-99", 10), span(8, 2));
        assert_eq!(parse("bytes=10-", 10), Err(RangeError::NoOverlap));
        assert_eq!(parse("bytes=0-", 0), Ok(None));
        assert_eq!(parse("bytes=0-1,3-4", 10), Ok(None), "mehrere Spannen: ganze Datei");
        for bad in [
            "items=1-2",
            "bytes=4-2",
            "bytes=a-",
            "bytes=-",
            "bytes=1",
            "bytes=+1-2",
            "bytes=1--2",
        ] {
            assert_eq!(parse(bad, 10), Err(RangeError::Invalid), "{bad}");
        }
    }
}
