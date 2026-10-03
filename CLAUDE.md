# K3C – Family Three Crowns

Couch-Koop-Side-Scroller im Stil von Kingdom Two Crowns. **TypeScript + Phaser 4 + Vite**, läuft im Browser.
Zielplattform ist **Edge auf der Xbox** (Gamepad API), gehostet im Heimnetz. Die Kommunikation mit dem Nutzer ist deutsch.

- Design & Regeln: `docs/game-design.md` (nur bei Bedarf lesen)
- Aktueller Stand & nächste Schritte: `docs/roadmap.md`
- **Arbeitsweise:** `docs/arbeitsweise.md` (Domänen, autonomer Ablauf, Review, Komplexitäts-Budget) – vor jeder Session lesen.
- **Branches:** Entwickelt wird auf `develop`, PRs zielen auf `develop`. `main` ist geschützt, nur Releases (Fast-Forward durch den Nutzer).
- **Sprints:** `docs/sprints/aktiv/` lesen und die nächste offene Session nehmen. `docs/sprints/geplant/` nur beim
  Planen lesen, `docs/sprints/erledigt/` nur auf Nachfrage. Übersicht: `docs/sprints/README.md`.
- **Tickets:** `docs/backlog/` (Index `README.md`, ein Ticket pro Datei). Neue Ideen/Probleme sofort als Ticket anlegen,
  nicht nebenbei umsetzen. Erledigte/verworfene liegen in `docs/backlog/archiv/` (nur auf Nachfrage lesen).
- **Vorlagen sind Pflicht:** Tickets, Sprints und Sessions nur als Kopie von `docs/vorlagen/*.md`; `tests/planning.test.ts` prüft das.
- **SDD:** Ticket und Sprint-README sind die Spec (Kriterien `AC-01` …), Sessions erfüllen genannte Kriterien.
  Freigabe (`Spec: freigegeben`) und manuelle Abnahmen nur durch den Nutzer. Details: `docs/arbeitsweise.md` › SDD.
- **Spielstruktur (003):** Ein Raum hat einen Spielstand mit n Inseln, jede Insel n Stufen; Stufen sind pro Spieler frei begehbar und laufen alle weiter (Umsetzung offen, Ist-Code: eine Stufe = eine Welt, `docs/rules/stufen.md`).
- Architektur-Entscheidungen: `docs/decisions/` – **001: Die Spiel-Engine wandert auf einen Go-Server**, der Browser
  wird reiner Client. Umgesetzt (SP09): Es gibt keine TS-Simulation mehr, neue Mechaniken entstehen in `engine/` (Go).
- Altes Godot-Projekt (nur Referenz): `C:\WORKSPACE\FamilyCrowns`

## Befehle

Voraussetzungen (Node, Go, golangci-lint mit Versionen): `requirements.md`.

```bash
task               # alle Aufgaben anzeigen (Go Task, Taskfile.yml; einziger Einstieg für Befehle)
task install       # Abhängigkeiten holen
task dev           # Dev-Server (auch im LAN erreichbar, Port 5173)
task test          # Vitest (Level-Generator, reine Logik); Filter: task test -- planning
task check         # Lint + Typecheck + Tests (vor jedem Abschluss)
task check:go      # go test + golangci-lint (Rechner mit Go)
task check:dev     # k3c-dev: Frontend, go test, golangci-lint
task check:all     # alles inklusive Build
task k3c-dev       # Entwickler-Werkzeug k3c-dev als Fenster starten (wails dev; EXE: task k3c-dev:build)
task build         # Typecheck + Produktions-Build nach dist/
task pages         # GitHub-Pages-Seite nach _site/ (site/ + dist unter app/)
task serve         # Build + Go-Server (Port 8080, bin/k3c-server)
task start         # Go-Server ohne Web-Build (task dev leitet /api und /ws an ihn weiter)
```

Die Gamepad-Testseite (`gamepad-test.html`) schickt Berichte von der Xbox nach `reports/*.json`. Dort die Ergebnisse nachlesen.

**Tasks und Ausführungen laufen ausschließlich über `task`** (Neues in `Taskfile.yml`, nicht in `package.json`).
Vor jedem Abschluss: `task check` muss grün sein.
Die CI (`.github/workflows/ci.yml`) prüft zusätzlich Build + Server-Smoke-Test. `tests/projectRules.test.ts`
prüft die Regeln unten automatisch (Seiten eingetragen, `installPageChrome()`, Vollbild, kein `Math.random()`).

## Struktur

- `data/` – Balancing als JSON (Biome, Gegner, Truppen, Gebäude, Monarch), einzige Quelle für Client (Import) und
  Go-Server (`go:embed`, `data/embed.go`). Werte gehören hierher, nicht in den Code.
