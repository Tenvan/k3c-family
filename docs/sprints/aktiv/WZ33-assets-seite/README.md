# WZ33 · DEV · Eine Seite Assets statt Grafik, Sound und Quellen

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-677
- **Start-Commit:** 88ea604b
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-30 🧑 Chat („dann jetzt umsetzen“); umfasst B-677 Revision 1

## Ausgangslage

Grafik, Sound und Quellen sind drei Seiten des Studios mit sieben Unter-Reitern. Es gibt drei Kandidaten-Listen, sechs Abspielstellen und vier Filterleisten. Eine Netz-Datei in einen Slot zu bringen braucht sechs Schritte und zwei Seitenwechsel. Details stehen in B-677 › Ausgangslage.

## Ziel

🧑 verwaltet alle Asset-Slots und Dateien auf einer Seite Assets und stellt sie per Fokus-Chip auf ein Thema scharf. Eine Lücke lässt sich aus dem Netz in zwei Schritten füllen.

**Am Ende sichtbar:** Im Studio-Fenster gibt es statt Grafik, Sound und Quellen eine Seite Assets mit folgenden Teilen:

- Fokus-Chips samt eigenen Chips
- den Sichten Slots, Dateien und Szene
- einem Inspektor mit Kandidaten
- einer Abspielleiste unten

## Beteiligte und Zielgruppen

- 🧑 Ralf arbeitet mit der Seite und nimmt sie ab.
- Claude setzt in autonomen DEV-Sessions um.

## Anforderungen

- B-677 › Anforderungen (Seite und Navigation, Inspektor, Abspielen und Werkzeuge, Vokabular).
- Die neue Seite entsteht neben den alten, bis WZ33.6 sie entfernt. Bis dahin bleibt das Studio in jedem Commit benutzbar.

## Nicht-Ziele

- B-677 › Nicht-Ziele.
- Die kompakten Seiten Planer, Monarch und Gegner (B-670–B-672) kommen in einem eigenen Sprint.

## Regeln und Einschränkungen

- B-677 › Regeln und Einschränkungen.
- WZ29 und WZ30 haben nur noch ihre manuellen Abnahmen offen (kein Code mehr); WZ33 darf ihre Ordner (graphics/, sound/, sources/) umbauen.
- Commit im ausgecheckten Branch develop, kein eigener Branch (Arbeitsweise dieser Session); ein Commit für die Umsetzung WZ33.1–WZ33.7.
- Code-Review vor dem Commit durch einen Reviewer-Agenten (Backend mit Download und Pfaden, über 150 Zeilen).

## Beispiele

B-677 › Beispiele.

## Ausnahme- und Fehlerfälle

B-677 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Eine Seite Assets ersetzt Grafik, Sound und Quellen (B-677/AC-01).
- **AC-02** Feste und eigene Fokus-Chips (B-677/AC-02).
- **AC-03** Sichten und Gruppierungen (B-677/AC-03).
- **AC-04** Filterzustand bleibt erhalten (B-677/AC-04).
- **AC-05** Kandidaten-Liste je Slot ohne Dubletten (B-677/AC-05).
- **AC-06** „Zuordnen“ in einem Aufruf mit Bestätigung in der Zeile (B-677/AC-06).
- **AC-07** Eine Abspielleiste, Hörprobe entfällt (B-677/AC-07).
- **AC-08** Werkzeuge-Fläche (B-677/AC-08).
- **AC-09** Sprünge und Chat-Kontext (B-677/AC-09).
- **AC-10** Alte Seiten entfernt, Prüfungen grün (B-677/AC-10).
- **AC-11** Layout im Browser-Pane ohne Scrollen (B-677/AC-11).
- **AC-12** Manuelle Abnahme durch 🧑 (B-677/AC-12).

## Offene Fragen

keine. Die vier Fragen aus dem Mock hat 🧑 am 2026-10-30 entschieden (B-677 › Offene Fragen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ33.1 | `WZ33.1-backend-kandidaten.md` | Umsetzung | autonom | fertig |
| WZ33.2 | `WZ33.2-bausteine-logik.md` | Umsetzung | autonom | fertig |
| WZ33.3 | `WZ33.3-seite-sichten.md` | Umsetzung | autonom | fertig |
| WZ33.4 | `WZ33.4-inspektor-kandidaten.md` | Umsetzung | autonom | fertig |
| WZ33.5 | `WZ33.5-szene-werkzeuge.md` | Umsetzung | autonom | fertig |
| WZ33.6 | `WZ33.6-umschalten-aufraeumen.md` | Umsetzung | autonom | fertig |
| WZ33.7 | `WZ33.7-review.md` | Review | autonom | fertig |
| WZ33.8 | `WZ33.8-abnahme-pc.md` | Workshop | Mensch | offen |
## Abnahme

2026-10-30: AC-01 bis AC-11 in WZ33.1–WZ33.7 belegt (Vitest, Go-Tests, Browser-Pane mit Mock), Review-Befunde behoben (WZ33.7). Neues Ticket B-678. Offen ist AC-12: die Abnahme am PC durch Ralf in WZ33.8.
