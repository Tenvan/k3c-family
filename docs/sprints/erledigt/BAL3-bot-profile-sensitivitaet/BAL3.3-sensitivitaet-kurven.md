# BAL3.3 · Sensitivitäts-Läufe und Kurven je Schwierigkeitsgrad

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** bal3/3-sensitivitaet-kurven
- **Abhängig von:** BAL3.2
- **Tickets:** B-158
- **Kriterien:** AC-03, AC-04, AC-06

## Ziel

Ein Sensitivitäts-Lauf variiert einen Wert aus `data/*.json` um ±10 % und ±25 % und berichtet die gekippten Kennzahlen; der Bericht enthält je Schwierigkeitsgrad eine Kurve über die Tage.

## Kontext

- **Entsteht in BAL2:** Korridor-Daten und Bewertung (`BAL2.1`), `task balance` mit Bericht (JSON und Markdown) und Regressions-Vergleich, Test-Option des Läufers zum Ändern von Werten ohne Eingriff in `data/*.json` (`BAL2.2`). Die Sensitivität baut darauf auf: „gekippt“ = die Bewertung (im Korridor, knapp, verletzt) ändert sich gegenüber dem Lauf mit unverändertem Wert. Ort und Namen vor dem Start im Code nachsehen.
- **Variation:** Pfad zu einem Zahlenwert (Beispiel aus B-158: `data/economy.json` › `purse`, dort z. B. `startGold`), Stufen ±10 % und ±25 %. Existiert der Pfad nicht, Fehler mit Pfad, kein stiller Lauf. Es wird **nichts automatisch geändert**, nur berichtet; `data/*.json` bleiben unberührt. Welche Werte standardmäßig variiert werden: Beschluss BAL3.1.
- **Grad-Kurven:** Grade aus `data/difficulty.json` (`dev`, `easy`, `normal`, `hard`, `ultra`; Faktoren auf Wellengröße, Gegner-HP und -Schaden). Je Grad Kennzahlen über die Tage als Tabelle im Bericht. **Welche Kennzahlen und wie viele Tage:** Beschluss BAL3.1 (nicht erfinden). Die Grad-Option im Szenario liefert BAL1; fehlt sie, „noch nicht messbar“ und Ticket (SIM). Vergleichswerte: `docs/rules/zielkorridore.md` § 2 (Burg hält Nacht 1–5 je Grad).
- Bericht-Format wie in BAL2.2 (JSON mit fester Feldreihenfolge, Structs statt Maps, plus Markdown); Ausgabeort wie dort (Vorschlag `reports/`).
- Laufzeit: Szenarien × 4 Stufen; ein Flag für weniger Seeds bleibt nötig. Im Ergebnis die Dauer messen.

## Erlaubte Dateien

- `tools/k3c-dev/internal/balance/` (Sensitivität, Kurven, Bericht, Tests, Befehl und Flags unter `cmd/`)
- `engine/sim/data.go` und ein Test dazu: nur `UseData(fs.FS) error`, das die Datenvariablen aus einem anderen Dateisystem neu lädt (Beschluss 🧑 2026-10-05, BAL3.1)
- `Taskfile.yml` (nur Task für die Sensitivität, EXE mit festem Pfad, kein `go run`, kein Teil von `task check`)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Automatische Wertsuche oder Optimierung, Änderung von `data/*.json`, neue Profile (BAL3.2), Abgleich mit echten Abenden (BAL4).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Beschluss BAL3.1 (Kennzahlen und Tage der Kurven) und Bericht aus BAL2.2 nachlesen.
2. Parameter-Variation über die Test-Option des Läufers aus BAL2.2 (Pfad → Wert ±10 % und ±25 %); unbekannter Pfad → Fehler mit Pfad.
3. Sensitivitäts-Bericht: je Variation die Liste der gekippten Kennzahlen mit Bewertung vorher und nachher.
4. Grad-Kurven: je Grad Tabelle der beschlossenen Kennzahlen über die Tage im Bericht.
5. Tests: Variation um +25 % auf einen Wert, der eine Kennzahl kippt (feste Werte), der Bericht nennt sie; unbekannter Pfad → Fehler; Kurven enthalten jeden Grad aus `data/difficulty.json`; gleiche Eingaben → byte-gleicher Bericht.
6. `task check:dev`. Ein Beispiel-Lauf (Laufzeit, Rechner) ins Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-03: Test belegt, dass ein Lauf mit ±10 % und ±25 % eines Werts einen Bericht mit der Liste gekippter Kennzahlen erzeugt.
- [x] AC-04: Test belegt, dass der Bericht je Grad aus `data/difficulty.json` eine Kurve über die Tage enthält.
- [x] AC-06: `task check:dev` grün; `git status` zeigt keine Änderung an `data/*.json`.

