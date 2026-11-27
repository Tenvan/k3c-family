//! Go `queue_test.go`.

use super::next_fixtures::{check_text, waves_repo};
use super::queue::{QueueFilter, device_of, norm_device, queue};

/// B-783/AC-02: Mensch-Sessions nach Gerät (Zeile „Gerät:“ im Kontext, sonst unbekannt), Specs im Entwurf mit
/// „Braucht 🧑: Freigabe“, Entscheidungen; ein freigegebenes Ticket steht nicht bei den Freigaben.
#[test]
fn nach_geraet() {
    let plan = waves_repo();
    let out = queue(&plan.root, &QueueFilter::default()).expect("queue");
    let want = "Ralf-Queue: 1 Freigaben · 1 Entscheidungen · 2 Abnahmen an 2 Geräten\n\
                ── Freigaben ──\n- B-006 · Ticket B-006 (Ticket, DEV, Spec Entwurf, Braucht 🧑: Freigabe)\n\
                ── Entscheidungen ──\n- B-007 · Ticket B-007 (Ticket, DEV, Spec Entwurf, Braucht 🧑: Entscheidung)\n\
                ── Abnahmen je Gerät ──\n\
                PC (1):\n- A1.3 · Titel A1.3 (Sprint A1, offen, Agent Mensch)\n\
                unbekannt (1):\n- C1.3 · Titel C1.3 (Sprint C1, offen, Agent Mensch)";
    assert_eq!(out, want);
    let pc = QueueFilter {
        device: "pc".to_owned(),
    };
    check_text(
        &queue(&plan.root, &pc).expect("queue"),
        &["PC (1):\n- A1.3"],
        &["C1.3", "── Freigaben", "B-006"],
    );
}

#[test]
fn geraete_schreibweisen() {
    for (input, want) in [("PC", "PC"), ("pc am TV", "PC"), ("Xbox", "Xbox"), ("Tablet", "Tablet")] {
        assert_eq!(norm_device(input), want, "{input}");
    }
    assert_eq!(
        device_of("# X\n\n- **Agent:** Mensch\n\n## Kontext\n\n- **Gerät:** Pi\n"),
        "Pi"
    );
    assert_eq!(device_of("# X\n\n## Kontext\n\nkein Gerät genannt\n"), "unbekannt");
}
