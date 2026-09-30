# Sprints

So arbeiten wir: **Backlog** ([`backlog.md`](backlog.md)) → **Sprint** (eine Domäne, 2–4 Sessions) → **Session** (ein PR).
Ein Sprint ist erst fertig, wenn seine **Review-Session** alle im Sprint geänderten Dateien abgenommen hat.
Die Meilensteine stehen in [`roadmap.md`](roadmap.md), das *Warum* der Regeln in [`game-design.md`](game-design.md).

Legende: ⬜ offen · 🟨 in Arbeit · ✅ fertig · ⏸ blockiert · 🧑 braucht den Menschen (Xbox, Entscheidung, Spieleabend) · 🔍 Review

---

## Arbeitsweise

### Domänen

Jeder Sprint gehört zu **genau einer Domäne** und ändert nur deren Dateien (plus Tests und Doku dazu).
Was eine andere Domäne braucht, kommt als Eintrag ins Backlog.

| Kürzel | Domäne | Dateien |
|---|---|---|
| **REG** | Regelwerk & Balancing | `docs/game-design.md`, `docs/rules/`, `docs/playtests/`, **Werte** in `data/*.json` |
| **SIM** | Spiel-Logik (Go) | `engine/sim/`, `engine/level/`, **neue Felder** in `data/*.json`; bis zur Löschung `src/world/` (nur Fehler) |
| **SRV** | Server & Betrieb (Go) | `engine/room/`, `engine/net/`, `engine/store/`, `cmd/`, Docker; bis zur Löschung `server/`, `src/online/room.ts`, `src/online/wsServer.ts` |
| **CLI** | Client: Darstellung, HUD, Grafik, Audio, Verbindung | `src/scenes/`, `public/`, `src/online/client.ts`, `src/core/saveStore.ts` |
| **PLAT** | Plattform: Eingabe, Shell, Seiten | `src/input/`, `src/core/shell.ts`, `src/core/fullscreen.ts`, `src/landing/`, `src/tools/`, `*.html` |
| **INF** | Frameworks, Tooling, CI, Repo-Aufbau | `package.json`, `go.mod`, `vite*.ts`, `tsconfig.json`, Lint-Konfiguration, `.github/`, `tests/projectRules.test.ts` |

Architektur und Zielbild: [`decisions/001-server-engine-go.md`](decisions/001-server-engine-go.md).
**Feature-Stopp:** In `src/world/` nur noch Fehlerbehebungen, neue Mechaniken entstehen in Go.

Grenzfälle, damit niemand raten muss:

- **Daten:** SIM legt neue Felder mit vorläufigen Werten an (aus dem REG-Beschluss). REG ändert danach nur Werte.
- **Protokoll** (`docs/protocol.md`, `engine/net/protocol*.go`, `src/online/protocol.ts`, gemeinsame Beispiel-Nachrichten
  in `testdata/protocol/`) betrifft Client und Server. Eine Änderung daran bekommt eine eigene Session, die nur
  das Protokoll und beide Enden anpasst.
- **Portierung:** Ein SIM-Port-Sprint darf `src/world/` lesen und Golden-Daten daraus erzeugen, ändert es aber nicht.
- **Status-Pflege** in `docs/sprints.md`, `docs/backlog.md`, `docs/roadmap.md` ist in jeder Session erlaubt.
- **Feature-Kette:** Ein neues Spielelement läuft als REG → SIM → CLI in **direkt aufeinanderfolgenden** Sprints.
  Der SIM-Sprint beweist sich durch Tests, am TV sichtbar wird es am Ende des CLI-Sprints.

### Sprint

