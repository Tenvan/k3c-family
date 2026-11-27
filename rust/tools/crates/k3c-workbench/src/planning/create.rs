//! `plan_get`. Anlegen (`Create`) folgt in RU69.10.

use std::path::Path;

use super::relations::{reverse_lines, with_reverse};
use super::store::{Ref, read_text, resolve};

/// Pfad und Inhalt eines Dokuments.
pub fn get(root: &Path, id: &str) -> Result<String, String> {
    get_ref(root, &resolve(root, id)?)
}

/// Liest das aufgelöste Dokument wie [`get`].
pub fn get_ref(root: &Path, r: &Ref) -> Result<String, String> {
    let (mut text, _) = read_text(root, &r.rel)?;
    if r.kind == "ticket" {
        text = with_reverse(text, reverse_lines(root, &r.id));
    }
    Ok(format!("docs/{}\n\n{text}", r.rel))
}
