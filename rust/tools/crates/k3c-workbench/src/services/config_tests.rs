//! Port von `config_test.go`.

use std::collections::BTreeMap;
use std::path::PathBuf;

use super::config::{Service, load, shift};
use crate::testutil::tmp_dir;

pub(crate) fn load_text(content: &str) -> Result<Vec<Service>, String> {
    let dir = tmp_dir();
    let path = dir.path().join("services.json");
    std::fs::write(&path, content).unwrap();
    load(&path)
}

/// Die echte Datei, nur lesend.
pub(crate) fn real_config() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../tools/k3c-workbench/services.json")
}

/// `services.json` des Moduls führt das Studio ohne Fenster und die Spiel-Dienste.
#[test]
fn services_json_im_modul() {
    let list = load(&real_config()).unwrap();
    assert_eq!(list.len(), 3, "{list:?}");
    assert_eq!((list[0].name.as_str(), list[0].port), ("k3c-studio", 5181));
}

#[test]
fn laden_lehnt_fehler_ab() {
    let ok = r#"{"name":"A","command":["x"],"cwd":".","port":1000,"health":"http"}"#;
    let double = format!("[{ok},{ok}]");
    let port = format!(r#"[{ok},{{"name":"B","command":["x"],"port":1000,"health":"http"}}]"#);
    let cases = [
        (
            r#"[{"name":"A","command":["x"],"port":1000,"health":"http","extra":1}]"#,
            "unknown field",
        ),
        (double.as_str(), "name doppelt"),
        (port.as_str(), "port 1000 doppelt"),
        (r#"[{"name":"A","port":1000,"health":"http"}]"#, "command fehlt"),
        (r#"[{"command":["x"],"port":1000,"health":"http"}]"#, "name fehlt"),
        (
            r#"[{"name":"A","command":["x"],"port":0,"health":"http"}]"#,
            "port 0 ungültig",
        ),
        (
            r#"[{"name":"A","command":["x"],"port":1000,"health":"ping"}]"#,
            r#"health "ping""#,
        ),
        (
            r#"[{"name":"A b","command":["x"],"port":1000,"health":"tcp"}]"#,
            "name fehlt oder enthält",
        ),
        (
            r#"[{"name":"A","command":["x"],"port":1000,"health":"tcp","log":"../x"}]"#,
            r#"log "../x""#,
        ),
        (
            r#"[{"name":"A","command":["x"],"port":1000,"health":"tcp","portEnv":"a b"}]"#,
            r#"portEnv "a b""#,
        ),
        // Go: „cannot unmarshal“; serde nennt den Typfehler anders.
        (r#"{"name":"A"}"#, "invalid type: map"),
    ];
    for (content, want) in cases {
        let error = load_text(content).expect_err(content);
        assert!(error.contains(want), "{content}: {error}, erwartet {want}");
    }
    assert!(load(&tmp_dir().path().join("fehlt.json")).is_err());
}

#[test]
fn shift_verschiebt_ports_und_gibt_sie_allen_mit() {
    let service = |name: &str, port, env_name: &str| Service {
        name: name.to_owned(),
        port,
        port_env: env_name.to_owned(),
        ..Service::default()
    };
    let mut list = vec![
        service("Vite", 5173, "K3C_VITE_PORT"),
        service("Spielserver", 8080, "K3C_HTTP_PORT"),
    ];
    list[0].env = BTreeMap::from([("X".to_owned(), "1".to_owned())]);
    let got = shift(&list, 10);
    assert_eq!((got[0].port, got[1].port), (5183, 8090));
    for s in &got {
        assert_eq!(s.env["K3C_VITE_PORT"], "5183", "{}", s.name);
        assert_eq!(s.env["K3C_HTTP_PORT"], "8090", "{}", s.name);
    }
    assert_eq!(got[0].env["X"], "1");
    assert_eq!((list[0].port, list[0].env.len()), (5173, 1), "Original geändert");
}
