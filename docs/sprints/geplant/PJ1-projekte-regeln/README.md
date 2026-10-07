# PJ1 · INF · Projekte, Rang und Domäne je Session in Regeln, Vorlagen und Planungstest

- **Status:** geplant
- **Domäne:** INF
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-355, B-356, B-338
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- PJ1.1 Regeln und Glossar: `docs/arbeitsweise.md` (Ebenen, Rang, Auswahl, Domäne je Session, Sperre, Größe 3–6, Review über alle Domänen), `docs/glossar.md` (AC-02, AC-06, Glossar-Teil von AC-01).
- PJ1.2 Vorlagen und Planungstest: `docs/vorlagen/projekt.md`, Felder `Projekt` und `Domäne`, `verworfen`, `docs/projekte/README.md`, `tests/planning.test.ts` mit Negativtests, Feld `Domäne` in allen Session-Dateien (AC-01, AC-03, AC-04, AC-05, AC-07, AC-08, AC-09).
- PJ1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
