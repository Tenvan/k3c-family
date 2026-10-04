# BAL2.2 · `task balance`: Bericht, Seeds, Baseline und Regressions-Vergleich

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** bal2/2-task-balance-bericht
- **Abhängig von:** BAL2.1
- **Tickets:** B-157
- **Kriterien:** AC-03, AC-04, AC-05

## Ziel

`task balance` läuft über die 100 festen Seeds, schreibt Pass/Fail je Kennzahl als JSON und Markdown mit den Seeds verletzter Ziele und vergleicht mit einer Baseline: „Wertänderung → welche Ziele kippen“.

## Kontext

- Anforderungen: B-157 › Anforderungen und Ausnahme- und Fehlerfälle (Lauf bricht ab → `ungültig` mit Seed; fehlende Baseline → Hinweis statt Fehler).
- Bausteine: Läufer und Bots aus BAL1.1, Replay aus BAL1.2 (Seeds zum Nachspielen sollen sich als Replay erzeugen lassen), Korridore und Bewertung aus BAL2.1.
- **Ort der Ausgabe** (B-157: legt die Session fest): Vorschlag `reports/balance-<Zeit>.json` und `.md` für Läufe, Baseline eingecheckt unter `testdata/balance/baseline.json` (Muster Golden-Dateien, `docs/arbeitsweise.md` › „Golden aktualisieren“: Update nur mit Begründung im Commit). Wahl im Ergebnis begründen.
- Regressions-Vergleich: aktuelle Kennzahlen gegen Baseline; ausgegeben werden die Ziele, deren Bewertung sich ändert (z. B. Pass → Fail). Test mit geänderten Daten: über eine Test-Option des Läufers (nicht durch Ändern von `data/*.json` im Repo).
- Laufzeit: 100 Seeds × Szenarien; im Ergebnis messen (Rechner und Dauer). Ist sie zu lang für den Alltag, Flag für weniger Seeds (BAL2.3 braucht das ohnehin).
- **Taskfile:** `task balance` (EXE mit festem Pfad, kein `go run`), kein Teil von `task check`.

## Erlaubte Dateien

- Balance-Paket und Befehl aus BAL1 (Bericht, Vergleich, Tests)
- `Taskfile.yml` (nur Task `balance`, ggf. `balance:baseline`)
- `testdata/balance/` (neu: Baseline)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

CI-Lauf (BAL2.3), Pflicht-Gate, Sensitivitäts-Analyse und neue Profile (BAL3), Ändern von `data/*.json`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Bericht JSON (maschinenlesbar) und Markdown (Tabelle je Ziel: Wert, Grenzen, Bewertung, Seeds bei Verletzung).
3. Baseline schreiben/lesen, Vergleich mit Liste gekippter Ziele; fehlende Baseline → Hinweis.
4. `task balance` und Baseline erzeugen (Begründung im Commit-Text).
5. Tests: Bericht nennt Seeds verletzter Ziele; Vergleich nennt gekippte Kennzahlen nach Wertänderung; ungültiger Lauf erscheint mit Seed.
6. Ein voller Lauf über 100 Seeds, Bericht und Laufzeit ins Ergebnis. `task check:go`. `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-03: `task balance` über 100 feste Seeds schreibt Pass/Fail je Kennzahl als JSON und Markdown (Lauf im Ergebnis).
- [x] AC-04: Bericht nennt je verletztem Ziel Kennzahl, Szenario und Seeds (Test).
- [x] AC-05: Test belegt die Liste gekippter Kennzahlen gegenüber der Baseline.
- [x] `task check:go` grün.

## Prüfen

```bash
task balance
task check:go
```

## Ergebnis

**Umgesetzt** (Balance-Paket `tools/k3c-dev/internal/balance/`):
- `summary.go`: `Targets.RunTargets(n)` spielt die Szenarien der Ziele über die festen Seeds (n = erste n, 0 = alle), `Summarize` bewertet (Pass = ok oder knapp), `Summary.JSON` und `Summary.Markdown` schreiben den Bericht, `Compare` nennt Ziele mit geändertem Status (`gekippt`, wenn Pass wechselt; `neu`/`entfällt` bei geänderter Zielliste). Fehlende Baseline: Hinweis im Markdown, kein Fehler.
- `cmd/targets.go` und Flags in `cmd/main.go`: `--targets`, `--seeds N`, `--report-dir`, `--baseline`, `--write-baseline`, `--replay-dir` (mit `--targets` nur Replays der Seeds verletzter Ziele, per `--play` nachspielbar). Verletzte Ziele sind kein Fehler (kein Gate).
- `Taskfile.yml`: `task balance` (EXE `bin/k3c-balance`, Bericht + Vergleich) und `task balance:baseline`; beide nicht in `task check`.

**Festlegung Ausgabe** (wie vorgeschlagen): Läufe nach `reports/balance-<Zeit>.json` und `.md` (gitignoriert, Zeit nur im Dateinamen), Replays verletzter Seeds nach `reports/balance-replays/`; Baseline eingecheckt unter `testdata/balance/baseline.json`, selbst ein Summary-JSON (kein zweites Format), Update nur mit Begründung im Commit.

**Baseline-Begründung:** Erstanlage aus dem vollen Lauf über 100 Seeds mit den Werten aus BAL2.1; sie hält den Ist-Stand fest, auch das Fail von „Burg hält Nacht 1–5“ (B-217).

**Messung:** voller Lauf `task balance` (100 Seeds, 2 Spieler, `saver`, Wald, 5 Tage = 100 Läufe) 1 min 37 s auf dem Windows-Entwicklungsrechner (ein Kern, sequenziell); `--seeds N` verkürzt (BAL2.3). Ergebnis: Burg hält 47,0 % (Ziel 75–90 %: Fail, 53 Seeds, Liste im Bericht), Erste Mauer 100 % (Pass), zerstörte Gebäude je Welle Median 0 (Pass); Vergleich mit der Baseline: keine Änderung.

**Nachweis:**
- AC-03: `task balance` über 100 Seeds, Bericht `reports/balance-20261004-143439.json` und `.md` (Lauf oben); `TestBerichtNenntSeeds` prüft JSON-Felder `pass` und `failSeeds`.
- AC-04: `TestBerichtNenntSeeds` (Zeile mit Ziel, Szenario, Seeds 2, 3), `TestBerichtUngueltigMitSeed` (abgebrochener Lauf mit Seed).
- AC-05: `TestVergleichNenntGekippte` (Untergrenze 70 → 85 % als Test-Option: „x: ok → verletzt gekippt“, unverändertes Ziel fehlt), `TestBaselineLesen` (neu, entfällt).
- `task check`, `task check:go`, `task check:dev` grün (`-race` übersprungen: kein C-Compiler).

Neues Ticket: B-217 (Burg hält nur 47 %, Ursache Bot oder Balance).
