# WZ2.8 · Review gegen alle Kriterien

- **Status:** fertig
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

Review 2026-10-11 durch einen unabhängigen Reviewer-Agenten (nicht der umsetzende Kontext), Diff `2b0d0891...HEAD`; `task check` und `task check:dev` nach den Korrekturen grün.

- AC-01, AC-03, AC-05, AC-06, AC-07, AC-10: umgesetzt, geprüft (Tests laut Ergebnis der Sessions WZ2.1, WZ2.2, WZ2.4 bis WZ2.7).
- AC-02: Befund „Meldung nennt die Datei nicht“ behoben (`Set` setzt den Pfad davor), Test `TestFehlendesFeldNenntFeldUndDatei`.
- AC-04: Befund „kein Ticket-Test für Prio, Umgebung, Projekt“ behoben (`TestTicketFlagsAendernZiehtIndexNach`); Abnahme am Bildschirm steht aus (🧑).
- AC-09: H1 behoben (`filterSprints` beachtet Prio über die Tickets und Umgebung über die Sessions), M2 (Speichern/Laden beider Bereiche getestet), M3 (gemerkte Domäne aus der Zeit vor dem Projekt-Filter wird verworfen). M1 und M5 sind Entscheidungen für 🧑 (B-403).
- AC-08: verschoben, Ziehen und Doppelklick nur statisch getestet, Abnahme durch 🧑 (B-403).
- Weitere behobene Befunde: M4 („läuft bereits“ nennt die Lauf-ID), N2 (feste Reihenfolge der Refs), N3 („11.6 Hardware“), Glossar um Backlog-Filter, Projekt-Filter, Splitter und Lauf-ID ergänzt.
- Nicht geprüft: Aussehen und Bedienung am Bildschirm; Herkunft des Timeouts (B-372 › Notizen); Race-Detector (cgo fehlt); Windows-Pfade über Junction oder 8.3-Namen.
- Neue Tickets: B-403 (Review-Nachträge: M1, M5, N1, N4–N6, Splitter-Interaktionstest, Doku-Reste).