- `engine/` – Go: `sim/` (Simulation, deterministisch), `level/` (Level-Generator), `room/`, `net/` (HTTP, WebSocket `/ws`,
  Protokoll v2), `store/` (Spielstände → `saves/`, Berichte → `reports/`). `cmd/k3c-server` liefert `dist/` und die API aus.
- `src/model/` – Typen und Daten, die der Client vom Server kennt (`World`, `GameEvent`, `BIOMES`, `SaveGame` …), keine Logik.
- `src/online/` – Client des Go-Servers (`clientConnection.ts`, Protokoll v2): sendet nur Eingaben, zeichnet Snapshots. Ein Monarch pro Gerät.
- `src/input/` – `PlayerInput`-Abstraktion (Tastatur, Gamepad, Touch-Overlay `touchInput.ts`, per `?touch=1` erzwingbar). Spiel-Code fragt Aktionen ab, nie konkrete Tasten.
- `src/scenes/` – Phaser-Szenen (`GameScene` = Eingabe, `step()`, Kameras; `worldRenderer.ts` zeichnet den Zustand;
  `HudScene` = bildschirmfeste Anzeigen). Neue Mechanik: Logik + Test in `engine/sim/`, dann nur zeichnen.
- Seiten: `index.html` = Landingpage/Shell (Kacheln aus `src/landing/pages.ts`), `game.html` = Spiel, weitere `*.html` = Testseiten.
  Jede `*.html` im Root wird automatisch gebaut.
- `site/` – Präsentationsseite für GitHub Pages (B-183), rein statisch, ohne Server. `task pages` baut sie mit dem Build unter `app/` nach `_site/`.
- `src/core/shell.ts` – Seiten-Rahmen (Home-Button, Home-Kombi, Zurück-Falle), `src/core/fullscreen.ts` – Vollbild über die Shell.

## Regel: Seiten & Navigation

Die Landingpage bleibt **dauerhaft geöffnet** und zeigt alle anderen Seiten in einem Vollflächen-iframe.
Nur so bleibt Vollbild auf der Xbox über Seitenwechsel erhalten. Für **jede** Seite außer der Landingpage gilt:

1. Im Script **`installPageChrome()`** aus `src/core/shell.ts` aufrufen. Das liefert den sichtbaren **Home-Button**
   (oben mittig), **View + Menu** gemeinsam halten bzw. **Pos1** = zurück, und die Zurück-Falle für B.
   Oben ca. 70 px frei lassen, damit der Home-Button nichts verdeckt.
2. In `src/landing/pages.ts` eintragen. Sonst ist die Seite vom Controller aus nicht erreichbar.
3. Vollbild nur über `toggleFullscreen()` aus `src/core/fullscreen.ts`. Nie `requestFullscreen()` direkt
   oder `this.scale.toggleFullscreen()`, das würde nur das iframe betreffen.
4. Seiten nie per Link oder `location` untereinander wechseln, außer über `openPage()` aus `src/core/shell.ts` (die Shell öffnet nur `name.html` dieses Ordners). Zurück zur Übersicht immer über `goHome()`.

## Regeln

- Keine Sprünge, nur horizontale Bewegung. Welt-Koordinaten in **Units** (1 Unit = `UNIT_PX` = 32 px).
- Level-Generierung und Simulation sind deterministisch und laufen in Go: nur `engine/rng` (`rng.New(seed)`) verwenden, niemals `math/rand`; im Client gibt es keine Würfel (kein `Math.random()` für Spiel-Logik).
- Jede Mechanik muss mit **2 Spielern gleichzeitig** funktionieren (Split-Screen, eigene Eingabe pro Spieler).
- Controller-Taste **B** nicht belegen (Edge-Zurück auf der Xbox, wird von der Zurück-Falle geschluckt).
  **View + Menu** gemeinsam = zurück zur Landingpage (reserviert, auf keiner Seite anders belegen).
- Klein bleiben: kein Framework-Overhead. Prozess steht nur in `docs/arbeitsweise.md`, keine weiteren Prozess-Dokumente.
  Ein Sprint bleibt in seiner Domäne; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Lieber spielbarer Code.

## Im Browser-Pane testen

- Der Dev-Build stellt `window.game` bereit. In der Shell liegt die Seite im iframe: `document.getElementById('frame').contentWindow`.
- Ist das Pane im Hintergrund (`document.hidden`), läuft die Game-Loop nicht. Dann Frames manuell takten:
  `let t = performance.now(); for (...) { t += 16.7; game.loop.step(t); }`
- Tastatur: `KeyboardEvent`s auf `window` dispatchen (keydown/keyup mit `code` + `keyCode`).
- Controller mocken: `navigator.getGamepads = () => [pad, null, null, null]` + `gamepadconnected`-Event.
  Das Pad-Objekt braucht `mapping: 'standard'`, 17 `buttons`, 4 `axes`. **Wichtig:** Nach jeder Änderung
  `pad.timestamp = performance.now() + 100000` setzen, sonst ignoriert Phaser das Update.
