//! Port von `applog_test.go`: das eigene Log (`k3c_kit::applog`) schreibt im Format, das [`scan`] liest (B-046).

use std::sync::Arc;

use k3c_kit::applog;
use k3c_kit::console::Store;
use serde_json::json;

use super::{Query, scan};
use crate::ports::LOG_NAME;
use crate::testutil::tmp_dir;

/// `TestScanLiestApplog`.
#[test]
fn scan_liest_applog() {
    let tmp = tmp_dir();
    let dir = tmp.path().join("logs");
    let (log, error) = applog::open_named(&dir, LOG_NAME, Arc::new(Store::new(0, None)));
    assert!(error.is_none(), "{error:?}");
    log.logger().warn(
        "lauf beendet",
        &[
            ("ns", json!("check")),
            ("target", json!("task:test")),
            ("exit", json!(1)),
        ],
    );
    log.close().unwrap();
    let res = scan(&dir.join(format!("{LOG_NAME}.jsonl")), &Query::default()).unwrap();
    assert_eq!(res.entries.len(), 1, "{res:?}");
    let entry = &res.entries[0];
    assert_eq!(
        (entry.level.as_str(), entry.ns.as_str(), entry.msg.as_str()),
        ("WARN", "check", "lauf beendet")
    );
    let data = entry.data.as_ref().unwrap();
    assert_eq!((&data["target"], &data["exit"]), (&json!("task:test"), &json!(1)));
}
