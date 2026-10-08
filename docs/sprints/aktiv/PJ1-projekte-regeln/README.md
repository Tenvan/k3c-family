# PJ1 · INF · Projekte, Rang und Domäne je Session in Regeln, Vorlagen und Planungstest

- **Status:** aktiv
- **Domäne:** INF
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-355, B-356, B-338
- **Start-Commit:** ffac0f9
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-07, Chat, durch 🧑, Revision 1

## Ausgangslage

Die Planung kennt nur Sprint und Session, jeder Sprint gehört einer Domäne, die Reihenfolge ergibt sich aus der Sprint-Prio (höchste Ticket-Prio) und dem Feld `Einschiebbar`. Themen stehen nur in `docs/plan-weiterentwicklung.md`. Sessions können nicht `verworfen` werden. Details: B-355, B-356, B-338.

## Ziel

Regeln, Glossar, Vorlagen und `tests/planning.test.ts` kennen die drei Ebenen Projekt → Sprint → Session mit Rang, die Domäne je Session und den Session-Status `verworfen`.

**Am Ende sichtbar:** `docs/vorlagen/projekt.md`, `docs/projekte/README.md` (noch leer), neue Abschnitte in `docs/arbeitsweise.md`, `task test -- planning` grün mit Negativtests für die neuen Regeln.

## Beteiligte und Zielgruppen

🧑 gibt die Spec frei; Agenten arbeiten danach nach den neuen Regeln. Werkzeug folgt in PJ2, Umzug in PJ3.

## Anforderungen

- `B-355 › Anforderungen` (Projekt-Ebene, Rang, Auswahl der nächsten Session, Sprint-Prio und `Einschiebbar` entfallen).
- `B-356 › Anforderungen` (Domäne je Session, Sperre je Domäne auf Session-Ebene, Sprint-Größe 3–6).
- `B-338 › Anforderungen`, nur der INF-Teil (Vorlage und Test); `plan_set` folgt in PJ2.

## Nicht-Ziele

`plan_*`-Tools und Planungsseite (PJ2: B-357, B-358, B-338/AC-02), Projekte anlegen und Sprints zuordnen (PJ3: B-359).

## Regeln und Einschränkungen

- Domäne INF; Planungsdateien unter `docs/sprints/` nur für das neue Feld `Domäne` der Sessions.
- **Übergang:** Ohne Projekt bleiben Sprints und Tickets gültig, bis PJ3 die strenge Prüfung einschaltet. Vorlagen und Test ändern sich in derselben Session, damit `task test -- planning` nach jeder Session grün ist.
- Zwischen PJ1 und PJ2 kennen die `plan_*`-Tools die neuen Felder noch nicht: PJ2 folgt direkt; neue Sessions in dieser Zeit bekommen das Feld `Domäne` von Hand.
- Prozess nur in `docs/arbeitsweise.md`; neue Begriffe zuerst ins Glossar.

## Beispiele

Siehe `B-355 › Beispiele` und `B-356 › Beispiele`.

## Ausnahme- und Fehlerfälle

Siehe `B-355 › Ausnahme- und Fehlerfälle` und `B-356 › Ausnahme- und Fehlerfälle`; jeder dort genannte Fall hat einen Negativtest.

## Akzeptanzkriterien

- **AC-01** B-355/AC-01 (Vorlage Projekt, Übersicht, Glossar).
- **AC-02** B-355/AC-02 (Arbeitsweise: Projekt, Rang, Auswahl, ein aktiver Sprint je Projekt).
- **AC-03** B-355/AC-03 (Feld `Projekt` an Sprint und Ticket).
- **AC-04** B-355/AC-04 (Planungstest für Projekte).
- **AC-05** B-356/AC-01 (Vorlagen: Domäne je Session, Liste am Sprint).
- **AC-06** B-356/AC-02 (Arbeitsweise und Glossar: Domäne je Session).
- **AC-07** B-356/AC-03 (Planungstest für Domänen).
- **AC-08** B-356/AC-04 (alle Session-Dateien mit `Domäne`).
- **AC-09** B-338/AC-01 (`verworfen` in Vorlage und Test).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| PJ1.1 | `PJ1.1-regeln-glossar.md` | Umsetzung | autonom | fertig |
| PJ1.2 | `PJ1.2-vorlagen-felder.md` | Umsetzung | autonom | in Arbeit |
| PJ1.3 | `PJ1.3-planungstest-regeln.md` | Umsetzung | autonom | offen |
| PJ1.4 | `PJ1.4-review.md` | Review | autonom | offen |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
