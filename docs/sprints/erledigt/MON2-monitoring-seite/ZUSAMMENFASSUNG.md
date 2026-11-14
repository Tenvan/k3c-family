# MON2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Monitoring-Seite mit Dashboard
- **Projekt:** WZG

## Ziel

Die Monitoring-Seite zeigt den Serverzustand als Dashboard mit Verläufen, Perzentilen und Fehler-Zeitleiste. Am Ende sichtbar: Kachel „Monitor“ auf der Landingpage, am Handy während `task load` Ampel, Diagramme und Ereignisse.

## Ergebnis

2026-10-05 · AC-01 bis AC-04 geprüft (Nachweise MON2.1 bis MON2.3; AC-04 im Browser-Pane unter Last); AC-05 angenommen, Validierung offen (MON2.4, Handy); `task check` und `task check:go` grün. Keine schweren Befunde (MON2.3): Server-Daten nur über `textContent`, Token nur in `localStorage` und Header (nie URL oder Anzeige), Seiten-Regeln und B-Taste eingehalten. Keine neuen Tickets. Version: v0.13.0 vorgeschlagen (Minor: neue Seite `monitor.html`; DBG3 und MON1 schlagen ebenfalls die nächste Minor vor, Tags der Reihe nach v0.11.0, v0.12.0, v0.13.0). 2026-10-07 · AC-05 am PC unter `task load` …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Perzentile und Ausreißer (`B-282/AC-01`). |
| AC-02 | geprüft | Delta-Puffer und Neustart (`B-282/AC-02`). |
| AC-03 | geprüft | Seite eingetragen, `task check` grün (`B-282/AC-03`). |
| AC-04 | geprüft | Browser-Pane-Nachweis unter Last (`B-282/AC-04`). |
| AC-05 | – | Abnahme 🧑 am Handy (`B-282/AC-05`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-282 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 – | Die Monitoring-Seite zeichnet Verläufe, Perzentile und die Fehler-Zeitleiste aus … |
