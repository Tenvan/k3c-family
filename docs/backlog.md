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
| B-001 | SRV | Idee | niedrig | Server auf ein richtiges Framework umstellen, falls mehr Leistung nötig wird | SP8 |
| B-002 | SRV | Idee | mittel | Server mit TUI für Diagnosen im laufenden Betrieb | SP8, SP10 |
| B-003 | SRV | Frage | mittel | Server in Python oder Go/Wails? Ziel: läuft auch nur im Docker-Container | SP8 |
| B-004 | REG | Idee | hoch | Regelwerk ausführlich diskutieren und ausarbeiten | SP3, SP4 |
| B-005 | REG | Problem | hoch | `game-design.md` sagt „Online-Koop ist kein Ziel“, der Online-Modus existiert aber | SP3 |
| B-006 | PLAT | Frage | hoch | Gamepad-Test auf der Xbox steht aus (B-Taste, Vollbild, HTTPS, Sprite-Budget) | SP2 |
| B-007 | SIM | Idee | hoch | Skill-Baum (Tank + Zauberer) und 2 aktive Skills | SP5 |
| B-008 | REG | Frage | hoch | Vertical-Slice-Spieleabend mit der Familie | SP7 |
| B-009 | INF | Idee | hoch | Komplexitäts-Budget automatisch prüfen (ESLint, Dateigröße, Schichtgrenzen, Ratsche für Bestandscode) | SP1 |
| B-010 | CLI | Idee | mittel | Grafiken für Gebäude, Ressourcen, Hintergrund (bisher Platzhalter-Formen) | |
| B-011 | CLI | Idee | mittel | Sound & Musik (Kenney Audio, freesound.org) | |
| B-012 | SIM | Idee | mittel | Mine (Tiefe 2) vollständig: Kupfer, eigene Gegner | |
| B-013 | SIM | Idee | mittel | Restliche Gegner und Elite-KI (Kiting, AoE, `fleesAtHalfHp`, `swarm`) | |
| B-014 | SIM | Idee | mittel | Krieger (Schwert) und Elite-Truppen (Stein/Kupfer) | |
| B-015 | REG | Problem | mittel | Gebäude-HP und -Kosten sind Platzhalter (außer Turm, Treppen) | |
| B-016 | REG | Frage | niedrig | 3–4 Spieler: flache Streifen, 2×2-Raster oder gemeinsame Kamera? | |
| B-017 | REG | Frage | mittel | Eine Klasse pro Spieler als Preset, damit sich die Rollen ergänzen? | SP4 |
| B-018 | CLI | Schuld | mittel | `worldRenderer.ts` (383 Z.) und `GameScene.ts` (371 Z.) liegen über dem Ziel von 300 Zeilen | SP6 |
| B-019 | INF | Idee | niedrig | Test-Abdeckung für `src/world/` messen und in der CI anzeigen | |
| B-020 | INF | Schuld | niedrig | Server-Smoke-Test steckt in der CI-YAML, lokal nicht ausführbar | SP1 |
| B-021 | REG | Frage | mittel | Taste X (Interagieren) ist frei: belegen oder streichen? | SP3 |
| B-022 | SIM | Idee | hoch | Monarch-Level und Skills im Spielstand speichern (Rest von S2.1) | SP5 |
| B-023 | INF | Idee | niedrig | itch.io-Release | |
| B-024 | REG | Idee | niedrig | Tiefe 3 (Eisen, Lava) und Tiefe 4 (Kristall) | |
| B-025 | REG | Frage | mittel | Was ist das Ziel einer Kampagne, wann ist sie gewonnen? | SP3 |
| B-026 | PLAT | Frage | hoch | Skill-Tasten am Controller; vorläufige Annahme LB/RB, bis SP2 entschieden hat | SP2 |
| B-027 | SRV | Problem | hoch | Diagnose-Schnittstelle und TUI dürfen nicht offen im Netz hängen (Token, nur Heimnetz) | SP9 |
| B-028 | SRV | Idee | mittel | Spielstände sichern: rotierende Sicherungen, Docker-Volume, Wiederherstellen ohne Umbenennen | SP9 |
| B-029 | CLI | Idee | mittel | Lade-Szene mit Fortschritt, damit die Xbox beim Start nicht leer bleibt | |
| B-030 | SRV | Idee | mittel | Online robuster: Wiederverbinden nach Abbruch, leere Räume aufräumen, Grenzen für Räume/Geräte | |
| B-031 | INF | Idee | niedrig | Online-Modus im Smoke-Test (WebSocket verbinden, Raum beitreten, Snapshot empfangen) | |
| B-032 | PLAT | Problem | niedrig | GitHub Pages hat keinen Server: Online-Modus und Speichern dort ausblenden oder erklären | |

