# B-1055 · Fluss-Shader in Rust (WGSL)

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU13
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Was ist heute so? Problem oder Idee in 1–5 Sätzen, mit Dateipfaden, wenn es um Code geht.

## Ziel

`water/river.kage` (Go, Kage) als WGSL-Shader in `k3c-platform` nachbauen, damit der Fluss in Rust wie in Go (Fassung „shader“, F3) gezeichnet werden kann. Bis dahin zeichnet Rust den Fluss immer als Kacheln (RU13.2); die Rechnung der Uniforms (`water_layout`, `RiverWater::set`, Phasen) ist schon portiert und getestet.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Geprüft 2026-11-18 (Claude), nicht umgesetzt: größer als eine Session (Größe L) und ohne Fenster nicht prüfbar. Der Fluss-Shader braucht (1) einen WGSL-Port von `native/k3c-game/world/water/river.kage` (66 Zeilen) samt Uniform-Puffer in `backend/setup.rs`/`gpu.rs`, (2) eine neue Canvas-Schnittstelle für Shader-Flächen mit Uniforms (Headless-, Frame- und SoftCanvas-Seite, Run-Art neben Quads/Tris), (3) einen Software-Nachbau für das Standbild und (4) die Wahl der Fassung „shader“ in `world/water.rs`. Vor der Freigabe in Tickets je Teil zerlegen (Schnittstelle und Run-Art; WGSL-Port samt Pipeline; Anbindung `draw_river`) und erst nach Ralfs G2-Messung (RU39.2) angehen, weil Kacheln bis dahin als Fassung genügen.
