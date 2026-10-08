# PJ3 · INF · Planung in Projekte umziehen und aufräumen

- **Status:** geplant
- **Projekt:** –
- **Domäne:** INF
- **Prio:** hoch
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-359
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- PJ3.1 Projekte anlegen, Sprints und Tickets zuordnen, Ziel-Tickets setzen, veraltete Tickets bereinigen (AC-01, AC-04).
- PJ3.2 Sieben Sprints schließen mit Gerät-Sessions nach HW1, Sprints zusammenlegen, RM1 teilen (AC-02, AC-03).
- PJ3.3 Fahrplan nach Projekten, § 11 und `CLAUDE.md`, strenge Prüfung einschalten (AC-05, AC-06).
- PJ3.4 Review (Code-Sprint wegen `tests/planning.test.ts`): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
