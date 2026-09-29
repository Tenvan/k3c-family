# K3C – Family Three Crowns

Couch-Koop-Side-Scroller im Stil von Kingdom Two Crowns. **TypeScript + Phaser 4 + Vite**, läuft im Browser.
Zielplattform ist **Edge auf der Xbox** (Gamepad API), gehostet im Heimnetz. Die Kommunikation mit dem Nutzer ist deutsch.

- Design & Regeln: `docs/game-design.md` (nur bei Bedarf lesen)
- Aktueller Stand & nächste Schritte: `docs/roadmap.md`
- Arbeitspakete pro Session: `docs/sessions.md` (zu Beginn die nächste offene Session nehmen, am Ende abhaken)
- Altes Godot-Projekt (nur Referenz): `C:\WORKSPACE\FamilyCrowns`

## Befehle

```bash
npm run dev        # Dev-Server (auch im LAN erreichbar, Port 5173)
npm test           # Vitest (Level-Generator, reine Logik)
npm run build      # Typecheck + Produktions-Build nach dist/
npm run serve      # Build + Heimnetz-Server (Port 8080, server/server.mjs)
```

Die Gamepad-Testseite (`gamepad-test.html`) schickt Berichte von der Xbox nach `reports/*.json`. Dort die Ergebnisse nachlesen.

Vor jedem Abschluss: `npm test` und `npm run typecheck` müssen grün sein.
Die CI (`.github/workflows/ci.yml`) prüft zusätzlich Build + Server-Smoke-Test. `tests/projectRules.test.ts`
prüft die Regeln unten automatisch (Seiten eingetragen, `installPageChrome()`, Vollbild, kein `Math.random()`).

## Struktur

- `src/data/` – Balancing als JSON (Biome, Gegner, Truppen, Gebäude, Monarch). Werte gehören hierher, nicht in den Code.
- `src/world/` – Spiel-Logik **ohne Phaser-Import**, Tests daneben (`*.test.ts`). `levelGenerator.ts` baut das Level,
  `sim/` simuliert es (`createWorld()` + `step()`, deterministisch, Zustand in `sim/types.ts`).
- `src/input/` – `PlayerInput`-Abstraktion (Tastatur, Gamepad, Touch-Overlay `touchInput.ts`, per `?touch=1` erzwingbar). Spiel-Code fragt Aktionen ab, nie konkrete Tasten.
- `src/scenes/` – Phaser-Szenen (`GameScene` = Eingabe, `step()`, Kameras; `worldRenderer.ts` zeichnet den Zustand;
  `HudScene` = bildschirmfeste Anzeigen). Neue Mechanik: Logik + Test in `sim/`, dann nur zeichnen.
- Seiten: `index.html` = Landingpage/Shell (Kacheln aus `src/landing/pages.ts`), `game.html` = Spiel, weitere `*.html` = Testseiten.
  Jede `*.html` im Root wird automatisch gebaut.
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
4. Seiten nie per Link oder `location` untereinander wechseln. Zurück zur Übersicht immer über `goHome()`.

## Regeln

- Keine Sprünge, nur horizontale Bewegung. Welt-Koordinaten in **Units** (1 Unit = `UNIT_PX` = 32 px).
- Level-Generierung ist deterministisch: nur `createRng(seed)` verwenden, niemals `Math.random()`.
- Jede Mechanik muss mit **2 Spielern gleichzeitig** funktionieren (Split-Screen, eigene Eingabe pro Spieler).
- Controller-Taste **B** nicht belegen (Edge-Zurück auf der Xbox, wird von der Zurück-Falle geschluckt).
  **View + Menu** gemeinsam = zurück zur Landingpage (reserviert, auf keiner Seite anders belegen).
- Klein bleiben: kein Framework-Overhead, keine Prozess-Dokumente. Lieber spielbarer Code.

## Im Browser-Pane testen

- Der Dev-Build stellt `window.game` bereit. In der Shell liegt die Seite im iframe: `document.getElementById('frame').contentWindow`.
- Ist das Pane im Hintergrund (`document.hidden`), läuft die Game-Loop nicht. Dann Frames manuell takten:
  `let t = performance.now(); for (...) { t += 16.7; game.loop.step(t); }`
- Tastatur: `KeyboardEvent`s auf `window` dispatchen (keydown/keyup mit `code` + `keyCode`).
- Controller mocken: `navigator.getGamepads = () => [pad, null, null, null]` + `gamepadconnected`-Event.
  Das Pad-Objekt braucht `mapping: 'standard'`, 17 `buttons`, 4 `axes`. **Wichtig:** Nach jeder Änderung
  `pad.timestamp = performance.now() + 100000` setzen, sonst ignoriert Phaser das Update.
