# REL · Release & Betrieb

- **Status:** aktiv
- **Rang:** 11
- **Ziel-Tickets:** –

## Ziel

Releases sind belegt und reproduzierbar (CI, Image ohne Dev-Mode, Repo-Hygiene), der Server ist im Heimnetz leicht zu starten und zu finden, das Spiel ist veröffentlicht.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| RL1 | Release-Checkliste | erledigt |
| CI1 | CI-Nachweis, Release-Image ohne Dev-Mode, Test-Abdeckung | erledigt |
| RP1 | Repo-Hygiene: Branches aufräumen, Altlasten, Regeln | erledigt |
| BT1 | Server im Heimnetz finden, Windows-Starter, Start mit Seed | aktiv |
| PG1 | Präsentationsseite mit echten Spielbildern | geplant |
| PB1 | Veröffentlichung auf itch.io | geplant |

## Nicht-Ziele

Entwickler-Werkzeuge, Asset-Studio und Workbench (WZG); Testläufe, Lasttest und `sim_test` (TST); Balancing (BAL).

## Notizen

Stand 2026-10-18 (Abgleich mit `develop`):
- CI läuft auf `develop` grün in allen Jobs (Typecheck/Tests/Build, Go, Go arm64, Docker mit Smoke-Test, k3c-workbench, Asset-Studio); PRs prüfen nur Merge-Konflikte, Tests laufen nach dem Merge. Damit ist B-053 (CI-Nachweis SP01) überholt.
- `K3C_DEV=0` setzen weder `Dockerfile` noch `compose.yaml` (B-273 offen); eine Test-Abdeckung für `engine/` zeigt die CI nicht (B-019 offen).
- BT1: B-040 (Server-Suche, QR) ist verworfen, die Session dafür entfällt; Start per URL mit Seed (B-095) ist nicht gebaut (`lobbyLogic.ts` kennt kein `seed`).
- `site/` enthält nur `index.html`, keine Spielbilder (PG1 offen); Bilder können aus der Live-Vorschau des Asset-Studios (Welt-Betrachter) oder dem Browser-Pane stammen.
