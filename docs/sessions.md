# Session-Plan

Die Roadmap (`docs/roadmap.md`) in Arbeitspakete zerlegt, die **je eine Claude-Session** füllen.
Jede Session endet mit grüner CI, einem Commit/PR und etwas, das man am TV mit Controller ausprobieren kann.

## Ablauf jeder Session

1. **Start:** diese Datei + `docs/roadmap.md` lesen, nächste offene Session nehmen. Branch pro Session.
2. **Umsetzen:** Werte nach `src/data/*.json`, reine Logik nach `src/world/` (ohne Phaser) mit `*.test.ts` daneben,
   Darstellung in `src/scenes/`. Alles mit **2 Spielern** testen (Tastatur + Controller-Mock, siehe `CLAUDE.md`).
3. **Abschluss:** `npm run typecheck` + `npm test` grün, Häkchen hier und in der Roadmap setzen, PR öffnen.
   CI (`.github/workflows/ci.yml`) muss grün sein, bevor gemergt wird. Nach dem Merge liegt der Stand auf GitHub Pages.
4. **TV-Check** (Familie/Xbox): kurzer Punkt „Am TV prüfen“ je Session. Ergebnis als Notiz in die Session eintragen.

Legende: ⬜ offen · 🟨 in Arbeit · ✅ fertig · 🧑 braucht den Menschen (Xbox, Entscheidung)

---

## Phase 0 – Xbox-Machbarkeit

### S0.1 🧑 Gamepad-Test auf der Xbox auswerten

- **Mensch:** `npm run serve`, Test laut README auf der Xbox, Bericht landet in `reports/`.
- **Claude:** Bericht auswerten → `docs/game-design.md` › Steuerung: B-Verhalten, Button-Indizes, Vollbild,
  Secure Context (HTTPS nötig?), max. Sprites bei 60 FPS. Offene Punkte in `src/input/playerInput.ts` nachziehen.
- **Fertig, wenn:** Steuerungstabelle ohne „vermutlich“, Sprite-Budget als Konstante in `src/core/constants.ts`.

---

## Phase 1 – Vertical Slice „Ein Tag, eine Nacht“ (Roadmap Schritt 2)

Reihenfolge ist so gewählt, dass jede Session auf der vorigen aufbaut und einzeln spielbar ist.

**Stand:** S1.1–S1.11 sind zusammen umgesetzt (ohne Xbox-Test, der ist für nichts davon nötig). Abweichungen vom Plan:

- Simulation in `src/world/sim/` (reine Logik, Tests in `sim.test.ts`), Darstellung in `src/scenes/worldRenderer.ts`.
- **Eine Taste für alles (K2C):** A / Leertaste halten = im Takt Münzen an das nächste Ziel in Reichweite
  (Bauplatz, Werkstatt, Landstreicher, Baum/Fels markieren). Ohne Ziel fällt die Münze → so gibt man dem anderen Gold.
  X (Interagieren) bleibt vorerst frei.
- Gold pro Spieler, Baumaterial gemeinsam im Hub (Vorschlag aus dem GDD übernommen, am Spieleabend prüfen).
- Bauplätze fest im Hub (`src/data/hub.json`), nicht pro Biom. Start mit 1 Bauer + 2 Bogenschützen,
  sonst ist Nacht 1 nicht zu schaffen (per Simulation geprüft, Test „Balancing“).
- Wölfe/Skelette kommen als Nacht-Gegner mit der Welle aus den Portalen.
- Dev: `?fast=1` (Tag/Nacht 8x), `?dev=1` bzw. Dev-Server: **G** +10 Gold, **H** +50 Material, **T** nächste Tageszeit.

### S1.1 ✅ Welt-Zustand & Entitäten-Grundlage

- `src/world/worldState.ts`: reiner Zustand (Entitäten mit `x` in Units, Typ, HP, Besitzer), Update pro Tick ohne Phaser.
- `GameScene` rendert Entitäten aus dem Zustand (Platzhalter-Formen), statt sie selbst zu verwalten.
- Tests: Tick-Update, Entität hinzufügen/entfernen, Determinismus bei gleichem Seed.
- **Am TV:** Spiel sieht aus wie vorher, läuft flüssig mit 2 Spielern.

