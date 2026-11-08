# tools/k3c-workbench/internal/balance/

## Responsibility
Leseteil des Balancing-Testers in der Workbench (seit RS2.2, B-726): liest Berichte von `bin/k3c-sim balance`
(`balance-*.json`, `tuning-*.json`), bewertet sie gegen die Zielkorridore neu, baut die Auswertung (Insights, Markdown,
HTML) und verwaltet Baselines. Gerechnet wird nicht hier, sondern in `engine/balance` (Bots, Matrix, Kennzahlen,
Replays, Sensitivität, Abstimmen) über das Engine-CLI `cmd/k3c-sim`. Kein Import aus `engine/*` (depguard).

## Design
- Lese-Typen (`report_types.go`): `Report`, `Result`, `Scenario`, `Matrix`, `Metrics` samt Tages-, Wellen-, Morgen- und
  Insel-Kennzahlen, `ShipNeed`; Felder und Reihenfolge wie `engine/balance`, `Stock`/`Cost` als Kopien der JSON-Form.
- Namen (`names.go`): `BotNames`, `KnownBot` (Mischung mit `MixSep`), `HelpsNames`, `Worlds`; `names_test.go` hält sie per
  Quelltext-Vergleich gleich mit `engine/balance/bots.go` und `helps.go`.
- Zielkorridore (`targets.go`, `evaluate.go`, `scope.go`, `seedmode.go`): `Targets` aus `data/balance-targets.json`,
  `Evaluate`, `Summarize`, `Compare`, `Restrict`.
- Auswertung (`insights*.go`, `hints.go`, `figures*.go`, `ship_figures.go`, `progress.go`, `curves.go` › `Grades`):
  `BuildInsights`, `Insights.Markdown`/`HTML` (`report_html.go`, Vorlage `report/*`), `WriteReports`, `Reevaluate`.
- Abstimmen (`tune.go`, `tuneparam.go`): Lese-Typen `Tuning`, `TuneBest`, Grenzen; `ParseTuneParam` für Vorschläge.
- Baselines (`baselines.go`, `delta.go`, `groups.go`): sichern, listen, Delta je Werte-Gruppe.

## Integration
- Abhängigkeiten: `k3c/data` (Zielkorridore, Hinweise, Welten).
- Konsumenten: `internal/simrun` (Berichte, Baselines, Spec-Prüfung), `internal/mcpsrv` (`sim_test`), `app_balance.go`.
- Handweg: `Taskfile.yml` (Namensraum `balance`) ruft `bin/k3c-sim balance` bzw. `go test ./engine/balance`.
