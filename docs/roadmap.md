# Roadmap

Kleine, spielbare Schritte. Jeder Schritt endet mit etwas, das man am TV mit Controller ausprobieren kann.

## Schritt 0 – Xbox-Machbarkeit (als Nächstes!)

- [ ] **Gamepad-Testseite** (`public/gamepad-test.html` oder eigene Route):
  - alle Controller mit Live-Anzeige von Tasten und Achsen (Test mit 2+ Controllern),
  - welche Tasten Edge abfängt (**B** = Zurück? Xbox-Taste?),
  - Vollbild-Button (Fullscreen API),
  - FPS-Test mit einigen hundert bewegten Sprites.
- [ ] **Hosting im Heimnetz**: kleiner Node-Server, der `dist/` ausliefert (plus später Speicherstände).
  Prüfen, ob die Gamepad API auf der Xbox **HTTPS** verlangt. Falls ja: lokales Zertifikat (z. B. mkcert).
- [ ] Ergebnis in `docs/game-design.md` → Steuerung eintragen.

## Schritt 1 – Grundgerüst ✅

- [x] Vite + TypeScript + Phaser 3
- [x] Spieldaten aus dem alten GDD als JSON (`src/data/`)
- [x] Prozeduraler Level-Generator mit Seed + Tests (500 Seeds pro Biom)
- [x] Platzhalter-Rendering mit Parallax
- [x] Couch-Koop: Beitreten per A/Leertaste, 2-Spieler-Split-Screen

## Schritt 2 – Vertical Slice „Ein Tag, eine Nacht“ (Oberwelt)

- [ ] Gold: Münzen aufsammeln, Münzbeutel pro Spieler, Münzen fallen lassen/geben
- [ ] Rekrutierungs-Camp: Landstreicher → Bauer (Münze geben)
- [ ] Bauer fällt Bäume → Holz
- [ ] Bauplätze im Hub: Mauer und Turm bauen (Münzen/Holz einzahlen, Bauer baut)
- [ ] Werkstatt: Bogen → Bogenschütze
- [ ] Tag/Nacht-Zyklus (für Tests verkürzt) + Warnung „Nacht naht!“
- [ ] Portal-Welle: Greed laufen zum Hub, greifen Mauer/Truppen an, klauen Gold
- [ ] Bogenschützen schießen automatisch, Gegner droppen Gold
- [ ] Niederlage/Respawn-Regeln
- [ ] HUD pro Split-Screen-Hälfte

## Schritt 3 – Fortschritt

- [ ] Truhen + versteckte Skill-Punkte einsammeln
- [ ] Skill-Baum (erst 1–2 Linien), 2 aktive Skills
- [ ] Tiefen-Eingang → Höhle (Stufe 1) mit Aggressionspool
- [ ] Speichern/Laden (Seeds + Hub-Zustand) auf dem Heimnetz-Server

## Schritt 4 – Inhalt & Politur

- [ ] Echte Grafiken (Pixel-Art-Pack) + Animationen, Sounds
- [ ] Mine (Stufe 2), Treppen zwischen den Hubs
- [ ] Restliche Gegner und Truppen, Elite-Gegner
- [ ] Balancing-Abende mit der Familie

## Später / vielleicht

- 3–4 Spieler, alle 4 Skill-Linien, weitere Tiefen, itch.io-Release