### S1.2 ✅ Gold: Münzbeutel, Aufsammeln, Fallenlassen

- Münzbeutel pro Spieler (Start-Gold in `monarch.json`), Münzen als Welt-Entitäten, Aufsammeln im Vorbeigehen.
- **A** halten = Münze fallen lassen/geben (K2C-Stil). Münzen über dem Kopf anzeigen.
- Tests: Beutel-Limit, Aufsammeln-Radius, Geben zwischen Spielern.
- **Am TV:** Beide sammeln Münzen, einer gibt dem anderen etwas ab.

### S1.3 ✅ Rekrutierungs-Camp: Landstreicher → Bauer

- Landstreicher spawnen in `recruitCamp`-Chunks, wandern umher. Münze geben → Bauer (`troops.json`).
- Bauer folgt dem Hub, idle-Verhalten im Hub.
- Tests: Rekrutieren kostet genau `recruitCost`, Camp respawnt Landstreicher (Rate als Daten).
- **Am TV:** Zwei Bauern rekrutieren, sie laufen zum Hub.

### S1.4 ✅ Bauer fällt Bäume → Holz

- Bäume aus dem Level-Layout als Entitäten, Bauer sucht nächsten markierten Baum, fällt ihn, bringt Holz zum Hub.
- Markieren per Münze am Baum (K2C-Stil). Holz = gemeinsamer Hub-Vorrat (offene Frage im GDD: hier entscheiden 🧑).
- Tests: Job-Zuweisung, Holz-Zähler, Baum verschwindet.
- **Am TV:** Baum markieren, Holz steigt im HUD.

### S1.5 ✅ HUD pro Split-Screen-Hälfte

- `HudScene`: Gold, Holz, Tag/Nacht-Platzhalter pro Hälfte. Große Schrift, hoher Kontrast (Couch-Abstand).
- Meldungs-Banner in der Mitte (API `hud.announce(text)` für spätere Sessions).
- **Am TV:** Lesbar vom Sofa, beide Hälften korrekt.

### S1.6 ✅ Bauplätze: Mauer und Turm

- Feste Bauplätze im Hub (Positionen in der Biom-JSON), Kosten als Münz-Slots über dem Platz.
- Münzen/Holz einzahlen (Interagieren **X**), Bauer kommt und baut (Bauzeit als Daten in `buildings.json`).
- Tests: Kostenprüfung, Bau-Fortschritt, Gebäude-HP.
- **Am TV:** Eine Mauer links, einen Turm rechts bauen.

### S1.7 ✅ Werkstatt: Bogen → Bogenschütze

- Werkstatt bauen, Bogen kaufen, Bauer holt Bogen ab → Bogenschütze. Bogenschützen besetzen Türme.
- Tests: Upgrade-Pfad aus `troops.json` (`upgradeFrom`), Turm-Kapazität.
- **Am TV:** Zwei Bogenschützen stehen auf dem Turm.

### S1.8 ✅ Tag/Nacht-Zyklus

- `src/world/dayCycle.ts` (reine Logik) mit Dauern aus der Biom-JSON, Dev-Faktor für verkürzte Tests (URL `?fast=1`).
- Himmelsfarbe/Parallax dunkeln ab, Banner „Nacht naht!“ in der Dämmerung.
- Tests: Phasenwechsel, Zeitskalierung.
- **Am TV:** Ein kompletter Zyklus in ~2 min mit `?fast=1`.

### S1.9 ✅ Portal-Welle: Greed

- Portale öffnen sich nachts, Welle nach GDD-Tabelle (Wellen-Nummer → Anzahl) aus `enemies.json`.
- Greed laufen zum Hub, greifen Mauer/Truppen an, klauen Gold vom Monarch (`stealsGold`).
- Tests: Wellen-Größen, Ziel-Wahl (`prefersBuildings` …), Gold-Diebstahl.
- **Am TV:** Erste Nacht übersteht man nur mit Mauer.

