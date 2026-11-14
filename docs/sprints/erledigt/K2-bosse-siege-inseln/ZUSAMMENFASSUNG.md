# K2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Bosse, Siegvarianten und Inselwechsel
- **Projekt:** KMP

## Ziel

Je Stufe ein Miniboss, je Insel ein Endboss, wählbare Siegvarianten und Niederlage-Modi sowie der gemeinsame Inselwechsel sind in der Simulation spielbar. Am Ende sichtbar: Tests je Boss, Siegvariante und Modus grün, Spielstand mit besiegten Bossen und aktueller Insel.

## Ergebnis

2026-10-09, Review K2.4: AC-01 bis AC-08 mit Nachweis in K2.1a–K2.3b › Ergebnis; `task check` und `task check:go` grün. Keine schweren Befunde; leicht: `GateOpen` nicht im Spielstand (Ereignis nach Laden erneut, ohne Wirkung). Offen außerhalb SIM: B-344, B-345 (SRV), Anzeige über K4/K5. Neues Ticket: B-372 (`check_run` Timeout). Version: v0.x Minor vorgeschlagen (Bosse, Siege, Niederlage-Modi und Inselwechsel wirken im Spiel).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Miniboss erscheint mit Welle 5 bzw. 3, Fähigkeit und Belohnung laut Daten (Test) (B-130/AC … |
| AC-02 | – | Endboss wird bei Ankunft ausgelöst, wartet sonst, wechselt die Phasen (Test) (B-130/AC-02) … |
| AC-03 | – | Boss-HP skaliert mit der Spieleranzahl der Insel (Test) (B-130/AC-03). |
| AC-04 | – | Der Spielstand merkt besiegte Bosse und die aktuelle Insel, Bosse kehren nie zurück (Test) … |
| AC-05 | erfüllt | Je Siegvariante: Bedingung nicht erfüllt, erfüllt, Ereignis genau einmal (Test) (B-102/AC- … |
| AC-06 | – | Je Niederlage-Modus gelten die Folgen laut Regelwerk, „Komplett verloren“ lässt den letzte … |
| AC-07 | – | Endboss besiegt gibt den Inselwechsel frei, Wechsel nur mit allen lebenden Spielern am Pun … |
| AC-08 | geprüft | Golden-Daten aktualisiert, `task check:go` grün (B-130/AC-05). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-130 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-08 geprüft | Minibosse und Endboss sind spielbar |
| B-102 | erledigt | AC-05 erfüllt, AC-06 – | Siegvarianten und Niederlage-Modi der Raum-Optionen sind umgesetzt |
| B-103 | erledigt | AC-04 –, AC-07 – | Inseln mit Endboss und gemeinsamem Inselwechsel sind spielbar |
