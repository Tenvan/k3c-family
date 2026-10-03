# Family Three Crowns (K3C) – Game Design

Kompakte Fassung des alten GDD (`C:\WORKSPACE\FamilyCrowns\docs\gdd\`), bereinigt um Godot-Spezifika.
Zahlenwerte stehen als Daten in `data/*.json`. Dieses Dokument erklärt das *Warum* und die Regeln.

## Pitch

2D-Side-Scroller-Strategiespiel im Stil von **Kingdom Two Crowns (K2C)** für **gemischten Koop** (Couch und Online) auf der
Xbox (Browser/Edge) und weiteren Geräten. Die Monarchen bauen einen Hub aus, rekrutieren Truppen und verteidigen ihn gegen
Portal-Angriffe. Unterschied zu K2C: Die Welt geht **nach unten** (Oberwelt → Höhle → Mine), der Monarch
kämpft **aktiv** mit Skills mit, und die Level werden **prozedural aus festen Eckdaten** erzeugt.

Zielgruppe: die eigene Familie (ab 12), später evtl. itch.io.

## Harte Anforderungen

1. **Koop auf der Xbox** (Edge-Browser, Gamepad API), gemischt Couch und Online (z. B. zwei Controller an der Xbox, weitere Spieler per Handy). Alles andere ist verhandelbar.
2. **Prozedurale Level aus festen Eckdaten** (Biom-Config + Seed), damit Levels abwechslungsreich sind.
3. Läuft auch am PC (Tastatur + Controller) für Entwicklung und Tests.
4. **Der Go-Server rechnet** (Entscheidung `decisions/001-server-engine-go.md`): Der Browser schickt nur Eingaben und zeichnet Snapshots, auch beim reinen Couch-Spiel.

## Kern-Loop (wie K2C)

```text
Tag:   Erkunden → Ressourcen/Gold sammeln → Rekrutieren → Hub ausbauen → Truppen ausrüsten
Nacht: Portale öffnen sich → Gegnerwellen laufen auf den Hub zu → Verteidigen
Ziel:  Hub halten, Stufe erkunden, Tiefen-Eingang am Ende nutzen → nächste Stufe
```

Typische Session: 30–60 Minuten.

## Koop (Couch und Online gemischt)

- Jeder Spieler ist **ein eigener Monarch** mit eigener Figur und eigener Kamera, auf demselben oder auf einem anderen Gerät.
- **Beitreten** jederzeit mit **A** (Controller), Leertaste (Tastatur) oder der Münz-Taste (Touch).
- **2–4+ Spieler pro Raum.** Lokale Spieler teilen sich den Bildschirm (Split-Screen, Layout für 1–4 Spieler).
- **Mehrere Räume** laufen parallel auf dem Server (z. B. ein 2er-Spiel auf der Xbox, ein 3er-Spiel per Handy). Ein Gerät sitzt in einem Raum.
- Ressourcen: Gold pro Spieler (wie K2C), Baumaterial (Holz/Stein/Kupfer) gemeinsam für den Hub. *So umgesetzt, im Playtest prüfen.*
- Jede Regel muss mit 2+ Spielern gleichzeitig funktionieren, egal ob sie auf einem oder mehreren Geräten sitzen.
- **Die Wellen wachsen mit der Spieleranzahl** (Gegnerzahl ×(1 + 0,5 je Zusatzspieler)), siehe `rules/wirtschaft.md`.
- **Schwierigkeitsgrade** Dev, Leicht, Normal, Hart, Ultra verändern Wellen und Gegner, nicht die Wirtschaft. Live wählt man den Grad beim Anlegen des Raums, im Dev-Mode lässt er sich im Debug-Panel jederzeit umschalten (wirkt ab der nächsten Welle). Details und Faktoren: `rules/wirtschaft.md`.

## Monarch

- Bewegung nur **horizontal** (links/rechts), Sprint, **kein Springen**.
- Interagieren (Münze geben, Truhe öffnen, Eingang nutzen; A halten), Bau-Menü, Skill-Menü, **Schlag** (X) und vier Skill-Slots.
- Hauptrolle ist Truppen-Management. Im Kampf unterstützt er per Skill und mit einem einfachen Schlag, trägt aber nicht den Hauptschaden.
- Werte: HP 100, Speed 5 Units/s, Verteidigung 5, Schlag 10 Schaden. **Kein Level**, der Monarch wächst nur über Skills.
- Tod: Der gefallene Monarch bleibt liegen; ein Mitspieler belebt ihn durch A halten (3 s, 50 % HP), sonst Respawn an der Burg nach 15 s, ohne Strafe, Truppen bleiben. Details: `rules/monarch.md`.

### Skill-System (offene Archetypen-Linien)

Jeder Spieler hat einen **eigenen Skill-Baum** mit 4 Linien: **Tank, Zauberer, Heiler, Dieb** (zuerst Tank, Zauberer, Heiler). Punkte werden frei verteilt, hybride Builds sind erlaubt. Die Presets aus `monarch.json` sind nur Startverteilungen. Respec kostenlos an der Burg jedes Hubs, nur am Tag.

- Skill-Punkte kommen aus einem **gemeinsamen Fund-Pool** je Insel (versteckt in der Welt 3–5 je Stufe, Minibosse, Endboss, Meilensteine des Hub-Ausbaus, jede 3. Truhe); jeder Spieler verteilt die gefundenen Punkte für sich.
- Tier-Gating: Tier 2 ab 5, Tier 3 ab 10, Tier 4 (Ultimate) ab 15 Punkten in der Linie.
- 4 Skill-Slots: LB, RB, LT, D-Pad hoch (Tastatur Q, R, T, Z), Skill-Menü D-Pad runter (K).

| Linie | Tier 1 (aktiv) | Tier 3 | Tier 4 (Ultimate) | Passive (Auswahl) |
|---|---|---|---|---|
| Tank | Taunt (10u, CD 15s), Shield Bash (Stun 2s, CD 10s) | Iron Wall (Barriere 500 HP, CD 30s) | Last Stand (überlebt mit 1 HP, CD 120s) | Armor Aura, Thick Skin, Regeneration, Guardian, Fortified |
| Zauberer | Fireball (30 AoE 3u, CD 8s), Ice Wall (Slow 50%, CD 20s) | Lightning Storm (10/s, 5s, CD 40s) | Meteor (200 AoE 15u, CD 120s) | Arcane Power, Spell Echo, Frost Armor, Elemental Mastery |
| Heiler | Heal (50 HP, CD 10s), Group Heal (30 HP 10u, CD 30s) | Divine Shield (100 HP Schild, CD 20s) | Resurrection (Truppen 50% HP, CD 180s) | Healing Aura, Speed-/Damage-Blessing, Holy Ground |
| Dieb | Backstab (40 / 80 von hinten, CD 12s), Smoke Bomb (unsichtbar 5s, CD 25s) | Poison Blade (10/s, 5s, CD 15s) | Shadow Strike (Teleport + 150, CD 90s) | Treasure Hunter, Swift, Critical Strike, Evasion, Resource Master, Shadow Step |

Die Skills kommen **erst nach dem Vertical Slice**.

## Truppen (`data/troops.json`)

- **Landstreicher** im Rekrutierungs-Camp → Münze geben → **Bauer** (folgt, sammelt, baut).
- Werkstatt: Bauer + Bogen → **Bogenschütze** (Fernkampf, besetzt Türme). Bauer + Schwert → **Krieger** (Nahkampf, Frontlinie).
- Elite-Upgrades in der Schmiede (Stein/Kupfer), Rüstung in der Rüstkammer (Eisen): +50% HP und Schaden, +20% Angriffstempo bzw. +20 % HP je Stufe. Bürger haben **kein Level und keine Skills**, nur Upgrades und **Berufe** (Bergmann, Baumeister, Handwerker; Händler kommt zu Besuch).
- Truppen kämpfen automatisch (KI), Bauern fliehen bei Gefahr. **Truppen-Limit je Hub:** Basis 10, Kaserne +10, es zählen nur Kämpfer. Heilung nur am Heilplatz. Details: `rules/buerger.md`. *Krieger, Elite-Upgrades, Kaserne, Limit und Berufe sind beschlossen, aber noch nicht im Code.*

## Gebäude (`data/buildings.json`)

Der Hub hat **Ausbaustufen 1 bis 5** (Holz, Stein, Kupfer, Eisen, Kristall); jede schaltet Mauer- und Turm-Stufe und neue Gebäude frei. Gebäude: Burg/Thron (Hub-Kern, Basislager), Mauer und Turm (Stufen 1–5, Turm Stufe 5 = Zaubertum), Tor, Werkstatt, Farm, Kaserne, Lager, Taverne, Heilplatz, Schmiede, Rüstkammer, Treppe hoch/runter (je 1 pro Hub). Plätze sind fest je Gebäude (Daten), gebaut wird von Bauern; Material liegt im Insel-Vorrat mit Maximum (Burg + Lager), Arbeiter bringen es zum Lager; Holz wächst über Farm-Plantagen nach, Stein bis Kristall kommen unendlich aus Adern mit begrenzter Abbaurate. Liste, Kosten, HP und Wirkung: `rules/materialien-gebaeude.md`. **Startvorrat:** Eine neue Insel startet mit 100 Holz (`data/hub.json` › `islandStartStock`, B-177). Werte sind Startwerte (B-015).

## Welt & Stufen

**Aufbau:** Ein Raum hat einen Spielstand, der Spielstand hat **n Inseln**, jede Insel hat **n Stufen** (Level). Die Stufen einer Insel sind **pro Spieler frei begehbar** (jeder wechselt allein über Tiefen-Eingang oder Treppe) und laufen alle weiter, auch ohne Spieler. Der **Endboss** der tiefsten Stufe macht den Weg zur nächsten Insel frei (gemeinsamer Wechsel); je Stufe gibt es einen **Miniboss**. Das Baumaterial gehört der Insel (alle Stufen teilen einen Vorrat). Die Inseln folgen klassisch als Insel 1 bis n; später sind mehrere Inseln je Ebene wählbar (nächste Ebene nach k besiegten Inseln). Die Stufen werden **nach unten schmaler, dafür dichter**. Erste Ausbaustufe: 1 Insel mit 5 Stufen; die drei Stufen unten kommen zuerst, Eisenstollen (Tiefe 3) und Kristallhöhle (Tiefe 4) folgen. Details: `rules/stufen.md`, Entscheidung `decisions/003-spielstruktur-inseln-stufen.md`.

| Stufe | Biom | Länge (Units) | Primär-Ressource | Zyklus | Gegner |
|---|---|---|---|---|---|
| 0 | Oberwelt (Wald) | 900–1100 | Holz | Tag 10 min / Nacht 5 min, 1 min Dämmerung | Greed, Goblin, Goblin Archer (Elite), Wolf (nachts) |
| 1 | Höhle | 700–900 | Stein | Aggressionspool | Fledermaus, Höhlentroll (Elite), Skelett (nachts) |
| 2 | Mine | 550–700 | Kupfer | Aggressionspool | Zombie, Rattenschwarm, Minengeist (Elite) |

Gold gibt es überall (Truhen, Gegner-Drops). Post-MVP: Tiefe 3 (Eisen, Lava), Tiefe 4 (Kristall).

- **Jede Stufe hat einen eigenen Hub**, der von Grund auf neu gebaut wird. Alle Hubs bleiben bestehen.
- **Tiefen-Eingang** am Ende der Stufe führt nach unten. Zurück geht es nur über gebaute **Treppen**. Jeder Spieler wechselt für sich.
- **Aggressionspool** (unter Tage statt Tag/Nacht): +1%/min, +5% pro Kill, +1% pro gesammelter Ressource.
  Bei 100% folgt ein Portal-Angriff, danach Reset auf 0. So bestimmen die Spieler das Tempo selbst.
- Globaler Tag/Nacht-Zyklus läuft auch unter Tage weiter (Skelette nachts; Events: jede 7. Nacht Vollmond mit Alpha-Wolf, jede 13. Blutmond, siehe `rules/bosse.md`).

### Prozedurale Generierung (Pflicht-Feature)

Die festen Eckdaten stehen in `data/biomes/<biom>.json`, der Generator in `engine/level/` (Go, deterministisch über `engine/rng`).

```text
[Rand|Ausgang] … [Portal] … [Chunks] [ HUB ] [Chunks] … [Portal] … [Ausgang|Rand]
```

- Welt = Reihe von **Chunks** (50 Units breit). Der Hub liegt immer in der Mitte.
- Deterministisch per **Seed**: gleicher Seed ergibt das gleiche Level. Gespeichert wird nur der Seed, nicht das Level.
- Reihenfolge: Hub, Ausgang und Rand festlegen → Portale (Mindestabstand zum Hub, abwechselnd links/rechts)
  → Event-Chunks (Truhe, Rekrutierungs-Camp nahe am Hub) → restliche Chunks gewichtet aus `chunkWeights`
  → Ressourcen pro Chunk-Typ aus `resourcesPerChunk` → versteckte Skill-Punkte.
- `validateLevel()` prüft die Spielbarkeits-Regeln. Die Tests jagen jede Biom-Config durch 500 Seeds.
  **Jede Änderung an einer Biom-JSON muss `task check` bestehen.**
- Später möglich: Chunk-Vorlagen mit Untervarianten, Gegner-Camps in der Welt, Biom-spezifische Hazards.

## Gegner (`data/enemies.json`)

- Laufen geradeaus auf den Hub zu, **keine Sprünge**. Zustände: Idle → Move → Attack (→ Flee).
- Elite-Gegner: je eine Fähigkeit: Kiting (Fernkampf hält Abstand), Flächenschlag (`aoe`), Phasenwechsel (`phases`).
- Eigenschaften: `stealsGold` (Greed klaut Gold bei Kontakt mit dem Monarch), `ignoresWalls` (Fledermaus, Geist),
  `prefersBuildings`/`prefersTowers`/`prefersTroops`/`prefersMonarch`, `swarm`, `aoe`, `phases`, `fleesAtHalfHp`. Wolf-Rudel und Holzdiebstahl entfallen.
- Pools je Stufe (2 Standard + 1 Elite): Wald, Höhle, Mine wie bisher; **Eisenstollen:** Lavaschleim, Eisenkäfer, Feuergeist; **Kristallhöhle:** Kristallspinne, Splitterwicht, Kristallwächter (`rules/gegner.md`).
- Wellen: 1–5 → 5–10 Standard. 6–10 → 10–15 Standard + 1–2 Elite. Ab 11 → 15–20 Standard + 3–5 Elite; dazu Wellenfaktor je Spieleranzahl und Schwierigkeitsgrad. Eine Welle je Nacht (Oberwelt) bzw. je 100 % Aggressionspool, 2 Portale je Stufe (ab Tiefe 3 drei), Wellenzähler je Stufe. Das Tor blockiert Gegner wie die Mauer.
- Skalierung je Tiefe und je Insel (eigene Tabelle je Insel, Startwerte HP +50 %, Schaden +30 %, Tempo +10 % je Tiefe).
- **Bosse:** je Stufe ein Miniboss (kommt mit Welle 5 im Wald, Welle 3 unten), je Insel ein Endboss in seinem Bau in der tiefsten Stufe (wartet, wird bei Ankunft ausgelöst); Gold, Material und Skill-Punkte als Belohnung, sie kehren nie zurück. Der Sieg über den Endboss öffnet die nächste Insel. Details: `rules/bosse.md`.
- **Events:** Vollmond (jede 7. Nacht), Blutmond (jede 13.), Händler-Überfall (jeder 4. Besuch), siehe `rules/bosse.md`.
- Drops: Gold je Gegnerart wie in `data/enemies.json` (Standard etwa 3–25, Elite etwa 15–80), 10% Chance auf Stufen-Ressource.

## Niederlage und Ziel

- **Burg einer Stufe zerstört:** Der **Niederlage-Modus** des Raums entscheidet: *Gold/Material-Verlust* (je −50 %), *Stufenverlust* (Hub der Stufe zurückgesetzt) oder *Komplett verloren* (Game Over). Standard je Schwierigkeitsgrad: Dev/Leicht Gold/Material, Normal/Hart Stufenverlust, Ultra komplett verloren.
- **Monarch tot:** Respawn am Hub, keine Strafe.
- **Ziel:** Standard ist der Endboss der letzten Insel; Varianten als Raum-Option: Gold sammeln, N Tage überleben, alles abbauen, alles ausbauen. Details: `rules/stufen.md`.
- **Raum-Optionen:** Schwierigkeitsgrad, Ziel und Niederlage-Modus werden beim Anlegen des Raums gewählt.

## Steuerung

| Aktion | Controller (Xbox) | Tastatur |
|---|---|---|
| Laufen | Linker Stick / D-Pad ←→ | A/D, ←/→ |
| Sprint | RT | Shift |
| Beitreten; halten = Münzen geben/fallen lassen | A | Leertaste |
| Interagieren | A (halten) | Leertaste (halten) |
| Schlag | X | E |
| Skills 1–4 | LB, RB, LT, D-Pad hoch | Q, R, T, Z |
| Wiederbeleben (neben einem gefallenen Mitspieler) | A halten (3 s) | Leertaste halten |
| Bau-Menü | Y | B |
| Skill-Menü | D-Pad runter (nicht View, da View + Menu reserviert ist) | K |
| Pause (Regel: [`rules/bedienung.md`](rules/bedienung.md) § 1) | Menu (kurz, < 600 ms) | Esc |
| Vollbild | RS (Stick drücken) | F |

**B bleibt unbelegt**, weil Edge auf der Xbox B vermutlich als „Zurück“ nutzt. Der Gamepad-Test klärt das.

## UI

- Minimalistisch wie K2C: möglichst viel in der Welt anzeigen (Münzen über dem Kopf, Baupreise als Münz-Slots). **Gültige Aktionen erscheinen überall in der Welt als Overlay am Ort** (Taste und Aktion, passend zum benutzten Gerät), wie die Preise an den Gebäuden (`rules/monarch.md` § 4).
- HUD pro Split-Screen-Hälfte: HP, Gold, Ressourcen, Tag/Nacht bzw. Aggressionspool, Skill-Slots mit Cooldown.
- Meldungen in der Mitte: „Nacht naht!“, „Portal öffnet sich!“, „Skill-Punkt gefunden!“.
- Mindestgröße für Texte auf dem TV (Couch-Abstand!) und Kontrast ≥ 4,5 : 1 je Layout: [`rules/bedienung.md`](rules/bedienung.md) § 2 (≥ 28 px Vollbild, ≥ 24 px im Viertel). Verbindungsverlust und Latenz-Ziel: § 3.
- Sprache: Deutsch und Englisch mit Auswahl in den Optionen, neue Texte nur zentral ([`rules/bedienung.md`](rules/bedienung.md) § 4, B-172).

## Speichern

JSON mit Monarch-Zustand (Level, Skills), pro Hub (Gebäude, Truppen, Ressourcen), Fortschritt (freigeschaltete Tiefen)
und den **Seeds** der Stufen. Gegner und Level-Layout werden nicht gespeichert. Speicherort ist der Heimnetz-Server
(Browser-Speicher auf der Xbox gilt als unzuverlässig), mit Fallback auf localStorage.
Umgesetzt in `engine/sim/campaign.go` und `engine/store/`: Autosave bei Tagesanbruch und beim Stufenwechsel. Zusätzlich gespeichert wird,
was aus der Welt schon entfernt wurde (gefällte Bäume, geöffnete Truhen), sonst kämen sie beim Laden zurück.

## Grafik & Audio

- Figuren (Monarchen, Truppen, Gegner): Seitenansicht-Sprites von **LuizMelo (CC0)** für unsere Seite und
  **Gothicvania von ansimuz (CC0)** für die Gegner (Wolf, Skelett, Zombie, Geist, Fledermaus, Höhlentroll). Zuordnung in
  `data/sprites.json`, Bilder und Credits in `public/sprites/`. Referenz: Testseiten `aufstellung.html` (Rollen) und
  `figuren.html` (alle Figuren, auch ungenutzte).
- Reittiere: 13 Tiere (LPC-Pferde, Einhorn, Pegasus, Elefant, Hirsch, Wölfe, Gothicvania-Tiere) in `sprites.json` → `mounts`.
  Der Reiter ist der Oberkörper des Monarchen, auf den Sattelpunkt gesetzt. LPC-Tiere sind CC-BY 3.0 (Credits Pflicht). Gebäude, Ressourcen und Hintergrund sind noch
  **Platzhalter-Formen**. Ziel: 2D-Pixel-Art mit Parallax-Ebenen.
- Paletten pro Biom stehen in der Biom-JSON (Wald grün/braun, Höhle grau/orange, Mine braun/kupfer).
- Audio: Kenney Audio / freesound.org (CC).

## Offene Fragen

- 3–4 Spieler: entschieden (2×2-Raster bzw. zwei oben und einer breit unten, `src/scenes/layout.ts`); Schriftgrößen je Layout in [`rules/bedienung.md`](rules/bedienung.md) § 2.
- Skill-Tasten am Controller (hängt vom Gamepad-Test ab).
- Eine Klasse pro Spieler als Preset, damit sich die Rollen im Koop ergänzen?
