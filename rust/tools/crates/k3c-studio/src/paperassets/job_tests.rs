//! Port von `job_test.go`.

use std::path::PathBuf;
use std::sync::mpsc;

use super::super::basis::assets;
use super::super::test_helpers::{BANNER_UNGUELTIG, schreibe, spec_root, test_repo};
use super::*;

fn asset(id: &str) -> Asset {
    Asset {
        id: id.to_owned(),
        ..Asset::default()
    }
}

/// Ohne bun: Der Renderer der Tests ruft die CLI nie auf.
fn ohne_cli() -> Cli {
    Cli::mit_skript(PathBuf::new(), PathBuf::new())
}

/// Ein Renderer, der wartet, bis der Sender fällt.
fn blockiert(pngs: usize) -> (mpsc::Sender<()>, Jobs) {
    let (frei, warte) = mpsc::channel::<()>();
    let warte = Mutex::new(warte);
    let mut jobs = Jobs::new();
    jobs.render = Arc::new(move |_, a| {
        let _ = halte(&warte).recv();
        AssetErgebnis {
            id: a.id.clone(),
            pngs,
            manifest: format!("{}/manifest.json", a.id),
            ..AssetErgebnis::default()
        }
    });
    (frei, jobs)
}

/// MC1/AC-05: Start kehrt vor dem ersten Render zurück; Status zeigt den Stand, Wait das Ergebnis.
#[test]
fn job_alle() {
    let w = spec_root();
    let liste = assets(&w.pfad).unwrap();
    let (frei, jobs) = blockiert(2);
    let st = jobs.start(&ohne_cli(), &liste);
    assert!(st.id == "j-1" && st.n == 3 && !st.fertig, "{st:?}");
    let st = jobs.status("j-1").unwrap();
    assert!(st.erledigt == 0 && st.aktuell == "mauer" && !st.fertig, "{st:?}");
    drop(frei);
    let st = jobs.wait("j-1", None).unwrap();
    assert!(st.fertig && st.erledigt == 3 && st.ergebnisse.len() == 3, "{st:?}");
    assert_eq!(st.ergebnisse[1].id, "monarch");
    assert!(st.aktuell.is_empty());
    assert_eq!(jobs.start(&ohne_cli(), &liste).id, "j-2");
}

/// Ein Job mit der echten Attrappe überspringt ungültige Specs mit ihrer Regel.
#[test]
fn job_ueberspringt_ungueltige() {
    let Some((w, cli)) = test_repo() else { return };
    schreibe(&w.pfad, "public/grafik/requisiten/banner.paper.json", BANNER_UNGUELTIG);
    let liste = assets(&w.pfad).unwrap();
    let jobs = Jobs::new();
    let st = jobs.wait(&jobs.start(&cli, &liste).id, None).unwrap();
    assert_eq!(st.erledigt, 4, "{st:?}");
    for e in st.ergebnisse {
        if e.id == "banner" {
            assert_eq!(e.uebersprungen, "material-unbekannt", "{e:?}");
        } else {
            assert!(e.pngs > 0 && !e.manifest.is_empty(), "{e:?}");
            assert!(e.uebersprungen.is_empty() && e.fehler.is_empty(), "{e:?}");
        }
    }
}

#[test]
fn job_unbekannt() {
    let jobs = Jobs::new();
    assert_eq!(jobs.status("j-99").unwrap_err().to_string(), "Job „j-99“ unbekannt");
    assert!(jobs.wait("j-99", None).is_err());
}

/// Eine abgelaufene Frist beendet nur das Warten, der Job läuft weiter.
#[test]
fn job_wait_abbruch() {
    let (frei, jobs) = blockiert(0);
    let id = jobs.start(&ohne_cli(), &[asset("a")]).id;
    let st = jobs.wait(&id, Some(Duration::from_millis(20))).unwrap();
    assert!(!st.fertig && st.dauer >= Duration::from_millis(20), "{st:?}");
    drop(frei);
    let st = jobs.wait(&id, None).unwrap();
    assert!(st.fertig && st.id.starts_with("j-"), "{st:?}");
}

/// Review MC1.1: Eine Panic beim Rendern beendet den Job nicht, sondern wird Fehler des Assets.
#[test]
fn job_panic() {
    let mut jobs = Jobs::new();
    jobs.render = Arc::new(|_, _| panic!("kaputt"));
    let id = jobs.start(&ohne_cli(), &[asset("a"), asset("b")]).id;
    let st = jobs.wait(&id, None).unwrap();
    assert!(st.fertig && st.ergebnisse.len() == 2, "{st:?}");
    assert_eq!(st.ergebnisse[0].fehler, "Absturz: kaputt");
}
