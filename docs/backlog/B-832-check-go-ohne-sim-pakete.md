# B-832 · task check:go führt trotz Sperre die Balance- und Lasttests aus

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`task check:go` ruft `go:test` mit `./...` ohne `-short` auf (`Taskfile.yml`, `engine/Taskfile.yml`), damit laufen `engine/balance`, `cmd/k3c-sim` und `cmd/k3c-load` mit ihren Simulations- und Lasttests. Das widerspricht der Sperre (Beschluss Ralf 2026-11-11), die Review-Session soll check:go aber fahren. Gefunden im Review WZ41.3: dort ersetzt durch den CI-Befehl (`go test -short` ohne Sim-Pakete) plus `go:lint`.

## Ziel

`task check:go` (und `check_run task:check:go`) lässt die Sim-, Balance- und Lasttest-Pakete aus, solange die Sperre gilt, gleiche Liste wie `tools/checkdiff` › `simDirs` und `ci.yml` › `GO_SIM_PKGS`; `CHECK_SIM=1` hebt das auf.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task check:go` testet `engine/balance`, `cmd/k3c-sim`, `cmd/k3c-load` nicht, außer mit `CHECK_SIM=1`.
- **AC-02** Die Paketliste steht an einer Stelle oder ein Test prüft, dass Taskfile, `ci.yml` und `simDirs` gleich sind.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Gefunden von WZ41.3 (2026-11-13).
