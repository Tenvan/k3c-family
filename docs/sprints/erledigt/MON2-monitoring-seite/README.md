# MON2 · PLAT · Monitoring-Seite mit Dashboard

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** PLAT
- **Reife:** bereit
- **Tickets:** B-282
- **Start-Commit:** 4ab45b8
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑 (umfasst B-282 Revision 1)

## Ausgangslage

Nach MON1 liefert `/api/metrics` Verläufe und Ereignisse, aber nichts zeigt sie an. Details: B-282.

## Ziel

Die Monitoring-Seite zeigt den Serverzustand als Dashboard mit Verläufen, Perzentilen und Fehler-Zeitleiste. Am Ende sichtbar: Kachel „Monitor“ auf der Landingpage, am Handy während `task load` Ampel, Diagramme und Ereignisse.

## Beteiligte und Zielgruppen

Autonome Umsetzung; 🧑 gibt die Spec frei und nimmt am Handy ab (B-282/AC-05).

## Anforderungen

`B-282 › Anforderungen`.

## Nicht-Ziele

`B-282 › Nicht-Ziele`. Fehlt dem Server eine Kennzahl, wird das ein SRV-Ticket.

## Regeln und Einschränkungen

`B-282 › Regeln und Einschränkungen`. Startet erst, wenn MON1 erledigt ist.

## Beispiele

`B-282 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-282 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Perzentile und Ausreißer (`B-282/AC-01`).
- **AC-02** Delta-Puffer und Neustart (`B-282/AC-02`).
- **AC-03** Seite eingetragen, `task check` grün (`B-282/AC-03`).
- **AC-04** Browser-Pane-Nachweis unter Last (`B-282/AC-04`).
- **AC-05** Abnahme 🧑 am Handy (`B-282/AC-05`).

## Offene Fragen

`B-282 › Offene Fragen` (blockiert nicht).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| MON2.1 | `MON2.1-daten.md` | Umsetzung | autonom | fertig |
| MON2.2 | `MON2.2-seite.md` | Umsetzung | autonom | fertig |
| MON2.3 | `MON2.3-review.md` | Review | autonom | fertig |
| MON2.4 | `MON2.4-abnahme-handy.md` | Workshop | Mensch | verworfen |

## Abnahme

2026-10-05 · AC-01 bis AC-04 geprüft (Nachweise MON2.1 bis MON2.3; AC-04 im Browser-Pane unter Last); AC-05 angenommen, Validierung offen (MON2.4, Handy); `task check` und `task check:go` grün.
Keine schweren Befunde (MON2.3): Server-Daten nur über `textContent`, Token nur in `localStorage` und Header (nie URL oder Anzeige), Seiten-Regeln und B-Taste eingehalten. Keine neuen Tickets.
Version: v0.13.0 vorgeschlagen (Minor: neue Seite `monitor.html`; DBG3 und MON1 schlagen ebenfalls die nächste Minor vor, Tags der Reihe nach v0.11.0, v0.12.0, v0.13.0).

2026-10-07 · AC-05 am PC unter `task load` durch 🧑 geprüft (MON2.4); Abnahme am Handy weiter offen.
2026-10-08, PJ3.2 (B-359): AC-05 angenommen, Validierung in HW1.6 (Projekt ABN); Sprint geschlossen.
