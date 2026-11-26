//! Port von `tools_gamedata_test.go` (`TestSpieldatenTools`): ohne Ordner keine Berichte, `..` abgelehnt.

use serde_json::json;

use super::super::{Config, Server};
use crate::mcpsrv::kit::Logger;
use crate::test_helpers::{mcp_client, tmp_dir};

#[tokio::test(flavor = "multi_thread")]
async fn spieldaten_tools() {
    let root = tmp_dir();
    std::fs::write(root.path().join("go.mod"), "module k3c\n").unwrap();
    let server = Server::new(Config {
        root: root.path().to_path_buf(),
        port: 0,
        version: "test".to_owned(),
        log: Logger::discard(),
        usage: None,
    });
    server.start().await.unwrap();
    let mut client = mcp_client::Client::connect(server.url()).await;
    let (text, is_error) = client.call_text("reports_list", json!({})).await;
    assert_eq!((text.as_str(), is_error), ("keine Berichte", false));
    let (text, is_error) = client.call_text("report_read", json!({ "name": "../x.json" })).await;
    assert!(is_error && text.contains("abgelehnt"), "report_read mit ..: {text}");
    let (text, _) = client.call_text("saves_list", json!({})).await;
    assert_eq!(text, "keine Spielstände");
    server.stop().await.unwrap();
}
