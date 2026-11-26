use std::time::Duration;

use rmcp::handler::server::router::tool::ToolRouter;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::mcp::{Options, Server};

/// Schickt einen POST und liest die Antwort bis zum Abschluss-Chunk; liefert Kopf (klein geschrieben) und Rohkörper.
async fn post(connection: &mut TcpStream, session: &str, body: &str) -> (String, String) {
    let mut head = String::from(
        "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\n\
         Accept: application/json, text/event-stream\r\n",
    );
    if !session.is_empty() {
        head += &format!("Mcp-Session-Id: {session}\r\nMcp-Protocol-Version: 2025-06-18\r\n");
    }
    let request = format!("{head}Content-Length: {}\r\n\r\n{body}", body.len());
    connection.write_all(request.as_bytes()).await.unwrap();
    let mut got = Vec::new();
    let mut buffer = [0_u8; 4096];
    loop {
        let text = String::from_utf8_lossy(&got).into_owned();
        if let Some((head, body)) = text.split_once("\r\n\r\n")
            && body.ends_with("0\r\n\r\n")
        {
            return (head.to_lowercase(), body.to_owned());
        }
        let read = tokio::time::timeout(Duration::from_secs(5), connection.read(&mut buffer)).await;
        let n = read
            .unwrap_or_else(|_| panic!("Body endet nicht sauber: {text:?}"))
            .unwrap();
        assert!(n > 0, "Verbindung zu: {text:?}");
        got.extend_from_slice(&buffer[..n]);
    }
}

/// Regression: Das 202 auf `notifications/initialized` muss von Anfang an chunked und abgeschlossen sein, sonst hängt
/// Claude Code (siehe `chunked.rs`).
#[tokio::test(flavor = "multi_thread")]
async fn benachrichtigung_antwort_ist_chunked_abgeschlossen() {
    let options = Options {
        name: "test".to_owned(),
        version: "test".to_owned(),
        ..Options::default()
    };
    let server = Server::new(options, (), ToolRouter::new(), None);
    server.start().await.unwrap();
    let url = server.url();
    let address = url.trim_start_matches("http://").trim_end_matches("/mcp");
    let mut connection = TcpStream::connect(address).await.unwrap();
    let initialize = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-06-18","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}"#;
    let (head, _) = post(&mut connection, "", initialize).await;
    let session = head
        .lines()
        .find_map(|line| line.strip_prefix("mcp-session-id:"))
        .expect("Session-Kopf")
        .trim()
        .to_owned();
    let initialized = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
    let (head, body) = post(&mut connection, &session, initialized).await;
    assert!(head.starts_with("http/1.1 202"), "{head}");
    assert!(head.contains("transfer-encoding: chunked"), "{head}");
    assert!(!head.contains("content-length"), "{head}");
    assert_eq!(body, "0\r\n\r\n");
    // Auch eine Fehlerantwort (unbekannte Session) kommt chunked.
    let (head, _) = post(&mut connection, "gibt-es-nicht", initialized).await;
    assert!(head.starts_with("http/1.1 4"), "{head}");
    assert!(head.contains("transfer-encoding: chunked"), "{head}");
    assert!(!head.contains("content-length"), "{head}");
    drop(connection);
    server.stop().await.unwrap();
}
