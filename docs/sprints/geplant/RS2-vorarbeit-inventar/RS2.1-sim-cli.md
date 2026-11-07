# RS2.1 · Engine-CLI k3c-sim (run, balance, level)

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/rs2
- **Abhängig von:** –
- **Tickets:** B-726
- **Kriterien:** AC-01

## Ziel

`cmd/k3c-sim` (feste EXE `bin/k3c-sim`) bietet die Unterbefehle `run`, `balance` und `level` mit JSON auf stdin oder Datei, Fortschritt als JSON-Zeilen auf stdout, Berichte unter `reports/`. Liefert dieselben Ergebnisse wie die heutigen Workbench-internen Aufrufe. Voraussetzung für RS2.2 (B-726).

## Kontext

B-726 (freigegeben): Die Workbench importiert `k3c/engine/sim`, `room`, `net`, `level` in `internal/balance` (14 Dateien), `botdev`, `enginetools`, `botfeed`, `mcpsrv`, `studioview` (36 Importstellen, Entscheidung 006 › Codeumfang). Heute laufen `sim_run`, `sim_test` offline (BAS-Standardlauf, Meilenstein-Lauf) und `level_generate` im Workbench-Prozess. Die Logik für Balancing liegt in `tools/k3c-workbench/internal/balance` und darf nicht nach `engine/` wandern, wenn sie `tools/` importiert (`engine/internal/rules/imports_test.go`). Daher zuerst prüfen, was davon reine Engine-Logik ist (gehört in `engine/` oder `cmd/k3c-sim`) und was Workbench-Berichtslogik bleibt. Feste EXE-Pfade unter `bin/`, keine Firewall-Abfrage, kein `go run` (Memory: firewall-prompt-feste-exe). Muster Kindprozess/Job-Objekt: `engine/devrun`. Ausgabeformat und Berichte müssen byte-gleich zum heutigen Stand bleiben (bis auf Zeitstempel).

## Erlaubte Dateien

- `cmd/k3c-sim/**`
- `Taskfile.yml`, `cmd/k3c-sim/Taskfile.yml` (Namensraum `k3c-sim`: `build`, `run`)
- `engine/**` nur für das Herauslösen reiner Logik aus `tools/k3c-workbench/internal/balance` (Verschieben ohne Änderung)
- `docs/protocol.md` nicht anfassen

## Nicht-Ziele

- Die Workbench umstellen (RS2.2).
- Rust. Sim-Logik ändern. `studioview`.
- Neue Berichtsformate.

## Schritte

1. `plan_get B-726` lesen; Importe der Workbench auflisten (`go list -deps ./tools/k3c-workbench/...`) und je Aufruf (`sim_run`, `sim_test` offline, `level_generate`) den Einstiegspunkt benennen.
2. Schnittstelle festlegen: Eingabe-JSON je Unterbefehl, Fortschrittszeilen (`{"event":"progress",…}`), Exit-Codes, Berichtsablage `reports/simtest-<id>/`. Als Abschnitt in `cmd/k3c-sim/README` (kurz) oder Kopfkommentar.
3. `cmd/k3c-sim` bauen: ruft dieselben Funktionen auf; Abbruch durch Prozess-Ende (SIGTERM bzw. CTRL_BREAK, Job-Objekt wie `devrun`).
4. Vergleichstest: Aufruf `bin/k3c-sim run` mit festem Seed gegen den bestehenden Workbench-Pfad, Ergebnis-JSON gleich; ein Mini-Balancing-Lauf (1 Seed) gleicher Bericht.
5. Taskfile-Namensraum `k3c-sim` (`build`, `run`), EXE-Pfad in `buildall` aufnehmen.
6. Messen: Dauer eines Einzel-Seeds (`k3c-dev`, 50–57 s laut 006) gegen den heutigen Weg ≤ +5 %.

## Fertig, wenn

- [ ] AC-01: `bin/k3c-sim run|balance|level` liefern auf festen Seeds dieselben JSON-/Berichtsdateien wie der bisherige Weg (Vergleichstest grün).
- [ ] AC-01: Fortschritt als JSON-Zeilen, Abbruch per Prozess-Ende getestet (Test mit kurzem Lauf).
- [ ] AC-01: Ein Einzel-Seed-Lauf ist höchstens 5 % langsamer als vorher (Zahl im Ergebnis).
- [ ] CI-Voraussetzung: `task check:diff` grün, `golangci-lint` ohne neue Ausnahmen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf). Domäne SRV: Reviewer-Agent vor dem Commit (Hintergrund, parallel zu `check:diff`).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
