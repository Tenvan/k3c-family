//! Hilfen für die Tools der Apps (Port von `ReadOnly` und `AddImage` aus `mcpkit/tool.go`); registriert werden sie
//! mit den Makros von `rmcp`.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD;
use rmcp::model::{CallToolResult, ContentBlock, ToolAnnotations};

/// Annotations eines Tools, das nur liest.
pub fn read_only() -> ToolAnnotations {
    ToolAnnotations::new()
        .read_only(true)
        .idempotent(true)
        .open_world(false)
}

/// Ergebnis aus einem PNG und Text.
pub fn image_result(png: &[u8], text: &str) -> CallToolResult {
    let mut result = CallToolResult::success(vec![
        ContentBlock::image(STANDARD.encode(png), "image/png"),
        ContentBlock::text(text),
    ]);
    // Go lässt isError bei Erfolg weg (`omitempty`).
    result.is_error = None;
    result
}
