# Backlog

Alle Ideen, Probleme und offenen Entscheidungen. **Jederzeit ergänzen**, auch mitten in einer Session, aber nicht
nebenbei umsetzen. Bei der Sprint-Planung werden Einträge in einen Sprint gezogen ([`sprints.md`](sprints.md)).

- **Neuer Eintrag:** nächste freie Nummer, eine Zeile in die Tabelle. Braucht es mehr Text, einen Abschnitt unter
  „Details“ anlegen.
- **Typ:** Idee · Problem · Schuld (technische Schulden) · Frage (Entscheidung nötig, meist 🧑)
- **Prio:** hoch · mittel · niedrig · ? (noch nicht bewertet, wird bei der nächsten Planung bewertet)
- **Sprint:** leer = offen, `SP5` = eingeplant, `✅ SP5` = erledigt. Erledigte Einträge bleiben stehen.

| Nr. | Domäne | Typ | Prio | Titel | Sprint |
|---|---|---|---|---|---|
| B-001 | SRV | Idee | niedrig | Server auf ein richtiges Framework umstellen, falls mehr Leistung nötig wird → Go, Standardbibliothek reicht | ✅ SP0 |
| B-002 | SRV | Idee | mittel | Server mit TUI für Diagnosen im laufenden Betrieb → Bubble Tea, `cmd/k3c-tui` | SP10 |
| B-003 | SRV | Frage | mittel | Server in Python oder Go/Wails? Ziel: läuft auch nur im Docker-Container → Go, Wails optional | ✅ SP0 |
| B-004 | REG | Idee | hoch | Regelwerk ausführlich diskutieren und ausarbeiten | R1 |
| B-005 | REG | Problem | hoch | `game-design.md` sagt „Online-Koop ist kein Ziel“; laut Entscheidung 001 ist gemischter Koop jetzt Kern | R1 |
| B-006 | PLAT | Frage | hoch | Gamepad-Test auf der Xbox steht aus (B-Taste, Vollbild, HTTPS, Sprite-Budget) | X1 |
| B-007 | SIM | Idee | hoch | Skill-Baum (Tank + Zauberer) und 2 aktive Skills, erst in Go | nach SP11 |
| B-008 | REG | Frage | hoch | Spieleabend mit der Familie | nach SP11 |
| B-009 | INF | Idee | hoch | Komplexitäts-Budget automatisch prüfen (Oxlint, golangci-lint, Dateigröße, Schichtgrenzen, Ratsche) | SP1 |
| B-010 | CLI | Idee | mittel | Grafiken für Gebäude, Ressourcen, Hintergrund (bisher Platzhalter-Formen) | |
| B-011 | CLI | Idee | mittel | Sound & Musik (Kenney Audio, freesound.org) | |
| B-012 | SIM | Idee | mittel | Mine (Tiefe 2) vollständig: Kupfer, eigene Gegner | |
| B-013 | SIM | Idee | mittel | Restliche Gegner und Elite-KI (Kiting, AoE, `fleesAtHalfHp`, `swarm`) | |
| B-014 | SIM | Idee | mittel | Krieger (Schwert) und Elite-Truppen (Stein/Kupfer) | |
| B-015 | REG | Problem | mittel | Gebäude-HP und -Kosten sind Platzhalter (außer Turm, Treppen) | |
| B-016 | CLI | Frage | mittel | Mehr als 2 lokale Spieler an einem Gerät: flache Streifen, 2×2-Raster oder gemeinsame Kamera? | SP8 |
| B-017 | REG | Frage | mittel | Eine Klasse pro Spieler als Preset, damit sich die Rollen ergänzen? | nach SP11 |
| B-018 | CLI | Schuld | mittel | `worldRenderer.ts` (383 Z.) und `GameScene.ts` (371 Z.) über dem Ziel; beim Umbau zum reinen Client aufteilen | SP8 |
| B-019 | INF | Idee | niedrig | Test-Abdeckung für `engine/` messen und in der CI anzeigen | |
| B-020 | INF | Schuld | niedrig | Server-Smoke-Test steckt in der CI-YAML → entfällt, Go-Tests mit `httptest` ersetzen ihn | SP3 |
| B-021 | REG | Frage | mittel | Taste X (Interagieren) ist frei: belegen oder streichen? | R1 |
| B-022 | SIM | Idee | hoch | Monarch-Level und Skills im Spielstand speichern (Rest von S2.1), erst in Go | nach SP11 |
| B-023 | INF | Idee | niedrig | itch.io-Release (passt nur noch mit mitgeliefertem Server, siehe Entscheidung 001) | |
| B-024 | REG | Idee | niedrig | Tiefe 3 (Eisen, Lava) und Tiefe 4 (Kristall) | |
| B-025 | REG | Frage | mittel | Was ist das Ziel einer Kampagne, wann ist sie gewonnen? | R1 |
| B-026 | PLAT | Frage | hoch | Skill-Tasten am Controller; vorläufige Annahme LB/RB, bis der Xbox-Test entschieden hat | X1 |
| B-027 | SRV | Problem | hoch | Diagnose-Schnittstelle und TUI dürfen nicht offen im Netz hängen (Token, nur Heimnetz) | SP3 |
| B-028 | SRV | Idee | mittel | Spielstände sichern: rotierende Sicherungen, Docker-Volume, Wiederherstellen ohne Umbenennen | SP3, SP11 |
| B-029 | CLI | Idee | mittel | Lade-Szene mit Fortschritt, damit die Xbox beim Start nicht leer bleibt | |
| B-030 | SRV | Idee | hoch | Wiederverbinden nach Abbruch, leere Räume aufräumen, Grenzen für Räume/Geräte | SP2, SP7 |
| B-031 | INF | Idee | niedrig | Online-Modus im Smoke-Test → wird Go-Test in SP7 | SP7 |
| B-032 | PLAT | Problem | mittel | GitHub Pages hat keinen Server: nach SP9 nur noch Testseiten dort, Spiel-Kacheln ausblenden | SP9 |
| B-033 | INF | Schuld | mittel | `tests/` wird nicht typgeprüft (`tsconfig.json` › `include: ["src"]`) | SP1 |
| B-034 | PLAT | Schuld | niedrig | `tools/gamepadTest.ts` (332), `tools/spriteReference.ts` (330), `landing/landing.ts` (319) über 300 Zeilen | |
| B-035 | SRV | Idee | hoch | Betrieb auf Raspberry Pi im Docker (arm64), Autostart, Updates | SP11 |
| B-036 | SRV | Idee | hoch | Mehrere Spiele gleichzeitig (z. B. 2er auf der Xbox + 3er per Handy), im Kern von Anfang an | SP2, SP7 |
| B-037 | CLI | Idee | mittel | Lobby: laufende Räume sehen, beitreten, neues Spiel starten, Spielstand pro Raum wählen | SP8 (einfach), später mehr |
| B-038 | SRV | Idee | hoch | Mehrere lokale Spieler pro Gerät gemischt mit Online-Geräten im selben Raum | SP2, SP7, SP8 |
| B-039 | CLI | Idee | mittel | Interpolation zwischen Snapshots, evtl. Vorhersage der eigenen Laufbewegung | SP8 |
| B-040 | SRV | Idee | niedrig | Server im Heimnetz finden: Adresse/QR-Code auf der Landingpage oder in der TUI, evtl. mDNS | |
| B-041 | SRV | Idee | niedrig | Wails-Desktop-Starter für Windows (Fenster mit Status und QR-Code) um denselben Kern | |
| B-042 | SRV | Frage | hoch | Welches Pi-Modell, welches Leistungsziel (Räume × Spieler, Tick-Dauer p99)? | SP11 |
| B-043 | SIM | Problem | hoch | Portierungs-Fallen: UTF-16 in `hashSeed`, `Math.imul`, Fließkomma-Reihenfolge, Objekt-Reihenfolge | SP4–SP6 |
---

