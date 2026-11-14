# SP14 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Raum auf Insel
- **Projekt:** –

## Ziel

Der Raum rechnet mit einer `Island`: alle Stufen laufen im Takt, Spieler wechseln einzeln, jedes Gerät sieht die Stufe seines ersten Spielers, Spielstände werden als Insel gespeichert und geladen (alte Stände werden überführt). Das Protokoll (Version 3) trägt die Stufe je Platz und die Raum-Optionen (Grad, Ziel, Niederlage-Modus) beim Anlegen; der Server prüft die Optionen und sperrt den Grad „Dev …

## Ergebnis

- 2026-10-02, Review SP14.4 (getrennter Review-Agent): AC-01 bis AC-07 belegt in den Ergebnissen SP14.1 bis SP14.4; `task check` und `task check:go` grün. - Behobene mittlere Befunde: Stand Version 1 mit allen drei Stufen, dauerhafte Sicherung bei Versionswechsel, Speichern und `seats` nach Stufenwechsel jedes Monarchen. Ungeprüft: Test für den Wechsel im zweiten Slot. - Bekannt/offen: Träger-Material wird beim Speichern nicht gespeichert, Summary bezieht sich auf die Startstufe, Client liest `depth` noch nicht (B-106), Dialog (B-105), Debug-Panel (B-107).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Der Raum tickt alle Stufen der Insel; Test mit zwei Spielern in verschiedenen Stufen (B-13 … |
| AC-02 | – | Speichern und Laden einer Insel über den Store; ein Stand Version 1 wird geladen, die Date … |
| AC-03 | – | Status und TUI zeigen Stufen und Spieler je Stufe (B-133/AC-03). |
| AC-04 | – | `docs/protocol.md` beschreibt Version 3 (Stufe je Platz, Raum-Optionen), `testdata/protoco … |
| AC-05 | – | Der Server prüft Raum-Optionen; „Dev“ ist nur im Dev-Modus wählbar; ungültige Werte → `bad … |
| AC-06 | – | Die Protokollversion ist 3; ein Client mit Version 2 erhält `version` (B-104/AC-03). |
| AC-07 | – | `task check` und `task check:go` sind grün, die Golden-Daten unverändert, `-race` in der C … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-133 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-07 – | Der Raum rechnet mit einer Insel statt mit einer Kampagne |
| B-104 | erledigt | AC-04 –, AC-05 –, AC-06 – | Das Protokoll kennt die Stufe je Spieler und die Raum-Optionen |
