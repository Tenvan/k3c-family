# BS3.1 · Preise von Gebäuden, Hub und Truppen auf der Münzskala

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bs3/1-preise
- **Abhängig von:** BS2.1
- **Tickets:** B-635
- **Kriterien:** AC-01

## Ziel

Alle regulären Goldpreise in `data/buildings.json`, `data/hub.json` und `data/troops.json` liegen auf der Münzskala 2–30 (Mauer Stufe 1 = 2), ein Daten-Test sichert das.

## Kontext

- Regel: `docs/rules/wirtschaft.md` (aus BS2.1), Stufen und Ausnahmen-Liste dort.
- Ist: Gebäude 5–300 Gold inkl. `levels[].cost`, Hub-Stufen bis 200, Truppen bis 50. Materialkosten bleiben, außer die Regel sagt anderes.
- Daten-Tests liegen in `engine/sim/*_data_test.go` (z. B. `building_levels_data_test.go`).
- Golden: `task go:golden` nur mit Begründung im Commit. Keine Rücksicht auf alte Spielstände.
- Die Farm heißt bis BS4 noch `farm`.

## Erlaubte Dateien

- `data/buildings.json`, `data/hub.json`, `data/troops.json`
- `engine/sim/*_data_test.go` (neuer Skala-Test)
- `testdata/golden/**` (über `task go:golden`)

## Nicht-Ziele

Einnahmen, Beute, Ziele, Schiff (BS3.2).

## Schritte

1. Tabelle alt → neu je Gebäude/Stufe nach Regel (Reihenfolge erhalten, größtes = 30).
2. Werte eintragen, `notes` anpassen.
3. Daten-Test: reguläre Goldpreise in 2…30, Mauer Stufe 1 = 2, Ausnahmen explizit.
4. Golden erneuern, `check_run task:check:diff`, Commit.

## Fertig, wenn

- [ ] AC-01: Daten-Test grün, kein regulärer Preis außerhalb 2…30.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
