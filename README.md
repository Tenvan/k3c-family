# Family Three Crowns (K3C)

[![CI](https://github.com/tenvan/k3c-family/actions/workflows/ci.yml/badge.svg)](https://github.com/tenvan/k3c-family/actions/workflows/ci.yml)

Couch-Koop-Strategie-Side-Scroller im Stil von *Kingdom Two Crowns*, gebaut für den Browser,
damit er auf der **Xbox (Edge)** mit mehreren Controllern läuft.

## Loslegen

```bash
task install
task dev
```

Dann `http://localhost:5173` öffnen, oder im Heimnetz `http://<PC-IP>:5173` (z.B. von der Xbox aus).
Die Startseite ist eine Landingpage, die man komplett mit dem Controller bedient (D-Pad/Stick + **A**,
**Y** bzw. **F** = Vollbild). Sie bleibt dauerhaft geöffnet und zeigt Spiel und Testseiten in sich an.
Dadurch bleibt Vollbild beim Seitenwechsel erhalten.
Der Dev-Server reicht `/api` und `/ws` an den Go-Server (Port 8080) weiter: Zum Spielen zusätzlich `task start` laufen lassen.

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
task serve
```

Baut das Spiel und startet den Go-Server (`bin/k3c-server`) auf Port **8080**; `task start` startet ihn ohne Web-Build. Die Konsole zeigt die Adressen, z.B.
`http://192.168.2.230:8080/`. Diese Adresse in Edge auf der Xbox öffnen. Falls die Windows-Firewall fragt:
Zugriff im **privaten** Netzwerk erlauben.

Der Go-Server (`cmd/k3c-server`) liefert Seiten, Spielstände, Berichte und den Online-Modus (WebSocket `/ws`, Protokoll v2).
Einstellungen per Umgebung: `K3C_HTTP_PORT`,
`K3C_HTTPS_PORT`, `K3C_DIST`, `K3C_SAVES_DIR`, `K3C_REPORTS_DIR`, `K3C_CERTS_DIR`, `K3C_LOG_DIR` (JSON-Log `k3c-server.jsonl`; ohne Angabe nur, wenn ein Ordner `logs/` existiert), `K3C_DEV` (Dev-Mode: leer oder `1` = an, `0` = aus; im Dev-Mode ist der Grad `dev` wählbar und der Standardgrad neuer Räume, sonst `normal`; Standard in der Entwicklungsphase: an); `GET /api/health` meldet
`{"ok":true}`.

- **Level ansehen:** `GET /api/level?seed=test&biome=forest` liefert das Level, das ein Raum mit diesem Seed und Biom bekäme
  (`chunks`, `entities`, Breite und `warnings` der Spielbarkeits-Prüfung), als JSON. Ohne `seed` gilt `k3c`, ohne `biome` `forest`
  (`forest`, `cave`, `mine`); ein unbekanntes Biom oder ein Seed über 64 Zeichen ergibt 400, andere Methoden als GET 405.
  Reine Berechnung: kein Token, kein Raum, keine Datei.
- **Diagnose:** `GET /api/status` mit `Authorization: Bearer <K3C_STATUS_TOKEN>`; ohne gesetzte Variable ist sie aus
  (404), mit falschem Token 401. Der Status nennt Räume, Tick-Dauer
  und `memory` (Heap und Systemspeicher in MB); `?room=CODE` zeigt den Raum samt `devices` (Kennung = Anfang der Geräte-ID,
  verbunden, Slots). `GET /api/status/log?since=<Byte-Cursor>&limit=<n>` liefert Zeilen des JSON-Logs (`k3c-server.jsonl`)
  ab dem Cursor und den neuen Cursor (404 „Log aus“ ohne Log-Ordner). Aktionen (nur `POST`, gleiches Token,
  Eintrag im Log mit `ns` `diag`): `/api/status/disconnect?room=CODE&device=KENNUNG` trennt die Verbindung eines Geräts (es
  verhält sich wie nach einem Abbruch und darf sich bis zur Frist wieder verbinden), `/api/status/save?room=CODE` sichert den
  Spielstand des Raums sofort.
- **Diagnose-TUI:** `go run ./cmd/k3c-tui` (oder im Container `docker exec -it k3c k3c-tui`) zeigt Räume, Geräte, Tick-Dauer, Speicher
  und Abstürze live (jede Sekunde). Token `K3C_STATUS_TOKEN`, Adresse `K3C_SERVER_URL` oder `K3C_HTTP_PORT` (Standard 8080);
  im Container gelten die Variablen des Servers. `k3c-tui -once` druckt den Zustand einmal als Text und endet (Skripte, CI,
  `docker exec` ohne Terminal). Tasten: ↑↓ oder `j` `k` wählen, Enter öffnet einen Raum (Gold, Truppen, Geräte), dort `d` trennt
  das gewählte Gerät (erst nach `y`), `s` sichert den Spielstand, `l` zeigt das Log des Servers (`f` folgen, ↑↓ blättern),
  Esc geht zurück, `q` in der Übersicht beendet. Das Log braucht ein JSON-Log am Server (`K3C_LOG_DIR`, siehe oben).
- **Sicherungen:** Jeder Speichervorgang legt den vorigen Stand unter `saves/backups/<slot>/` ab, je Spielstand bleiben
  die letzten 5. `GET /api/save/backups?slot=autosave` listet sie,
  `curl -X POST -H "Authorization: Bearer $K3C_STATUS_TOKEN" "http://<server>:8080/api/save/restore?slot=autosave&backup=<name>"`
  macht eine davon wieder zum aktuellen Stand (der bisherige wird dabei gesichert).
- **Schutzmodell** (B-143): Restore braucht `K3C_STATUS_TOKEN` wie `/api/status`; ohne gesetzte Variable ist es aus (404),
  ohne oder mit falschem Token 401. Jede Ablehnung steht im Log (`ns` `save`, Pfad, Aufrufer, Grund, nie das Token).
  Spielen, Lobby, `/ws`, Speichern, Berichte und Client-Log brauchen kein Token; Berichte und Client-Log sind nur durch
  Größenlimits und Rotation begrenzt. Aufrufer ist die Adresse der Verbindung, `X-Forwarded-For` wird nicht ausgewertet
  (hinter einem Proxy steht dessen Adresse im Log). Der Server ist fürs Heimnetz gedacht: **kein Port-Forwarding** am
  Router, nicht aus dem Internet erreichbar machen.
- **Docker** (z. B. Raspberry Pi, 64-Bit-Betriebssystem): `docker compose up -d` zieht das Image `ghcr.io/tenvan/k3c-family`
  (amd64 und arm64, vom Release-Tag `v*` gebaut) und startet es auf Port 8080; Spielstände und Berichte liegen im Volume
  `k3c-data` unter `/data`. **Update:** `docker compose pull && docker compose up -d`. **Rückfall:** in `compose.yaml` den
  Tag des vorigen Images eintragen (z. B. `…:v0.2.0`). Selbst bauen: `docker compose build`.
- **Backup auf den USB-Stick** (Pi, im Ordner mit `compose.yaml`): `sh scripts/backup-saves.sh /media/usb` kopiert
  `/data/saves` nach `/media/usb/k3c-saves-<Datum-Uhrzeit>/`; Fehler stehen mit Zeit auf stderr (Exit ≠ 0), das Spiel
  läuft weiter. Täglich um 4 Uhr per `crontab -e`:
  `0 4 * * * cd /home/pi/k3c && sh scripts/backup-saves.sh /media/usb >> /media/usb/k3c-backup.log 2>&1`.
  **Restore:** `docker compose cp /media/usb/k3c-saves-<…>/. k3c:/data/saves/`, dann `docker compose restart`.
  Schützt nur vor einem Ausfall der SD-Karte, nicht vor dem Verlust des Pi. Lokal: `task saves:backup -- <ziel>` mit
  `K3C_SAVES_DIR=<ordner>`. Berichte (`reports/`, die letzten 100) und das Client-Log (`k3c-client.jsonl`, 1 MB plus
  eine alte Generation `k3c-client.1.jsonl`) rotieren von selbst; Spielstände fasst die Rotation nie an.
- **Release:** Ein Tag `v*` hängt `k3c-server` für Windows, Linux amd64 und arm64 an den Release.

### Gamepad-Test auf der Xbox

1. `task serve` am PC starten.
2. Auf der Xbox in Edge `http://<PC-IP>:8080/` öffnen und die Kachel **Gamepad-Test** wählen.
3. Vor der ersten Taste den Abschnitt **Audio** ablesen. Dann **A** auf einem Controller drücken und den Button
   **Audio entsperren und Testton** anklicken; notieren, ob der Ton hörbar war (ja/nein). **Y** sendet den Bericht.
4. Beide Controller verbinden, auf jedem alle Tasten einmal drücken, auch **B** und die Sticks.
5. **Vollbild** anklicken, dann **View** für den FPS-Test drücken (dauert ca. 30 s, danach View = zurück).
6. **Y** drücken. Der Bericht landet am PC in `reports/gamepad-*.json`.

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
Portale, Gegner). Nach Änderungen `task test` ausführen. Die Tests prüfen 500 Seeds pro Biom auf Spielbarkeit.

## Entwickler-Werkzeug k3c-dev

`tools/k3c-dev` ist ein MCP-Server für Coding-Agenten. Er führt Prüfungen aus einem festen Katalog verdichtet aus
(`check_run`: nur Exit-Code, Dauer und Fehlerzeilen), macht die JSON-Logs unter `logs/` lesbar (`logs_*`) und zählt jeden
Aufruf. Die Dienste aus `tools/k3c-dev/services.json` (Vite-Dev-Server, Heimnetz-Server) startet und stoppt er über
`svc_*`; der Go-Server startet bei Änderungen an `cmd/`, `engine/` und `data/` von selbst neu (`watch` in `services.json`); schon laufende übernimmt er, beim Beenden stoppt er nur die eigenen. Er läuft als Fenster `K3C Dev` (Wails),
Schließen beendet ihn. Voraussetzungen: Go, Wails-CLI und `npm ci --prefix tools/k3c-dev/frontend` (`requirements.md`).

```bash
task k3c-dev:build
```

baut `tools/k3c-dev/build/bin/k3c-dev.exe` (starten per Doppelklick, die EXE muss im Repo liegen). Zum Entwickeln
mit Neuladen: `task k3c-dev` (`wails dev`). Die Oberfläche allein läuft im Browser gegen erfundene Daten:
`npm --prefix tools/k3c-dev/frontend run dev` (Port 5181).

Er lauscht nur an `http://127.0.0.1:5180/mcp` (anderer Port: `K3C_DEV_PORT`). Claude Code verbindet sich über die
eingecheckte `.mcp.json` (gilt so auch in jedem Worktree), freigegeben in `.claude/settings.json`. Läuft k3c-dev nicht,
fehlen die Tools nur in der Session.

Es läuft genau **eine** Instanz, gestartet aus der Repo-Wurzel; alle Worktrees teilen sie. Der `headersHelper` der
`.mcp.json` schickt bei jeder Anfrage das Arbeitsverzeichnis der Session (Header `X-K3C-Root`), ohne ihn gelten die MCP-roots
des Clients. Jedes Tool liest, schreibt, prüft und startet dann nur in diesem Checkout. Dienste eines Worktrees bekommen
eigene Ports (Versatz 10, 20 …: Vite 5183, Spielserver 8090 …); `svc_status` nennt sie. Ein fremder Ordner wird abgelehnt.

Prüfen: `task check:dev`. Plan für Statistik, Dienste und Oberfläche: Sprints M2–M5 in [`docs/sprints/`](docs/sprints/README.md).

## CI/CD (GitHub Actions)

- **CI** (jeder Push/PR): Lint, Typecheck, Tests, Build; Go-Job mit Tests (auch Heimnetz-Server per `httptest`), `golangci-lint` und Cross-Build für Windows und Raspberry Pi; Windows-Job für `tools/k3c-dev`. Der Build liegt als Artefakt `k3c-dist` am Lauf.
- **GitHub Pages** (Push auf `main`): nur was ohne Server geht. Die Testseiten laufen; die Spiel-Kacheln sind deaktiviert
  („Braucht den Heimnetz-Server“), `game.html` zeigt denselben Hinweis.
  Einmalig aktivieren: *Settings → Pages → Source: GitHub Actions*.
- **Release** (Tag `v*`, z.B. `git tag v0.2.0 && git push --tags`): Zip mit `dist/` und `k3c-server` für Windows, Linux amd64 und arm64.
  Das Zip entpacken, `k3c-server` daneben legen und starten (`K3C_DIST` zeigt auf `dist/`), Node ist dafür nicht nötig.

## Lizenz

Quelltext: [PolyForm Noncommercial 1.0.0](LICENSE). Eigene Grafiken und Sounds: [CC BY-NC 4.0](LICENSE-ASSETS).
Das Projekt ist *source-available*, aber **nicht** für kommerzielle Nutzung frei (kein Open Source im Sinne der OSI).
Fremdmaterial behält seine Lizenz: [Figuren-Credits](public/sprites/CREDITS.md), im Spiel die Seite „Lizenzen & Danksagung“ (`lizenzen.html`).

Mehr: [Game Design](docs/game-design.md) · [Roadmap](docs/roadmap.md) · [Arbeitsweise](docs/arbeitsweise.md) · [Sprints](docs/sprints/README.md) · [Backlog](docs/backlog/README.md)