- **Klein:** 2–4 Sessions, die letzte ist immer das **Review**. Mehr Arbeit → zweiter Sprint.
- **Ein Sprint zur Zeit.** Was unterwegs auffällt, kommt ins Backlog, nicht in den laufenden Sprint.
- **Planung** (am Ende des vorigen Reviews, ~15 min):
  1. Backlog sichten: neue Einträge bewerten (Prio statt `?`), passende Einträge in den Sprint ziehen.
  2. Sessions formulieren, jede mit **Ziel** und **Fertig, wenn** (prüfbar: Test, Befehl, Beobachtung am TV).
  3. Tag setzen und pushen: `git tag sp<N>-start main && git push origin sp<N>-start`.
  Nur der **nächste** Sprint wird so detailliert, spätere bleiben grob (Stichpunkte).
- **Blockade** (🧑 fehlt, z. B. Xbox-Test): Sprint auf ⏸ setzen und den nächsten **unabhängigen** Sprint vorziehen.
  Wer auf das Ergebnis wartet, arbeitet mit einer vorläufigen Annahme, die im Backlog als `Frage` steht.

### Session

Eine Session ist **bereit**, wenn Ziel, „Fertig, wenn“ und Domäne feststehen und ihre Abhängigkeiten erledigt sind.

1. **Start:** diese Datei lesen, nächste offene Session des aktuellen Sprints nehmen, Branch `sp<N>/<kurzname>`.
2. **Umsetzen:** Werte nach `src/data/*.json`, Logik nach `src/world/` mit `*.test.ts`, Darstellung in `src/scenes/`.
   Alles mit **2 Spielern** testen (Tastatur + Controller-Mock, siehe `CLAUDE.md`).
3. **Abschluss:** `npm test` + `npm run typecheck` grün (ab SP1: `npm run check`). Session abhaken und 1–3 Zeilen
   **Ergebnis** notieren (was anders lief, was offen bleibt). Neue Ideen/Probleme → Backlog.
   PR mit Vorlage öffnen, CI grün, per **Squash** nach `main` mergen.
4. **Am TV:** Liefert die Session etwas Sichtbares, eine Zeile „Am TV prüfen“. Ergebnis später nachtragen.

Richtwert: **eine Session = ein PR mit ≤ ~400 geänderten Code-Zeilen** (ohne Bilder, Daten-JSON, Lockfile).
Commit-Titel mit Domäne, z. B. `feat(sim): Taunt`, `fix(srv): Raum aufräumen`, `docs(reg): Wirtschaft v1`.

### Review-Session (Sprint-Abnahme) 🔍

In einer **frischen Session**, ohne den Kontext der Umsetzung:

1. `git fetch --tags && git diff --stat sp<N>-start..origin/main` → **alle** im Sprint erstellten oder geänderten Dateien.
2. Jede Datei **vollständig** lesen (nicht nur den Diff) und gegen die Checkliste prüfen. `/code-review` und
   `/simplify` helfen, ersetzen aber das Lesen nicht.
3. Befunde **in der Domäne** sofort im Review-PR beheben. Befunde **außerhalb** der Domäne → Backlog,
   außer Kleinstes (Tippfehler, Link, ≤ 5 Zeilen).
4. Abnahme hier eintragen: Datum, Anzahl geprüfter Dateien, behobene Befunde, neue Backlog-Nummern.
5. Review-PR mergen, dann `git tag sp<N>-done main && git push origin sp<N>-done`. Danach den nächsten Sprint planen.

**Checkliste**

- [ ] Datei gehört zur Domäne des Sprints (oder ist ein erlaubter Grenzfall)
- [ ] Komplexitäts-Budget eingehalten, nichts auf Vorrat gebaut, kein toter Code
- [ ] Werte stehen in `src/data/`, nicht im Code
- [ ] Logik in `src/world/` ist getestet, deterministisch, ohne Phaser
- [ ] Funktioniert mit 2 Spielern (lokal, und online, falls betroffen)
- [ ] Regeln aus `CLAUDE.md` eingehalten (Seiten, Vollbild, B-Taste, kein `Math.random()`)
- [ ] Doku passt zum Code (`game-design.md`, README, Kommentare)

