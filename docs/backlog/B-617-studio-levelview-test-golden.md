# B-617 · Der Studio-Test levelView passt wieder zur Golden-Datei level-forest.json

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ34
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Commit 87e0831c (#455, dichter Wald) hat `testdata/golden/level-forest.json` neu geschrieben. `tools/k3c-studio/frontend/src/world/levelView.test.ts` (Zeile 25 f.) erwartet noch die alten Zahlen (`tree: 42, skillPoint: 5`), die Golden-Datei hat jetzt u. a. `tree: 88, skillPoint: 4`. `task check:studio` und damit `task check:diff` sind auf `develop` rot.

## Ziel

`task check:studio` ist auf `develop` wieder grün.

## Beteiligte und Zielgruppen

🧑 Ralf; alle Sessions, die `check:diff` mit Studio-Änderungen fahren.

## Anforderungen

- Die erwarteten Anzahlen im Test kommen aus der Golden-Datei oder sind an sie angepasst.

## Nicht-Ziele

Keine Änderung am Level-Generator.

## Regeln und Einschränkungen

Golden-Dateien nur über `task go:golden` neu schreiben.

## Beispiele

nicht relevant (reiner Test-Nachzug).

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** `task k3c-studio:test` ist grün, `levelView.test.ts` prüft weiter die Sortierung nach Häufigkeit.

## Offene Fragen

keine

## Notizen

Gefunden in WZ31.1 (`task check:diff`, 2026-10-24).

2026-10-31 (Claude, Team WZG): Kein Fehler mehr. `vitest run src/world/levelView.test.ts` in `tools/k3c-studio/frontend`: 14 Tests grün, Test und Golden unverändert.
