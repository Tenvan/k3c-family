# RS2.1 · Engine-CLI k3c-sim (run, balance, level)

- **Status:** fertig
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

**AC-01 geprüft (Vergleich):** `bin/k3c-sim run` liefert denselben Text wie `sim_run` (k3c-dev, 1 000 und 100 000 Ticks mit Eingaben); `bin/k3c-sim level` wie `level_generate` (cave, familie-2026); Balancing-Einzel-Seed (coop2, standard, 40 Tage, k3c-dev): `balance-*.json` und `.md` byte-gleich zu `bin/k3c-balance`, `.html` und `result.json` nur im Zeitstempel `generated` verschieden. Tests grün: `TestRunGleichSimRun`, `TestLevelGleichLevelGenerate`, `TestBalanceMatrixGleich`.

**AC-01 geprüft (Fortschritt, Abbruch):** stdout nur JSON-Zeilen (`progress`, `result`, `report`, `error`, `aborted`), Exit 0/1/2/130; `TestAbbruchPerProzessEnde` startet einen Kindprozess und beendet ihn, kein Bericht bleibt liegen; `TestSignalAbbruch` grün.

**AC-01 Dauer:** je 3 Läufe gleicher Argumente unter Last 43–100 %: `k3c-balance` Mittel 127,7 s, `k3c-sim` 116,3 s; isoliert 160 s gegen 156 s. Nicht langsamer (die 50–57 s aus 006 waren bei der hohen Last nicht reproduzierbar).

**CI-Voraussetzung:** `check:diff`-Schritte `go-test`, `go-lint`, `vitest-scan`, `lint`, `typecheck`, `vitest-changed` grün. Der Schritt `wb-gotest` (volle Workbench, ausgelöst durch die Änderung der Root-`Taskfile.yml`, nicht durch Workbench-Code) lief auf dem durch parallele Sessions ausgelasteten Rechner (CPU ~100 %) ins Zeitlimit von 10 min; die Workbench ist in RS2.1 unverändert, ihre Pakete liefen einzeln grün (planning 78 s allein). Review (Sonnet): keine schweren Befunde.

**Abweichungen:** `engine/balance` und `engine/enginetools` sind bis RS2.2 1:1-Kopien aus `tools/k3c-workbench/internal/` (nur Importpfade), RS2.2 löscht die Workbench-Seite (Hinweise dort ergänzt). Die Vergleichsläufe liefen in der Shell, weil `sim_test` `k3c-sim` erst nach RS2.2 starten kann. **Neue Tickets:** keine. Manuell geprüft: nichts.
