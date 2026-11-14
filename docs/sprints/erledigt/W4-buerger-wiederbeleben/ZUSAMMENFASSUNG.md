# W4 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Wiederbeleben, Berufe, Händler, Elite und Limit
- **Projekt:** –

## Ziel

Wiederbeleben, Berufe, Händler, Krieger und Elite, Rüstung, Limit je Hub und Heilung sind in der Simulation spielbar. Am Ende sichtbar: Tests je Regel grün, aktualisierte Golden-Daten.

## Ergebnis

- 2026-10-06 (W4.4, autonom): AC-01 (W4.1: `revive_test.go`), AC-02 und AC-03 (W4.2: `professions_test.go`, `merchant_test.go`), AC-04 bis AC-06 (W4.3a: `barracks_test.go`, `warrior_test.go`, `disarm_test.go`; W4.3b: `upgrades_test.go`, `healing_w4_test.go`), AC-07 (Golden je Regel erklärt, `rng.json` unverändert; `task check:go` und `task check` grün) mit Nachweis. - Review des Diffs: keine schweren Befunde, keine behoben; Hinweis: Spielstand lädt Krieger, Elite, Berufe, Rüstung und Händler noch nicht (B-201, W7). - Neue Tickets: keine. - Version: v0.12.0 gesetzt (2026-10-06)

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Wiederbeleben nach 3 s mit 50 % HP, Abbruch bei Treffer oder Loslassen; Respawn nach 15 s … |
| AC-02 | – | Bergmann und Baumeister erhöhen Abbau- und Bautempo um 50 %, Umschulung kostet Gold; Handw … |
| AC-03 | – | Händler erscheint nach Regel, tauscht korrekt und verschwindet nach einem Tag (Test) (B-12 … |
| AC-04 | – | Elite-Upgrade und Rüstung wirken mit Werten aus den Daten, das Limit je Hub zählt nur Kämp … |
| AC-05 | – | Heilplatz heilt in Reichweite, ohne Heilplatz keine Heilung; Verlust-Kaskade statt Tod und … |
| AC-06 | – | Die Werkstatt bietet Schwerter, das Elite-Upgrade wirkt laut Daten, Tests decken beides ab … |
| AC-07 | offen | `task check:go` grün, Golden-Daten aktualisiert (B-120/AC-04, B-121/AC-04). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-120 | erledigt | AC-01 –, AC-07 offen | Gefallene Monarchen bleiben liegen, Mitspieler beleben sie wieder, sonst Respawn … |
| B-121 | erledigt | AC-02 –, AC-03 –, AC-07 offen | Bauern haben Berufe (Bergmann, Baumeister, Handwerker), und ein Händler tauscht … |
| B-122 | erledigt | AC-04 –, AC-05 – | Elite-Upgrades, Rüstung, Truppen-Limit je Hub und Heilung der Truppen sind umges … |
| B-014 | erledigt | AC-06 – | Krieger und Elite-Truppen sind umgesetzt |
