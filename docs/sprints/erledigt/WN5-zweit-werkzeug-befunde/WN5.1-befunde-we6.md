# WN5.1 · gocyclo, MCP-Beschreibung und Roundtrip-Test (B-1266)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/wn5
- **Abhängig von:** –
- **Tickets:** B-1266
- **Kriterien:** AC-01, AC-02

## Ziel

Die Werkzeug-Befunde aus B-1266 (Teil 1 und 2) sind behoben.

## Kontext

`tools/k3c-workbench/internal/services/config.go` (Funktion `check`) reißt gocyclo (18 > 15). Die Beschreibung des Welten-MCP nennt `endBoss`, `skins`, `events` nicht und es fehlt ein Roundtrip-Test; der MCP-Server liegt seit WZ63 in `tools/k3c-studio/internal/mcpsrv/`. Stand vor Beginn prüfen: Teile können inzwischen erledigt sein.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/**`
- `tools/k3c-studio/internal/**`

## Nicht-Ziele

`engine/**`, `testdata/**`, `Taskfile.yml`, neue Funktionen.

## Schritte

1. `check_run task:check:dev` laufen lassen und den Stand festhalten.
2. `check` ohne Verhaltensänderung aufteilen.
3. Beschreibung ergänzen, Roundtrip-Test schreiben.

## Fertig, wenn

- [ ] AC-01: `check_run task:check:dev` grün.
- [ ] AC-02: Roundtrip-Test für `endBoss`, `skins`, `events` grün.

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

- AC-01 geprüft: `task:check:dev` war schon vor der Session grün (gocyclo ist behoben); nach der Änderung `task:check:diff` grün.
- AC-02 umgesetzt für `endBoss` und `skins` (Beschreibung und Roundtrip-Test `tools_world_skins_test.go`); Welt-Events verschoben (B-1260).

### Für den Hauptrechner
- B-1260: Ursache gefunden. `welt-events` steht in der Studio-Liste und in `rust/crates/k3c-sim/src/world/values.rs`, fehlt aber in `OVERRIDE_FILES` in `rust/crates/k3c-data/src/world.rs`; der Parser lehnt den Override ab (`world_override_set` scheitert). Der Erwartungstext in `tree_rest_tests.rs` muss mit.
- B-1266/AC-03 (Go-Testfehler in `engine/**`) bleibt offen, Bereich von RU41.
