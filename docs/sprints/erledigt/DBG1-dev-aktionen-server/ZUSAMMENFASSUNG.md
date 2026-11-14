# DBG1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Dev-Aktionen: Gold, Material, Zeitraffer
- **Projekt:** –

## Ziel

Im Dev-Mode des Raums lassen sich Gold und Material droppen und die Zeit beschleunigen. Am Ende sichtbar: Go-Tests je Aktion, Protokoll-Beispiele, Log-Einträge.

## Ergebnis

- 2026-10-03, Review DBG1.4 (getrennter Review-Agent): AC-01 bis AC-06 belegt in den Ergebnissen DBG1.1 bis DBG1.3; `task check` und `task check:go` grün. - Keine schweren Befunde; keine neuen Tickets. B-178 archiviert, B-179 (DBG2) kann auf dem Protokoll aufbauen. - Version: v0.5.0 gesetzt (2026-10-03, Bestätigung 🧑; Minor: neue Nachricht `dev` mit Gold, Material und Zeitraffer im Dev-Mode).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `dev` ohne Dev-Mode wird abgelehnt und im Log gewarnt (B-178/AC-01). |
| AC-02 | geprüft | Gold droppen funktioniert und ist aufhebbar (B-178/AC-02). |
| AC-03 | geprüft | Material erhöht den Insel-Vorrat, begrenzt durch das Lager-Maximum (B-178/AC-03). |
| AC-04 | geprüft | Der Zeitraffer ist deterministisch gleich N normalen Ticks (B-178/AC-04). |
| AC-05 | geprüft | Protokoll beschrieben, Beispiele vorhanden, beide Enden parsen (B-178/AC-05). |
| AC-06 | – | Dev-Aktionen stehen im Log; `task check` und `task check:go` grün (B-178/AC-06). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-178 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 – | Im Dev-Mode lassen sich Gold und Material droppen und die Zeit beschleunigen |
