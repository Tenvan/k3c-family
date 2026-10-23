# WZ26 · DEV · CI rot beheben, Prüf- und Planungswerkzeuge härten

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-465, B-473, B-474, B-519, B-520, B-496
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

CI und `task lint` auf develop sind rot (B-465, B-473, B-474); `plan_next` und `check_run` haben Lücken (B-519, B-520); das Studio merkt sich Fenstergröße nicht (B-496).

## Ziel

develop ist wieder grün und die Planungs- und Prüfwerkzeuge liefern verlässliche Antworten. Am Ende sichtbar: grüne CI, `check_run` mit Lauf-ID.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-465, B-473, B-474, B-519, B-520, B-496 › Anforderungen.

## Nicht-Ziele

Neue Workbench-Seiten.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `task lint` und die CI auf develop sind grün (B-465/AC, B-473/AC).
- **AC-02** `check:all` scheitert nicht mehr am TLS-Zertifikat (B-474/AC).
- **AC-03** `plan_next` beachtet Sprint-Branches auf origin, `check_run` liefert eine Lauf-ID (B-519/AC, B-520/AC).
- **AC-04** Das Studio stellt Fensterposition und Größe wieder her (B-496/AC).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Stichpunkte (Reife Entwurf): WZ26.1 CI und Lint (AC-01, AC-02); WZ26.2 plan_next und check_run (AC-03); WZ26.3 Studio-Fenster (AC-04); WZ26.4 Review.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
