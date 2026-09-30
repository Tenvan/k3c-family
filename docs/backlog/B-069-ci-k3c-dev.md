# B-069 · Der CI-Job k3c-dev ist einmal grün gelaufen

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

M1 hat in `.github/workflows/ci.yml` den Job `k3c-dev` (windows-latest, `go test` und `golangci-lint` in
`tools/k3c-dev`) angelegt. M1 lief ohne Push (Wunsch von 🧑), der Job ist also nie gelaufen; lokal ist
`npm run check:dev` grün. Damit fehlt der Nachweis für B-046/AC-06 („der CI-Job ist grün“).

## Ziel

Der CI-Job k3c-dev ist einmal grün gelaufen. Nutzen: Die Prüfung von `tools/k3c-dev` schützt `main` tatsächlich,
nicht nur auf dem Entwickler-PC.

## Beteiligte und Zielgruppen

🧑 pusht; wer danach an `tools/k3c-dev` arbeitet.

## Anforderungen

- Nach dem nächsten Push von `main` den Lauf des Jobs `k3c-dev` ansehen; ist er rot, die Ursache beheben (Job oder Code).

## Nicht-Ziele

Weitere Jobs; Frontend- oder Wails-Build (kommt mit M4).

## Regeln und Einschränkungen

Push nur durch 🧑. Eine Korrektur am Job ist INF, am Code SRV.

## Beispiele

Push → Lauf `k3c-dev · Tests · Lint (Windows)` grün → B-046/AC-06 vollständig belegt.

## Ausnahme- und Fehlerfälle

`golangci-lint-action` findet das Unterverzeichnis-Modul nicht → `working-directory` bzw. `args` anpassen.

## Akzeptanzkriterien

- **AC-01** Ein Lauf des Jobs `k3c-dev` auf `main` ist grün (Link im Ergebnis).

## Offene Fragen

keine

## Notizen

Aus der Abnahme von M1 (M1.4, 2026-09-30).

Ergebnis (2026-09-30): AC-01 geprüft von 🧑 (Chat „ci build ist durch“). Der Job `k3c-dev · Frontend · Tests · Lint
· Wails (Windows)` ist auf `main` grün, Commit [0000f7a](https://github.com/Tenvan/k3c-family/commit/0000f7a22bbbcc6a9c5555e6045b5ed151d0902a) mit den
Checks am Commit. Davor war er einmal rot: Der Frontend-Typecheck fand `vitest` nicht, weil der Job das Hauptprojekt
nicht installierte; behoben in `229ca6d`.
