//! Nur Windows ist Ziel; anderswo gibt es keine Auskunft.

use super::{Conn, Usage};

pub(super) fn listeners() -> Vec<Conn> {
    Vec::new()
}

pub(super) fn usage(_pid: u32) -> Result<Usage, String> {
    Err("Prozesse messen geht nur unter Windows".to_owned())
}

pub(super) fn parent(_pid: u32) -> Option<u32> {
    None
}

pub(super) fn image(_pid: u32) -> Option<String> {
    None
}
