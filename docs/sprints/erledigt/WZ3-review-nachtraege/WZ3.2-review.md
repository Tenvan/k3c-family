# WZ3.2 · Review der Umsetzung WZ3

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz3/2-review
- **Abhängig von:** WZ3.1
- **Tickets:** B-403
- **Kriterien:** alle

## Ziel

Die Umsetzung WZ3.1 wird gegen alle Akzeptanzkriterien AC-01 bis AC-06 geprüft. Ergebnis: Befunde sind behoben oder als Tickets angelegt, und der Abschnitt `Abnahme` der Sprint-README ist ausgefüllt.

## Kontext

- Review-Session nach `arbeitsweise.md` (Review: alle Kriterien). Sie prüft, schreibt aber keine neuen Features.
- Umsetzung in WZ3.1 steht auf Branch `wz3/1-umsetzung`; Sprint-Branch `sprint/wz3`.
- Sprint-README: `docs/sprints/aktiv/WZ3-review-nachtraege/README.md` (nach der Aktivierung); Abnahme ist höchstens fünf Zeilen.
- Ticket: `docs/backlog/B-403-wz2-review-nachtraege.md`.

## Erlaubte Dateien

- `docs/sprints/aktiv/WZ3-review-nachtraege/README.md` (Abschnitt `Abnahme`)
- `docs/sprints/aktiv/WZ3-review-nachtraege/WZ3.2-review.md` (Ergebnis)
- `docs/backlog/B-403-wz2-review-nachtraege.md` (Status)
- Neue Tickets nur über `plan_create`, keine Codeänderung.

## Nicht-Ziele

- Keine Codeänderung. Befunde werden als Ticket angelegt, nicht hier behoben.
- Keine Freigabe und keine manuelle Abnahme durch die Session; die Abnahme trägt der Nutzer.

## Schritte

1. `task check:dev` über `check_run` ausführen, Ergebnis festhalten.
2. Je AC-01 bis AC-06 den Nachweis aus WZ3.1 prüfen (Testname oder Datei).
3. Befunde notieren; behebbare Fehler in WZ3.1 nachziehen, sonst neues Ticket über `plan_create`.
4. Abschnitt `Abnahme` der Sprint-README ausfüllen (höchstens fünf Zeilen): Datum, Kriterien, behobene Befunde, neue Tickets.
5. B-403 auf `erledigt` setzen, wenn alle Kriterien geprüft sind.

## Fertig, wenn

- [ ] AC-01 bis AC-06 mit Nachweis je Kriterium im Ergebnis.
- [ ] `task check:dev` grün (Nachweis im Ergebnis).
- [ ] Abnahme in der Sprint-README ausgefüllt.

## Prüfen

```bash
task check:dev
```

Über k3c-dev: `check_run` mit `checkout` der Repo-Wurzel.

## Ergebnis

Review 2026-10-11, Maßstab: Kriterien AC-01 bis AC-06 der Sprint-README, Nachweise aus WZ3.1.

- AC-01 geprüft: `visibleSprints` in `projects.ts`, in `SprintsBacklog.tsx` für die Zählung genutzt; Vitest `projects.test.ts` 8/8, mit `Splitter.test.tsx` 12/12 grün (Lauf in WZ3.2).
- AC-02 geprüft: `TestTicketNummerKeineRepoWurzelNennt` und das Paket `planning` waren nach der Änderung grün (Lauf in WZ3.1, in WZ3.2 nicht wiederholt).
- AC-03 geprüft: `TestFahrplanOhneProjektBehaeltPlatz` war vor dem Fix rot (`X2 X1`), danach grün (Lauf in WZ3.1).
- AC-04 geprüft: `TestCheckJobHaeltNurFehlerzeilen`, Paket `mcpsrv` grün (Lauf in WZ3.1).
- AC-05 geprüft: `check_background_test.go` wartet über `<-s.checks.job(id).done`, Paket `mcpsrv` grün (Lauf in WZ3.1).
- AC-06 geprüft: `splitShare` und `clampSplit` in `TaskBars.tsx`, getestet in `Splitter.test.tsx`; tautologischer Test entfernt.
- `task check:dev`: von 🧑 als teilweise akzeptiert (`dev:test`, `task:typecheck`, `go:lint`, `go:test` Root-Modul grün).
- Befunde: keine neuen am Code. Beobachtungen als Tickets: B-404 (Fahrplan-Projekt `–`), B-406 (Splitter-Doppelklick ohne DOM-Test). B-405 (Hänger bei `balance`) wurde angelegt und verworfen, die Läufe gehörten nicht zu WZ3.
- Manuell geprüft: niemand. Keine Codeänderung in dieser Session.
