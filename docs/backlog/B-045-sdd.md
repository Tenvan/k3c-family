# B-045 · Tickets und Sprints sind Specs nach Spec-Driven Development

- **Domäne:** INF
- **Typ:** Idee
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** SP00
- **Erstellt:** 2026-09-30
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Tickets hatten nur Beschreibung, Warum und Akzeptanz, Sprints nur Ziel und Stichpunkte. Welche Session welches Kriterium erfüllt, stand nirgends, und Freigaben waren nicht festgehalten.

## Ziel

Tickets und Sprints sind Specs nach Spec-Driven Development. Nutzen: Eine Session setzt einen freigegebenen Vertrag um statt einer Vermutung; das Review prüft Nachweise je Kriterium.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten lesen und arbeiten ab; 🧑 gibt Specs frei; die Review-Session prüft.

## Anforderungen

- Tickets und Sprint-READMEs enthalten die zehn Spec-Abschnitte (Ausgangslage bis Offene Fragen) und die Felder `Spec`, `Revision`, `Freigabe`.
- Akzeptanzkriterien tragen stabile IDs `AC-01` …; Sprints verweisen auf Ticket-Kriterien als `B-009/AC-01`.
- Jede Session nennt im Feld `Kriterien` die Sprint-Kriterien, die sie erfüllt; die Review-Session `alle`.
- Ein aktiver Sprint hat eine freigegebene oder rückwirkende Spec.
- `tests/planning.test.ts` prüft Abschnitte, Felder, ID-Folge, Verweise und bei `Reife: bereit` die Abdeckung jedes Kriteriums durch eine Session.
- Bestehende Tickets und Sprints sind migriert, erledigte als `rückwirkend`.

## Nicht-Ziele

Keine eigene Spec-Datei neben Ticket und Sprint; kein neues Werkzeug; kein Spiel-Code.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget. Freigabe nur ausdrücklich durch 🧑, mit Datum und Quelle im Feld `Freigabe`. Eine Spec-Freigabe ist keine Freigabe für manuelle oder Browser-Abnahmen.

## Beispiele

SP01 soll aktiv werden, seine Spec ist `Entwurf` → `npm test` scheitert, bis 🧑 sie freigibt.

## Ausnahme- und Fehlerfälle

Ein Session-Kriterium verweist auf ein fehlendes AC → `npm test` scheitert. Eine Anforderung ändert sich während der Umsetzung → erst die Spec (Revision +1, zurück auf `Entwurf`), dann der Code.

## Akzeptanzkriterien

- **AC-01** Die Vorlagen für Ticket, Sprint und Session enthalten die Spec-Abschnitte und -Felder.
- **AC-02** `npm test` scheitert bei fehlendem Abschnitt, unbekanntem `Spec`-Wert, Lücke in der AC-Folge, `freigegeben` ohne `Freigabe`, falschem Kriterien-Verweis, Kriterium ohne Session und aktivem Sprint mit Spec `Entwurf`.
- **AC-03** Alle Tickets und Sprints sind migriert, `npm test` ist grün.
- **AC-04** `docs/arbeitsweise.md` beschreibt SDD in einem Abschnitt.

## Offene Fragen

keine

## Notizen

Umgesetzt in SP00.4. Muster: Skill `todo-planner` (orga-planning), angepasst an die Ablage in `docs/`.
