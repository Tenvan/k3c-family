# Family Three Crowns (K3C)

[![CI](https://github.com/tenvan/k3c-family/actions/workflows/ci.yml/badge.svg)](https://github.com/tenvan/k3c-family/actions/workflows/ci.yml)

Couch-Koop-Strategie-Side-Scroller im Stil von *Kingdom Two Crowns*, als natives Spiel für den PC (Go + Ebitengine,
Entscheidung 008) mit mehreren Controllern. Browser-Client, Spielserver und Online-Spiel sind entfernt (B-850, Sprint WR1).

## Loslegen

```bash
task install
task native:game
```

`task native:game` startet das Spiel direkt aus dem Repo. Menü: Leertaste, Enter oder **A**; Mitspieler treten mit Bestätigen
bei; im Spiel laufen mit **A**/**D**, Pfeiltasten oder Stick; Esc oder **Menu** pausiert. Direkt ins Spiel:
`task native:game -- --seed <seed>`.

```bash
task build
```

baut das Spiel als Paket nach `dist/k3c-game/` (EXE, Papierschnitt-Grafik, Ton, Schrift, `LIZENZEN.txt`; wie
`task native:game:build`). Den Ordner kopieren und dort `k3c-game.exe` starten.

Im Spiel:

- Münzen geben: an Bauplätze, Werkstatt (Bögen), Landstreicher (werden Bauern), Bäume/Felsen (Bauer holt das Material).
- Nachts kommen Gegner aus den Portalen. Mauern halten sie auf, Bogenschützen schießen automatisch.
- Am Levelende liegt der **Tiefen-Eingang**: Stehen alle Spieler dort, geht es eine Stufe tiefer (eigener Hub).
  Zurück nach oben geht es über eine gebaute **Treppe hoch** im Hub.

## Level anpassen

Die Eckdaten jeder Stufe stehen in `data/biomes/*.json` (Länge, Chunk-Häufigkeiten, Ressourcen,
Portale, Gegner). Nach Änderungen `task check:go` ausführen. Die Go-Tests prüfen die Seeds pro Biom auf Spielbarkeit.

## Entwickler-Werkzeug k3c-workbench

`tools/k3c-workbench` ist ein MCP-Server für Coding-Agenten. Er führt Prüfungen aus einem festen Katalog verdichtet aus
(`check_run`: nur Exit-Code, Dauer und Fehlerzeilen), macht die JSON-Logs unter `logs/` lesbar (`logs_*`), plant Tickets,
Sprints und Sessions (`plan_*`), ändert Papierschnitt-Assets (`assets_*`) und zählt jeden Aufruf. Er läuft als Fenster
`K3C Dev` (Wails), Schließen beendet ihn. Voraussetzungen: Go, Wails-CLI und
`bun install --frozen-lockfile --cwd tools/k3c-workbench/frontend` (`requirements.md`).

```bash
task k3c-workbench:build
```

baut `tools/k3c-workbench/build/bin/k3c-workbench.exe` (starten per Doppelklick, die EXE muss im Repo liegen). Zum Entwickeln
mit Neuladen: `task k3c-workbench` (Backend mit `-dev`, Oberfläche unter http://localhost:5191; das Fenster mit `wails dev`: `task k3c-workbench:window`). Die Oberfläche allein läuft im Browser gegen erfundene Daten:
`bun run --cwd tools/k3c-workbench/frontend dev` (Port 5191).

Er lauscht nur an `http://127.0.0.1:5180/mcp` (anderer Port: `K3C_DEV_PORT`). Claude Code verbindet sich über die
eingecheckte `.mcp.json` (gilt so auch in jedem Worktree), freigegeben in `.claude/settings.json`. Läuft k3c-workbench nicht,
fehlen die Tools nur in der Session.

Es läuft genau **eine** Instanz, gestartet aus der Repo-Wurzel; alle Worktrees teilen sie. Der `headersHelper` der
`.mcp.json` schickt bei jeder Anfrage das Arbeitsverzeichnis der Session (Header `X-K3C-Root`), ohne ihn gelten die MCP-roots
des Clients. Jedes Tool liest, schreibt, prüft und startet dann nur in diesem Checkout. Ein fremder Ordner wird abgelehnt.

Prüfen: `task check:dev`.

## CI/CD (GitHub Actions)

- **CI** (Push auf `main`, von Hand): Lint, Typecheck und Tests der Wurzel (Regeln, Planung, Papierschnitt); Go-Job mit
  Tests, `golangci-lint` und Cross-Build; Windows-Jobs für das native Spiel und `tools/k3c-workbench`.
- **GitHub Pages** (Push auf `main`): die statische Präsentationsseite `site/`, ohne Build.
  Einmalig aktivieren: *Settings → Pages → Source: GitHub Actions*.
- **Release** (Tag `v*`, z.B. `git tag v0.2.0 && git push --tags`): Zip mit dem nativen Spiel für Windows (`dist/k3c-game/`).

## Lizenz

Quelltext: [PolyForm Noncommercial 1.0.0](LICENSE). Eigene Grafiken und Sounds: [CC BY-NC 4.0](LICENSE-ASSETS).
Das Projekt ist *source-available*, aber **nicht** für kommerzielle Nutzung frei (kein Open Source im Sinne der OSI).
Fremdmaterial behält seine Lizenz: [Sound-Credits](public/audio/README.md), im Spiel die Lizenz-Seite im Menü und
`LIZENZEN.txt` im Paket.

Mehr: [Game Design](docs/game-design.md) · [Roadmap](docs/roadmap.md) · [Arbeitsweise](docs/arbeitsweise.md) · [Sprints](docs/sprints/README.md) · [Backlog](docs/backlog/README.md)
