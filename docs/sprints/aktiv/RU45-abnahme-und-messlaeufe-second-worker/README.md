# RU45 · SIM, CLI · Abnahme- und Messläufe auf dem Second Worker

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM, CLI
- **Reife:** bereit
- **Tickets:** B-1080, B-1089
- **Start-Commit:** 4fb6488c3
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18

## Ausgangslage

RU6 (Level-Generator in Rust) und RU44 (Bild-Pfad, `--novsync`) sind erledigt; ihre Abnahme (B-1080) und die Weitergabe von `-Vsync` in `native/measure.ps1` (B-1089) fehlen. Gen, Race und Level-Vergleich liefen bisher nicht (B-742); auf dem Second Worker gilt die Sim-Sperre nicht.

## Ziel

Nach dem Sprint sind `gen:paper`, `check:race` und der Level-Vergleich Go gegen Rust (1000 Seeds je Biom) gelaufen und belegt, und `measure.ps1 -Engine replay` misst ohne VSync.

Am Ende sichtbar: Ergebnis-Abschnitte RU45.1 bis RU45.3 mit Messwerten, B-1080 und B-1089 erledigt (oder Befund benannt).

## Beteiligte und Zielgruppen

Der Second Worker (Lane `zweit`) setzt um und läuft die Abnahmen; der Hauptrechner legt Befund-Tickets an; Ralf misst später mit dem Skript (Gate G2).

## Anforderungen

B-1080 › Anforderungen und Akzeptanzkriterien (AC-01, AC-02), B-1089 › Anforderungen (AC-03, AC-04). Keine sprint-eigenen Anforderungen.

## Nicht-Ziele

B-1081, B-1084, B-1086 (warten auf RU14 und RU22), B-1074 Gate-Messung (Messungen zum Schluss), alles, was an RU14, RU26, RU18, RU19, RU21 oder RU22 hängt; Fehler beheben (Befunde werden Tickets des Hauptrechners).

## Regeln und Einschränkungen

Alle Sessions `Lane: zweit`, Modell sonnet, nacheinander (RU45.1 → .2 → .3 → .4): gemeinsame Sprint-Branch-Historie und die Last der Läufe (B-284). Vor dem Push `git merge origin/main`. Der Second Worker legt keine Tickets an. Keine Messläufe von `measure.ps1` (Ralfs Gate).

- **Bereich:** `rust/crates/k3c-level/**`, `engine/level/vergleich_dump_test.go`, `native/measure.ps1`
- **Abhängig von:** RU6, RU44
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`K3C_LEVEL_VERGLEICH=.work/level-vergleich cargo test --release -p k3c-level vergleich_go -- --nocapture` meldet `verglichen 5000, abweichend 0`. `measure.ps1 -Engine replay` ohne `-Vsync` startet `k3c-game replay … --novsync`.

## Ausnahme- und Fehlerfälle

Kein C-Compiler auf dem Second Worker: `check:race` wird übersprungen und als Befund für den Hauptrechner genannt. Abweichung im Level-Vergleich: Befund mit Beispiel-Seeds, kein Fix in diesem Sprint.

## Akzeptanzkriterien

- **AC-01** `task check:race` und `task gen:paper` laufen grün (`B-1080/AC-01`; ohne C-Compiler als übersprungen benannt).
- **AC-02** Level-Vergleich Go gegen Rust über mindestens 1000 Seeds je Biom ohne Abweichung oder Befund benannt (`B-1080/AC-02`).
- **AC-03** `measure.ps1` hängt für `replay` `--novsync` an, wenn `-Vsync` fehlt (`B-1089/AC-01`).
- **AC-04** Hinweis und Kommentare nennen nicht mehr „immer mit VSync“, Skript parst fehlerfrei (`B-1089/AC-02`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU45.1 | `RU45.1-gen-und-race.md` | Umsetzung | autonom | fertig |
| RU45.2 | `RU45.2-level-vergleich-go-rust.md` | Umsetzung | autonom | fertig |
| RU45.3 | `RU45.3-measure-novsync-replay.md` | Umsetzung | autonom | fertig |
| RU45.4 | `RU45.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
