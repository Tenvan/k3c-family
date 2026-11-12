# WZ41 · INF · CI und Release grün, check:diff-Protokoll belegt

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-739, B-756, B-757, B-758, B-759, B-825
- **Start-Commit:** ebbf1d9fd
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Nach Welle 4 ist der Stand der CI unklar: B-756 (Grafik vor den Go-Tests rendern, #526), B-757 (Race ohne `engine/balance`, datadoc-Test, #527), B-759 (`.work` aus dem Docker-Kontext, #529) und B-739 (check:diff-Protokoll und Prüf-Slot, #517) sind laut Commits umgesetzt, die Tickets aber offen und nicht belegt. B-758 hat den Release-Workflow mit #528 nur vorläufig ohne Typecheck und Tests laufen lassen; das muss zurück. Entscheidung 009: Der Browser-Client bleibt bis B-823 lauffähig, wird aber nicht weiterentwickelt; CI und Release prüfen ihn bis dahin weiter.

## Ziel

CI auf `develop` und der Release-Workflow sind grün und prüfen vor dem Bauen; die schon umgesetzten Tickets sind belegt und geschlossen.

Am Ende sichtbar: GitHub Actions auf `develop` alle Jobs grün; `release.yml` mit `task typecheck` und `task test` vor `task build`; `logs_query service=k3c-workbench pattern=schritt` zeigt Schritte mit Dauer.

## Beteiligte und Zielgruppen

Claude (Umsetzung WZ41.1, WZ41.2; Review WZ41.3); alle Agenten profitieren von verlässlicher CI. 🧑 Ralf entscheidet nur, falls ein Release-Tag nötig wird (B-759/AC-01).

## Anforderungen

`B-739 › Akzeptanzkriterien`, `B-756 › Anforderungen`, `B-757 › Anforderungen`, `B-758 › Anforderungen`, `B-759 › Anforderungen`. Sprint-eigen: Belegen heißt je Kriterium Befehl und Ergebnis im Session-Ergebnis; fehlt etwas, wird es in dieser Session nachgezogen, solange es im Bereich liegt.

## Nicht-Ziele

Browser-Client entfernen, Dockerfile oder CI auf das native Spiel umstellen (Teil-Tickets von B-823); Workbench-Tests beschleunigen (WZ42, B-787); neue Prüfschritte in `check:diff`; Sim-, Balance- und Lasttest-Läufe (gesperrt).

## Regeln und Einschränkungen

Domäne INF; Tasks nur über `task`; Release-Checkliste in `docs/arbeitsweise.md` › Release. Alle Tickets Klasse A (Problem/Schuld, INF, ohne Spielregeln, Protokoll, Spielstand, Abhängigkeiten). Ein Release-Tag setzt nur 🧑; ohne Tag gilt B-759/AC-01 als belegt durch einen manuell ausgelösten Workflow-Lauf ohne Veröffentlichung (`workflow_dispatch`) oder einen lokalen `docker build`.

- **Bereich:** `.github/workflows/**`, `Taskfile.yml`, `engine/Taskfile.yml`, `Dockerfile`, `.dockerignore`, `tools/checkdiff/**`, `tools/k3c-workbench/internal/datadoc/**`, `tools/k3c-workbench/internal/mcpsrv/check_steps*.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** taskfile

## Beispiele

Push auf `develop` → Jobs `go`, `go-arm64`, `k3c-workbench`, `web` grün. Release-Lauf → Typecheck und Tests vor `task build`, Image für amd64 und arm64.

## Ausnahme- und Fehlerfälle

Ein CI-Job ist rot aus einem Grund außerhalb des Bereichs → Ticket mit Fundstelle, Kriterium `verschoben`. Docker-Build lokal nicht möglich (kein Docker) → Workflow-Lauf als Nachweis, sonst `blockiert` mit Grund.

## Akzeptanzkriterien

- **AC-01** Grafik-Golden je Gruppe, Schritt-Protokoll und Prüf-Slot belegt (`B-739/AC-01` bis `B-739/AC-04`).
- **AC-02** CI-Jobs, die Grafik lesen, rendern sie vorher; CI auf `develop` grün (`B-756/AC-01`).
- **AC-03** `task go:race` ohne `engine/balance`, datadoc-Test grün, Jobs `go` und `k3c-workbench` grün (`B-757/AC-01` bis `B-757/AC-03`).
- **AC-04** Docker-Image entsteht ohne `.work` im Kontext (`B-759/AC-01`).
- **AC-05** `release.yml` mit `task typecheck` und `task test` vor `task build`, CI grün, `task check:all` nachgeholt mit Befunden als Tickets (`B-758/AC-01` bis `B-758/AC-03`).
- **AC-06** Die CI führt die Tests wieder aus, ohne Sim- und Balance-Pakete, Laufzeit je Job belegt (`B-825/AC-01` bis `B-825/AC-03`; nach WZ42, Entscheidung Ralf 2026-11-12).

## Offene Fragen

keine blockierenden. Default (Claude, 2026-11-12): B-758/AC-03 prüft den aktuellen Stand von `develop` statt des alten Tags v0.22.0, weil der Tag-Stand nicht mehr gepflegt wird; Meldung im Nachgang.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ41.1 | `WZ41.1-ci-belegen.md` | Umsetzung | autonom | fertig |
| WZ41.2 | `WZ41.2-release-pruefungen.md` | Umsetzung | autonom | fertig |
| WZ41.3 | `WZ41.3-review.md` | Review | autonom | offen |
| WZ41.4 | `WZ41.4-ci-tests-einschalten.md` | Umsetzung | autonom | offen |

Keine eigenen Sim- oder Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