### Komplexitäts-Budget

Niedrige Komplexität ist in **jeder** Session Pflicht, nicht erst im Review.

| Regel | Ziel | Harte Grenze | Gilt für |
|---|---|---|---|
| Zeilen pro Datei | 300 | 400 | Code; Tests nur harte Grenze |
| Zeilen pro Funktion | 40 | 60 | Code, nicht Tests |
| Verschachtelung | 3 | 4 | alles |
| Zyklomatische Komplexität | 10 | 15 | Code |

- **Ratsche für Bestandscode:** Dateien, die beim Einführen (SP1) schon über dem Ziel liegen, stehen mit ihrem
  heutigen Wert in einer Ausnahmeliste. Der Wert darf nur sinken. Wer eine solche Datei ändert, verkleinert sie
  oder legt einen Backlog-Eintrag an.
- Keine neue Abhängigkeit ohne Backlog-Eintrag und Zustimmung im Review.
- Keine Abstraktion für nur einen Fall. Erst beim dritten gleichen Fall verallgemeinern.
- Schichtgrenzen: `engine/sim` und `engine/level` importieren nichts aus `engine/room`, `engine/net`, `cmd/`;
  `engine/` nichts aus `cmd/`. Im Client: `src/scenes` rechnet nichts, es zeichnet Snapshots.
  Bis zur Löschung: `src/world` importiert nichts aus `scenes/`, `online/`, `input/`.
- Budget gilt für TypeScript (Oxlint) und Go (`golangci-lint`: `funlen`, `gocyclo`, `nestif`, `lll` nicht nötig).

### Entscheidungen

Größere Entscheidungen (Framework, Server-Sprache, Grundsätze des Regelwerks) als kurze Notiz in
`docs/decisions/NNN-titel.md`: **Kontext · Optionen · Entscheidung · Folgen**, höchstens eine Seite.
Der Ordner entsteht mit der ersten Entscheidung.

### Versionen

Nach jeder abgeschlossenen Feature-Kette oder jedem Spieleabend ein Release-Tag `v0.<n>.0`
(`release.yml` baut das Zip). So gibt es immer einen bekannten, spielbaren Stand zum Zurückgehen.

---

## Aktueller Sprint

### SP0 · INF · Arbeitsweise einführen 🟨

- **0.1 ✅ Plan, Backlog, PR-Vorlage.** `docs/sprints.md` (ersetzt `sessions.md`), `docs/backlog.md`,
  `.github/pull_request_template.md`, `CLAUDE.md`, `roadmap.md`, `README.md` angepasst. Gemergt mit PR #16.
- **0.2 ✅ 🧑 Architektur-Entscheidung.** Go-Server als einzige Engine, mehrere Räume, mehrere lokale Spieler pro Gerät,
  Heimnetz auf PC und später Raspberry Pi: [`decisions/001-server-engine-go.md`](decisions/001-server-engine-go.md).
  Sprint-Plan und Backlog darauf umgebaut.
- **0.3 ⬜ 🔍 Review** aller Dateien aus 0.1 und 0.2 (Diff `df6e1de..main`). Danach `sp0-done` und `sp1-start` taggen.
  **Fertig, wenn:** Abnahme hier eingetragen, SP1 bestätigt.

---

## Fahrplan

Der Weg zur Go-Engine läuft über kleine Sprints, jeder in einer Domäne. Nach **SP9** spielt man wieder am TV,
dann schon über den Go-Server, mit mehreren Räumen und gemischten Spielern.

