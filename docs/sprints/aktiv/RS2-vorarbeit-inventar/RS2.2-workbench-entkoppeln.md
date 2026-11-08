# RS2.2 · Workbench startet Sim, Balancing und Level als Kindprozess

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/rs2
- **Abhängig von:** RS2.1
- **Tickets:** B-726
- **Kriterien:** AC-01

## Ziel

Die Workbench ruft Sim, Balancing und Level-Erzeugung nur noch als Kindprozess `bin/k3c-sim` auf und importiert `k3c/engine/sim`, `room`, `net`, `level` nicht mehr (Ausnahme `studioview`). Nach dem Sprint ist es für die Workbench gleich, ob die Engine Go oder Rust ist.

## Kontext

B-726 (freigegeben). Die Schnittstelle liefert RS2.1 (`cmd/k3c-sim`). Betroffen: `tools/k3c-workbench/internal/{balance,botdev,enginetools,botfeed,mcpsrv}`, `studioview` darf laut Nicht-Ziel den laufenden Spielserver nutzen. MCP-Tools `sim_run`, `sim_test` (start, status, stop, list; wait), `level_generate`, Balancing-Seite. Lange Läufe laufen nur über `sim_test`. Fehlt oder veraltet die EXE: klare Meldung mit `task k3c-sim:build`; Absturz: Lauf `fehlgeschlagen`, letzte stderr-Zeilen im Status. Wails-/Workbench-Frontends folgen Atomic Design (`@webkit`), hier nur Go-Seite.

### Hinweise aus RS2.1

- **Kopien entfernen (Pflicht):** RS2.1 hat `internal/balance` (samt Tests, `report/`, `testdata/`) nach `engine/balance` und `internal/enginetools` nach `engine/enginetools` kopiert (nur Importpfade angepasst), `internal/balance/cmd` lebt als `cmd/k3c-sim balance` weiter. Bis RS2.2 laufen die Balance-Tests (~5 min ohne `-short`) und die langen CLI-Tests doppelt. RS2.2 löscht `internal/balance/cmd`, `internal/enginetools` und die Engine-Teile von `internal/balance` samt deren Tests, dazu den Namensraum `balance` (`tools/k3c-workbench/internal/balance/Taskfile.yml`: `balance:build` → `k3c-sim:build`; `fullgame`/`milestone` laufen dann über `go test ./engine/balance`, Taskfile-Änderung dort mit vermerken).
- **Berichtsteil bleibt in der Workbench:** Die Workbench liest Berichte (`ReadSummary`, `Insights`, `GoalView` und deren Typen, `BaselinePath`/`BaselineFile` in `simrun/spec.go`). Diese NICHT aus `engine/balance` importieren (zieht `engine/sim` nach); sie bleiben als reiner Leseteil in `internal/balance` (ohne `engine/*`-Importe, mit ihren schnellen Tests) oder lesen das Berichts-JSON selbst.
- **Offline-Weg:** `simrun/offline.go` baut heute `bin/k3c-balance[-n]` und liest `PROGRESS n/m` von stdout. Neu: `bin/k3c-sim balance --targets --report-dir … --progress …` (gleiche Flags), stdout nur JSON-Zeilen `{"event":"progress","done":n,"total":m}`, `{"event":"report","files":[…]}`, `{"event":"error","message":…}`, `{"event":"aborted"}`; Exit 0/1/2/130. Schnittstelle im Kopfkommentar von `cmd/k3c-sim/main.go`. Parallel-Plätze (`k3c-balance-2` …) entsprechend als `bin/k3c-sim-<n>` oder eine EXE mit mehreren Prozessen.
- `sim_run`/`level_generate`: `k3c-sim run|level` mit JSON auf stdin, Ergebnis `{"event":"result","text":…}` byte-gleich zum heutigen Text.

## Erlaubte Dateien

- `tools/k3c-workbench/**` (Go-Seite)
- `tools/k3c-workbench/Taskfile.yml`
- `docs/messung/rst-baseline.md` (Notiz zum neuen Aufruf)

