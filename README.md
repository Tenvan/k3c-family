# Family Three Crowns (K3C)

[![CI](https://github.com/tenvan/k3c-family/actions/workflows/ci.yml/badge.svg)](https://github.com/tenvan/k3c-family/actions/workflows/ci.yml)

Couch-Koop-Strategie-Side-Scroller im Stil von *Kingdom Two Crowns*, gebaut für den Browser,
damit er auf der **Xbox (Edge)** mit mehreren Controllern läuft.

## Loslegen

```bash
npm install
npm run dev
```

Dann `http://localhost:5173` öffnen, oder im Heimnetz `http://<PC-IP>:5173` (z.B. von der Xbox aus).
Die Startseite ist eine Landingpage, die man komplett mit dem Controller bedient (D-Pad/Stick + **A**,
**Y** bzw. **F** = Vollbild). Sie bleibt dauerhaft geöffnet und zeigt Spiel und Testseiten in sich an.
Dadurch bleibt Vollbild beim Seitenwechsel erhalten.

Jede Seite hat oben mittig einen **Start**-Button. Zurück zur Übersicht geht es auch mit **View + Menu**
(kurz gemeinsam halten) bzw. **Pos1**. **B** wird abgefangen und schließt nichts versehentlich.

Im Spiel:

- **A** (Controller) / **Leertaste**: Beitreten (bis zu 2 Spieler, Split-Screen)
- **A** / **Leertaste halten**: Münzen geben – an Bauplätze, Werkstatt (Bögen), Landstreicher (werden Bauern),
  Bäume/Felsen (Bauer holt das Material). Ohne Ziel fällt die Münze, der andere Spieler kann sie aufheben.
- Linker Stick / **A**,**D**: laufen, **RT** / **Shift**: sprinten
- **F** / rechten Stick drücken: Vollbild
- Nachts kommen Gegner aus den Portalen. Mauern halten sie auf, Bogenschützen schießen automatisch.
- Am Levelende liegt der **Tiefen-Eingang**: Stehen alle Spieler dort, geht es eine Stufe tiefer (eigener Hub).
  Zurück nach oben geht es über eine gebaute **Treppe hoch** im Hub.
- Dev: **N** neuer Seed, **1/2/3** Tiefe wechseln, URL-Parameter `?seed=abc&depth=1`,
  `?fast=1` (Tag/Nacht 8x schneller), `?dev=1` (**G** +10 Gold, **H** +50 Material, **T** nächste Tageszeit, **S** speichern)

## Im Heimnetz hosten (Xbox)

```bash
npm run serve
```

Baut das Spiel und startet den Server auf Port **8080**. Die Konsole zeigt die Adressen, z.B.
`http://192.168.2.230:8080/`. Diese Adresse in Edge auf der Xbox öffnen. Falls die Windows-Firewall fragt:
Zugriff im **privaten** Netzwerk erlauben.

### Gamepad-Test auf der Xbox

1. `npm run serve` am PC starten.
2. Auf der Xbox in Edge `http://<PC-IP>:8080/` öffnen und die Kachel **Gamepad-Test** wählen.
3. Beide Controller verbinden, auf jedem alle Tasten einmal drücken, auch **B** und die Sticks.
4. **Vollbild** anklicken, dann **View** für den FPS-Test drücken (dauert ca. 30 s, danach View = zurück).
5. **Y** drücken. Der Bericht landet am PC in `reports/gamepad-*.json`.

### HTTPS (nur falls nötig)

Falls der Test zeigt, dass die Xbox die Gamepad API ohne HTTPS nicht freigibt: `certs/key.pem` und
`certs/cert.pem` ablegen. Dann läuft zusätzlich HTTPS auf Port **8443**. Ein selbstsigniertes Zertifikat
erzeugt auf der Xbox eine Warnung. Ob Edge die Seite danach als sicher behandelt, zeigt der Test
(„Secure Context“). Sonst bräuchte es ein echtes Zertifikat, etwa eine eigene Domain mit Let's Encrypt.

### Spielstände

**Weiterspielen** lädt den letzten Stand, **Neues Spiel** beginnt von vorn. Gespeichert wird automatisch bei Tagesanbruch,
beim Wechsel in eine andere Tiefe und beim Verlassen des Spiels, auf dem Heimnetz-Server in `saves/autosave.json`
(zusätzlich im Browser). Ein neues Spiel sichert den alten Stand vorher als `saves/autosave-<Datum>.json`.
Zum Zurückholen die Sicherung einfach in `autosave.json` umbenennen.

## Neue Seite hinzufügen (z.B. weitere Testseiten)

1. `meine-seite.html` in den Projektordner legen. Der Build nimmt jede `*.html` automatisch auf.
2. Im Script `installPageChrome()` aus `src/core/shell.ts` aufrufen. Das bringt Start-Button, View + Menu und B-Schutz mit.
3. Eintrag in `src/landing/pages.ts` ergänzen. Danach erscheint die Kachel auf der Startseite.
4. Vollbild nur über `toggleFullscreen()` aus `src/core/fullscreen.ts`.

Die vollständige Regel steht in `CLAUDE.md` unter „Regel: Seiten & Navigation“.

## Level anpassen

Die Eckdaten jeder Stufe stehen in `data/biomes/*.json` (Länge, Chunk-Häufigkeiten, Ressourcen,
Portale, Gegner). Nach Änderungen `npm test` ausführen. Die Tests prüfen 500 Seeds pro Biom auf Spielbarkeit.

## Entwickler-Werkzeug k3c-dev

`tools/k3c-dev` ist ein MCP-Server für Coding-Agenten. Er führt Prüfungen aus einem festen Katalog verdichtet aus
(`check_run`: nur Exit-Code, Dauer und Fehlerzeilen), macht die JSON-Logs unter `logs/` lesbar (`logs_*`) und zählt jeden
Aufruf. Die Dienste aus `tools/k3c-dev/services.json` (Vite-Dev-Server, Heimnetz-Server) startet und stoppt er über
`svc_*`; schon laufende übernimmt er, beim Beenden stoppt er nur die eigenen. Starten (Go aus `requirements.md`), läuft bis Strg+C:

```bash
cd tools/k3c-dev && go run .
```

Er lauscht nur an `http://127.0.0.1:5180/mcp` (anderer Port: `K3C_DEV_PORT`). Für Claude Code eine lokale `.mcp.json`
im Repo anlegen (steht in `.gitignore`):

```json
{ "mcpServers": { "k3c-dev": { "type": "http", "url": "http://127.0.0.1:5180/mcp" } } }
```

Prüfen: `npm run check:dev`. Plan für Statistik, Dienste und Oberfläche: Sprints M2–M5 in [`docs/sprints/`](docs/sprints/README.md).

## CI/CD (GitHub Actions)

- **CI** (jeder Push/PR): Lint, Typecheck, Tests, Build, Smoke-Test des Heimnetz-Servers; Go-Job mit Tests, `golangci-lint` und Cross-Build für Windows und Raspberry Pi; Windows-Job für `tools/k3c-dev`. Der Build liegt als Artefakt `k3c-dist` am Lauf.
- **GitHub Pages** (Push auf `main`): Spiel und Testseiten online, ohne Bericht-Server.
  Einmalig aktivieren: *Settings → Pages → Source: GitHub Actions*.
- **Release** (Tag `v*`, z.B. `git tag v0.2.0 && git push --tags`): Zip mit `dist/` + `server/` am Release.
  Entpacken und `node server/server.mjs` starten, `npm install` ist dafür nicht nötig.

Mehr: [Game Design](docs/game-design.md) · [Roadmap](docs/roadmap.md) · [Arbeitsweise](docs/arbeitsweise.md) · [Sprints](docs/sprints/README.md) · [Backlog](docs/backlog/README.md)
