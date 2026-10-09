# F0 · INF · Parallele Sprints je Domäne

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-174
- **Start-Commit:** 975e6d8
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), Revision 1; umfasst B-174

## Ausgangslage

Höchstens ein aktiver Sprint (plus einschiebbare) steht in `docs/arbeitsweise.md` und im Test `tests/planning.test.ts`. SP11 (SRV) ist aktiv und wartet auf 🧑. Zwei Spuren (Mensch hier, autonome Agenten auf einem zweiten Account) brauchen parallele Sprints.

## Ziel

Je Domäne darf ein Sprint aktiv sein, und Sessions werden per Branch auf `origin` beansprucht. Am Ende sichtbar: `task check` grün mit zwei aktiven Sprints verschiedener Domänen, `docs/arbeitsweise.md` mit der neuen Regel.

## Beteiligte und Zielgruppen

🧑 gibt die Spec frei und prüft die Regel; ein Agent setzt sie um.

## Anforderungen

B-174 › Anforderungen.

## Nicht-Ziele

Automatische Verteilung der Sessions, Sperrdateien, Änderung der Vorlagen.

## Regeln und Einschränkungen

Domäne INF (`docs/arbeitsweise.md`, `tests/planning.test.ts`, `docs/sprints/README.md` nur für den Fahrplan-Hinweis). Dieser Sprint ändert den Prozess selbst: Er ist der einzige Sprint, der vor den anderen Phase-0-Sprints laufen muss.

## Beispiele

Nach F0: SP11 (SRV), F1 (REG) und F2 (INF) sind gleichzeitig `aktiv`, der Test ist grün; ein zweiter aktiver SIM-Sprint ließe ihn scheitern.

## Ausnahme- und Fehlerfälle

Die Regel widerspricht einem vorhandenen Abschnitt der Arbeitsweise → die Session ändert beide Stellen und nennt es im Ergebnis.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` enthält die Regel je Domäne, das Beanspruchen per Branch und das Rebase vor dem PR, ohne Widerspruch (B-174/AC-01).
- **AC-02** Der Test prüft „höchstens ein aktiver Sprint je Domäne“ und besteht bzw. scheitert wie in B-174/AC-02 beschrieben (B-174/AC-02).
- **AC-03** `task check` ist grün (B-174/AC-03).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| F0.1 | `F0.1-regel-und-test.md` | Umsetzung | autonom | fertig |
| F0.2 | `F0.2-review.md` | Review | autonom | fertig |

## Abnahme

2026-10-03: AC-01 bis AC-03 geprüft in F0.1 (Durchsicht `docs/arbeitsweise.md`, `tests/planning.test.ts`, `task check` grün), in F0.2 bestätigt.
AC-02 belegt mit Tests der reinen Funktion `crowdedDomains` statt mit temporären Ordnern (so von F0.1 vorgegeben), Verhalten wie B-174/AC-02.
Review ohne schweren Befund, nichts behoben. Neue Tickets: keine.