## Prüfen

```bash
task check:dev
```

## Ergebnis

- **Sim-Hook:** `sim.UseData(fs.FS) error` in `engine/sim/data.go` lädt buildings, troops, economy, hub, monarch, enemies
  und waves aus einem anderen Dateisystem neu (gemeinsame Ladefunktion `loadFrom` mit dem Init; bei Fehler bleibt alles
  unverändert); difficulty, biomes und `skillCatalog` bleiben. Test `TestUseDataReloadsAndRestores`
  (`engine/sim/data_usedata_test.go`): geändertes Startgold wirkt auf einen neuen Spieler, leeres FS → Fehler ohne
  Änderung, `UseData(data.Files)` stellt zurück.
- **AC-03 umgesetzt und geprüft:** `vary.go` (`variedFS`: Pfad `datei.json:a.b.c` → Wert ×(1+p), Ganzzahl bleibt ganz,
  Kopie von `data.Files` im Speicher; `withData` tauscht Sim-Daten und Bot-Preise und stellt sie zurück),
  `sensitivity.go` (`Targets.Sensitivity`, „gekippt“ = Bewertung aus `RunTargets`/`Summarize` ändert sich).
  Tests in `sensitivity_test.go`: `TestSensitivityNenntGekippteZiele` (−25/−10/+10/+25 % auf `purse.startGold`, +25 % kippt
  ein Testziel „Gold Tag 1“ ok → verletzt, Markdown nennt es), `TestSensitivityUnbekannterPfad` (fünf falsche Pfade →
  Fehler mit Pfad vor dem ersten Lauf), `TestSensitivityByteGleichUndZurueckgesetzt` (JSON + Markdown byte-gleich,
  danach Startgold und Mauerpreis wieder Original).
- **AC-04 umgesetzt und geprüft:** `curves.go` (`Grades` aus `data/difficulty.json` nach Wellengröße, `Curves` je Grad über
  `Matrix.Grade` → `Scenario.Grade` (`omitempty`, Baseline und Replays unverändert) → `sim.SetGrade` mit Dev-Mode).
  Test `TestKurvenJeGrad`: jeder Grad der Datei hat eine Kurve, dev zuerst, ultra hat in Welle 1 mehr Gegner als dev.
- **AC-06:** `task check:dev` und `task check:go` grün (0 Lint-Meldungen); `git status` zeigt keine Änderung an
  `data/*.json`; `testdata/balance/baseline.json` unverändert.
- **Befehl:** `task balance:sensitivity` (`--sensitivity --curves`; `--vary pfad` mehrfach/mit Komma, `--seeds N`),
  Bericht `reports/sensitivity-<Zeit>.json` und `.md`.
- **Beispiel-Lauf** (`task balance:sensitivity -- --seeds 10`, Windows-PC, 220 Läufe à 5 Tage): 1 min 58 s inkl. Build.
  Gekippt ist nur „Burg hält Nacht 1–5“ (Basis 100 % = verletzt, Korridor 75–90 %): Startgold +10/+25 % und
  `dawnGoldPerPlayer` +10/+25 % (5 → 6) senken auf 90 % (knapp); Mauerpreis und `perExtraPlayer` kippen nichts
  (Mauerpreis 5 → 4/6 durch Rundung grob). Kurven: dev, easy, normal halten 100 % aller Nächte; hard 40 % → 0 %
  (Nacht 1 → 5), ultra 10 % → 0 %.
