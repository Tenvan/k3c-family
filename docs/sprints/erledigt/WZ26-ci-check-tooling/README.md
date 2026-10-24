# WZ26 · DEV · CI rot beheben, Prüf- und Planungswerkzeuge härten

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-465, B-473, B-474, B-519, B-520, B-496
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

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
| WZ26.1 | `WZ26.1-ci-rot-beheben.md` | Umsetzung | autonom | fertig |


## Abnahme

- 2026-10-23, Claude autonom: AC-01 bis AC-03 erfüllt, `task check:diff` grün. CI-Rot auf develop nachgezogen: Sound-Test (Archiv-Tickets), `.dockerignore` (pipeline-liste.json), `TestSimTestMitCheckout` (Kurzpfade), `TestDownloadHit` (ohne Umgebungs-Proxy).
- AC-04 angenommen, Validierung offen: Fensterposition des Studios am PC (B-496) und Live-Messung `check_run` (B-520) liegen bei 🧑.
