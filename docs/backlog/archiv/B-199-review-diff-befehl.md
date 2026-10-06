# B-199 · Review-Sessions lesen den Sprint-Diff mit dem Befehl aus der Arbeitsweise

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** rückwirkend
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`docs/arbeitsweise.md` › Review-Session Schritt 2 verlangt `git fetch && git diff origin/develop...origin/sprint/<präfix>`.
Die geplanten Review-Sessions in `docs/sprints/geplant/*/*-review.md` nannten noch den alten Befehl
`git fetch && git diff <Start-Commit>..origin/develop` aus der Zeit vor „ein PR je Sprint“. LT1.4 wurde in PR #93 korrigiert, W4.4 stimmte schon.

## Ziel

Jede geplante Review-Session liest genau den Diff des Sprint-Branches, wie es die Arbeitsweise vorschreibt.

## Beteiligte und Zielgruppen

Agenten, die Review-Sessions ausführen; 🧑 nimmt den PR ab.

## Anforderungen

- Schritt 2 jeder geplanten Review-Session nennt `git diff origin/develop...origin/sprint/<präfix>`, präfix = Teil vor `/` im Branch-Feld der Session.

## Nicht-Ziele

Erledigte Sprints in `docs/sprints/erledigt/` bleiben unverändert (Historie). Das Feld `Start-Commit` in den Sprint-READMEs bleibt.

## Regeln und Einschränkungen

Domäne INF, nur Doku. Vorlagen nach `docs/vorlagen/`.

## Beispiele

`W1.4-review.md` mit Branch `w1/4-review` → `git fetch && git diff origin/develop...origin/sprint/w1`.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Textanpassung.

## Akzeptanzkriterien

- **AC-01** `grep -rn "Start-Commit>..origin/develop" docs/sprints/geplant docs/sprints/aktiv docs/vorlagen` findet nichts.
- **AC-02** `npx vitest run tests/planning.test.ts` ist grün.

## Offene Fragen

keine

## Notizen

`docs/vorlagen/` enthielt die alte Zeile nicht; die Vorlage `session.md` nennt keinen Diff-Befehl.
