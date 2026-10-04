# BAL2.1 · Korridor-Daten laden und bewerten

- **Status:** fertig
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

- [x] AC-01: Test belegt Laden und Ladefehler bei Ziel ohne Kennzahl.
- [x] AC-02: Test belegt „im Korridor“, „knapp“ und „verletzt“ mit festen Werten.
- [x] `task check:go` grün.

## Prüfen

```bash
task check:go
```

## Ergebnis

**Umgesetzt** (Balance-Paket `tools/k3c-dev/internal/balance/`, nicht `engine/balance`: BAL1 liegt in k3c-dev):
- `data/balance-targets.json` (neu, über `*.json` schon eingebettet, `embed.go` unverändert): Version, Randbreite (5 pp / 10 %), 100 feste Seeds `"1"`…`"100"`, drei Ziele mit Kennzahl-ID, Szenario (2 Spieler, `saver`, Wald, 5 Tage), Grenzen und Regelbezug. Zahlen aus `zielkorridore.md` § 1 und § 3.
- `targets.go`: Laden und Prüfen (`LoadTargets`, `DefaultTargets`), Kennzahl-IDs `castleHeld`, `firstWallBeforeDusk1`, `destroyedPerWave`. Unbekannte Kennzahl, falsche Art, unbekannter Bot, fehlende oder verdrehte Grenzen sind Ladefehler.
- `evaluate.go`: `Targets.Evaluate(Report) []Verdict` (rein): Status `ok`, `knapp`, `verletzt`, `ungültig`, Wert, Seeds zum Nachspielen bei `verletzt`, Seeds abgebrochener Läufe.

**Festlegungen** (ungeprüft durch 🧑, bei Bedarf ändern): „knapp“ = Wert im Korridor, aber höchstens die Randbreite von einer Grenze entfernt (besteht, Ampel gelb); knapp außerhalb zählt als „verletzt“. Die natürlichen Enden eines Anteils (0 %, 100 %) zählen nicht als Grenze für „knapp“. Median bei nur einer Grenze: Randbreite = 10 % vom Betrag der Grenze (Obergrenze 1 → 0,1). Ein abgebrochener Lauf oder fehlende Messwerte machen das Urteil „ungültig“. `FailSeeds` bei Anteilen: unter der Untergrenze die Seeds ohne erfüllte Bedingung, über der Obergrenze die mit erfüllter; bei Medianen die Seeds mit Wert außerhalb. Hinweis: `DataHash` der Replays umfasst alle `data/*.json`, die neue Datei ändert ihn einmalig (nur Warnung beim Abspielen alter Replays).

**Noch nicht messbar** (nicht in der Datei):
- Gold am Morgen je Spieler (Median): Tester liefert Gold zu Dämmerungsbeginn und als Summe, nicht je Spieler am Morgen.
- Höhle erreicht vor Tag 6; Hub-Stufe 2 und 3; Abstand der Wellen unter Tage: Szenario hat nur eine Stufe, der Bot `saver` reist nicht, keine Hub-Stufe in den Kennzahlen.
- Erster Turm vor Ende Tag 2: nur die erste Mauer wird erfasst, kein Turm.
- Holz ≥ 100 ab Tag 3 und Kämpfer je Hub an Tag 3 und 6: Stand nur zu Dämmerungsbeginn bzw. Wellenbeginn, nicht zu Tagesbeginn.
- Gegner einer Welle bis Tagesanbruch besiegt, Skill-Punkte nach Tag 10: Kennzahl fehlt.
- Grad Dev, Leicht, Hart, Ultra (§ 2): Grad-Option fehlt im Szenario.
- Bosse (K2), Vollmond und Blutmond (K3): Mechanik fehlt.
- Tick-Dauer p99: Benchmark/Lasttest (LT1), nicht der Tester.

**Nachweis:**
- AC-01: `TestZielkorridoreLaden` (Datei lädt, 100 Seeds; unbekannte Kennzahl, falsche Art, unbekannter Bot → Fehler).
- AC-02: `TestBewertungAnteil` (ok, knapp unten und oben, verletzt unten und oben), `TestBewertungMedian` (ok, knapp, verletzt), dazu `TestFailSeedsAnteil`, `TestBewertungUngueltig`, `TestGrenzenWieTabelle` (Datei und Tabelle nennen dieselben Grenzen).
- `task check`, `task check:go`, `task check:dev` grün (`-race` übersprungen: kein C-Compiler).