## Details

### B-001 · B-003 · Server-Framework und Sprache

Entschieden in [`decisions/001-server-engine-go.md`](decisions/001-server-engine-go.md): Go-Server als einzige
Spiel-Engine, Browser als reiner Client, mehrere Räume und mehrere lokale Spieler pro Gerät im Kern.
Python schied aus (große EXE, GIL, keine Stärke gegenüber Go), Wails nur optional als Desktop-Starter (B-041).

### B-002 · Diagnose-TUI

Im laufenden Betrieb sehen: Räume, verbundene Geräte, Tick-Dauer, Speicher, letzte Fehler, Spielstände.
Aktionen: Raum ansehen, Spieler trennen, Spielstand sichern, Log folgen. Muss auch gegen einen Server im
Docker-Container funktionieren (`docker exec -it k3c k3c-tui` oder vom PC aus über das Netz).
Umsetzung mit Bubble Tea (`cmd/k3c-tui`). Voraussetzung: `/api/status` mit Token (SP3.2, Räume ab SP7).

### B-004 · Regelwerk

Ziel: ein vollständiges, widerspruchsfreies Regelwerk als Grundlage für alle weiteren SIM-Sprints.

- Bestandsaufnahme: was der Code heute tut vs. was `game-design.md` sagt (Regelwerk I, R1.1)
- Themenblöcke als eigene Workshop-Sessions: Kern-Loop & Wirtschaft, Koop & Besitz, Stufen & Niederlage (Regelwerk I),
  Monarch & Skills (Regelwerk II), Gegner & Truppen (später), Balancing nach Spieleabenden
- Ergebnis je Thema: eine Datei in `docs/rules/` mit Regel, Begründung und Zahlen-Verweis auf `data/`.
  `game-design.md` bleibt die kurze Übersicht und verlinkt dorthin.
- Offene Fragen landen als `Frage` im Backlog und werden im Review entschieden (🧑).
