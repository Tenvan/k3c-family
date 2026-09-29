# Family Three Crowns (K3C) – Game Design

Kompakte Fassung des alten GDD (`C:\WORKSPACE\FamilyCrowns\docs\gdd\`), bereinigt um Godot-Spezifika.
Zahlenwerte stehen als Daten in `src/data/*.json`. Dieses Dokument erklärt das *Warum* und die Regeln.

## Pitch

2D-Side-Scroller-Strategiespiel im Stil von **Kingdom Two Crowns (K2C)** für **Couch-Koop auf der Xbox**
(Browser/Edge). Die Monarchen bauen einen Hub aus, rekrutieren Truppen und verteidigen ihn gegen
Portal-Angriffe. Unterschied zu K2C: Die Welt geht **nach unten** (Oberwelt → Höhle → Mine), der Monarch
kämpft **aktiv** mit Skills mit, und die Level werden **prozedural aus festen Eckdaten** erzeugt.

Zielgruppe: die eigene Familie (ab 12), später evtl. itch.io.

## Harte Anforderungen

1. **Couch-Koop auf der Xbox** (Edge-Browser, Gamepad API). Alles andere ist verhandelbar.
2. **Prozedurale Level aus festen Eckdaten** (Biom-Config + Seed), damit Levels abwechslungsreich sind.
3. Läuft auch am PC (Tastatur + Controller) für Entwicklung und Tests.

## Kern-Loop (wie K2C)

```text
Tag:   Erkunden → Ressourcen/Gold sammeln → Rekrutieren → Hub ausbauen → Truppen ausrüsten
Nacht: Portale öffnen sich → Gegnerwellen laufen auf den Hub zu → Verteidigen
Ziel:  Hub halten, Stufe erkunden, Tiefen-Eingang am Ende nutzen → nächste Stufe
```

Typische Session: 30–60 Minuten.

## Couch-Koop

- Jeder Spieler ist **ein eigener Monarch** mit eigener Figur und eigener Kamera.
- **Beitreten** jederzeit mit **A** (Controller) oder Leertaste (Tastatur).
- **MVP: 2 Spieler, Split-Screen oben/unten** (wie K2C). Jede Hälfte zeigt die volle Welthöhe.
- Ressourcen: Gold pro Spieler (wie K2C), Baumaterial (Holz/Stein/Kupfer) gemeinsam für den Hub. *So umgesetzt, im Playtest prüfen.*
- Online-Koop ist **kein Ziel** (kein Netzwerkcode).

## Monarch

- Bewegung nur **horizontal** (links/rechts), Sprint, **kein Springen**.
- Interagieren (Münze geben, Truhe öffnen, Eingang nutzen), Bau-Menü, Skill-Menü.
- Hauptrolle ist Truppen-Management. Im Kampf unterstützt er per Skill, trägt aber nicht den Hauptschaden.
- Stats Level 1: HP 100, Schaden 10, Speed 5 Units/s, Verteidigung 5. Pro Level +10/+2/+0.5/+1, max. Level 20.
- Tod: Respawn am Hub ohne Strafe, Truppen bleiben.

### Skill-System (offene Archetypen-Linien)

Ein gemeinsamer Skill-Baum mit 4 Linien: **Tank, Zauberer, Heiler, Dieb**. Punkte werden frei verteilt,
hybride Builds sind erlaubt. Die Presets aus `monarch.json` sind nur Startverteilungen. Respec kostenlos im Hub.

- Skill-Punkte vor allem **versteckt in der Welt** (3–5 pro Stufe), optional zusätzlich per Level-Up.
- Tier-Gating: Tier 2 ab 5, Tier 3 ab 10, Tier 4 (Ultimate) ab 15 Punkten in der Linie.
- 4 Skill-Slots (Controller: Schultertasten/D-Pad, genaue Belegung nach dem Gamepad-Test).

| Linie | Tier 1 (aktiv) | Tier 3 | Tier 4 (Ultimate) | Passive (Auswahl) |
|---|---|---|---|---|
| Tank | Taunt (10u, CD 15s), Shield Bash (Stun 2s, CD 10s) | Iron Wall (Barriere 500 HP, CD 30s) | Last Stand (überlebt mit 1 HP, CD 120s) | Armor Aura, Thick Skin, Regeneration, Guardian, Fortified |
| Zauberer | Fireball (30 AoE 3u, CD 8s), Ice Wall (Slow 50%, CD 20s) | Lightning Storm (10/s, 5s, CD 40s) | Meteor (200 AoE 15u, CD 120s) | Arcane Power, Spell Echo, Frost Armor, Elemental Mastery |
| Heiler | Heal (50 HP, CD 10s), Group Heal (30 HP 10u, CD 30s) | Divine Shield (100 HP Schild, CD 20s) | Resurrection (Truppen 50% HP, CD 180s) | Healing Aura, Speed-/Damage-Blessing, Holy Ground |
| Dieb | Backstab (40 / 80 von hinten, CD 12s), Smoke Bomb (unsichtbar 5s, CD 25s) | Poison Blade (10/s, 5s, CD 15s) | Shadow Strike (Teleport + 150, CD 90s) | Treasure Hunter, Swift, Critical Strike, Evasion, Resource Master, Shadow Step |

Die Skills kommen **erst nach dem Vertical Slice**.

## Truppen (`src/data/troops.json`)

- **Landstreicher** im Rekrutierungs-Camp → Münze geben → **Bauer** (folgt, sammelt, baut).
- Werkstatt: Bauer + Bogen → **Bogenschütze** (Fernkampf, besetzt Türme). Bauer + Schwert → **Krieger** (Nahkampf, Frontlinie).
- Elite-Upgrades mit Stein/Kupfer: +50% HP und Schaden, +20% Angriffstempo.
- Truppen kämpfen automatisch (KI). Truppen-Limit hängt von den Kasernen ab.

## Gebäude (`src/data/buildings.json`)

Burg/Thron (Hub-Kern), Mauer, Turm, Tor, Werkstatt, Farm (optional), Kaserne (ab Tiefe 1), Treppe hoch/runter (ab Tiefe 1, max. je 1 pro Hub).
Platzierung auf einem Raster im Hub-Bereich, gebaut wird von Bauern.
**Achtung:** HP- und Kostenwerte außer bei Turm und Treppen sind **Platzhalter** (nicht im GDD definiert) und müssen gebalanced werden.

## Welt & Stufen

| Stufe | Biom | Länge (Units) | Primär-Ressource | Zyklus | Gegner |
|---|---|---|---|---|---|
| 0 | Oberwelt (Wald) | 900–1100 | Holz | Tag 10 min / Nacht 5 min, 1 min Dämmerung | Greed, Goblin, Goblin Archer (Elite), Wolf (nachts) |
| 1 | Höhle | 700–900 | Stein | Aggressionspool | Fledermaus, Höhlentroll (Elite), Skelett (nachts) |
| 2 | Mine | 550–700 | Kupfer | Aggressionspool | Zombie, Rattenschwarm, Minengeist (Elite) |

Gold gibt es überall (Truhen, Gegner-Drops). Post-MVP: Tiefe 3 (Eisen, Lava), Tiefe 4 (Kristall).

- **Jede Stufe hat einen eigenen Hub**, der von Grund auf neu gebaut wird. Alle Hubs bleiben bestehen.
- **Tiefen-Eingang** am Ende der Stufe führt nach unten. Zurück geht es nur über gebaute **Treppen**.
- **Aggressionspool** (unter Tage statt Tag/Nacht): +1%/min, +5% pro Kill, +1% pro gesammelter Ressource.
  Bei 100% folgt ein Portal-Angriff, danach Reset auf 0. So bestimmen die Spieler das Tempo selbst.
- Globaler Tag/Nacht-Zyklus läuft auch unter Tage weiter (Skelette nachts, Wölfe bei Vollmond).

### Prozedurale Generierung (Pflicht-Feature)

Die festen Eckdaten stehen in `src/data/biomes/<biom>.json`, der Generator in `src/world/levelGenerator.ts`.

```text
[Rand|Ausgang] … [Portal] … [Chunks] [ HUB ] [Chunks] … [Portal] … [Ausgang|Rand]
```

- Welt = Reihe von **Chunks** (50 Units breit). Der Hub liegt immer in der Mitte.
- Deterministisch per **Seed**: gleicher Seed ergibt das gleiche Level. Gespeichert wird nur der Seed, nicht das Level.
- Reihenfolge: Hub, Ausgang und Rand festlegen → Portale (Mindestabstand zum Hub, abwechselnd links/rechts)
  → Event-Chunks (Truhe, Rekrutierungs-Camp nahe am Hub) → restliche Chunks gewichtet aus `chunkWeights`
  → Ressourcen pro Chunk-Typ aus `resourcesPerChunk` → versteckte Skill-Punkte.
- `validateLevel()` prüft die Spielbarkeits-Regeln. Die Tests jagen jede Biom-Config durch 500 Seeds.
  **Jede Änderung an einer Biom-JSON muss `npm test` bestehen.**
- Später möglich: Chunk-Vorlagen mit Untervarianten, Gegner-Camps in der Welt, Biom-spezifische Hazards.

## Gegner (`src/data/enemies.json`)

- Laufen geradeaus auf den Hub zu, **keine Sprünge**. Zustände: Idle → Move → Attack (→ Flee).
- Elite-Gegner: Kiting (Fernkampf), Spezialangriffe (AoE).
- Eigenschaften: `stealsGold` (Greed klaut Gold bei Kontakt mit dem Monarch), `flying`/`ignoresWalls` (Fledermaus, Geist),
  `prefersBuildings`/`prefersTroops`/`prefersMonarch`, `swarm`, `fleesAtHalfHp`.
- Wellen: 1–5 → 5–10 Standard. 6–10 → 10–15 Standard + 1–2 Elite. Ab 11 → 15–20 Standard + 3–5 Elite.
- Skalierung pro Stufe: HP +50%, Schaden +30%, Speed +10%.
- Drops: Standard 5–15 Gold, Elite 20–50 Gold, 10% Chance auf Stufen-Ressource.

## Niederlage

- **Hub-Kern zerstört:** Respawn am Hub, Gebäude bleiben zerstört, 50% der Ressourcen und alle Truppen verloren.
- **Monarch tot:** Respawn am Hub, keine Strafe.

## Steuerung

| Aktion | Controller (Xbox) | Tastatur |
|---|---|---|
| Laufen | Linker Stick / D-Pad ←→ | A/D, ←/→ |
| Sprint | RT | Shift |
| Beitreten; halten = Münzen geben/fallen lassen | A | Leertaste |
| Interagieren | X | E |
| Bau-Menü | Y | B |
| Skill-Menü | View | K |
| Pause | Menu | Esc |
| Vollbild | RS (Stick drücken) | F |

**B bleibt unbelegt**, weil Edge auf der Xbox B vermutlich als „Zurück“ nutzt. Der Gamepad-Test klärt das.

## UI

- Minimalistisch wie K2C: möglichst viel in der Welt anzeigen (Münzen über dem Kopf, Baupreise als Münz-Slots).
- HUD pro Split-Screen-Hälfte: HP, Gold, Ressourcen, Tag/Nacht bzw. Aggressionspool, Skill-Slots mit Cooldown.
- Meldungen in der Mitte: „Nacht naht!“, „Portal öffnet sich!“, „Skill-Punkt gefunden!“.
- Mindestgröße für Texte auf dem TV beachten (Couch-Abstand!), hoher Kontrast.

## Speichern

JSON mit Monarch-Zustand (Level, Skills), pro Hub (Gebäude, Truppen, Ressourcen), Fortschritt (freigeschaltete Tiefen)
und den **Seeds** der Stufen. Gegner und Level-Layout werden nicht gespeichert. Speicherort ist der Heimnetz-Server
(Browser-Speicher auf der Xbox gilt als unzuverlässig), mit Fallback auf localStorage.
Umgesetzt in `src/world/sim/campaign.ts`: Autosave bei Tagesanbruch und beim Stufenwechsel. Zusätzlich gespeichert wird,
was aus der Welt schon entfernt wurde (gefällte Bäume, geöffnete Truhen), sonst kämen sie beim Laden zurück.

## Grafik & Audio

- Figuren (Monarchen, Truppen, Gegner): Seitenansicht-Sprites von **LuizMelo (CC0)** für unsere Seite und
  **Gothicvania von ansimuz (CC0)** für die Gegner (Wolf, Skelett, Zombie, Geist, Fledermaus, Höhlentroll). Zuordnung in
  `src/data/sprites.json`, Bilder und Credits in `public/sprites/`. Referenz: Testseiten `aufstellung.html` (Rollen) und
  `figuren.html` (alle Figuren, auch ungenutzte). Gebäude, Ressourcen und Hintergrund sind noch
  **Platzhalter-Formen**. Ziel: 2D-Pixel-Art mit Parallax-Ebenen.
- Paletten pro Biom stehen in der Biom-JSON (Wald grün/braun, Höhle grau/orange, Mine braun/kupfer).
- Audio: Kenney Audio / freesound.org (CC).

## Offene Fragen

- 3–4 Spieler: vier Streifen übereinander werden sehr flach. Alternativen wären ein 2×2-Raster oder eine gemeinsame Kamera, solange die Spieler nah beieinander sind.
- Skill-Tasten am Controller (hängt vom Gamepad-Test ab).
- Eine Klasse pro Spieler als Preset, damit sich die Rollen im Koop ergänzen?
