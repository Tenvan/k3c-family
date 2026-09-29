# K3C – Family Three Crowns

Couch-Koop-Side-Scroller im Stil von Kingdom Two Crowns. **TypeScript + Phaser 3 + Vite**, läuft im Browser.
Zielplattform ist **Edge auf der Xbox** (Gamepad API), gehostet im Heimnetz. Die Kommunikation mit dem Nutzer ist deutsch.

- Design & Regeln: `docs/game-design.md` (nur bei Bedarf lesen)
- Aktueller Stand & nächste Schritte: `docs/roadmap.md`
- Altes Godot-Projekt (nur Referenz): `C:\WORKSPACE\FamilyCrowns`

## Befehle

```bash
npm run dev        # Dev-Server (auch im LAN erreichbar, Port 5173)
npm test           # Vitest (Level-Generator, reine Logik)
npm run build      # Typecheck + Produktions-Build nach dist/
```

Vor jedem Abschluss: `npm test` und `npm run typecheck` müssen grün sein.

## Struktur

- `src/data/` – Balancing als JSON (Biome, Gegner, Truppen, Gebäude, Monarch). Werte gehören hierher, nicht in den Code.
- `src/world/` – Spiel-Logik. Reine Logik (z.B. `levelGenerator.ts`) bleibt **ohne Phaser-Import** und bekommt Tests daneben (`*.test.ts`).
- `src/input/` – `PlayerInput`-Abstraktion. Spiel-Code fragt Aktionen ab, nie konkrete Tasten.
- `src/scenes/` – Phaser-Szenen (`GameScene` = Welt + Kameras, `HudScene` = bildschirmfeste Anzeigen).

## Regeln

- Keine Sprünge, nur horizontale Bewegung. Welt-Koordinaten in **Units** (1 Unit = `UNIT_PX` = 32 px).
- Level-Generierung ist deterministisch: nur `createRng(seed)` verwenden, niemals `Math.random()`.
- Jede Mechanik muss mit **2 Spielern gleichzeitig** funktionieren (Split-Screen, eigene Eingabe pro Spieler).
- Controller-Taste **B** nicht belegen (Edge-Zurück auf der Xbox).
- Klein bleiben: kein Framework-Overhead, keine Prozess-Dokumente. Lieber spielbarer Code.

## Im Browser-Pane testen

- Der Dev-Build stellt `window.game` bereit.
- Ist das Pane im Hintergrund (`document.hidden`), läuft die Game-Loop nicht. Dann Frames manuell takten:
  `let t = performance.now(); for (...) { t += 16.7; game.loop.step(t); }`
- Tastatur: `KeyboardEvent`s auf `window` dispatchen (keydown/keyup mit `code` + `keyCode`).
- Controller mocken: `navigator.getGamepads = () => [pad, null, null, null]` + `gamepadconnected`-Event.
  Das Pad-Objekt braucht `mapping: 'standard'`, 17 `buttons`, 4 `axes`. **Wichtig:** Nach jeder Änderung
  `pad.timestamp = performance.now() + 100000` setzen, sonst ignoriert Phaser das Update.