| Sprint | Domäne | Thema | Am Ende sichtbar |
|---|---|---|---|
| SP1 | INF | Leitplanken + Go-Gerüst | `npm run check` und `go test` in der CI |
| SP2 | SRV 🧑 | Protokoll v2 & Raummodell (Entwurf) | `docs/protocol.md`, Entscheidung 002 |
| SP3 | SRV | Go-Server Basis (ersetzt `server/*.mjs`) | EXE und Docker-Image liefern das Spiel aus |
| SP4 | SIM | Golden-Tests, RNG, Level-Generator in Go | gleiche Level in TS und Go |
| SP5 | SIM | Port I: Welt, Zyklus, Wirtschaft | Golden-Tests grün |
| SP6 | SIM | Port II: Einheiten, Gegner, Wellen, Reisen, Kampagne | Golden-Tests grün, TS-Sim eingefroren |
| SP7 | SRV | Räume & WebSocket in Go (Protokoll v2) | 3 Räume parallel im Test |
| SP8 | CLI | Browser als reiner Client | Couch + Online am TV über den Go-Server |
| SP9 | INF | Aufräumen: TS-Sim und Node-Server löschen | Release `v0.2.0` |
| SP10 | SRV | Diagnose-TUI (Bubble Tea) | `k3c-tui` zeigt Räume live |
| SP11 | SRV 🧑 | Raspberry Pi: Docker, Autostart, Sicherungen, Lastmessung | 2er- und 3er-Spiel parallel auf dem Pi |

**Einschiebbar**, sobald der Mensch Zeit hat (unabhängig vom Fahrplan, 🧑): **R1** Regelwerk I und **X1** Xbox-Machbarkeit
(Details unten). Beide ändern keinen Engine-Code. Ein eingeschobener Sprint unterbricht den Fahrplan zwischen zwei Sprints,
nie mitten in einem.

Nach SP11: REG Regelwerk II (Skills) → SIM Skills in Go → CLI Skills → REG Spieleabend 1 → Grafik/Sound → …

---

## Nächste Sprints

### SP1 · INF · Leitplanken + Go-Gerüst (B-009, B-033)

- **1.1 Client-Lint.** Oxlint (TS 7 hat keine JS-API, `typescript-eslint` scheidet aus) mit dem Komplexitäts-Budget,
  Ausnahmen für Bestandscode (Ratsche). `npm run lint`, `npm run check` = lint + typecheck + test, in die CI.
  **Fertig, wenn:** `npm run check` grün unter Windows und in der CI; eine Probe-Funktion mit 61 Zeilen lässt `lint` scheitern.
- **1.2 Go-Gerüst.** `go.mod` im Root, `src/data/` → `data/` verschieben (Client-Importe anpassen), `data/embed.go`
  mit `go:embed`, ein erster Test, der alle JSON-Dateien lädt. `golangci-lint` mit `funlen`, `gocyclo`, `nestif`.
  CI: `go test ./...`, Lint, Cross-Build `windows/amd64` + `linux/arm64`.
  **Fertig, wenn:** CI grün mit beiden Werkzeugketten; `npm test` und das Spiel laufen unverändert mit `data/`.
- **1.3 Regel-Tests.** `tests/projectRules.test.ts`: Dateigröße mit Ratsche (`tests/complexity-baseline.json`),
  Schichtgrenzen im Client. Go-Schichtgrenzen per `depguard` in `golangci-lint`.
  **Fertig, wenn:** ein verbotener Import (TS und Go) lässt die Prüfung scheitern.
- **1.4 🔍 Review**

### SP2 · SRV · Protokoll v2 & Raummodell 🧑 (B-030, B-036, B-038, B-039)

Entwurfs-Sprint, kein Engine-Code. Claude schlägt vor, der Mensch entscheidet.

- **2.1 Raummodell.** Raum, Gerät, lokaler Spieler, Rollen; Beitreten/Verlassen/Wiederverbinden; Raumcode oder Lobby;
  Grenzen (Räume, Spieler pro Raum); was passiert mit dem Spielstand eines Raums.
  **Fertig, wenn:** Abschnitt „Raummodell“ in `docs/protocol.md` mit Beispiel „2 an der Xbox + 1 Handy“.
