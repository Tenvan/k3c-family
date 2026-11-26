//! Hilfen der Unit-Tests; die Integrationstests binden dieselben Dateien aus `tests/support/` ein.

#[path = "../tests/support/mcp_client.rs"]
pub mod mcp_client;
#[path = "../tests/support/tmp.rs"]
mod tmp;

pub use tmp::tmp_dir;
