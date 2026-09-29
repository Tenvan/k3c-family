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
| **REG** | Regelwerk & Balancing | `docs/game-design.md`, `docs/rules/`, `docs/playtests/`, **Werte** in `src/data/*.json` |
| **SIM** | Spiel-Logik | `src/world/`, `src/core/rng.ts`, `src/core/constants.ts`, **neue Felder** in `src/data/*.json` |
| **CLI** | Client: Darstellung, HUD, Grafik, Audio | `src/scenes/`, `public/`, `src/online/client.ts`, `src/core/saveStore.ts` |
| **PLAT** | Plattform: Eingabe, Shell, Seiten | `src/input/`, `src/core/shell.ts`, `src/core/fullscreen.ts`, `src/landing/`, `src/tools/`, `*.html` |
| **SRV** | Server & Betrieb | `server/`, `src/online/room.ts`, `src/online/wsServer.ts`, `vite.server.config.ts`, Docker, TUI |
| **INF** | Frameworks, Tooling, CI | `package.json`, `vite.config.ts`, `tsconfig.json`, `.github/`, `tests/projectRules.test.ts` |

Grenzfälle, damit niemand raten muss:

- **Daten:** SIM legt neue Felder mit vorläufigen Werten an (aus dem REG-Beschluss). REG ändert danach nur Werte.
- **Online-Protokoll** (`src/online/protocol.ts`) betrifft Client und Server. Eine Änderung daran bekommt eine
  eigene Session, die nur das Protokoll und beide Enden anpasst.
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
- Schichtgrenzen: `src/world` importiert nichts aus `scenes/`, `online/`, `input/`; `server/` nichts aus `scenes/`.

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
  `.github/pull_request_template.md`, `CLAUDE.md`, `roadmap.md`, `README.md` angepasst.
  Ergebnis: Plan einmal per Critic-Prüfung überarbeitet (Domänen-Grenzfälle, Blockaden, Ratsche, Fertig-wenn).
- **0.2 ⬜ 🔍 Review** der Dateien aus 0.1, Planung von SP1 bestätigen. Diff-Basis ausnahmsweise `df6e1de`.
  Danach `sp0-done` taggen. **Fertig, wenn:** Abnahme hier eingetragen, `sp1-start` gesetzt.

---

## Nächste Sprints

### SP1 · INF · Leitplanken automatisieren (B-009, B-020)

- **1.1 ESLint.** Flat Config mit `typescript-eslint` und den Regeln aus dem Komplexitäts-Budget, Ausnahmeliste für
  Bestandscode (Ratsche). Skript `npm run check` = lint + typecheck + test, in die CI.
  **Fertig, wenn:** `npm run check` lokal und in der CI grün; eine Test-Datei mit 61-Zeilen-Funktion lässt lint scheitern.
- **1.2 Regel-Tests.** `tests/projectRules.test.ts` prüft Schichtgrenzen (Importe) und Dateigrößen (inkl. Ratsche).
  Server-Smoke-Test aus `ci.yml` nach `scripts/smoke.mjs` (Node, ohne curl), `npm run smoke`, CI ruft das Skript.
  **Fertig, wenn:** absichtlicher Import `phaser` in `src/world` lässt `npm test` scheitern; `npm run smoke` läuft unter Windows.
- **1.3 🔍 Review**

### SP2 · PLAT · Xbox-Machbarkeit 🧑 (B-006)

- **2.1 🧑 Gamepad-Test** auf der Xbox (Anleitung im README), zwei Controller. Bericht landet in `reports/`.
- **2.2 Auswertung.** Steuerungstabelle in `game-design.md` ohne „vermutlich“, Skill-Tasten festgelegt,
  Sprite-Budget als Konstante in `src/core/constants.ts` (Abstimmung mit SIM: nur eine Konstante), HTTPS ja/nein.
  **Fertig, wenn:** kein „vermutlich“ mehr in der Tabelle, B-026 entschieden.
- **2.3 🔍 Review**

Bei Blockade: SP3 vorziehen. Vorläufige Annahme für Skills: **LB/RB** (B-026).

### SP3 · REG · Regelwerk I – Fundament 🧑 (B-004)

Diskussions-Sprint: Claude bereitet vor und stellt Fragen einzeln, der Mensch entscheidet. Kein Code.

- **3.1 Ist-Regelwerk.** Regeln aus `src/world/sim/` und `src/data/` gegen `game-design.md` abgleichen.
  **Fertig, wenn:** Liste aller Widersprüche und Lücken (mind. B-005, B-025) im Backlog, Gliederung für `docs/rules/` steht.