- **2.2 Nachrichten.** Eingaben pro lokalem Spieler, Snapshot-Format (voll/Delta), Level-Übertragung, Takt 30 Hz,
  Versionsfeld. Beispiel-Nachrichten als JSON in `testdata/protocol/` (Grundlage für Tests auf beiden Seiten).
  **Fertig, wenn:** jede Nachricht hat ein Beispiel, Größe eines Snapshots für 4 Spieler geschätzt.
- **2.3 🔍 Review + Entscheidung** `docs/decisions/002-protokoll-v2.md`

### SP3 · SRV · Go-Server Basis (B-020, B-027, B-028)

- 3.1 `cmd/k3c-server`: liefert `dist/` aus, `/api/save`, `/api/report`, `/api/health` wie `server/*.mjs`,
  Konfiguration über Umgebungsvariablen, Tests mit `httptest` (ersetzen den Smoke-Test)
- 3.2 `/api/status` (Version, Laufzeit, Speicher; Räume folgen in SP7) mit Token, strukturierte Logs (`log/slog`),
  `Dockerfile` (multi-arch amd64/arm64), `compose.yaml` mit Volumes; Release baut Windows-EXE + Linux-arm64
- 3.3 🔍 Review · **Fertig, wenn:** Spiel läuft von der EXE aus wie heute mit `npm run serve` (lokaler Modus)

### SP4 · SIM · Golden-Tests, RNG, Level-Generator

- 4.1 TS-Skript erzeugt Golden-Daten nach `testdata/golden/`: RNG-Folgen, Level für je 20 Seeds pro Biom,
  Simulationsläufe (feste Seeds + Eingabe-Folgen, Snapshot alle N Ticks)
- 4.2 `engine/rng` (mulberry32, FNV-1a über UTF-16) + `engine/level` inkl. `validateLevel`, Golden-Tests grün
- 4.3 🔍 Review

### SP5 · SIM · Port I – Welt, Zyklus, Wirtschaft

- 5.1 Zustand (`types.ts` → Go-Typen), `createWorld`, Tag/Nacht (`cycle.ts`) · 5.2 Wirtschaft (`economy.ts`)
- 5.3 🔍 Review · **Fertig, wenn:** Golden-Läufe ohne Gegner stimmen Tick für Tick

### SP6 · SIM · Port II – Einheiten, Gegner, Wellen, Reisen, Kampagne

- 6.1 Einheiten + Gegner + Wellen · 6.2 Reisen + Kampagne/Spielstand (Format mit Version, alte Stände lesbar)
- 6.3 🔍 Review · **Fertig, wenn:** alle Golden-Läufe inkl. Nacht und Stufenwechsel stimmen

### SP7 · SRV · Räume & WebSocket in Go

- 7.1 `engine/room`: Raum-Verwaltung, eine Goroutine pro Raum, Takt 30 Hz, mehrere lokale Spieler pro Gerät
- 7.2 `engine/net`: WebSocket nach Protokoll v2, Wiederverbinden, `/api/status` zeigt Räume
- 7.3 🔍 Review · **Fertig, wenn:** Test mit 3 Räumen (2, 3 und 4 Spieler) parallel, Tick-Dauer im Status sichtbar

### SP8 · CLI · Browser als reiner Client

- 8.1 Client spricht Protokoll v2, zeichnet Snapshots mit Interpolation · 8.2 mehrere lokale Spieler pro Gerät
  (Split-Screen), Couch-Spiel ebenfalls über den Server · 8.3 einfache Raumwahl (erstellen/beitreten)
- 8.4 🔍 Review · **Am TV:** 2 Controller an der Xbox + 1 Handy im selben Raum

### SP9 · INF · Aufräumen

- 9.1 `src/world/`, `src/online/room.ts`, `src/online/wsServer.ts`, `server/*.mjs`, `vite.server.config.ts` löschen,
  CI/README/CLAUDE.md anpassen · 9.2 🔍 Review · Release `v0.2.0`

