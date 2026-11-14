# B-848 · src/, Phaser, die Spielseiten und die Web-Tasks des Browser-Clients sind entfernt, CI, Docker und Release bauen ohne dist

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WR1
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-13 (entfernt nur, keine neue Abhängigkeit; wartet auf B-846)

## Ausgangslage

Teil von B-823 (Entscheidung 009). Browser-Client: `src/`, Phaser, `index.html`/`game.html`/`monitor.html`/`lizenzen.html`, Vite-Konfiguration für das Spiel, Tasks `dev`, `preview`, `build`, `serve`, `start`, `pages`; CI (`task build`, Smoke-Test), `Dockerfile` Stufe `web`, `release.yml` Paket „dist + Server“.

## Ziel

Es gibt nur noch das native Spiel als Client; `tools/paper`, die Wails-Frontends und ihre Tests laufen weiter.

## Beteiligte und Zielgruppen

Claude (Umsetzung); 🧑 liest das Ergebnis.

## Anforderungen

- `src/`, Spielseiten, Phaser und nur für das Spiel nötige Vite-Teile entfernt; `package.json` behält, was `tools/paper` und Frontends brauchen.
- Tasks des Browser-Clients entfernt oder auf das native Spiel umgestellt; `tests/` ohne Browser-Tests, die Regeltests (`projectRules`, `atomicDesign`, `planning`) bleiben.
- CI, Docker, Release ohne `dist/` des Browser-Clients.
- `CLAUDE.md` und Codemaps nachgezogen.

## Nicht-Ziele

Server und Online-Spiel (B-846, B-850), `site/` (B-849), Workbench (B-847).

## Regeln und Einschränkungen

Entfernt eine Abhängigkeit (`phaser`), Sperre deps.

- **Bereich:** `src/**`, `index.html`, `game.html`, `monitor.html`, `lizenzen.html`, `vite*.ts`, `package.json`, `bun.lock`, `Taskfile.yml`, `Dockerfile`, `.github/workflows/**`, `tests/**`, `CLAUDE.md`
- **Abhängig von:** B-846, B-847
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** deps

## Beispiele

`task check` nach dem Entfernen grün; `task native:game:build` unverändert.

## Ausnahme- und Fehlerfälle

Ein Werkzeug nutzt noch Code aus `src/` → vorher dorthin verschieben, wo es gebraucht wird, mit Test.

## Akzeptanzkriterien

- **AC-01** `src/`, Spielseiten und `phaser` sind weg, `task check` und `task check:go` grün.
- **AC-02** CI, Docker und Release laufen ohne Browser-Build (Nachweis Workflow-Diff, lokaler Docker-Build optional).

## Offene Fragen

keine

## Notizen

Zerlegung B-823 am 2026-11-13 (Claude).