- **3.2 Workshop Kern-Loop & Wirtschaft.** Gold/Material, Besitz im Koop (lokal und online), Tag/Nacht, Wellen.
  **Fertig, wenn:** `docs/rules/wirtschaft.md` mit Regel, Begründung, Verweis auf die JSON-Werte.
- **3.3 Workshop Stufen & Niederlage.** Tiefen, Aggressionspool, Strafen, Ziel der Kampagne.
  **Fertig, wenn:** `docs/rules/stufen.md` wie oben.
- **3.4 🔍 Review + Beschluss.** Regelwerk v1, Umsetzungsaufgaben als Backlog-Einträge (SIM/CLI), Entscheidungen in `docs/decisions/`.

### SP4 · REG · Regelwerk II – Monarch & Skills 🧑

- 4.1 Skill-Linien Tank + Zauberer konkret: Werte, Tasten (aus SP2), Koop-Rollen (B-017), Level-Up ja/nein
- 4.2 🔍 Review + Beschluss `docs/rules/skills.md`

### SP5 · SIM · Skills – Logik (B-007, B-022)

- 5.1 `src/data/skills.json` (Felder + vorläufige Werte aus SP4), Punkte verteilen, Tier-Gating, Respec im Hub
- 5.2 Aktive Skills Tank (Taunt, Shield Bash) · 5.3 Zauberer (Fireball, Ice Wall) + Skills/Level im Spielstand
- 5.4 🔍 Review

### SP6 · CLI · Skills – Darstellung (B-018)

- 6.1 Aufräumen: `worldRenderer.ts` und `GameScene.ts` unter 300 Zeilen, Ausnahmeliste kürzen
- 6.2 Skill-Menü (View) und Skill-Slots mit Cooldown im HUD · 6.3 Skill-Effekte (Platzhalter reichen)
- 6.4 🔍 Review · danach Release `v0.2.0`

### SP7 · REG · Spieleabend 1 🧑 (B-008)

- 7.1 🧑 Familie spielt Tag + Nacht zu zweit, Claude schreibt `docs/playtests/1.md` (Beobachtungen, Zitate, Zahlen)
- 7.2 Balancing, **nur Werte in JSON**. Alles andere → Backlog · 7.3 🔍 Review · Release `v0.3.0`

### SP8 · SRV · Server-Architektur entscheiden (B-001, B-002, B-003)

- **8.1 Messen.** Benchmark-Skript: Räume × Spieler, Zeit pro `step()`, Speicher, Tick-Treue bei `TICK_HZ` (30 Hz).
  **Fertig, wenn:** Tabelle mit Messwerten und der Antwort „reicht der jetzige Server für N Räume?“.
- **8.2 Wegwerf-Prototypen** (je ≤ 1 h, nicht mergen): Node + Fastify/Hono, Go-Server mit Sim als Node-Beiprozess,
  TUI-Probe (Ink vs. Bubble Tea) gegen eine Fake-Status-Schnittstelle.
- **8.3 🔍 Review + Entscheidung** `docs/decisions/001-server.md`. Nur wenn nötig: Umbau-Sprint ins Backlog.

### SP9 · SRV · Server betriebsbereit (B-027, B-028)

- 9.1 `Dockerfile` + `compose.yaml`, Volumes für `saves/` und `reports/`, Healthcheck `/api/health`
- 9.2 Diagnose-Schnittstelle `/api/status` (Räume, Geräte, Tick-Dauer, Speicher, letzte Fehler), strukturierte Logs,
  Zugriff nur mit Token aus Umgebungsvariable
- 9.3 🔍 Review

### SP10 · SRV · Diagnose-TUI (B-002)

- 10.1 TUI zeigt den Status live (eigener Prozess, spricht nur mit `/api/status`)
- 10.2 Aktionen: Raum ansehen, Gerät trennen, Spielstand sichern, Log folgen · 10.3 🔍 Review

### Danach (Reihenfolge bei der Planung)

- CLI · Lade-Szene + Grafik für Gebäude, Ressourcen, Hintergrund (B-010, B-029) · CLI · Sound & Musik (B-011)
- Feature-Kette Gegner/Truppen/Mine: REG Regelwerk III (B-012–B-015) → SIM → CLI
- REG · Spieleabend 2 🧑 · SRV · Online robuster (B-030) · SRV · Framework-Umbau (nur nach Beschluss in SP8)
- Feature-Kette 3–4 Spieler (B-016) · INF · itch.io-Release (B-023)

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
