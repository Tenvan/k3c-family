# BAL2.1 · Korridor-Daten laden und bewerten

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** bal2/1-korridore-bewertung
- **Abhängig von:** BAL1.4
- **Tickets:** B-157
- **Kriterien:** AC-01, AC-02

## Ziel

Die bestätigten Zielkorridore liegen als Daten vor und werden geladen; ein Ziel ohne passende Kennzahl ist ein Ladefehler; eine reine Funktion bewertet eine Kennzahl über die Seeds als „im Korridor“, „knapp“ oder „verletzt“.

## Kontext

- **Zahlen:** `docs/rules/zielkorridore.md` § 1–3, von 🧑 am 2026-10-03 bestätigt (F1.4, Beschluss Q02: Pass/Fail je Kennzahl über 100 feste Seeds). Standardszenario: Insel 1, Wald-Start, Grad Normal, 2 Spieler, Bot „sparsam“. Lesart: Anteil in % = Anteil der Seeds mit erfüllter Bedingung; Median/Zeit über alle Seeds.
- **Daten:** neue Datei (Vorschlag `data/balance-targets.json`, B-157: Format legt die Session fest) mit je Ziel: Kennzahl-ID (aus BAL1), Szenario, Untergrenze, Obergrenze, Art (Anteil, Median), Bezug auf die Regel (Datei und §), dazu die Liste der 100 festen Seeds. Neue Felder in `data/` sind SIM, die **Werte** sind die bestätigten aus `zielkorridore.md` (keine eigenen Zahlen). Ein Test prüft, dass Datei und Tabelle dieselben Grenzen nennen, damit nichts doppelt auseinanderläuft.
- **Nur messbare Ziele laden:** Ziele, deren Kennzahl der Tester aus BAL1 noch nicht liefert (z. B. Hub-Stufe 2/3 vor W1, „Höhle erreicht“, Tick-Dauer p99 = Benchmark/LT1), stehen nicht in der Datei, sondern im Ergebnis als Liste „noch nicht messbar“ mit Grund. Grade Dev/Leicht/Hart/Ultra (§ 2) brauchen die Grad-Option im Szenario; fehlt sie in BAL1, ebenfalls „noch nicht messbar“.
- **„knapp“:** B-157 verlangt einen Randbereich mit Breite in den Daten, eine Zahl ist nicht beschlossen. **Vorschlag der Planung (🧑 bestätigt oder ändert mit der Spec-Freigabe, nicht als Beschluss behandeln):** 5 Prozentpunkte bei Anteilen, 10 % der Korridorbreite bei Medianen, als Feld je Ziel mit Standardwert.
- Ort: das Balance-Paket aus BAL1 (Vorschlag dort `engine/balance`); Kennzahl-IDs und Lauf-Ergebnis aus BAL1.1 (vor dem Start im Code prüfen).

## Erlaubte Dateien

- `data/balance-targets.json` (neu), `data/embed.go` (nur falls die Datei eingebettet werden muss)
- Balance-Paket aus BAL1 (Laden, Bewertung, Tests)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

`task balance`, Bericht, Baseline (BAL2.2), CI (BAL2.3), neue Kennzahlen oder Bot-Profile (BAL3), Änderung von Zahlen in `zielkorridore.md` oder Werten in `data/`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Kennzahl-IDs aus BAL1 nachlesen.
2. Korridor-Datei anlegen (messbare Ziele, 100 Seeds), Lader mit Fehler bei unbekannter Kennzahl.
3. Reine Bewertung (Anteil bzw. Median über Seeds → im Korridor, knapp, verletzt, dazu die Seeds, die das Ziel verfehlen).
4. Tests: Laden ok; Ziel mit unbekannter Kennzahl → Fehler; drei Fälle mit festen Werten je Art; Grenzen stimmen mit `zielkorridore.md` überein.
5. `task check:go`. Ergebnis mit Liste „noch nicht messbar“, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: Test belegt Laden und Ladefehler bei Ziel ohne Kennzahl.
- [ ] AC-02: Test belegt „im Korridor“, „knapp“ und „verletzt“ mit festen Werten.
- [ ] `task check:go` grün.

## Prüfen

```bash
task check:go
```

## Ergebnis

–
