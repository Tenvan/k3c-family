# tools/k3c-workbench/internal/balance/

## Responsibility
Leseteil des Balancing-Testers in der Workbench (seit RS2.2, B-726): liest Berichte von `k3c-game sim balance`
(`balance-*.json`, `tuning-*.json`), bewertet sie gegen die Zielkorridore neu, baut die Auswertung (Insights, Markdown,
HTML) und verwaltet Baselines. Gerechnet wird nicht hier, sondern in `engine/balance` (Bots, Matrix, Kennzahlen,
Replays, Sensitivität, Abstimmen) über die Spiel-EXE (`k3c-game sim balance`). Kein Import aus `engine/*` (depguard).

## Design
- Lese-Typen (`report_types.go`): `Report`, `Result`, `Scenario`, `Matrix`, `Metrics` samt Tages-, Wellen-, Morgen- und
  Insel-Kennzahlen, `ShipNeed`; `Downs` zählt das Ereignis `crownFallen`, `Lost` (`"crown"`) markiert einen durch Kronenraub verlorenen Lauf; Felder und Reihenfolge wie `engine/balance`, `Stock`/`Cost` als Kopien der JSON-Form.
- Namen (`names.go`): `BotNames`, `KnownBot` (Mischung mit `MixSep`), `HelpsNames`, `Worlds`; `names_test.go` hält sie per
  Quelltext-Vergleich gleich mit `engine/balance/bots.go` und `helps.go`.
- Zielkorridore (`targets.go`, `evaluate.go`, `economy_measures.go` für die Wirtschafts-Korridore, `scope.go`, `seedmode.go`): `Targets` aus `data/balance-targets.json`,
  `Evaluate`, `Summarize`, `Compare`, `Restrict`.
- Auswertung (`insights*.go`, `hints.go`, `figures*.go`, `ship_figures.go`, `progress.go`, `curves.go` › `Grades`):
  `BuildInsights`, `Insights.Markdown`/`HTML` (`report_html.go`, Vorlage `report/*`), `WriteReports`, `Reevaluate`.
- Abstimmen (`tune.go`, `tuneparam.go`): Lese-Typen `Tuning`, `TuneBest`, Grenzen; `ParseTuneParam` für Vorschläge.
- Baselines (`baselines.go`, `delta.go`, `groups.go`): sichern, listen, Delta je Werte-Gruppe.

## Flow
1. `simrun.WriteHTML` → `Reevaluate(jsonPath, root)` → `ReadSummary` + `ReadBaseline` (`BaselinePath`) → `WriteReports` → `BuildInsights` → `Insights.Markdown`/`HTML` (`report_html.go` mit `report/report.html`, `.css`, `.js`) → `.md` und `.html` neben dem Bericht.
2. `simrun.ReadReport`/`ReadTuning`/`ReadInsights` lesen `Summary`, `Tuning`, `Insights` für die Balancing-Seite und `sim_test`.
3. Baselines: `simrun.SaveBaseline` und `sim_test {action: baseline}` → `baselines.go`; Vergleich je Werte-Gruppe über `delta.go`/`groups.go`.

## Integration
- Abhängigkeiten: `k3c/data` (Zielkorridore, Hinweise, Welten).
- Konsumenten: `internal/simrun` (`baseline.go`, `reports.go`, `spec.go`), `internal/mcpsrv` (`simtest.go`, `simtest_baseline.go`, `simtest_spec.go`), `tools/k3c-workbench/app_balance.go`.
- Handweg: `Taskfile.yml` (Namensraum `balance`) ruft `k3c-game sim balance` bzw. `go test ./engine/balance`.
