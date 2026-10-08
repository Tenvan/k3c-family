# PJ3 · INF · Planung in Projekte umziehen und aufräumen

- **Status:** aktiv
- **Projekt:** PRZ
- **Domäne:** INF
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-359
- **Start-Commit:** 6c7837b
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-08, Chat, durch 🧑, Revision 1

## Ausgangslage

Nach PJ1 und PJ2 gibt es Regeln und Werkzeug für Projekte, aber noch kein Projekt. 11 aktive und 41 geplante Sprints sind ohne Projekt, 7 aktive warten nur auf Abnahmen am Gerät, mehrere geplante überlappen. Details: B-359.

## Ziel

Die ganze offene Planung liegt in Projekten mit Rang, `aktiv/` enthält nur Sprints mit offener autonomer Arbeit, und die Reihenfolge steht nur noch im Projekt-Rang.

**Am Ende sichtbar:** `docs/projekte/README.md` mit LST, GRA, SND, BED, WRT, SKL, KMP, WZ, REL nach Rang, BAL ruhend, ABN; Planungsseite der Workbench zeigt sie; `task test -- planning` grün mit strenger Prüfung.

## Beteiligte und Zielgruppen

🧑 (Zuschnitt und Rang am 2026-10-07 entschieden, prüft das Ergebnis im PR), Agenten beim Umzug.

## Anforderungen

`B-359 › Anforderungen` (Projekt-Tabelle, Schließen, Zusammenlegen, Tickets bereinigen, Fahrplan, § 11, `CLAUDE.md`, strenge Prüfung). Dazu: das Projekt `PRZ Arbeitsweise` trägt PJ1 → PJ2 → PJ3.

## Nicht-Ziele

Siehe `B-359 › Nicht-Ziele`: keine Inhalte von Sprints ändern, nichts bereit machen oder freigeben, keine offenen Reviews durchführen.

## Regeln und Einschränkungen

- Domäne INF; Planungsdateien, `CLAUDE.md` und die eine Stelle in `tests/planning.test.ts`, die die strenge Prüfung einschaltet. Weil der Test Code ist, endet der Sprint mit einem Review.
- Alles über die `plan_*`-Tools aus PJ2; Handarbeit nur als Rückfall.
- Hängt von PJ1 und PJ2 ab.

## Beispiele

Siehe `B-359 › Beispiele`.

## Ausnahme- und Fehlerfälle

Siehe `B-359 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** B-359/AC-01 (Projekte mit Rang und Sprint-Reihenfolge).
- **AC-02** B-359/AC-02 (sieben Sprints geschlossen, Gerät-Sessions in HW1).
- **AC-03** B-359/AC-03 (RG3, BAL5, SO5, DBG4 aufgegangen; B-214 an BED).
- **AC-04** B-359/AC-04 (veraltete Tickets bereinigt, Ziel-Tickets gesetzt).
- **AC-05** B-359/AC-05 (Fahrplan, § 11, `CLAUDE.md`).
- **AC-06** B-359/AC-06 (strenge Prüfung an, Planungstest grün).

## Offene Fragen

Beim Bereitmachen 2026-10-08 aufgefallen, alle Vorschläge von 🧑 mit der Freigabe 2026-10-08 bestätigt:

- **Zwei aktive Sprints in WZ:** M11 und TR3 sind aktiv mit offenem Review; der Planungstest erlaubt je Projekt einen. Vorschlag: PJ3.1 hängt von M11.2 und TR3.2 ab (autonom, SRV und CLI, parallel zu nichts in INF).
- **PRZ während PJ3:** Ein erledigtes Projekt mit aktivem Sprint widerspricht der Regel. Vorschlag: PRZ aktiv mit Rang 10 (hinter REL), PJ3.4 setzt es auf `erledigt`.
- **Fahrplan „nach Projekten“:** k3c-dev findet die Fahrplan-Tabellen über die Überschriften Aktiv/Geplant/Erledigt. Vorschlag: Diese bleiben, `Prio` wird zur Spalte `Projekt`, Zeilen nach Rang sortiert; dass das Tool die Spalte selbst füllt, wird ein SRV-Ticket.
- **SO4 verliert seine Freigabe**, weil SO5 darin aufgeht (B-359 › Regeln): Spec zurück auf `Entwurf`, neue Freigabe nötig.
- **Tickets ohne Sprint und ohne Zuordnung in B-359** ordnet PJ3.1 nach Thema zu; was nicht passt, sammelt ein Frage-Ticket.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PJ3.1 | `PJ3.1-projekte-anlegen.md` | Umsetzung | autonom | fertig |
| PJ3.2 | `PJ3.2-sprints-schliessen-zusammenlegen.md` | Umsetzung | autonom | in Arbeit |
| PJ3.3 | `PJ3.3-fahrplan-strenge-pruefung.md` | Umsetzung | autonom | offen |
| PJ3.4 | `PJ3.4-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
