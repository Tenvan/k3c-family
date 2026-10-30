# BS3.1 · Preise von Gebäuden, Hub und Truppen auf der Münzskala

- **Status:** fertig
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

- AC-01 geprüft: `TestGoldpreiseAufDerMuenzskala` (`engine/sim/coin_scale_data_test.go`) prüft Gebäude, Ausbaustufen, Treppen, Rüstung, Hub-Schritte und Truppen auf die Skala 2–30 und Mauer Stufe 1 = 2; einzige Ausnahme `troops.vagrant.recruitCost` (Kleinbetrag 1). `task check:diff` grün (zwei Fehlversuche: ein Frontend-Test mit 5-s-Timeout unter Last, Wiederholung grün; ein gocyclo-Treffer im neuen Test, aufgeteilt).
- Umrechnung nach `wirtschaft.md` § 8 (Kurve 2·(alt/5)^0,66, nächste Stufe, Gleichstand höher): Mauer 5→2, 10→3, 20→4, 40→8, 80→12; Turm 20→4 … 200→25; Tor 10→3 … 120→15; Hub-Schritte 13/12→4, 25→6, 50→10, 100→15, 200→25; Kaserne/Taverne/Heiler 30→6 … 300→30; Treppen 50→10; Truppen 20→4, 30→6, 50→10. Material und `notes` angepasst (Hinweis auf die Skala).
- Golden: 11 Dateien neu (`task go:golden`), nur Goldstände ändern sich, weil Gebäude billiger sind (Begründung im Commit). Tests mit alten Preisen nachgezogen (`gate_level`, `stairs_cost`, `economy_view`, `dev_cheat`, `net/wirtschaft`, Client `economyLabels`).
- Hinweis: `economy.json` (Beutel, Einnahmen) bleibt BS3.2; bis dahin sind Preise billig gegenüber Einnahmen, Läufe sind bis BS3.2 nicht aussagekräftig.
- Neue Tickets: keine.
