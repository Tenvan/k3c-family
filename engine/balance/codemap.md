# engine/balance/

## Responsibility

Balancing-Tester (Test Harness / Batch Runner): spielt eine Szenario-Matrix headless mit Bots über `engine/sim`,
misst Kennzahlen je Lauf, bewertet sie gegen Zielkorridore und schreibt Berichte (JSON, Markdown, HTML). Gleiche Matrix
und gleiche Daten ergeben byte-gleiche Ausgabe (keine Wanduhr, keine Map-Iteration in der Ausgabe).

## Design

- **Lauf-Kern** (`run.go`): `Matrix`, `Scenario`, `Result`, `Report`, `Scenarios`, `Run`; Obergrenze
  `maxTicksPerDay`, Takt `enginetools.TickHz`. `island_world.go` baut die Insel des Szenarios (eine Stufe oder alle
  Tiefen einer Welt, `Worlds`); `parallel.go` rechnet Läufe parallel (je Lauf eigene Insel); `scope.go` (`Scope`,
  `Restrict`: `--depth`, `--players`, `--bot`, `--grade`, `--days`); `seedmode.go` (`WithSeeds`).
- **Bots** (Strategy): `bots.go` (`Bot` liefert aus dem Zustand nur einen `sim.PlayerCommand`; `BotNames`,
  `BotByName`), Profil `bot_wirtschaft.go` und Verhaltensteile `bot_abstieg.go` (Abstieg), `bot_boss.go` (Bosskampf),
  `bot_clear.go` (Freiraum für Plätze), `bot_learn.go` (Skills lernen), `crown_bot.go` (`retrieveCrown`, B-931: ein Bot ohne Krone hebt die nächste liegende Krone auf, trägt er eine fremde, bringt er sie zum Besitzer; `run.go` ruft es vor dem Profil auf); `helps.go` (`HelpsNames`, Hilfen wie im
  Stufen-Test).
- **Messung**: `metrics.go` (`Metrics`, `DawnMetrics`, `DayMetrics`, `WaveMetrics`), `metrics_island.go`
  (`StageMetrics`, `BossMetrics`, `IslandMetrics`), `economy_measures.go` (Wirtschafts-Korridore), `progress.go`
  (Meilenstein-Leiter: `Progress`, `MilestoneRow`, `RunConfig`), `ship_need.go` (Schiff-Bedarf gegen Bestand).
- **Bewertung**: `targets.go` (`Targets`, `Target`, `DefaultTargets`, `LoadTargets` aus `data/balance-targets.json`),
  `evaluate.go` (`Verdict`, `Evaluate`), `summary.go` (`Summary`, `RunTargets`, `RunTargetsProgress`, `Summarize`,
  `ReadSummary`, `Compare`, `Markdown`), `figures.go`/`figures_island.go` (`Figures`, `FiguresOf`, `IslandFigures`),
  `groups.go` (Werte-Gruppen, `IsFrozen`), `curves.go` (`GradeCurve`, `Curves` je Grad aus `data/difficulty.json`).
- **Baselines** (`baselines.go`, `delta.go`): benannte Baselines (`SaveBaseline`, `ListBaselines`, `HasBaseline`) und
  Delta-Bericht je Gruppe (`GroupDelta`).
- **Auswertung ohne KI** (`insights*.go`, B-632): `BuildInsights` (Ampel je Ziel und Kennzahl, Trend, Baseline),
  Richtwerte aus `data/balance-hints.json` (`hints.go`: `Hints`, `LoadHints`, `Light`), Hebel (`insights_levers.go`),
  Texte (`insights_text.go`, `insights_plain.go`), nächster Lauf (`insights_next.go`: `NextRun`, `Command`), Fortschritt
  (`insights_progress.go`), Markdown (`insights_markdown.go`); `insights_files.go` (`WriteReports`, `Reevaluate`,
  `ReadBaseline`) schreibt `.json`, `.md` und `.html`.
- **HTML-Bericht** (`report_html.go`): eine Datei ohne externe Ressourcen; Vorlage, Stil und Skripte per `embed` aus
  `report/` (`report.html`, `report.css`, `report-*.js`).
- **Werte variieren**: `vary.go` (Pfad `datei.json:a.b.c`), `variant.go` (`VariantFS`, `WithVariants`: feste Werte im
  Lauf), `sensitivity.go` (`Sensitivity`, `SensitivityReport`; Werte über `sim.UseData` in `vary.go`), `tune.go`/`tuneparam.go` (`Tune`,
  `TuneOptions`, `ParseTuneParam`: Sweep oder Suche über ein bis zwei Werte).
- **Replay** (`replay.go`): `Replay` (Szenario, Datenstand-Hash `DataHash`, Eingaben je Tick als `Span`),
  `ReadReplay`, `Play`/`Playback`.

## Flow

1. Aufrufer setzt `Matrix`/`Scope` (bzw. `RunTargets` mit `DefaultTargets`).
2. Je Szenario: Insel bauen → Bots liefern Befehle → `sim.StepIsland` je Tick bis Tagesende oder Burgfall →
   `Metrics` sammeln.
3. `Summarize`/`Evaluate` gegen die Zielkorridore → `BuildInsights` → `WriteReports` (`balance-<Zeit>.json`, `.md`,
   `.html`), optional Vergleich mit Baseline.
4. Varianten, Sensitivität und Tuning wiederholen Schritt 2–3 mit geänderten Werten über ein überlagertes `fs.FS`.

## Integration

- Abhängigkeiten: `k3c/engine/sim`, `k3c/engine/enginetools`, `k3c/data` (`balance-targets.json`,
  `balance-hints.json`, `difficulty.json`, `bosses.json`, `ship.json` u. a.).
- Konsument: `engine/simcli` (Unterbefehl `balance`, von `cmd/k3c-sim` und der Workbench im Prozess gerufen: `Run`, `Play`, `ReadReplay`, `WithVariants`, `Reevaluate`,
  `WriteReports`, `FiguresOf`); die Workbench erreicht es über `simcli` (`sim_test`, Balancing-Seite).