### SP10 · SRV · Diagnose-TUI (B-002)

- 10.1 `cmd/k3c-tui` (Bubble Tea): Räume, Geräte, Tick-Dauer, Speicher, Log live
- 10.2 Aktionen: Raum ansehen, Gerät trennen, Spielstand sichern · 10.3 🔍 Review

### SP11 · SRV · Raspberry Pi 🧑 (B-035, B-042, B-028)

- 11.1 🧑 Pi einrichten, Docker-Compose, Autostart, rotierende Sicherungen der Spielstände
- 11.2 Lastmessung auf dem Pi (2er- + 3er-Spiel parallel), Ergebnis gegen das Ziel aus B-042 · 11.3 🔍 Review

## Einschiebbare Sprints 🧑

### R1 · REG · Regelwerk I – Fundament (B-004, B-005, B-021, B-025)

Diskussions-Sprint: Claude bereitet vor und stellt Fragen einzeln, der Mensch entscheidet. Kein Code.

- **R1.1 Ist-Regelwerk.** Regeln aus `src/world/sim/` und `data/` gegen `game-design.md` abgleichen.
  **Fertig, wenn:** Liste aller Widersprüche und Lücken im Backlog, Gliederung für `docs/rules/` steht.
- **R1.2 Workshop Kern-Loop & Wirtschaft.** Gold/Material, Besitz im gemischten Koop (Couch + Online, 2–4+ Spieler),
  Tag/Nacht, Wellen. **Fertig, wenn:** `docs/rules/wirtschaft.md` mit Regel, Begründung, Verweis auf die JSON-Werte.
- **R1.3 Workshop Stufen & Niederlage.** Tiefen, Aggressionspool, Strafen, Ziel der Kampagne.
  **Fertig, wenn:** `docs/rules/stufen.md` wie oben.
- **R1.4 🔍 Review + Beschluss.** Regelwerk v1, Umsetzungsaufgaben als Backlog-Einträge (SIM in Go, CLI).

### X1 · PLAT · Xbox-Machbarkeit (B-006, B-026)

- **X1.1 🧑 Gamepad-Test** auf der Xbox (Anleitung im README), zwei Controller. Bericht landet in `reports/`.
- **X1.2 Auswertung.** Steuerungstabelle in `game-design.md` ohne „vermutlich“, Skill-Tasten, Sprite-Budget, HTTPS ja/nein.
  **Fertig, wenn:** kein „vermutlich“ mehr in der Tabelle, B-026 entschieden.
- **X1.3 🔍 Review**

---

## Erledigt (vor der Sprint-Einteilung)

Die früheren Sessions S1.1–S1.11, S2.1, S2.2 und S2.4 sind umgesetzt (Vertical Slice, Speichern, Truhen, Tiefen).
Details stehen in der Git-Historie (`git log -- docs/sessions.md`). Wichtige Abweichungen vom alten Plan:

- **Eine Taste für alles (K2C):** A / Leertaste halten = im Takt Münzen an das nächste Ziel. Ohne Ziel fällt die Münze.
  X (Interagieren) ist noch frei.
- Gold pro Spieler, Baumaterial gemeinsam im Hub. Bauplätze fest im Hub (`src/data/hub.json`).
- Start mit 1 Bauer + 2 Bogenschützen, sonst ist Nacht 1 nicht zu schaffen (Test „Balancing“).
- Spielstand: `src/world/sim/campaign.ts`, `server/saves.mjs`, Autosave bei Tagesanbruch und Stufenwechsel.
- Online-Modus (`?online=RAUM`), Touch-Steuerung und Figuren-Sprites kamen außerhalb des alten Plans dazu.

Infrastruktur: CI (`ci.yml`), GitHub Pages (`deploy-pages.yml`), Release per Tag `v*` (`release.yml`).
