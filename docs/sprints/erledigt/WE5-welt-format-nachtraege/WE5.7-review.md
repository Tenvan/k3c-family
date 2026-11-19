# WE5.7 · Review WE5 und Abnahme-Ticket

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/we5
- **Abhängig von:** WE5.1, WE5.2, WE5.3, WE5.4, WE5.5, WE5.6
- **Tickets:** B-979, B-981, B-982, B-983, B-984, B-985, B-986
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07, AC-08

## Ziel

Der Sprint WE5 ist geprüft, Befunde sind behoben oder als Ticket angelegt, das Abnahme-Ticket steht.

## Kontext

Sprint-README `docs/sprints/*/WE5-welt-format-nachtraege/README.md` mit AC-01 bis AC-08, Tickets B-979, B-981 bis B-986. Review getrennt vom Schreiben: Agenten `k3c-welten` (Welt-Format, Level), `k3c-reviewer` (Engine), `k3c-workbench` (MCP). Sim, Balance, Gen und `check:race` laufen nur im Abnahme-Ticket (B-742); Simulationen sind gesperrt.

## Erlaubte Dateien

- `engine/sim/**`
- `engine/level/**`
- `engine/room/**`
- `data/bosses.json`
- `data/biomes/**`
- `docs/rules/welten.md`
- `docs/rules/bosse.md`
- `docs/rules/stufen.md`
- `tools/k3c-workbench/internal/world/**`
- `tools/k3c-workbench/internal/mcpsrv/tools_world.go`
- `tools/k3c-workbench/internal/mcpsrv/tools_world_test.go`

## Nicht-Ziele

Neue Funktionen, Simulationen, Balancing.

## Schritte

1. Diff des Sprints gegen den Start-Commit lesen, Reviewer-Agenten parallel starten.
2. Jedes Kriterium AC-01 bis AC-08 mit Nachweis prüfen (Test, Datei, `world_check`).
3. Schwere Befunde beheben (unter etwa 30 Zeilen ohne zweites Review), Rest als Ticket mit Projekt und Sprint.
4. `task check`, `task check:go` nacheinander.
5. Abnahme-Ticket anlegen (Gen, Race), Abschnitt Abnahme der Sprint-README füllen, Glossar-Begriffe (Skin, Bau) prüfen.

## Fertig, wenn

- [ ] AC-01 bis AC-08 mit Nachweis in der Sprint-README › Abnahme.
- [ ] `task check` und `task check:go` grün.
- [ ] Genau ein Abnahme-Ticket angelegt.

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

AC-01 bis AC-08 erfüllt, Nachweise in den Ergebnissen WE5.1 bis WE5.6. Merge von origin/main konfliktfrei (ddd5d85fb), Review-Commit acf3fce12 auf `sprint/we5`. Behoben: Workbench-Tests und Tool-Text nach dem Merge mit WE5.6 (`events` übersteuerbar, gesperrt `hub › sitePlacement`); `simcli level` setzt den Bau in die tiefste Stufe (`TestWorldLevelBauInTiefsterStufe`); leerer Gegner-Name abgelehnt. `go vet`, Lint 0, `go test ./engine/...` nach `gen:paper` vollständig grün, `task check`, `check:go`, `check:diff` grün. Offen, Entscheidung 🧑: Camp hinter einem Portal außerhalb der Kristallhöhle (Mine 147/500); `bosse.md` § 1 als vorläufig markiert. Folgetickets: skins für Asset-Varianten mit @; Glossar Skin und Bau.
