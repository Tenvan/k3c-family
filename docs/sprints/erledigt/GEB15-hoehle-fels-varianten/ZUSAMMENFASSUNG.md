# GEB15 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Höhlen-Wurzeln, Felsen und Minen: mehr Varianten, größer
- **Projekt:** GEB

## Ziel

Höhle, Mine und Felsen wirken abwechslungsreich. Am Ende sichtbar: Studio-Vorschau und Spiel mit mehreren Wurzel- und Fels-Varianten.

## Ergebnis

2026-10-25, Claude (GEB15.3): `task check` und `task check:go` grün; `check:race` lokal übersprungen (kein C-Compiler), die CI prüft es. AC-01 erfüllt mit GEB15.1 (fünf Wurzel-Formen, Deko-Dichte `rootsPerNode`), AC-02 mit GEB15.2 (Fels 1,5 × 1,5, drei Formen je Look). Simulation und Golden sind unverändert. 2026-10-31: AC-03 von Ralf (🧑) am Gerät abgenommen, im Chat bestätigt; Claude hat die Prüfung nicht selbst gesehen. Die Defaults (Dichte 2, Faktor 1,5) gelten unverändert, eine Änderung wurde nicht gemeldet. Alle Kriterien erfüllt, Sprint erledigt. Keine neuen Tickets.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | Mindestens drei Wurzel-Varianten, Höhle dichter (B-582/AC-01, B-582/AC-02). |
| AC-02 | geprüft | Fels und Mine-Fels größer, je mindestens zwei neue Varianten (B-583/AC-01, B-583/AC-02). |
| AC-03 | – | Ralf hat AC-01 und AC-02 am Gerät abgenommen. |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-582 | erledigt | AC-01 erfüllt | Höhlen-Wurzeln sind zahlreicher und unterscheiden sich in Form und Größe |
| B-583 | erledigt | AC-02 geprüft | Minen und Felsen sind größer und haben zwei bis drei Varianten mehr |
