# WZ8.2 · Feld plannedChunks und Prüfung der Planung

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** wz8/2-feld-planung-pruefung
- **Abhängig von:** WZ8.1
- **Tickets:** B-461
- **Kriterien:** AC-02, AC-06

## Ziel

`data/islands.json` kennt das leere Feld `plannedChunks`; eine vom Seed unabhängige Prüfung lehnt jede ungültige Planung beim Laden mit Insel, Tiefe, Platz und Grund ab. Die Prüfung nimmt die Mauerlinien als Parameter. Ohne Planung ändert sich nichts (Golden byte-gleich).

## Kontext

Spec: B-461 und Sprint-README WZ8 (Anforderungen, Ausnahme- und Fehlerfälle, Messwerte) sowie der Regeltext aus WZ8.1 in `docs/rules/`; bei Abweichung gilt der Regeltext. Beginnt erst, wenn WZ8.1 `fertig` und die Spec `freigegeben` ist.

Stand des Codes: `engine/level/level.go` (`frame()` würfelt die Länge, `eligible()`, `chunkWeights`), `engine/level/validate.go`, `engine/level/lines.go` (lädt die Mauerlinien einmal in die Paket-Variable `wallLines`, Typ `wallLineData` unexportiert), `engine/sim/island_data.go` (`loadIslands`), `data/islands.json` (je Insel `depths`, `depthScaling`), `data/embed.go`. Ein ungültiges Feld lässt den Start scheitern wie heute bei `loadIslands`.

Entwurf der Namen (Annahme, B-461): Options-Struktur in `engine/level`, Prüfung als Funktion mit Linien als Argument, `wallLineData` exportiert. `level.GenerateWith` entsteht erst in WZ8.3. Die Planung kommt in eine eigene Datei (z. B. `engine/level/planned.go`); `level.go` nicht aufblähen (Datei ≤ 400, Funktion ≤ 60).

Rechnung der Plätze: Anfang von +n = Hub + 50 + (n−1)·Chunk-Breite, Anfang von −n = Hub − 50 − n·Chunk-Breite. Planbare Plätze bei kürzester Länge: Wald ±1…±7, Höhle ±1…±5, Mine ±1…±4, Eisenstollen ±1…±7 und Kristallhöhle ±1…±5 (gerechnet, hier gegen `frame()`/`eligible()` bestätigen). Portale ab Platz ±3 (Mindestabstand), in 25-Unit-Biomen Platz ±4 mit `at` ≥ 10.

## Erlaubte Dateien

- `engine/level/` (Prüfung der Planung, Options-Struktur, `lines.go` für den exportierten Linien-Typ und Parameter), zugehörige Tests
- `engine/sim/island_data.go` und zugehörige Tests (Feld laden, Prüfung beim Laden aufrufen)
- `data/islands.json` (nur das leere Feld; keine echte Planung)
- `testdata/` für Test-Planungen und Fixtures
- Planungsdateien von WZ8 (Status, Ergebnis) über k3c-dev

## Nicht-Ziele

Generator ändern oder geplante Chunks einsetzen (WZ8.3); `sim.CreateIsland` oder dessen Aufrufer anfassen (WZ8.3); echte Planungen in `data/islands.json`; Golden-Daten ändern; Felder für Teil 2 umsetzen (sie werden mit „noch nicht unterstützt“ abgelehnt); globalen Zustand zum Umschalten der Linien einführen.

## Schritte

1. Test zuerst für den Gültig-Fall und die Ablehnung je Fall (rot), dann umsetzen.
2. Typen für Planung (Platz, Art, Chunk-Seed, Objekte, reservierte Felder) und Feld `plannedChunks` je Insel und Tiefe in `data/islands.json` (leer) mit Laden in `island_data.go`.
3. `wallLineData` exportieren; Prüfung der Planung mit Linien als Parameter (Standard eingebettet, kein globaler Zustand); `lines.go` liefert die eingebetteten Linien nur noch als Standardwert.
4. Prüfung gegen die kürzeste Länge des Bioms und jede Streuung der Linien: Hub (Platz 0, Art `hub`), Länge, `exit`/`edge`, doppelter Platz, doppeltes `kind@x`, unbekannte Tiefe/Art/Objekt-Art, Teil-2-Felder („noch nicht unterstützt“), Lage (Chunk-Grenze ≥ 2 Units, Adern ≥ 4 Units zu Mauer/Turm/Tor und Farm-Weltplatz), Portale (Mindestabstand, außerhalb des äußersten Tors, Platz ±3), Summen, „kein Platz mehr“ (Mine-Beispiel, Adern ohne freien Ressourcen-Chunk). Fehler nennen Insel, Tiefe, Platz, Grund.
5. Je Fall ein Test; darunter Ader bei Hub +136 (Höhle, Platz +2, Lage 36): mit Linie 5 bei 124 gültig, bei 126 abgelehnt. Leeres Feld, `{}` und leere Liste = keine Planung.
6. `task check:go`; `task golden:update` darf keinen Diff erzeugen. Commit `feat(sim): Planung prüfen (WZ8.2)`.

## Fertig, wenn

- [ ] AC-06: Jeder Fehlerfall aus der Spec hat einen Test und nennt Insel, Tiefe, Platz und Grund; Linien-Beispiel (124 gültig, 126 abgelehnt) grün; zwei Aufrufe mit verschiedenen Linien nacheinander liefern je ihr Ergebnis.
- [ ] AC-02: `task golden:update` erzeugt keinen Diff in `testdata/golden/`; `data/islands.json` enthält keine echte Planung.
- [ ] Kein globaler Zustand für Planung oder Linien; Datei ≤ 400, Funktion ≤ 60 Zeilen.
- [ ] `task check:go` grün.

## Prüfen

```bash
task check:go
task golden:update
git diff --stat testdata/golden
```

Der Golden-Diff muss leer sein; sonst nicht einchecken. Bei laufender Workbench `check_run` nutzen. Manuelle Prüfungen nennt diese Session nicht.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
