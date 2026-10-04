# tools/k3c-dev/internal/balance/cmd/

## Responsibility
CLI `k3c-balance` (`package main`) über dem Paket `balance`: spielt eine Szenario-Matrix mit Bots, prüft Zielkorridore gegen eine Baseline und spielt Replay-Dateien ab. Reine Flag-Verdrahtung und Dateiausgabe, keine Balancing-Logik.

## Design
- Zwei Betriebsarten in `main.go`: Matrix-Lauf (`--seeds`, `--players`, `--bots`, `--days`, JSON-Bericht), Replay (`--play DATEI` über `playFile`, `--replay-dir` über `writeReplays`).
- `targets.go`: Modus `--targets` (`runTargets`, `targetOpts`) mit Baseline-Lesen/-Schreiben (`readBaseline`, `writeBaseline`), Bericht als JSON + Markdown (`writeSummary`) und `onlyFailSeeds` zum Eingrenzen der Replays auf verletzte Ziele.
- Verletzte Ziele sind kein Fehler (kein Pflicht-Gate, BAL2); nur ungültige Eingaben liefern Exit-Code != 0.
- Hilfen: `seedNames`, `split`, `ints` parsen Flag-Listen.

## Flow
1. `main` ruft `run(os.Args[1:])`; Fehler gehen nach stderr mit Präfix `k3c-balance:`.
2. Matrix-Modus: Flags → `balance.Matrix` → `balance.Run` → `Report.JSON` → `write`.
3. Mit `--replay-dir`: je gültigem Lauf `writeReplays` (Replay-Datei je Szenario).
4. `--targets`: `balance.DefaultTargets` → `Targets.RunTargets(n)` → `Targets.Summarize` → Vergleich mit Baseline (`balance.Compare`) → `writeSummary` (`reports/balance-<Zeit>.json/.md`).
5. `--play`: `balance.ReadReplay` → `balance.Play` → Hash-Vergleich ausgeben.

## Integration
- Abhängigkeit: `k3c/tools/k3c-dev/internal/balance`.
- Konsument: `Taskfile.yml` (`task balance`, `balance:baseline`, `balance:run`) baut `bin/k3c-balance` aus diesem Ordner; kein Go-Importer.
