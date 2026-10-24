# WZ30 · DEV · Oberflächen zur Abnahme: Szenen, Filter, Quellen, Chat-Karten mit Mock-Daten

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-597, B-598, B-599, B-600, B-601, B-602
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-24, Revision 1 (Vorgabe Ralf im Chat: erst alle Dialoge und Seiten von WZG zur Abnahme, Funktionen und Simulation zuletzt)

## Ausgangslage

Ralf (2026-10-24) will alle Seiten und Dialoge des Projekts WZG zuerst abnehmen; Funktionen und Simulation folgen danach. Aus WZ28 sind Slot-Details, Ticket-Button, Balancing-Seite und Tuning-Ansicht fertig. Die Oberflächen aus WZ29 (B-597 bis B-602) fehlen; WZ29 wartet für die Funktionen auf SO8.2.

## Ziel

Alle in WZG geplanten Seiten und Dialoge stehen im Studio und in der Workbench, bedienbar mit Mock-Daten, sodass Ralf sie abnehmen kann. Am Ende sichtbar: Studio im Mock (`?mock=1`) zeigt Filter, Sound-Szenen und Proben, Grafik-Szenen mit Kampf-Anzeigen, Seite „Quellen“ mit Download-Dialog und Chat-Vorschlagskarten.

## Beteiligte und Zielgruppen

Ralf (Abnahme der Oberflächen); Agent DEV.

## Anforderungen

B-597, B-598, B-599, B-600, B-601, B-602 › Anforderungen, soweit sie Oberfläche sind (Aufbau, Bedienung, Texte, Zustände). Daten kommen aus dem Mock-Backend des Studios; wo die Oberfläche schon echte Daten oder Spiel-Funktionen ohne neues Backend nutzen kann (Szenen aus `data/`, Zeichnen mit den Spiel-Funktionen), tut sie das.

## Nicht-Ziele

Backend-Funktionen von Vorrat, Download, Chat-Vorschlägen und die Füllung der Vorschlagsliste (WZ29.5 bis WZ29.7); die Abbildung Skill → Slot aus SO8.2 (WZ29.2 bindet sie an); Simulation.

## Regeln und Einschränkungen

Atomic Design über `@webkit`; jede neue Seite meldet sich mit `useTicketPage`; Mock-Daten in `tools/k3c-studio/frontend/src/api/mock*.ts`, die API-Typen sind schon die endgültigen (WZ29 tauscht nur die Implementierung); Zeichnen und Hören mit den Funktionen des Spiels; keine externen Abrufe; Texte deutsch wie die Nachbarseiten.

## Beispiele

Studio `?mock=1` › Sound › Szenen › Filter „Lücke“ + „Heiler“ → nur Heiler-Skills ohne Ton; Quellen › Auswahl › „Herunterladen“ → Bestätigungsdialog mit Größe und Quellen.

## Ausnahme- und Fehlerfälle

Mock liefert auch Fehler- und Leerzustände (Download scheitert, Chat offline, keine Treffer), damit Ralf sie sieht.

## Akzeptanzkriterien

- **AC-01** Filter- und Sortier-Baustein in `@webkit`, Sound- und Grafik-Szenen nutzen ihn (B-599/AC-01 bis AC-03).
- **AC-02** Sound-Szenen Skills, Gegner/Bosse, Bauen, Reiten, UI, Warnungen sowie Misch-, Lautheits- und Abstandsprobe sind als Oberfläche bedienbar (B-597, mit vorläufiger Abbildung Linie → Slot).
- **AC-03** Grafik-Szenen Skill-Effekte, Schilder/Gerüste, Gegner je Biom, Reittiere, Bosse und „Kampf-Anzeigen“ mit Mock-Zuständen, Biom und Licht wählbar (B-598/AC-01 bis AC-03, B-602/AC-01, B-602/AC-02).
- **AC-04** Seite „Quellen“ mit Übersicht je Quelle, Vorschlagsliste, Bestätigungsdialog für den Download, Fortschritt, Offline-Anzeige und Aufräumen-Dialog, mit Mock-Daten (Oberfläche zu B-601).
- **AC-05** Chat-Vorschlagskarten mit Vorschau/Hörprobe, „Übernehmen“/„Verwerfen“, gesperrtem „Übernehmen“ bei Lizenzverstoß und Offline-Zustand, mit Mock-Daten (Oberfläche zu B-600).
- **AC-06** Ralf hat alle WZG-Oberflächen (WZ28 und WZ30) im Mock abgenommen.

## Offene Fragen

keine. Default (Claude): Die Oberflächen nutzen die endgültigen API-Typen; WZ29 ersetzt nur Mock durch Backend.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ30.1 | `WZ30.1-filter-baustein.md` | Umsetzung | autonom | offen |
| WZ30.2 | `WZ30.2-sound-szenen-proben.md` | Umsetzung | autonom | offen |
| WZ30.3 | `WZ30.3-grafik-szenen-kampf.md` | Umsetzung | autonom | offen |
| WZ30.4 | `WZ30.4-quellen-seite.md` | Umsetzung | autonom | offen |
| WZ30.5 | `WZ30.5-chat-karten.md` | Umsetzung | autonom | offen |
| WZ30.6 | `WZ30.6-review.md` | Review | autonom | offen |
| WZ30.7 | `WZ30.7-abnahme-oberflaechen.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
