# B-606 · Die Studio-Tests sind nach dem dichten Wald wieder grün und check_run hängt nicht bei k3c-studio:test

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-24 (mit WZ28 Revision 2)

## Ausgangslage

Gefunden in WZ28.1 (2026-10-24), unabhängig von deren Diff, auf `develop` seit `87e0831c` (dichter Wald, Golden neu geschrieben):
- `tools/k3c-studio/frontend/src/world/levelView.test.ts:25` erwartet `tree: 42, skillPoint: 5`, das Golden `testdata/golden/level-forest.json` liefert jetzt 88 und 4.
- `tools/k3c-studio/backend/app_preview_fixture_test.go:38`: `previewFixture.json` weicht von `net.Preview(k3c, forest, Hub 5)` ab (nicht neu erzeugt).
- `tools/k3c-studio/backend/chat_cli_test.go:73` (`TestChatZeitlimit`): Status `abgebrochen` statt des erwarteten Werts.
- `check_run task:check:diff` lief 10 min in das Zeitlimit, hängend bei `k3c-studio:test`; derselbe Vitest-Lauf direkt in der Shell braucht 51 s.

## Ziel

`task check:studio` und `check_run task:check:diff` sind auf `develop` grün und enden ohne Hänger.

## Beteiligte und Zielgruppen

Ralf; Agent DEV.

## Anforderungen

- Erwartung in `levelView.test.ts` an das neue Golden anpassen (nur Werte, keine Regeländerung).
- `previewFixture.json` aus `net.Preview` neu erzeugen.
- `TestChatZeitlimit` klären: Test oder Status-Zuordnung bei Zeitlimit korrigieren.
- Ursache für den Hänger von `k3c-studio:test` unter `check_run` finden und beheben.

## Nicht-Ziele

Regel- oder Datenwerte ändern.

## Regeln und Einschränkungen

Golden nur mit `task go:golden` neu schreiben, wenn es um Engine-Werte geht; hier sind nur die Studio-Fixtures betroffen.

## Beispiele

nicht relevant: reine Testpflege.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** `levelView.test.ts`, `TestMockVorschauIstDieVorschauDesServers` und `TestChatZeitlimit` sind grün.
- **AC-02** `check_run task:check:diff` mit Studio-Anteil endet ohne Zeitlimit.

## Offene Fragen

Warum hängt der Vitest-Lauf nur unter `check_run`? Vermutet: Kindprozess (`bunx` über `cmd` mit `startup.cmd`) wartet auf stdin; ungeprüft.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
