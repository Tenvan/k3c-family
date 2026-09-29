# Roadmap

Kleine, spielbare Schritte. Jeder Schritt endet mit etwas, das man am TV mit Controller ausprobieren kann.
Aufteilung in Arbeitspakete pro Claude-Session: [`docs/sessions.md`](sessions.md).

## Schritt 0 – Xbox-Machbarkeit (als Nächstes!)

- [x] **Gamepad-Testseite** `gamepad-test.html`: Live-Anzeige aller Controller, Falle für Zurück-Navigation (B),
  Vollbild, Vibration, FPS-Test (Phaser/WebGL, 100–4000 Sprites), Bericht per **Y** an den Server.
- [x] **Heimnetz-Server** `server/server.mjs` (ohne Abhängigkeiten): liefert `dist/` aus, speichert Berichte in `reports/`,
  optional HTTPS mit `certs/`. Start: `npm run serve`.
- [ ] **Test auf der Xbox durchführen** (Anleitung im README) → Bericht in `reports/` auswerten.
- [ ] Ergebnis in `docs/game-design.md` → Steuerung eintragen (Skill-Tasten, B, Vollbild, max. Sprites).

## Schritt 1 – Grundgerüst ✅

- [x] Vite + TypeScript + Phaser 4
- [x] Spieldaten aus dem alten GDD als JSON (`src/data/`)
- [x] Prozeduraler Level-Generator mit Seed + Tests (500 Seeds pro Biom)
- [x] Platzhalter-Rendering mit Parallax
- [x] Couch-Koop: Beitreten per A/Leertaste, 2-Spieler-Split-Screen

## Schritt 2 – Vertical Slice „Ein Tag, eine Nacht“ (Oberwelt) – umgesetzt, Spieleabend steht aus

- [x] Gold: Münzen aufsammeln, Münzbeutel pro Spieler, Münzen fallen lassen/geben
- [x] Rekrutierungs-Camp: Landstreicher → Bauer (Münze geben)
- [x] Bauer fällt Bäume → Holz
- [x] Bauplätze im Hub: Mauer und Turm bauen (Münzen/Holz einzahlen, Bauer baut)
- [x] Werkstatt: Bogen → Bogenschütze
- [x] Tag/Nacht-Zyklus (für Tests verkürzt) + Warnung „Nacht naht!“
- [x] Portal-Welle: Greed laufen zum Hub, greifen Mauer/Truppen an, klauen Gold
- [x] Bogenschützen schießen automatisch, Gegner droppen Gold
- [x] Niederlage/Respawn-Regeln
- [x] HUD pro Split-Screen-Hälfte

## Schritt 3 – Fortschritt

- [x] Truhen + versteckte Skill-Punkte einsammeln
- [ ] Skill-Baum (erst 1–2 Linien), 2 aktive Skills
- [x] Tiefen-Eingang → Höhle (Stufe 1) mit Aggressionspool, Treppen zwischen den Hubs
- [x] Speichern/Laden (Seeds + Hub-Zustand) auf dem Heimnetz-Server

## Schritt 4 – Inhalt & Politur

- [ ] Echte Grafiken (Pixel-Art-Pack) + Animationen, Sounds
- [ ] Mine (Stufe 2), Treppen zwischen den Hubs
- [ ] Restliche Gegner und Truppen, Elite-Gegner
- [ ] Balancing-Abende mit der Familie

## Später / vielleicht

- 3–4 Spieler, alle 4 Skill-Linien, weitere Tiefen, itch.io-Release
