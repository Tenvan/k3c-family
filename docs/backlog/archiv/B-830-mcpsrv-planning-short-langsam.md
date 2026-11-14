# B-830 · mcpsrv und planning bleiben mit -short unter 3 s je Test und 10 s je Paket

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ48
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13

## Ausgangslage

Nach WZ42.2 (B-787) liegen mit `go test -short ./...` in `tools/k3c-workbench` noch 9 Tests ≥ 3 s, alle außerhalb des Sprint-Bereichs: 5 in `internal/mcpsrv`, 4 in `internal/planning` (3,1–4,3 s); die Pakete brauchen allein 58 s (`mcpsrv`) und 40 s (`planning`). Damit bleibt `check:diff` bei Änderungen an diesen Paketen langsam und B-787/AC-03 nur für den Sprint-Bereich erfüllt.

## Ziel

Ein geändertes `mcpsrv`- oder `planning`-Paket prüft `check:diff` in Sekunden.

## Beteiligte und Zielgruppen

Claude (Umsetzung); alle Agenten, die Planung oder MCP-Tools ändern.

## Anforderungen

- Kein Test ≥ 3 s mit `-short` in `internal/mcpsrv` und `internal/planning`, jedes Paket allein ≤ 10 s.
- Skip nur per `testing.Short()` mit Grund, kein Test gelöscht, keine Sleeps als Fix (Regeln aus WZ42).

## Nicht-Ziele

`check_run`-Teilaufgaben (B-788).

## Regeln und Einschränkungen

Domäne DEV, Klasse A.

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/internal/planning/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`go test -short -count=1 ./internal/planning` dauert Sekunden statt 40 s.

## Ausnahme- und Fehlerfälle

Ein Test braucht Git-Repo oder `go build` → `testing.Short()`-Skip mit Grund, voller Lauf in `check:dev`.

## Akzeptanzkriterien

- **AC-01** `go test -short -count=1 -json ./internal/mcpsrv ./internal/planning` ohne Test ≥ 3 s, jedes Paket allein ≤ 10 s.

## Offene Fragen

keine

## Notizen

Messwerte aus WZ42.2 (B-787): `-short ./...` 97 → 9 Tests ≥ 3 s, 123 → 60 s Wanduhr.

WZ48.2 (2026-11-13, `go test -short -count=1 -json`, Entwickler-PC, 22 Kerne): `planning` 40,9 → 5,8–7,9 s, 0 Tests ≥ 3 s (größter 2,8 s); `mcpsrv` 41,3 → 9,8–10,3 s (drei Läufe), 0 Tests ≥ 3 s. Kosten je Prozess: git rund 130 ms, Render-Attrappe (bun) 230–400 ms. Maßnahmen: t.Parallel mit Obergrenze (planning 4, mcpsrv 6), kein git in Ordnern ohne `.git` (`planning/gitrepo.go`), Vorlage für `paperRepo`, Hardlink für das falsche k3c-sim, `K3C_PAPER_CLI` einmal in TestMain, `testing.Short()`-Skip für Tests mit echten Worktrees (`gitRepo`), Render-lastige Asset-Tests (12 bun-Aufrufe) und zwei Läufe von 3 s; voller Lauf in `check:dev`.
