# WZ2.8 · Review gegen alle Kriterien

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** sprint/wz2
- **Abhängig von:** WZ2.1, WZ2.2, WZ2.3, WZ2.4, WZ2.5, WZ2.6, WZ2.7
- **Tickets:** B-387, B-360, B-366, B-363, B-372, B-367, B-341, B-400, B-401, B-402
- **Kriterien:** alle

## Ziel

Der Sprint WZ2 ist gegen alle Akzeptanzkriterien (AC-01 bis AC-10) geprüft, Befunde sind behoben oder als Tickets angelegt, die Abnahme in der README ist ausgefüllt.

## Kontext

- Review-Session nach `docs/arbeitsweise.md` › Review (leicht): schlanke Prüfung gegen die Kriterien, Begriffs-Treue laut Glossar (Abweichungen gleicht der Review-Commit an).
- Geprüft wird gegen die Sprint-README (AC-01 bis AC-10) und die Ticket-Specs B-387, B-360, B-366, B-363, B-372, B-367, B-341, B-400, B-401, B-402 in ihrer aktuellen Revision; die Ergebnisse der Sessions WZ2.1 bis WZ2.7 stehen in deren Abschnitt „Ergebnis“.
- Die Abnahme der Planungsseite am Bildschirm (B-363, B-400, B-401, B-402) gehört 🧑 und wird nur mit seiner Freigabe für diesen Lauf gemacht.
- Der Reviewer ist ein anderer Kontext als die Umsetzung (kein Selbst-Approval).

## Erlaubte Dateien

- `docs/sprints/aktiv/WZ2-planungs-werkzeuge/README.md` (Abnahme) und die Session-Datei WZ2.8
- Korrekturen kleiner Befunde in den von WZ2.1 bis WZ2.7 geänderten Dateien; größere Befunde → Ticket

## Nicht-Ziele

Neue Funktionen; Kriterien umschreiben, damit sie zum Code passen; manuelle Abnahme am Bildschirm ohne Freigabe von 🧑.

## Schritte

1. Ergebnisse von WZ2.1 bis WZ2.7 lesen; je Kriterium AC-01 bis AC-10 den Nachweis prüfen.
2. `task check:dev` und `task test -- planning` selbst ausführen.
3. Befunde: kleine direkt beheben, größere als Ticket (`plan_create`) anlegen.
4. Abnahme in der Sprint-README eintragen (höchstens fünf Zeilen: Datum, Kriterien, behobene Befunde, neue Tickets).

## Fertig, wenn

- [ ] Alle Kriterien AC-01 bis AC-10 sind je mit `umgesetzt`, `geprüft`, `verschoben` oder `blockiert` nachgewiesen.
- [ ] `task check:dev` und `task test -- planning` grün.
- [ ] Abnahme in der Sprint-README ausgefüllt; offene Befunde sind Tickets.

## Prüfen

```bash
task check:dev
task test -- planning
```

Manuelle Prüfungen (Planungsseite am Bildschirm) nur durch 🧑.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
