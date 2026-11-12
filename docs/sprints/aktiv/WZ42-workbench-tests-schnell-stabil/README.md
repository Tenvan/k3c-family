# WZ42 · DEV · Workbench-Tests schnell und stabil

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-738, B-787, B-704, B-695, B-702, B-621, B-747
- **Start-Commit:** ebbf1d9fd
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Die Go-Suite der Workbench ist langsam und wackelt unter Last: einzelne Tests in `assetsui` und `paperassets` brauchen mit `-short` 40–57 s (B-787), mehrere Tests scheitern nur bei parallelen Agenten (B-704, B-695, B-702, B-621). B-738 (rote Tests: Style-Fixture, Rig-Vorlage `vierbeiner`, Review-Hashes) ist laut #517 behoben, aber nicht belegt. B-747: Doku und Codemaps nennen noch `bin/k3c-balance` und `internal/enginetools`. Entscheidung 009: Online-Spiel und Lasttest entfallen künftig; bis B-823 bleiben `cmd/k3c-load` und `internal/botdev` im Repo und werden nur stabil gehalten.

## Ziel

Ein geändertes Workbench-Paket prüft `check:diff` in Sekunden, und `check:dev` ist auch bei parallelen Agenten verlässlich grün.

Am Ende sichtbar: `go test -short -count=1 -json ./...` in `tools/k3c-workbench` ohne Test ≥ 3 s; `check_run task:check:dev` zweimal nacheinander grün.

## Beteiligte und Zielgruppen

Claude (Umsetzung WZ42.1–WZ42.3, Review WZ42.4); alle parallel arbeitenden Agenten profitieren.

## Anforderungen

`B-738 › Anforderungen`, `B-787 › Anforderungen`, `B-704 › Anforderungen`, `B-695 › Anforderungen`, `B-702 › Anforderungen`, `B-621 › Anforderungen`, `B-747 › Anforderungen`. Sprint-eigen: keine Behebung per Sleep; ein Test wird nur mit Grund übersprungen (`testing.Short()` mit Kommentar), nie gelöscht, solange sein Paket bleibt.

## Nicht-Ziele

Workbench-Tests ohne Spielserver (B-746, WZ46); `cmd/k3c-load`, `internal/botdev` oder Online-Teile entfernen (B-823); `check_run`-Teilaufgaben und Sim-Sperre im Werkzeug (B-788, WZ44); Asset-Tools (WZ43); Sim-, Balance- und Lasttest-Läufe (gesperrt).

## Regeln und Einschränkungen

Domäne DEV; `docs/arbeitsweise.md` › Tempo im Lauf (lange Tests, B-619); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Alle Tickets Klasse A (Problem/Schuld, DEV, ohne Spielregeln, Protokoll, Spielstand, Abhängigkeiten). `docs/rules/zielkorridore.md` nur für den Pfadverweis aus B-747, keine Regeländerung. Kein Neubau der Workbench-EXE nötig.

- **Bereich:** `tools/k3c-workbench/internal/simrun/**`, `tools/k3c-workbench/internal/botdev/**`, `tools/k3c-workbench/internal/assetsui/**`, `tools/k3c-workbench/internal/paperassets/**`, `tools/k3c-workbench/internal/review/**`, `tools/k3c-workbench/internal/mcpsrv/check_run*.go`, `tools/k3c-workbench/internal/mcpsrv/check_test.go`, `tools/k3c-workbench/app_logs*.go`, `tools/k3c-workbench/frontend/src/balancing/**`, `cmd/k3c-load/**`, `cmd/k3c-sim/**`, `codemap.md`, `engine/codemap.md`, `engine/level/codemap.md`, `engine/sim/codemap.md`, `docs/messung/rst-baseline.md`, `docs/rules/zielkorridore.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

`go test -short ./internal/assetsui` dauert Sekunden statt einer Minute. `TestRunProcessExitUndAusgabe -count=20` grün. `TestSourcesUndConsoleTail -count=50` grün, während eine zweite `go test ./...` läuft.

## Ausnahme- und Fehlerfälle

Ein Test braucht echte Umgebung (Git-Repo, `go build`, `tools/paper`-CLI, Server-Start) → `testing.Short()`-Skip mit Grund, voller Lauf in `check:dev`. Ein Flake liegt außerhalb des Bereichs → Ticket mit Fundstelle. `golangci-lint` parallel (B-704/AC-02) ist durch den Prüf-Slot aus B-739 erledigt → belegen statt ändern.

## Akzeptanzkriterien

- **AC-01** Workbench-Go-Suite auf frischem Checkout grün, Tests unverändert aussagekräftig (`B-738/AC-01`, `B-738/AC-02`).
- **AC-02** Wackelnde Tests stabil: `B-702/AC-01`, `B-621/AC-01`, `B-695/AC-01`, `B-695/AC-02`, `B-704/AC-01`, `B-704/AC-02`.
- **AC-03** Kein Test ≥ 3 s mit `-short`, jedes Paket allein ≤ 10 s, `check:dev` zweimal grün (`B-787/AC-01` bis `B-787/AC-03`).
- **AC-04** Keine unbegründeten Verweise auf `bin/k3c-balance`, `internal/enginetools`, `internal/balance/cmd` (`B-747/AC-01`).

## Offene Fragen

keine blockierenden. Defaults (Claude, 2026-11-12): Online- und Lasttests (`cmd/k3c-load`, `internal/botdev`) bekommen einen `testing.Short()`-Skip statt Umbau, weil sie mit B-823 vermutlich entfallen; der optionale Umzug des Namensraums `balance` aus B-747 entfällt (Sim gesperrt).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ42.1 | `WZ42.1-rote-und-wackelnde-tests.md` | Umsetzung | autonom | offen |
| WZ42.2 | `WZ42.2-short-unter-drei-sekunden.md` | Umsetzung | autonom | offen |
| WZ42.3 | `WZ42.3-verweise-balance.md` | Umsetzung | autonom | offen |
| WZ42.4 | `WZ42.4-review.md` | Review | autonom | offen |

Keine eigenen Sim- oder Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
