# tools/k3c-dev/internal/balance/

## Responsibility
Balancing-Tester (B-099, BAL1/BAL2): spielt eine Szenario-Matrix (Seed x Spieleranzahl x Bot x Tiefe) headless gegen die Go-Simulation, sammelt Kennzahlen, bewertet sie gegen Zielkorridore und erzeugt/spielt Replays. Ausgabe ist deterministisch und byte-gleich bei gleichen Daten.

## Design
- Strategy: `Bot` (`bots.go`) = `func(*sim.World, *sim.Player) sim.PlayerCommand`, registriert in der Map `bots` (`passive`, `saver`, `walls`, `economy`, `coop2`, `coop4`; Verhalten: `README.md`); `BotNames` sortiert, `minPlayers`/`checkPlayers` = Mindest-Spieleranzahl je Profil (Fehler in `Matrix.Scenarios` und `Target.validate`). Zufall nur über `engine/rng`.
- Matrix/Runner (`run.go`): `Matrix`, `Scenario`, `Result`, `Report`; `Matrix.Scenarios` validiert und zählt auf, `Run`/`runOne` spielen sequenziell, ein abgebrochener Lauf wird als ungültig (`Error`) vermerkt.
- Collector (`metrics.go`): `collector` beobachtet Welt und `sim.Event`s je Tick und liefert `Metrics` mit `DayMetrics`/`WaveMetrics`.
- Replay (`replay.go`): `Replay` mit `Span`-Lauflängen je Spieler, `ReadReplay` (Version `ReplayVersion`=1, Zeilennummern in Fehlern), `Play` rechnet ohne Bot nach und vergleicht Endzustand-Hash (`endHash`) und `DataHash` des eingebetteten Datenstands.
- Zielkorridore (`targets.go`, `evaluate.go`): `Targets`/`Target` aus `data/balance-targets.json` (`measures`-Map: Anteil in % oder Median), `Targets.Evaluate` liefert `Verdict` (ok/knapp/verletzt) mit `FailSeeds`.
- Sensitivität (`vary.go`, `sensitivity.go`): `variedFS` ändert einen Wert `datei.json:a.b.c` um ±10/±25 % in einer Kopie von `data.Files` im Speicher (`fstest.MapFS`), `withData` lädt sie per `sim.UseData` und `loadPrices` und stellt danach `data.Files` wieder her; `Targets.Sensitivity` vergleicht die Bewertungen je Variation mit dem unveränderten Lauf (`Flip`), `SensitivityReport.JSON`/`Markdown`.
- Grad-Kurven (`curves.go`): `Grades` (aus `data/difficulty.json`, nach Wellengröße), `Curves` spielt Wald, 2 Spieler, `saver` je Grad (`Matrix.Grade` → `Scenario.Grade` → `sim.SetGrade` in `newIsland`) und liefert je Nacht Anteil Burg hält, Median zerstörte Gebäude, Median Gold zur Dämmerung.
- Summary (`summary.go`): `Summary`/`Entry`/`Change`, `RunTargets`, `Summarize`, `Compare` gegen Baseline (selbst ein Summary-JSON), `Summary.Markdown`.

## Flow
1. `DefaultTargets` lädt `balance-targets.json` aus `data.Files` und validiert.
2. `Targets.RunTargets(n)` baut `Matrix` aus den Zielszenarien, ruft `Run`.
3. `runOne`: `newIsland` → je Tick Bot-Befehle (`Replay.record`) → `sim.Step` mit `enginetools.TickHz` → `collector.observe/event` → `Metrics`.
4. `Summarize` → `Evaluate` je Ziel → `Compare` zur Baseline → `JSON`/`Markdown`.
5. Replay: `Play` setzt Befehle aus `Span`s ein und liefert `Playback` mit Warnings bei abweichendem Datenstand.

## Integration
- Abhängigkeiten: `k3c/engine/sim`, `k3c/data` (embed), `internal/enginetools` (`TickHz`, `MaxTicks`).
- Konsumenten: `internal/mcpsrv/tools_replay.go` (MCP-Tool Replay), CLI `internal/balance/cmd`.
