# BAL8 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Welt-Werte: Basis erben, Overrides je Welt, Simulation rechnet mit dem Wertesatz der Welt
- **Projekt:** BAL

## Ziel

Eine Welt-Datei erbt die Basis-Werte und übersteuert nur, was sie ändern will; jeder Raum rechnet mit dem aufgelösten Wertesatz seiner Welt. Am Ende sichtbar: zwei Räume mit verschiedenen Welten und verschiedenen Kosten beeinflussen sich nicht; Go-Tests und Golden grün.

## Ergebnis

–

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Regeldatei beschreibt Erben und Übersteuern der Basis-Werte, 🧑 bestätigt (`B-680/AC-01` bi … |
| AC-02 | – | Die Simulation rechnet je Raum mit dem Wertesatz seiner Welt, Welten beeinflussen sich nic … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-680 | eingeplant | AC-01 – | Eine Welt-Datei erbt die Basis-Werte und übersteuert nur, was sie ändern will |
| B-681 | eingeplant | AC-02 – | Die Simulation rechnet mit den Werten ihrer Welt statt mit globalen Datei-Variab … |
