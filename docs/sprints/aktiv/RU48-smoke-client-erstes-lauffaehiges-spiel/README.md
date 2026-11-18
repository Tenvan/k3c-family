# RU48 · CLI · Smoke-Client: erstes lauffähiges Spiel vom Hauptmenü bis zur Niederlage

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18

## Ausgangslage

Das Rust-Spiel hat Fenster, Menüs, HUD, Zeichnen und die Schleife (`rust/crates/k3c-game/src/game`, RU26), aber `main.rs` startet nur `demo`, `replay` und `still`. Die Sitzung (`local`, RU27), der Raum (`k3c-room`, RU21, RU49) und die Sim (`k3c-sim`, RU18, RU19) sind getrennt entstanden und noch nie zusammen gelaufen; ein Teil der Sim-Schritte steht als `unimplemented!`-Platzhalter in `world/pending.rs` und `state/pending.rs`.

## Ziel

Ein Rust-Fenster-Programm (`k3c-game` ohne Unterbefehl, `task rust:run`), das vom Hauptmenü ein neues Spiel mit einer Insel und mindestens einem Spieler startet, die Sim wirklich rechnet (Monarch, Einheiten, Gegner, Kampf), HUD und Welt zeichnet und Tastatur und Pad annimmt. Speichern, Dev-Menüs, Messmodus, Aufnahme, Icon und Optik-Feinschliff dürfen fehlen. Am Ende sichtbar: Fenster mit Hauptmenü, „Neues Spiel“, laufender Tag und Nacht mit Gegnern.

## Beteiligte und Zielgruppen

🧑 will den echten Client sehen (Smoke-Version); Agenten auf beiden Rechnern setzen um. Danach folgt die vollständige Abnahme (RU36).

## Anforderungen

`B-1014 › Anforderungen`. Weg und Reihenfolge: `docs/plan-weiterentwicklung.md` › Weg zur Smoke-Version. RU48 verdrahtet nur; neue Spielregeln entstehen hier nicht. Vorgezogen aus RU30: nur der Start (Verteiler in `main.rs`, Grafik einmal laden); Ladeanzeige je Stufe, Schiffsfahrt, Aufnahme, Messmodus, Stresstest und Icon bleiben in RU30.

## Nicht-Ziele

Speichern und Laden, Dev-Menüs, Messen, Aufnahme, Icon, Vergleich mit Go (RU22, RU32), Optimierung, Balancing, Mehrspieler über Netz (entfernt). Neue Crates.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig. Jede Mechanik mit 2 Spielern gleichzeitig (Split-Screen); B ist in Menüs zurück, im Spiel unbelegt. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Beim ersten Lauf (RU48.3) dürfen Platzhalter der Sim durch die echte Funktion ersetzt werden, die Sim-Regeln bleiben wie in Go. Die Sessions RU48.1 und RU48.2 können schon vor RU18 und RU19 laufen (Tests, die Takte rechnen, mit `ignore`); RU48.3 braucht beide gemergt.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:
- **Bereich:** `rust/crates/k3c-game/src/main.rs`, `rust/crates/k3c-game/src/game/**`, `rust/crates/k3c-game/tests/run_smoke.rs`, `rust/Taskfile.yml`, `rust/crates/k3c-sim/src/**` (nur RU48.3), `rust/crates/k3c-room/src/**` (nur RU48.3)
- **Abhängig von:** RU27, RU49
- **Braucht 🧑:** –
- **Sperre:** taskfile

## Beispiele

`task rust:run` → Fenster mit Hauptmenü; Taste Enter auf „Neues Spiel“ → Insel, Monarch läuft mit den Pfeiltasten, nach einigen Minuten kommt die Nacht und Gegner greifen die Burg an. Zwei Pads → zwei Monarchen im Split-Screen.

## Ausnahme- und Fehlerfälle

Fehlt `data/` oder die Grafik (`task gen:paper` nicht gelaufen) → Fehlerfenster mit deutscher Meldung und dem Hinweis auf den Befehl, kein Absturz. Ein Panic im Takt beendet das Fenster nicht stumm, sondern zeigt die Meldung (Go `fault`).

## Akzeptanzkriterien

- **AC-01** `k3c-game` ohne Unterbefehl öffnet das Fenster mit dem Hauptmenü; „Neues Spiel“ startet eine Insel der Standard-Welt mit einem Spieler (Tastatur) oder zwei (Pad).
- **AC-02** Ein Test ohne Fenster (`tests/run_smoke.rs`) startet ein neues Spiel, rechnet 6000 Takte (mehr als ein Tag-Nacht-Zyklus) mit zwei Spielern und endet ohne Panic, die Zeit der Welt wächst, in der Nacht erscheinen Gegner.
- **AC-03** Die Tastaturbelegung (Spieler 1 und 2, Menü) und die Pad-Belegung stehen im Ergebnis von RU48.2 und stimmen mit `game/keytable.rs` überein.
- **AC-04** `task rust:run` baut und startet das Spiel; `task rust:test` und `task rust:lint` bleiben grün.
- **AC-05** Kein `unimplemented!` aus `world/pending.rs`, `state/pending.rs` oder `pending.rs` der Sim ist im Spielpfad (Neues Spiel, Takt, Zeichnen) erreichbar.

## Offene Fragen

keine

Mit Default entschieden (autonom, Claude, 2026-11-18; 🧑 kann widersprechen): Die Grafik wird beim Start einmal geladen (wie `demo`); die Ladeanzeige je Stufe (RU30.1) folgt nach der Smoke-Version. Skills und Monarch-Passive in der Sim sind Teil von RU18.5 und bleiben im Sim-Zug; ein Abbau auf No-op-Platzhalter ist möglich, falls RU18.5 den Start verzögert (siehe Plan-Notiz).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU48.1 | `RU48.1-start-verdrahten.md` | Umsetzung | autonom | fertig |
| RU48.2 | `RU48.2-smoke-test-run-task-tasten.md` | Umsetzung | autonom | offen |
| RU48.3 | `RU48.3-erster-lauf-luecken-schliessen.md` | Umsetzung | autonom | offen |
| RU48.4 | `RU48.4-review.md` | Umsetzung | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
