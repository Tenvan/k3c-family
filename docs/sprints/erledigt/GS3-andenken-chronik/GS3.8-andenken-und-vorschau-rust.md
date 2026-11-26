# GS3.8 · Andenken auf der Seekarte und Vorschau in der Spielstand-Auswahl im Rust-Spiel (ersetzt GS3.3 und GS3.5)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs3
- **Abhängig von:** GS3.2, GS3.7
- **Tickets:** B-966, B-978
- **Kriterien:** AC-03, AC-05

## Ziel

Die Seekarte im Rust-Spiel zeigt Trophäe, Abschlusszeile und Zahlen für abgeschlossene Inseln (für beide Spieler gleich), und die Spielstand-Auswahl zeigt die Vorschau des markierten Stands.

## Kontext

Spec: `B-966 › Anforderungen` (AC-05, AC-06) und `B-978 › Anforderungen` (AC-02, AC-03). Die Go-Sessions GS3.3 und GS3.5 sind verworfen; diese Session ersetzt sie in Rust (Entscheidung 013): Anzeige in `rust/crates/k3c-game/src/hud/`, `view/`, `local/`, `menu/`. Die Trophäen-Assets kommen aus GS3.2, die Chronik und die Liste aus GS3.7.

Laden, Löschen und die Taste B (zurück) im Menü bleiben unverändert. Jede Anzeige mit 2 Spielern prüfen; HUD-Prüfung durch `k3c-hud` vor dem Commit.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/view/**`
- `rust/crates/k3c-game/src/local/**`
- `rust/crates/k3c-game/src/menu/**`
- `rust/crates/k3c-game/tests/still_*.rs`

## Nicht-Ziele

Neue Spielregeln, Spielstand-Format (GS3.7). Neue Assets (GS3.2); fehlt eines, vorläufig zeichnen und melden.

## Schritte

1. Andenken (Trophäe, Abschlusszeile, Zahlen) auf der Seekarte für abgeschlossene Inseln zeichnen; Standbild-Test.
2. Vorschau des markierten Stands in der Spielstand-Auswahl; Laden, Löschen und B unverändert; Test.
3. `k3c-hud` vor dem Commit.

## Fertig, wenn

- [ ] AC-03: Die Seekarte zeigt Trophäe, Abschlusszeile und Zahlen für abgeschlossene Inseln, für beide Spieler gleich (B-966/AC-05, AC-06).
- [ ] AC-05: Die Spielstand-Auswahl zeigt die Vorschau des markierten Stands; Laden, Löschen und B unverändert (B-978/AC-02, AC-03).
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

AC-03 geprüft (Logik und Text, Sicht am PC durch 🧑 offen): Die Seekarte zeigt auf abgeschlossenen Inseln Trophäe und Fahne; steht der Zeiger darauf, erscheinen die erste Zeile von `done` und „<Tage> Tage · <n> Monarchen“ (Einzahl „1 Tag“, „1 Monarch“); ohne `done` nur die Zahlen, ohne Chronik-Eintrag nur Trophäe und Fahne. Links/Rechts läuft über die Ziele und danach über die abgeschlossenen Inseln, A wirkt dort nicht und meldet die Ablehnung (`CueKind::Denied`, `sea.denied`). Beide Spielerhälften zeigen dasselbe (je Ansicht gebaut, gleiche Daten); Texte nur bei Änderung gebaut, keine Allokation je Frame im Ruhezustand (`alloc_seekarte_sperre` grün). Daten: `RouteIsland.done` und `Frame.chronicle` aus `local/route.rs`; `SwitchState` in `k3c-sim` blieb unverändert. Dateien: `hud/drawsea_memo.rs`, `view/trophy.rs`, `hud/seekarte*.rs`, `drawsea*.rs`, `texts_*`.
AC-05 geprüft (Logik; Sicht am PC durch 🧑 offen): Die Spielstand-Auswahl zeigt rechts neben der Liste die Vorschau des markierten Stands (Diorama aus Kacheln mit Schiff an der aktuellen Insel, Trophäen auf den abgeschlossenen, darunter Abschlusszeile und Zahlen der zuletzt abgeschlossenen Insel). Auf/Ab wechselt sie sofort; Laden, Löschen und B unverändert; nicht ladbarer Stand, unbekannte oder unlesbare Welt → keine Vorschau; Stand ohne abgeschlossene Insel → Karte ohne Trophäen; bei zu wenig Platz entfällt sie. Dateien: `menu/saves_preview.rs`, `menu/saves.rs`, `menu/draw.rs`, `local/saves.rs`, `view/session.rs` (`SaveInfo` mit `route`, `island`, `completed`, `chronicle`). Tests: `hud/seekarte_memo_tests.rs`, `menu/saves_preview_tests.rs` und ergänzte Tests in `seekarte_*`, `local/saves_tests.rs`, `texts_tests`.
Review (k3c-hud) vor dem Commit: Befunde mittel behoben: Vorschau hält 5 % Safe-Area rechts und unten frei, `done`-Zeile schrumpft nicht mehr (Mindestschrift `small_px`, Umbruch in höchstens zwei Zeilen, sonst „...“ gekürzt), Zahlenzeilen in heller Textfarbe und auf der Seekarte in Namensgröße (`MEMO_H` 2,5). Das Aussehen am TV (aus 3 m lesbar, Trophäe, Vorschau, beide Hälften) hat 🧑 zu prüfen.
Geprüft: `task rust:lint` grün, `cargo test -p k3c-game --lib` 1069 grün, 3 rot (`props::plan_tests`, nicht aus dieser Session). `task check:diff` Stufe 1 grün, Stufe 2 rot nur an diesen drei Tests. Integrationstests des Crates: drei Ziele rot, alle schon vor dieser Session rot (Ergebnis von WD1.1 nennt `alloc_je_bild`, `e2e_flow`, `still_hud`).

Abweichungen:
- **Trophäe vorläufig gezeichnet** (`view/trophy.rs`), weil GS3.2 (Trophäen-Assets) noch offen ist; sie ersetzt im Rückfall den Haken, im Papier-Diorama steht sie neben der vorhandenen Fahne. Sobald GS3.2 die Assets liefert, sind sie über `data/paper.json` › `seekarte` › `trophaeen` anzubinden.
- `tests/hud_bild.rs` (nicht in den erlaubten Dateien, nur `still_*.rs`) musste für das neue Feld `done` angepasst werden (Compile-Fix mit `..Default::default()`).
- „Oder keine wählbare Insel markiert“ (B-966/R4) gelesen als: dann gibt es nichts zu zeigen; die Info-Zeilen erscheinen nur bei Zeiger auf einer abgeschlossenen Insel.
- Die Vorschau im Menü ist ein vereinfachter Papierschnitt aus Kacheln, weil das Menü keinen Zugriff auf `Hud` und `SeaArt` hat; „...“ statt „…“ wegen der Latin-1-Debug-Schrift der Menüs.
- GS3.8 wurde begonnen, obwohl GS3.2 (Abhängigkeit, Lane –) noch offen ist: 🧑 hat die Session ausdrücklich genannt, die Trophäe fällt auf die gezeichnete Form zurück (Nicht-Ziel „fehlt eines, vorläufig zeichnen und melden“).

### Für den Hauptrechner

- GS3.2 (Trophäen-Assets) bleibt offen; danach `data/paper.json` › `seekarte` › `trophaeen` anbinden und die gezeichnete Trophäe nur als Rückfall lassen.
- Tickets: `k3c-game` `props/plan_tests.rs` (Standard-Welt-Fixture, drei rote Tests, schon im Ergebnis von WD1.4 und GS3.7 genannt); Integrationstests `alloc_je_bild` (941 Allokationen bei Obergrenze 786), `e2e_flow` (Spieler 2 findet bis Takt 535 keinen Skill-Punkt), `still_hud` (Bild „pad-tastatur“ weicht ab).
- Glossar `docs/glossar.md`: Andenken, Chronik, Trophäe (nicht Teil dieser Session; Texte für Flaschenpost und Markierung stehen in GS4.1).
- Am TV prüfen (🧑, Abnahme-Ticket des Sprints): Sengoku, Insel 1 zu zweit abgeschlossen, an Insel 2 die Seekarte öffnen: Trophäe, Zeile und Zahlen aus 3 m lesbar; Spielstand-Auswahl mit Vorschau nicht am Rand abgeschnitten.
