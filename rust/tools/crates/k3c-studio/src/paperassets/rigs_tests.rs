//! Port von `rigs_test.go`.

use super::super::test_helpers::{leere_wurzel, repo_root, schreibe, stub_skript, werkzeuge};
use super::*;
use crate::paperassets::Cli;

/// Prüft jede Vorlage mit `skript`; die Figur darf nur `proportion` melden.
fn pruefe_vorlagen(skript: std::path::PathBuf) {
    let w = leere_wurzel();
    let mut cli = Cli::mit_skript(w.pfad.clone(), stub_skript());
    cli.skript = skript;
    for rig in rig_namen() {
        let text = vorlage(rig, "probe", "Probe").unwrap();
        let rel = format!("{rig}/gruppe/probe.paper.json");
        schreibe(&w.pfad, &rel, &text);
        let Err(e) = cli.validate(&super::super::test_helpers::unter(&w.pfad, &rel)) else {
            continue;
        };
        let nur_proportion = e
            .abgelehnt()
            .is_some_and(|f| f.len() == 1 && f[0].regel == "proportion");
        assert!(rig == "figur" && nur_proportion, "{rig}: {e}");
    }
}

#[test]
fn rig_vorlagen() {
    if !werkzeuge() {
        return;
    }
    pruefe_vorlagen(stub_skript());
    let err = vorlage("drache", "x", "y").unwrap_err().to_string();
    assert!(
        err.contains("figur, vierbeiner, gebaeude, requisite, hintergrund"),
        "{err}"
    );
    let figur = finde_rig("figur").unwrap();
    assert_eq!((figur.pflicht.len(), figur.pflicht[0]), (10, "rumpf"));
}

/// Review MC1.5: die Vorlagen bestehen auch das echte validate (RD1, inkl. Beinpfad des Vierbeiners).
#[test]
fn rig_vorlagen_echte_cli() {
    if !werkzeuge() {
        return;
    }
    pruefe_vorlagen(repo_root().join("tools/paper/bin/paper.ts"));
}

/// Neu: Zahlform und Reihenfolge der Vorlage wie Go `json.Marshal`.
#[test]
fn vorlage_requisite_bytes() {
    let want = "{\n  \"$schema\": \"../../../tools/paper/schema/paper-spec.schema.json\",\n  \"id\": \"fass\",\n  \
                \"rig\": \"requisite\",\n  \"version\": 1,\n  \"beschreibung\": \"A \\u003c B\",\n  \"teile\": [\n    \
                {\n      \"id\": \"korpus\",\n      \"pivot\": [0, 0],\n      \"lagen\": []\n    }\n  ],\n  \
                \"ausgabe\": {\n    \"hoehe_px\": 64,\n    \"rahmen\": [-16, -32, 32, 32]\n  }\n}\n";
    assert_eq!(vorlage("requisite", "fass", "A < B").unwrap(), want);
}