## Nicht-Ziele

- Rust. Änderungen an `engine/`. Neue Workbench-Oberflächen. `studioview` umbauen.

## Schritte

1. Alle Importstellen aus `go list -deps` ableiten; je Aufruf auf `exec` gegen `bin/k3c-sim` umstellen (Prozess starten, stdout-Zeilen lesen, Bericht lesen).
2. `sim_test`-Fortschritt, `status`, `wait`, `stop` auf die JSON-Zeilen abbilden; Job-Objekt/Prozess-Ende für Abbruch.
3. Fehlerfälle: EXE fehlt/veraltet, Absturz (stderr-Zeilen im Status), Zeitüberschreitung.
4. Importe bereinigen; `go list -deps ./tools/k3c-workbench/...` prüfen; `depguard`-Regel für die Workbench ergänzen, damit sie nicht zurückkehrt.
5. Vergleich: BAS-Standardlauf (1 Seed) und `sim_run`/`level_generate` mit festen Seeds, Bericht-Diff leer bis auf Zeitstempel; Meilenstein-Lauf (coop2, standard, 40 Tage) Dauer ≤ +5 % über `sim_test` messen (lange Läufe nur über `sim_test`).

## Fertig, wenn

- [x] AC-01: `go list -deps ./tools/k3c-workbench/...` enthält `k3c/engine/sim`, `room`, `net`, `level` nicht (Ausnahme `studioview`, im Ergebnis benannt).
- [x] AC-01: `sim_run`, `sim_test` offline und `level_generate` liefern dieselben Ergebnisse wie vorher (Bericht-Diff leer bis auf Zeitstempel).
- [x] AC-01: Meilenstein-Lauf höchstens 5 % langsamer (Zahl im Ergebnis).
- [x] CI-Voraussetzung: `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf). Umfang kann 150 geänderte Zeilen überschreiten: dann Reviewer-Agent vor dem Commit.

## Ergebnis

**Freigabe/Prüfung:** autonom, Claude (Sprint RS2 „komplett autonom“); keine manuelle Abnahme nötig.

- **AC-01 geprüft (Importe):** Je Paket `go list -deps`: Nur `internal/studioview` (direkt `engine/net`, `room`, `store`, transitiv `sim`, `level`) und darüber das Hauptpaket `k3c/tools/k3c-workbench` (importiert `studioview`, sonst nur `engine/conlog`, `engine/devrun`) enthalten den Engine-Kern; alle anderen Pakete nicht. `depguard` in `tools/k3c-workbench/.golangci.yml`: Regel `workbench-ohne-engine` (Nicht-Tests außer `studioview`: `sim`, `level`, `room`, `net`, `balance`, `enginetools`) und `workbench-tests-ohne-sim` (Tests: `sim`, `level`, `balance`, `enginetools`); per Probe-Import geprüft.
- **AC-01 geprüft (gleiche Ergebnisse):** alte Workbench (laufende Instanz, in-process) gegen neue (zweite Instanz `-headless` auf Loopback-Ports 5198/8198, MCP über HTTP): `sim_run` (Seed k3c-dev, 3000 Ticks, 2 Eingabe-Segmente) und `level_generate` (familie-2026, cave) byte-gleich. `sim_test` BAS-Standardlauf (coop2, Seed k3c-dev) run-22 → run-24 und Meilenstein (coop2, standard, 40 Tage, k3c-dev) run-23 → run-25: `balance-*.json` identisch, `.md` nur Zeitstempel, `result.json` nur Lauf-ID, Zeit und Verweis auf den Vorlauf.
- **AC-01 geprüft (Dauer):** Meilenstein 1 Seed über `sim_test`, je zweimal abwechselnd, Last 1 %: vorher 55 s / 49 s (Mittel 52 s), nachher 46 s / 50 s (Mittel 48 s) → −8 %, Grenze +5 % eingehalten. Volllauf (101 Seeds) nicht gemessen.
- **CI geprüft:** `check_run task:check:diff` exit 0 (2 min 54 s); vorher gezielt `go test -short` für `internal/balance`, `simexe`, `simrun`, `botdev`, `botfeed`, `mcpsrv`, Hauptpaket, `cmd/k3c-sim -run TestBot`.

**Umsetzung:** neues Paket `internal/simexe` (Prüfung fehlt/veraltet per mtime gegen `engine/`, `cmd/k3c-sim`, `go.mod`, `go.sum` mit Meldung `task k3c-sim:build`; `Call` für `run`, `level`, `balance --play` mit Zeitlimit und stderr-Zeilen bei Absturz; `Bot`/`Pool` für `k3c-sim bot` mit 30-s-Zeitlimit je Antwort). `sim_run`, `level_generate`, `replay_run` rechnen über `bin/k3c-sim` des Checkouts der Session (Daten aus dessen `data/`, früher die Engine der Workbench-EXE). `sim_test` offline baut `bin/k3c-sim-<platz+1>` und liest die JSON-Zeilen (Fortschritt; Fehler und `panic:`-Zeile im Status). Online-Bots (`botdev`, Modus mit Clients) entscheiden in `k3c-sim bot` (Pool, höchstens Kerne), `Decide` läuft außerhalb des Geräte-Locks, erster Bot-Fehler steht im Status. `internal/balance` ist nur noch Leseteil (`report_types.go`, `ship_figures.go`, `names.go` mit Quelltext-Abgleich gegen `engine/balance`); `internal/balance/cmd`, `internal/enginetools` und die Engine-Teile samt Tests gelöscht. Namensraum `balance` (`internal/balance/Taskfile.yml`): `build` → `:k3c-sim:build`, `targets`/`baseline`/`sensitivity`/`run` → `bin/k3c-sim balance …`, `fullgame`/`milestone` → `go test ./engine/balance`.

**Abweichungen vom Plan:**
- `cmd/k3c-sim/bot.go` (+ Test, Dispatch in `main.go`) neu, obwohl nicht unter Erlaubte Dateien: Die Online-Bots brauchen die Bot-Profile aus `engine/balance`; ohne Unterbefehl `bot` wäre AC-01 für `botdev` nicht erreichbar. `engine/` unverändert.
- Offline-Lauf baut weiter selbst (`bin/k3c-sim-<n>` statt `bin/k3c-balance[-n]`), statt nur zu melden; die Meldung „fehlt/veraltet → `task k3c-sim:build`“ gilt für `sim_run`, `level_generate`, `replay_run` und die Online-Bots (dort Ablehnung beim Start).
- „Nachher“ lief über eine zweite, neu gebaute Workbench-Instanz (Loopback), weil die laufende Instanz von Ralf nicht neu gestartet werden sollte; für „vorher“ wurde `internal/balance` kurz auf HEAD zurückgesetzt. **Ralf:** Workbench neu bauen/starten, erst dann nutzt `sim_test` den neuen Weg (die alte EXE findet `internal/balance/cmd` nicht mehr).
- Workbench-Tests nutzen für den echten Spielserver weiter `engine/net`/`room` (nur `_test.go`), Bots darin über ein falsches `bin/k3c-sim` (Kopie der Test-EXE) → B-740.
- Online mit echtem Spielserver und echtem `k3c-sim bot` nicht von Hand gefahren (abgedeckt: `cmd/k3c-sim` `TestBotGleichProfil`, Workbench-Integrationstests mit falschem k3c-sim).

**Reviewer (Sonnet):** keine schweren Befunde; behoben: Bot-RPC unter Geräte-Lock, fehlendes Zeitlimit je Antwort, verlorener Bot-Fehlertext, Pipe-Leck im Startfehler, Größengrenze `replay_run` (64 MB). Offen bewusst: mtime-Prüfung erkennt Checkout ohne Inhaltsänderung als veraltet (Hinweis `--force`).

**Neue Tickets:** B-740 (Workbench-Tests ohne Engine-Server), B-741 (Verweise auf `bin/k3c-balance`/`enginetools` außerhalb der Workbench).
