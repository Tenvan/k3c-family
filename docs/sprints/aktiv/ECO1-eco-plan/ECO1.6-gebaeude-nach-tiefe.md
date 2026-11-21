# ECO1.6 · Gebäude nach Tiefe: unten Verteidigung, Produktion oben

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/6-gebaeude-nach-tiefe
- **Abhängig von:** –
- **Tickets:** B-1196
- **Kriterien:** AC-06

## Ziel

Je Gebäudeart ist in den Daten festgelegt, auf welchen Ebenen sie stehen darf: Kampfgebäude (Mauer, Turm, Tor, Kaserne, Bogner, Heilplatz) überall, Produktion nur ganz oben.

## Kontext

Ticket B-1196. Heute baut jede Ebene, was die Hub-Stufe freigibt (`data/hub.json`, `data/buildings.json`). Zuordnung als Datenwert, Rust-Sim (`k3c-sim/src/hub`, `building`). Spielstand-Version erhöhen, alte Stände egal; Go-Golden nur mit `native_rules` aus.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/hub/**`
- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-data/src/**`
- `data/hub.json`
- `data/buildings.json`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Wo Plätze je Ebene entstehen (hub sites) finden.
2. Datenfeld je Art (z. B. `levels`/`topOnly`), Sim filtert Plätze nach Tiefe.
3. Tests: unten nur erlaubte Arten, oben alle.

## Fertig, wenn

- [ ] AC-06: Unten nur erlaubte Arten, oben alle; Zuordnung in den Daten.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-06 geprüft: `data/hub.json` kennt je Hub-Platz das Feld `topOnly`; gesetzt für Werkstatt, Taverne, Schmiede, Rüstkammer (Produktion). Kampfgebäude (Mauer, Turm, Tor über die wallLines, Kaserne, Bogner, Heilplatz) tragen es nicht und stehen auf jeder Ebene. `world_site_specs` (`k3c-sim/src/world/lines.rs`) überspringt `topOnly`-Plätze ab Tiefe 1, nur mit `WorldValues::native_rules`; Go-Läufe und Golden bleiben unverändert. Test `own_world_site_specs_zeigt_produktion_nur_ganz_oben` (Tiefe 0 alle, Tiefe 1 und 2 nur Kampfgebäude, ohne native Regeln alle). Daten: `HubSite.top_only` (k3c-data), Formtabelle `TopOnly`, Go-Struct `engine/sim/data.go` bekommt das Feld nur zum Mitlesen, `testdata/rust/data-tree.json.gz` neu geschrieben (`go test ./internal/detvec/ -write`, Grund: neues Datenfeld). Tests, die die Armory-Zeile der hub.json wörtlich suchen, angepasst (Go: `engine/room/preview_candidate_test.go`, hubplan-Tests; Rust: `preview_candidate_tests.rs`).

Go-Parität: `golden_campaign` und `spielstand_rundreise` setzen jetzt `native_rules = false` (waren seit HB1 rot, nextId 383 statt 380); sie sind wieder grün.

Abweichungen: Spielstand-Version nicht erhöht, weil sich das Format nicht ändert (ein Spielstand trägt seine Plätze selbst; nur neue Welten bekommen die gefilterte Liste). Die Zuordnung Taverne = Produktion ist eine Annahme (Taverne bringt Bürger), Bogner zählt als Kampfgebäude. `task check:diff`: Stufe rust-test nur noch durch die in B-1190 AC-04 geführten Rot-Tests (`eingebettete_welten_gueltig`, `insel_nach_1000_schritten_wie_go`); `cargo fmt` hat dazu eine Datei formatiert. `still_hud` (`pad-tastatur`), `alloc_je_bild`, `e2e_flow`: vorbestehend rot (still_hud ohne Siegel gemessen, identisch).

### Für den Hauptrechner
- Entscheidung 50:50 für Ralf: Soll die Taverne oben bleiben (Annahme jetzt: ja, Produktion) und der Bogner unten stehen dürfen? Umstellen ist eine Zeile `topOnly` in `data/hub.json`.
- `still_hud` `pad-tastatur` (gesamt 4,020 über 4,0) und `ablauf_wie_go_zwei_laeufe_gleich` (Münzen bis Takt 700) sind rot und stehen nicht in B-1190 AC-04.
