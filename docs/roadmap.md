# Roadmap

Kleine, spielbare Schritte. Jeder Schritt endet mit etwas, das man am TV mit Controller ausprobieren kann.
Umsetzung in Sprints und Sessions: [`docs/sprints/`](sprints/README.md). Tickets: [`docs/backlog/`](backlog/README.md). Arbeitsweise: [`docs/arbeitsweise.md`](arbeitsweise.md).

## Schritt 0 – Xbox-Machbarkeit (einschiebbar, Sprint X1) ✅

- [x] **Gamepad-Testseite** `gamepad-test.html`: Live-Anzeige aller Controller, Falle für Zurück-Navigation (B),
  Vollbild, Vibration, FPS-Test (Phaser/WebGL, 100–4000 Sprites), Bericht per **Y** an den Server.
- [x] **Heimnetz-Server** (früher ein Node-Server, seit SP03/SP09 der Go-Server `cmd/k3c-server`): liefert `dist/` aus, speichert Berichte
  in `reports/`, optional HTTPS mit `certs/`. Start: `task serve`.
- [x] **Test auf der Xbox durchführen** (Anleitung im README) → Bericht in `reports/` auswerten.
- [x] Ergebnis in `docs/game-design.md` → Steuerung eintragen (Skill-Tasten, B, Vollbild, max. Sprites).

## Schritt 1 – Grundgerüst ✅

- [x] Vite + TypeScript + Phaser 4
- [x] Spieldaten aus dem alten GDD als JSON (`data/`)
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

## Schritt 3b – Go-Engine (Entscheidung 001)

- [x] Protokoll v2: mehrere Räume, mehrere lokale Spieler pro Gerät, Couch + Online gemischt
- [x] Go-Server ersetzt `server/*.mjs` (Windows-EXE, Docker amd64/arm64)
- [x] Simulation nach Go portiert (Golden-Tests gegen die TS-Simulation)
- [x] Browser als reiner Client, TS-Simulation gelöscht
- [x] Diagnose-TUI (SP10); [x] Betrieb auf dem Raspberry Pi (SP11); [ ] Lastmessung (LT1)
- [x] Entwickler-Werkzeug `k3c-dev` mit MCP-Server für Agenten (M1: Prüfungen, Logs; M2: Statistik, Berichte, Spielstände;
  M3: Dienste; M4: Oberfläche mit Dienste- und Logs-Seite; M5: MCP-Seite; SP07: Räume, Simulation)

## Schritt 3c – Regelwerk I und Spielstruktur (R1, beschlossen am 2026-10-02)

- [x] Regelwerk I: `docs/rules/wirtschaft.md` und `stufen.md`, `game-design.md` ohne Widerspruch zu Entscheidung 001
- [x] Entscheidung 003: Spielstand → Inseln → Stufen, Stufen pro Spieler frei begehbar
- [x] SIM-Kern der Insel (B-100, SP12): mehrere Stufen ticken, Einzelwechsel, Vorrat je Insel, Spielstand Version 2
- [x] SIM-Teil von Raum-Optionen, Graden, Material und Lager (B-101, B-113, SP13)
- [x] Raum auf Insel und Protokoll Version 3 mit Raum-Optionen (B-133, B-104, SP14)
- [ ] Umsetzung: Raum-Optionen und Grade (B-101), Siegvarianten und Niederlage (B-102), Inseln und Bosse (B-103), Protokoll (B-104), Dialog (B-105), Kamera je Stufe (B-106), Debug-Panel (B-107)
- [x] Regelwerk Materialien und Gebäude (R2): fünf Materialien, Hub-Ausbau 1–5, Lager, Plantage und Adern
- [ ] Umsetzung R2: Hub-Ausbau (B-112), Material und Lager (B-113), Plantage und Adern (B-114), Stufen-Breite und neue Stufen (B-115), Gebäude-Wirkungen (B-116), Anzeige (B-117)
- [x] Regelwerk II (R3): Monarch (kein Level, Fund-Pool, Schlag, Skills Tank/Zauberer/Heiler, Wiederbeleben), Bürger (Berufe, Handwerker, Händler, Limit je Hub)
- [ ] Umsetzung R3: Monarch und Pool (B-118), Skills (B-119), Wiederbeleben (B-120), Berufe und Händler (B-121), Elite/Limit/Heilung (B-122), Protokoll (B-123), Skill-Menü (B-124), Aktionen-Overlay (B-125), Bürger-UI (B-126)
- [x] Regelwerk III (R4): Gegner je Stufe, Traits, Wellen, Bosse (Miniboss je Stufe, Endboss je Insel), Events (Vollmond, Blutmond, Händler-Überfall)
- [ ] Umsetzung R4: Traits und Kiting (B-128), neue Gegner (B-129), Bosse (B-130), Events (B-131), Anzeige (B-132)
- [ ] Automatischer Balancing-Tester (B-099); Regelwerk II (Skillung, Klassen, Level von Monarchen und Bürgern, B-110) und III (B-004); Material je Insel und Inselfolge entschieden (B-108)

## Schritt 5 – Weiterentwicklung (Plan vom 2026-10-02)

Gesamtplan: [`plan-weiterentwicklung.md`](plan-weiterentwicklung.md). Leitlinie: früh spielbar, dann Tiefe; Assets CC0 + CC-BY mit Credits.
Offene Entscheidungen für die nächste Session: [`fragenkatalog.md`](fragenkatalog.md). Tickets B-134 bis B-170, Sprints im [Fahrplan](sprints/README.md).

- [ ] **Phase 0 Fundament:** F1 Zielkorridore und Bedienungsregeln (🧑, erledigt), F2 Golden/Migration/Determinismus (erledigt), F3 Feedback-Events (Sim, erledigt), F4 Protokoll + Pi-Betrieb, F5 Doku/Version; parallel SP11 (Pi); vorgezogen DBG1 Dev-Aktionen im Raum (erledigt)
- [ ] **Phase 1 Spieleabend-Build:** S1 Schlag/Skills (erledigt), S2 Protokoll/Speichern/Metrik (erledigt), S3 Skill-Menü, S4 Kamera, S5 Optionen/Pause, S6 Onboarding, S7 Monarch auf dem Standard-Reittier, SO1 Audio-Kern, P1 Spieleabend 1
- [ ] **Phase 2 Tiefe:** W1–W6 Hub-Ausbau, Plantage/Adern, Gebäude, Bürger, Protokoll, Anzeige; BR1 Balancing + Spieleabend 2
- [ ] **Phase 3 Kampf:** K1–K5 Gegner, Bosse/Inseln, Events, Protokoll, Anzeige; BR2 Balancing + Spieleabend 3
- [ ] **Schienen (einschiebbar):** Balancing BAL1–4, Grafik GR1–6, Sound SO1–4, Release RL1 (erledigt, Checkliste in `arbeitsweise.md` › „Release“)

## Schritt 4 – Inhalt & Politur

- [x] Figuren-Sprites mit Animationen (LuizMelo für unsere Seite, Gothicvania für Gegner, CC0)
- [ ] Reittiere für die Monarchen (Grafiken und Sattelpunkte liegen in `sprites.json` → `mounts`, Mechanik fehlt noch)
- [ ] Grafiken für Gebäude, Ressourcen, Hintergrund; Sounds
- [ ] Mine (Stufe 2), Treppen zwischen den Hubs
- [ ] Restliche Gegner und Truppen, Elite-Gegner
- [ ] Balancing-Abende mit der Familie

## Später / vielleicht

- 3–4 Spieler, alle 4 Skill-Linien, weitere Tiefen, itch.io-Release