---

## Details

### B-001 · B-003 · Server-Framework und Sprache

Heute: `server/server.mjs` ohne Abhängigkeiten (Dateien, Berichte, Spielstände) plus WebSocket-Online-Modus
(`src/online/`), der die **TypeScript-Simulation** `step()` auf dem Server rechnet.

Das ist der entscheidende Punkt für die Sprachwahl: **Die Simulation ist TypeScript und wird von Client und Server
gemeinsam genutzt.** Ein Server in Python oder Go müsste sie

1. **nachbauen** – doppelte Regeln, die auseinanderlaufen. Nicht empfohlen.
2. in einer **eingebetteten JS-Engine** ausführen (Go: `goja`, reines Go, deutlich langsamer als Node;
   Python: V8-Anbindung wie `mini-racer`) – geht, aber eine zusätzliche Schicht, schwerer zu debuggen, oder
3. als **Node-Beiprozess** betreiben – dann gibt es zwei Laufzeiten im Container.

Docker ist mit **jeder** Variante möglich. Node bringt ein Image von grob 50–60 MB (`node:<lts>-alpine`),
Go ein sehr kleines (`scratch`/`distroless`) – der Größenvorteil fällt aber weg, sobald Node als Beiprozess mitkommt.

Zu **Wails**: Wails baut Desktop-Programme mit Web-Oberfläche (Fenster + WebView). Für einen Server, der nur im
Docker-Container läuft, passt das nicht, dort gibt es kein Fenster. Go allein (ohne Wails) passt dagegen gut zu Docker.

**Vorschlag für SP8:** Node/TypeScript für den Spiel-Server behalten (eine Sprache, gemeinsamer Code), bei Bedarf
ein schlankes Framework (Fastify oder Hono) und ein kleines Docker-Image (`node:alpine`). Die TUI als **eigenes
Programm**, das nur mit einer Diagnose-Schnittstelle des Servers spricht. Dann ist die TUI-Sprache frei wählbar
(z. B. Go + Bubble Tea als einzelne Binärdatei) und unabhängig vom Server. Entschieden wird erst nach der Messung
in SP8.1. Wenn der Server heute reicht, bleibt er, wie er ist (B-001 niedrige Priorität).

### B-002 · Diagnose-TUI

Im laufenden Betrieb sehen: Räume, verbundene Geräte, Tick-Dauer, Speicher, letzte Fehler, Spielstände.
Aktionen: Raum ansehen, Spieler trennen, Spielstand sichern, Log folgen. Muss auch gegen einen Server im
Docker-Container funktionieren (`docker exec -it k3c tui` oder vom PC aus über das Netz).
Voraussetzung: Diagnose-Schnittstelle (SP9.2), nur im Heimnetz bzw. mit Token erreichbar.

### B-004 · Regelwerk

Ziel: ein vollständiges, widerspruchsfreies Regelwerk als Grundlage für alle weiteren SIM-Sprints.

- Bestandsaufnahme: was der Code heute tut vs. was `game-design.md` sagt (SP3.1)
- Themenblöcke als eigene Workshop-Sessions: Kern-Loop & Wirtschaft, Koop & Besitz, Stufen & Niederlage (SP3),
  Monarch & Skills (SP4), Gegner & Truppen (später), Balancing nach Spieleabenden
- Ergebnis je Thema: eine Datei in `docs/rules/` mit Regel, Begründung und Zahlen-Verweis auf `src/data/`.
  `game-design.md` bleibt die kurze Übersicht und verlinkt dorthin.
- Offene Fragen landen als `Frage` im Backlog und werden im Review entschieden (🧑).