### S1.10 ✅ Kampf: Bogenschützen schießen, Drops

- Automatisches Zielen innerhalb `range`, Pfeile als Projektile (Sprite-Budget aus S0.1 beachten).
- Gegner droppen Gold (Standard 5–15), 10% Stufen-Ressource.
- Tests: Schaden/Angriffstempo aus Daten, Drop-Verteilung mit festem RNG.
- **Am TV:** Bogenschützen halten eine Welle, Gold liegt danach vor der Mauer.

### S1.11 ✅ Niederlage & Respawn

- Monarch tot → Respawn am Hub. Hub-Kern zerstört → Gebäude bleiben kaputt, 50% Ressourcen und Truppen weg.
- Tests: Strafen-Regeln aus dem GDD.
- **Am TV:** Absichtlich verlieren, Spiel geht sinnvoll weiter.

### S1.12 🧑 Vertical-Slice-Abend

- Familie spielt einen Tag + eine Nacht zu zweit. Claude sammelt Feedback in `docs/playtest-1.md` und
  macht daraus Balancing-Änderungen (nur JSON) plus Folge-Sessions in dieser Datei.

---

## Phase 2 – Fortschritt (Roadmap Schritt 3)

### S2.1 ⬜ Speichern/Laden auf dem Heimnetz-Server

- `POST/GET /api/save` in `server/` (analog zu `reports.mjs`), Fallback localStorage. Gespeichert: Seeds, Hub, Monarch.
- Tests: Serialisieren/Deserialisieren ohne Phaser, Server-Handler mit Fake-Requests.

### S2.2 ✅ Truhen & versteckte Skill-Punkte

- Truhen aus dem Layout öffnen (Gold), Skill-Punkte finden, Banner „Skill-Punkt gefunden!“.
- ✅ Einsammeln im Vorbeilaufen, Skill-Punkte zählen gemeinsam (`world.skillPoints`). Verteilen kommt mit S2.3.

### S2.3 ⬜ Skill-Baum (1–2 Linien) + 2 aktive Skills (braucht S0.1: Tastenbelegung)

- Daten in `src/data/skills.json`, Tier-Gating, Skill-Menü (**View**), Slots nach S0.1-Belegung. Start: Tank + Zauberer.

### S2.4 🟨 Tiefen-Eingang & Höhle mit Aggressionspool

- Eingang am Levelende → Stufe 1 mit eigenem Hub. `src/world/aggression.ts` (+1%/min, +5%/Kill, +1%/Ressource).
- Tests: Pool-Regeln, Übergang zwischen Stufen behält Stufe-0-Hub.
- ✅ Aggressionspool inkl. Wellen läuft (Dev: Tasten 2/3). Offen: Tiefen-Eingang nutzen, Hub pro Stufe behalten (braucht S2.1).

---

## Phase 3 – Inhalt & Politur (Roadmap Schritt 4)

Grober Schnitt, wird nach Phase 2 verfeinert:

- S3.1 Pixel-Art-Pack einbinden (Kenney, CC0) + Lade-Szene · S3.2 Animationen · S3.3 Sound & Musik
- S3.4 Mine (Stufe 2) + Treppen zwischen den Hubs · S3.5 restliche Gegner + Elite-KI
- S3.6 Krieger & Elite-Truppen · S3.7 Balancing-Abende (🧑)

---

## Infrastruktur (erledigt)

- ✅ **CI** `.github/workflows/ci.yml`: Typecheck, Tests (inkl. Regel-Tests `tests/projectRules.test.ts`), Build,
  Smoke-Test des Heimnetz-Servers, `dist/` als Artefakt.
- ✅ **CD** `.github/workflows/deploy-pages.yml`: `main` → GitHub Pages (einmalig in den Repo-Settings aktivieren).
- ✅ **Release** `.github/workflows/release.yml`: Tag `v*` → Zip mit `dist/` + Server am GitHub-Release.
