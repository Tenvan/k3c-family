# RU2.2 · Papierschnitt-Grafik laden: Index, Manifest, Sheets

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru2
- **Abhängig von:** RU1.1
- **Tickets:** B-1016
- **Kriterien:** AC-02

## Ziel

Modul `rust/crates/k3c-game/src/art`: lädt die Papierschnitt-Grafik wie das Go-Spiel und liefert je Asset RGBA-Bilder und Frame-Rechtecke, ohne Grafik-Kontext (testbar headless).

## Kontext

Go-Vorlage: `native/k3c-game/art/` (`library.go`, `manifest.go`, `sheet.go`, `plan.go`, `scales.go`). Dateien: `public/grafik/<gruppe>/index.paper.json`, je Asset `manifest.json` und PNG-Sheets (erzeugt von `task gen:paper`, nicht eingecheckt), Zuordnung `data/paper.json`. Nur lesen, was die Demo braucht: Hintergründe, Boden, Gebäude, Figuren mit Animationsphasen. Rig-Figuren (Monarch als Rig) bleiben außen vor; ist der Monarch nur als Rig vorhanden, zeigt die Demo stattdessen eine Sheet-Figur.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/art/**`
- `rust/crates/k3c-game/Cargo.toml` (`png`, `serde`, `serde_json`)
- `rust/crates/k3c-game/tests/fixtures/art/**`

## Nicht-Ziele

Hochladen auf die GPU (RU2.3 über `Canvas::texture`). Port von `view/`, `world/`, `actors/` (RU7, RU13).

## Schritte

1. Go-Dateien oben lesen; Formate von `index.paper.json` und `manifest.json` als serde-Typen.
2. `Library::open(grafik_dir, paper_json)`: Gruppen-Indizes lesen, Asset per Name finden.
3. `Asset { sheets: Vec<Image>, frames: Vec<Frame { sheet, rect, anker }>, animationen }` wie im Go-Manifest.
4. Fehlender Ordner → Fehler mit Hinweis `task gen:paper`.
5. Tests mit kleiner Fixture (ein Asset, ein 4×4-PNG) und, wenn `public/grafik` vorhanden ist, ein Test, der alle Assets aus `data/paper.json` lädt (sonst übersprungen mit Meldung).

## Fertig, wenn

- [ ] AC-02: `cargo test -p k3c-game art` grün.
- [ ] AC-02: Mit gerenderter Grafik lädt der Test alle Assets aus `data/paper.json` ohne Fehler; Anzahl im Ergebnis.

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

AC-02 geprüft: `cargo test -p k3c-game art` grün (4 Fixture-Tests + Test mit echter Grafik). Mit gerenderter Grafik (K3C_GRAFIK_DIR auf `public/grafik` des Hauptcheckouts) lädt der Test alle 133 Assets aus `data/paper.json` ohne Fehler; ohne gerenderte Grafik wird er mit Meldung übersprungen. clippy -D warnings und fmt grün.

Umsetzung: `art/{mod,manifest,paper,tests}.rs`; `Library::open/assets/load`, `Asset { sheets, frames, animationen }`, Fehler nennen `task gen:paper`. Abweichung: `src/lib.rs` bekommt `pub mod art;` (nicht in der Dateiliste, nötig). Assets aus `paper.json` werden generisch eingesammelt (Strings `gruppe/id[@variante]`, Reittiere, Objekte, Gebäude-Gruppe). Rig-Figuren/Rig-Teile werden nicht geladen. Keine neuen Tickets.
