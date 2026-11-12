# WZ46 · DEV · Workbench-Tests ohne Engine-Spielserver

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-746
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Workbench-Tests ziehen `engine/net`, `room` und `sim` über `botdev` und `sim_test` online herein (B-746). Entscheidung 009: Das Online-Spiel über `/ws` entfällt; ob `k3c-server`, `engine/net`, `k3c-load`, `k3c-tui` bleiben, klärt ein Teil-Ticket von B-823. Je nach Ergebnis wird B-746 umgesetzt oder entfällt.

## Ziel

Die Workbench-Tests brauchen keinen Go-Engine-Code außer über `internal/studioview`.

## Beteiligte und Zielgruppen

🧑 Ralf (entscheidet über Server-Werkzeuge in der Zerlegung von B-823); Claude (Umsetzung).

## Anforderungen

`B-746 › Anforderungen`.

## Nicht-Ziele

Server-Werkzeuge entfernen (B-823-Teil-Ticket).

## Regeln und Einschränkungen

Domäne DEV; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Erst nach der Zerlegung von B-823 planen.

- **Bereich:** `tools/k3c-workbench/internal/botdev/**`, `tools/k3c-workbench/internal/mcpsrv/simtest_online*.go`, `tools/k3c-workbench/.golangci.yml`
- **Abhängig von:** B-823
- **Braucht 🧑:** Entscheidung
- **Sperre:** –

## Beispiele

`go list -test -deps ./tools/k3c-workbench/...` ohne `k3c/engine/net` außerhalb `internal/studioview`.

## Ausnahme- und Fehlerfälle

Entfällt `botdev` mit B-823 → B-746 wird verworfen (Grund im Ticket), Sprint ebenso.

## Akzeptanzkriterien

- **AC-01** `B-746/AC-01`.

## Offene Fragen

Fake im Test oder echtes `bin/k3c-server` (`B-746 › Offene Fragen`); bleibt das Online-Werkzeug überhaupt (Zerlegung B-823)? Entscheidet 🧑.

## Sessions

Entwurf (Stichpunkte, Session-Dateien vor dem Aktivieren):

- WZ46.1 Fake-Spielserver bzw. Umbau der Online-Tests (AC-01)
- WZ46.2 Review (alle)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
