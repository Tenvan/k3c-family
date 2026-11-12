# B-823 · Der Browser-Client ist durch das native Spiel abgelöst und aus dem Repo entfernt

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-12
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Neben dem nativen Spiel (`native/k3c-game`, Go + Ebitengine, Entscheidung 008) lebt der Browser-Client weiter: `src/` (ca. 19,9k Zeilen TS, Stand 2026-11-10), Phaser 4, Vite, `index.html`/`game.html`/`monitor.html`/`lizenzen.html`, Tasks `dev`, `preview`, `build`, `serve`, `start`, `pages`. Er hängt an vielen Stellen (Stand 2026-11-12, ungeprüft auf Vollständigkeit):

- **Server:** `cmd/k3c-server` liefert `dist/` aus und startet nicht ohne `dist/index.html` (`cmd/k3c-server/main.go:123`); `engine/net` (WebSocket `/ws`, `docs/protocol.md`) bedient den Browser, `cmd/k3c-load` und `cmd/k3c-tui` nutzen dasselbe Protokoll.
- **Container und Release:** `Dockerfile` Stufe `web` (bun, `dist`), `K3C_DIST`; `.github/workflows/ci.yml` (`task build`, Smoke-Test `/`, `/game.html`), `release.yml` (Paket „dist + Server“, Docker-Image).
- **Workbench:** „Im Spiel ansehen“ (`tools/k3c-workbench/internal/studioview/game.go:77-89`) braucht `dist/index.html` und öffnet `game.html`.
- **Abhängigkeiten:** `package.json` mit `phaser`; `vite`, `vitest`, `typescript`, `oxlint`, `happy-dom` dienen auch `tools/paper` (bun) und den Wails-Frontends, `package.json` fällt also nicht ganz weg.
- **GitHub Pages:** `site/` ist statisch, eigene Bildkopien in `site/img/` (noch Pixel-Grafik, `site/index.html:52-113`).
- **Planung:** Projekte und Sprints der Domänen CLI und PLAT für Xbox/Edge (u. a. RX1, LG1, HW1, PF1) zielen auf den Browser.

## Ziel

Es gibt genau einen Spiel-Client, das native Spiel; Browser-Client, Phaser und die nur dafür nötige Infrastruktur sind entfernt, ohne dass Werkzeuge, Papierschnitt-Pipeline oder Spielstände ihren Weg verlieren.

## Beteiligte und Zielgruppen

🧑 Ralf entscheidet Zielplattform und Umfang; Familie als Spieler (heute Xbox/Edge am TV, künftig Mini-PC); Claude plant und setzt nach Freigabe um.

## Anforderungen

Vorläufig, erst nach den Offenen Fragen verbindlich:

- Das native Spiel deckt den Funktionsumfang ab, den die Familie heute im Browser nutzt (Abgleich aus NG9), bevor der Browser-Client fällt.
- `src/`, Phaser, Vite-Konfiguration für das Spiel, Seiten im Root, Tasks `dev`/`preview`/`build`/`serve`/`start` und die CI-/Docker-/Release-Teile für `dist/` entfallen oder werden auf das native Spiel umgestellt.
- `tools/paper`, die Wails-Frontends und ihre Tests laufen weiter.
- Je Teil ein eigenes Ticket (Größe L): Client löschen (INF), Server-Auslieferung (SRV), Workbench „Im Spiel ansehen“ (DEV), CI/Release/Docker (INF), Pages (PLAT), Planung bereinigen.

## Nicht-Ziele

- Test- und Werkzeugseiten (B-822, vorher).
- Pixel-Reste (B-824).
- Neue Funktionen im nativen Spiel; Online-Spiel nativ nachbauen (eigene Entscheidung, falls gewünscht).

## Regeln und Einschränkungen

Große Architektur-Entscheidung (Klasse B, ändert `package.json` und ggf. Protokoll und Server): Freigabe nur durch 🧑, vorher eine Entscheidung unter `docs/decisions/` (Nachfolger von 001 und 008). Ticket wird vor der Freigabe in Teil-Tickets zerlegt.

- **Bereich:** `src/**`, `index.html`, `game.html`, `monitor.html`, `lizenzen.html`, `vite*.ts`, `package.json`, `Taskfile.yml`, `Dockerfile`, `.github/workflows/**`, `cmd/k3c-server/**`, `tools/k3c-workbench/internal/studioview/**`, `site/**`
- **Abhängig von:** B-822, NG9
- **Braucht 🧑:** Entscheidung
- **Größe:** L
- **Sperre:** deps

## Beispiele

nicht relevant bis zur Entscheidung (Umfang offen).

## Ausnahme- und Fehlerfälle

nicht relevant bis zur Entscheidung.

## Akzeptanzkriterien

- **AC-01** Entscheidung von 🧑 zu den Offenen Fragen liegt unter `docs/decisions/` vor; das Ticket ist danach in Teil-Tickets mit eigenen Kriterien zerlegt (Größe L).

## Offene Fragen

Entscheidung 🧑, je mit Vorschlag von Claude:

1. **Zielplattform:** Xbox/Edge war Zielplattform; das native Spiel läuft auf Windows (Mini-PC), nicht auf einer Xbox im Handelsmodus. Gibt 🧑 Xbox/Edge als Plattform auf? Vorschlag: ja, Mini-PC am TV ersetzt die Xbox.
2. **Online-Spiel:** Der Browser ist heute der einzige Client des Go-Servers (`/ws`). Entfällt das Spiel über mehrere Geräte? Vorschlag: ja; `k3c-server`, `engine/net`, `k3c-load`, `k3c-tui` werden in einem eigenen Ticket geprüft (bleiben für Werkzeuge oder fallen).
3. **Auslieferung von `dist/`:** Was liefert `k3c-server` ohne Browser-Client aus (nichts, nur API, oder Server entfällt)? Vorschlag: abhängig von Frage 2.
4. **GitHub Pages `site/`:** bleibt die Präsentationsseite (statisch, unabhängig vom Client) und bekommt Papier-Bilder? Vorschlag: bleibt, Bilder aus Papier-Assets in einem PLAT-Ticket.
5. **Workbench „Im Spiel ansehen“:** Umstellen auf das native Spiel (Start mit Asset und Spielstand) oder entfernen? Vorschlag: auf das native Spiel umstellen, eigenes DEV-Ticket.
6. **Zeitpunkt:** Erst nach der Abnahme des nativen Spiels am Mini-PC (NG9) oder schon vorher? Vorschlag: nach NG9.
7. **Offene Browser-Planung** (RX1, LG1.6, HW1, PF1, Projekte BED/LST mit CLI/PLAT-Tickets): gesammelt verwerfen? Vorschlag: ja, mit Verweis auf dieses Ticket, nach Frage 1.

## Notizen

Beschluss 🧑 2026-11-12: als Entwurf einplanen, nicht freigeben. Projekt NAT nennt Browser-Client und Xbox/Edge bisher als Nicht-Ziel („bleiben bis zur Umstellung“).
