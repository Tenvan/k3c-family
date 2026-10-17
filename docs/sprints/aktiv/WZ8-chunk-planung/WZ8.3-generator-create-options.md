# WZ8.3 · Generator mit Planung, CreateOptions und Benchmark

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** wz8/3-generator-create-options
- **Abhängig von:** WZ8.2
- **Tickets:** B-461
- **Kriterien:** AC-02, AC-03, AC-04, AC-05, AC-07, AC-13

## Ziel

Der Generator setzt geplante Chunks deterministisch ein (eigener Strom, feste Objekte auf die Biom-Summen angerechnet); Planung und Mauerlinien sind Parameter (`level.GenerateWith`); `sim.CreateIsland` nimmt `sim.CreateOptions`, alle Aufrufer laufen mit Nullwert unverändert. Benchmark mit Baseline und neuer Golden-Fall mit Test-Planung liegen vor.

## Kontext

Spec: B-461 und Sprint-README WZ8 (Anforderungen, Regeln, Beispiele), Regeltext aus WZ8.1, Prüfung und Typen aus WZ8.2.

Stand: `engine/level/level.go` würfelt nacheinander aus dem Haupt-Strom `<biom>:<seed>` (Reihenfolge ist Vertrag, `testdata/golden/level-*.json`): `frame()`, Portale, `eventChunks`, übrige Chunks nach `chunkWeights`, Ressourcen, Skill-Punkte, Adern. `clearOfLines` und `generateLines` lesen die Paket-Variable `wallLines`. `sim.CreateWorld` übernimmt die Level-Objekte, `createIsland` erzeugt alle Stufen mit demselben Seed. `sim.IslandOptions` (`engine/sim/island_options.go`) wird gespeichert und bleibt unberührt.

Regeln: Ohne Planung ändert sich Reihenfolge und Zahl der Würfe nicht. Mit Planung reserviert der Generator die Plätze direkt nach `frame()`; Portale, Ereignisse, Arten, Skill-Punkte und Adern würfeln nur über freie Chunks (Verschiebung gegenüber dem Level ohne Planung gewollt). Geplanter Inhalt würfelt nur aus dem eigenen Strom (Namen laut WZ8.1). `sim.CreateOptions` ist NEU, nur für die Erzeugung, wird NICHT gespeichert (`IslandSaveVersion` unverändert). `level.Generate(b, seed)` bleibt und ruft `GenerateWith` mit Nullwert. Schichten: `engine/level` importiert nichts aus `sim`/`room`/`net`.

Aufrufer von `CreateIsland` (mechanisch, Nullwert): `engine/room/fullsave.go`, `engine/room/manager.go`, `tools/k3c-dev/internal/balance/run.go`, Testdateien (heute 16; per Grep bestätigen).

**Zwei Commits:** erst der Code-Commit (ohne Golden-Dateien, Benchmark-Messwerte im Commit-Text), danach der Golden-Fall mit Test-Planung (`testdata/`) als eigener Commit mit Begründung. Das ist die Ausnahme von „ein Commit je Session“ (`arbeitsweise.md` › Golden aktualisieren). `rng.json` nie ändern.

## Erlaubte Dateien

- `engine/level/` (Generator mit Planung und Linien-Parameter, Benchmark), zugehörige Tests
- `engine/sim/` (`CreateOptions`, `CreateIsland`, `CreateWorld`, `createIsland`), zugehörige Tests
- Grenzfall, mechanisch mit Nullwert: `engine/room/fullsave.go`, `engine/room/manager.go`, `tools/k3c-dev/internal/balance/run.go` und die Testdateien, die `CreateIsland` aufrufen
- `testdata/` (Test-Planung; nur im zweiten Commit die neue Golden-Datei)
- Planungsdateien von WZ8 über k3c-dev

## Nicht-Ziele

Vorschau-Funktion und Welt-Betrachter (WZ8.4, WZ8.5); Spielstand-Format oder `IslandSaveVersion` ändern; `IslandOptions` ändern; echte Planungen in `data/islands.json`; bestehende Golden-Dateien ändern; Ruinen und Gegnerlager (B-468); neue Sim-Logik für feste Objekte; Verhaltensänderung in `engine/room` oder `tools/k3c-dev` über das Durchreichen des Nullwerts hinaus.

## Schritte

1. **Baseline:** Benchmark der Level-Erzeugung anlegen (alle Biome, ohne Planung) und vor jeder Änderung am Generator messen; Werte notieren.
2. Tests zuerst (rot): Determinismus, Chunk-Seed über 100 Level-Seeds stabil, Inhalt nur aus eigenem Strom (AC-04), Summen über 500 Seeds je Biom mit `level.Validate` (AC-05), Linien-Parameter.
3. `level.Options` und `level.GenerateWith`; `clearOfLines` und `generateLines` nehmen die Linien als Argument; `Generate` bleibt. Planung in eigener Datei (`planned.go`).
4. Generator: Plätze reservieren, geplante Chunks mit Art setzen, Inhalt aus eigenem Strom oder festen Objekten, feste Objekte auf die Biom-Summen anrechnen, übrige Portale abwechselnd (laut WZ8.1).
5. `sim.CreateOptions` (Planung je Tiefe), `CreateIsland` und `createIsland`/`CreateWorld` durchreichen; alle Aufrufer anpassen (Nullwert).
6. Sim-Test mit 2 Spielern: fester Baum abbaubar, feste Truhe aufhebbar, festes Camp liefert Landstreicher, Welle aus festem Portal (AC-07).
7. Benchmark nach den Änderungen: ohne Planung ≤ +10 % je Biom, mit Test-Planung ≤ +10 % gegenüber ohne (AC-13). Überschreitung: Ursache beheben oder Grenze mit 🧑 neu festlegen (Spec-Revision).
8. `task check:go`, `task golden:update` darf noch keinen Diff erzeugen. **Code-Commit** `feat(sim): Geplante Chunks im Generator (WZ8.3)` mit Messwerten.
9. Test-Planung unter `testdata/`, `task golden:update`, `git diff --stat testdata/golden` zeigt nur die neue Datei; **zweiter Commit** `test(sim): Golden-Fall geplante Chunks (WZ8.3)` mit Begründung.

## Fertig, wenn

- [ ] AC-02: nach dem Code-Commit erzeugt `task golden:update` keinen Diff; `git diff --stat testdata/golden` im zweiten Commit zeigt nur die neue Datei.
- [ ] AC-03: Test mit Test-Planung deterministisch; Chunk-Seed über 100 Level-Seeds stabil; eingebettete Linien explizit übergeben = Layout ohne Parameter; verschiedene Linien nacheinander liefern je ihr Layout.
- [ ] AC-04: Test: Änderung von Chunk-Seed oder Ressourcen-Objekt eines geplanten Chunks lässt alle nicht geplanten Chunks gleich.
- [ ] AC-05: `level.Validate` über 500 Seeds je Biom mit Beispiel-Planung leer.
- [ ] AC-07: Sim-Test mit 2 Spielern grün; alle Aufrufer mit Nullwert unverändert; `IslandSaveVersion` unverändert.
- [ ] AC-13: Benchmark-Messwerte (Baseline, ohne Planung, mit Planung) im Commit-Text, Grenzen eingehalten.
- [ ] Zwei Commits wie beschrieben; `task check:go` grün.

## Prüfen

```bash
task check:go
task golden:update
git diff --stat testdata/golden
```

Benchmark und Sim-Läufe mit 2 Spielern bei laufender Workbench über `check_run` bzw. `sim_test`, nie in der Shell. Manuelle Prüfungen nennt diese Session nicht.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
