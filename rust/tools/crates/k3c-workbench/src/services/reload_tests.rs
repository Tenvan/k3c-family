//! Port von `reload_test.go`.

use super::controller_tests::controller;
use super::testutil::{Fake, svc};
use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn reload() {
    let f = Fake::new(true);
    let c = controller(&f, vec![svc("A", false), svc("B", false), svc("C", false)]);
    c.start(&Cancel::default(), "B").await.unwrap();
    let with_port = |name: &str, port| Service {
        port,
        ..svc(name, false)
    };
    // C entfällt, D neu, A geändert, B läuft.
    let next = vec![with_port("B", 8), with_port("A", 7), svc("D", false)];
    let pending = c.reload(next).await;
    assert_eq!(c.names(), ["B", "A", "D"]);
    assert_eq!(pending, ["B"]);
    // B behält die alte Konfiguration bis zum Neustart.
    for (name, port) in [("A", 7), ("B", 1)] {
        let unit = c.unit(name).unwrap();
        assert_eq!((unit.status().port, unit.svc().port), (port, port), "{name}");
    }
    assert_eq!(c.unit("B").unwrap().status().state, State::Running);
    c.stop_all().await;
}
