# B-464 · Eine Gegner-Seite im Asset-Studio zeigt alle Gegnertypen und Bosse mit ihren Vorschauen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ11
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

17 Gegnertypen in `data/enemies.json` (`greed`, `wolf`, `alphaWolf`, `goblin`, `goblinArcher`, `skeleton`, `bat`, `caveTroll`, `zombie`, `ratSwarm`, `mineGhost`, `lavaSlime`, `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling`, `crystalGuardian`) mit Grafik aus `data/sprites.json` › `enemies`: 8 Sheets, teils mit `tint`, `scale`, `alpha` oder `y`; sechs Typen sind Platzhalter (B-326, z. B. `lavaSlime` = `mushroom` rot getönt). Die Sheets haben nur `idle`, `run` und `attack`; `ghost` hat kein `run`, `cemetery-skeleton` und `hell-gato` haben ein einziges `idle`-Bild.

Biome (`data/biomes/*.json` › `enemies`, je `portal` und `night`): `forest` (Tiefe 0), `cave` (1), `mine` (2), `ironhold` (3), `crystal` (4). `alphaWolf` steht in keinem Biom, er kommt über das Event `fullMoon` (`data/events.json`).

Bosse: `data/bosses.json` › `bosses` je Tiefe 0–4 (`goblinLeader`, `trollKing`, `ratQueen`, `lavaGolem`, `shardTitan`, Fähigkeiten `summon`, `aoe`, `flameTrail`, `shardThrow`) und `endBosses` (`crystalHeartWarden`, Tiefe 4, 3 Phasen). Boss-IDs haben keinen Eintrag in `sprites.json` › `enemies`; das Spiel zeichnet sie mit dem Rückfall `goblin` (`src/scenes/worldRenderer.ts`). Phase und Warnkreis liegen im Zustand (`enemies[].phase`, `enemies[].warn`, `docs/protocol.md` › Kampf), gezeichnet werden Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner erst mit K5.1 (B-132, CLI, geplant). `figuren.html`/`aufstellung.html` zeigen Gegner nur statisch im Wechsel auf Canvas.

## Ziel

Eine Gegner-Seite im Asset-Studio (B-459) zeigt alle Gegnertypen nach Biom und alle Bosse je Tiefe mit den Animationen und Effekten des Spiels: laufen, angreifen, getroffen, sterben, Merkmale; bei Bossen dazu Name, Art, Fähigkeit und Phasen. Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner zeichnet das Spiel erst mit K5.1 (B-132); die Seite zeigt sie bis dahin als sichtbare Lücke, die Anbindung übernimmt das Folge-Ticket B-469. Alles erscheint im Spielmaßstab, mit zuschaltbarem Monarch-Vergleich und Grafik-Auswahl aus dem Slot-Register. Fehlendes ist als Lücke sichtbar.

## Beteiligte und Zielgruppen

🧑 prüft Gegner, Bosse und Lücken im Asset-Studio am PC und entscheidet über Grafik (Projekt GRA, echte Figuren B-329) und Anzeigen im Spiel (CLI, K5). Agenten setzen in DEV um, ein getrennter Reviewer prüft. Spieler sind nicht betroffen (keine Spieler-Seite verweist auf die bisherigen Referenzseiten).

## Anforderungen

- **Gleicher Aufbau:** dieselben Abspiel-Bausteine wie die Monarch-Seite (B-463: Steuerung, Maßstab, Lücken-Markierung) auf der Bühne aus B-460/WZ7.3 (dort entsteht die Phaser-Instanz mit Szene), kein zweiter Abspieler und keine eigene Phaser-Abhängigkeit.
- **Spiel-Animationen:** Gegner über `makeSprite` und `playAnim` aus `src/scenes/sprites.ts` (mit `tint`, `scale`, `alpha`, `y`), Effekte `hit` und `kill` aus `EFFECT_CONFIG` (`src/scenes/effects.ts`), Import über `@game`/`@data`.
- **Gruppen:** je Biom in Reihenfolge der Tiefe, darin `portal` vor `night`; ein Typ in mehreren Biomen erscheint in jedem; Typen ohne Biom in der Gruppe „ohne Biom“. Bosse als eigene Gruppe je Tiefe 0–4, Endbosse in ihrer Tiefe.
- **Merkmale:** `traits` und die Zusatzfelder `swarmSize`, `aoe`, `phases`, `kiteDistance` als Text an der Karte; ohne Darstellung im Spiel als Lücke.
- **Bosse:** Name, Art (Mini-/Endboss), Fähigkeit, Phasen mit Schwelle `belowHpPercent` als Text an der Karte; Grafik so, wie das Spiel sie zeichnet.
- **Boss-Effekte als Lücke:** Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner zeichnet das Spiel erst mit K5.1 (B-132). Die Seite markiert sie bis dahin je Boss nur dort, wo das Spiel sie für diesen Boss zeichnen wird, mit der Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“ und zeichnet keinen Ersatz:
  - **Warnkreis** nur bei Bossen mit Flächenschlag (Fähigkeit `aoe`: `trollKing`, `crystalHeartWarden` ab Phase 2); Bosse ohne Flächenschlag (z. B. `goblinLeader` mit `summon`) haben keine Warnkreis-Lücke.
  - **Phasen-Text** nur beim Endboss mit Phasen (`crystalHeartWarden`); Minibosse haben keine Phasen.
  - **Boss-Leiste** und **Event-Banner** bei jedem Boss (Event-Banner bei Minibossen für `bossSpawned` und `bossDefeated`, beim Endboss zusätzlich für `bossPhase`).
  - Die Anbindung an den Zeichencode des Spiels ist B-469.
- **Monarch-Vergleich:** zuschaltbar, Monarch (beritten, auf `MONARCH.mount.sprite`) auf derselben Bodenlinie.
- **Grafik-Auswahl:** Zuordnungen der Slots je Gegner-ID und `boss` aus `data/assets.json` als Vorschau wählbar, ohne das Register zu ändern.
- **Qualität:** reine Logik (Gruppen, Lücken, Bosse) mit Vitest-Test im Studio-Frontend; keine Zufallswerte; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

- Neue Gegner, Bosse oder Grafiken (Projekt GRA, B-329); Werte, Wellen, Fähigkeiten (REG/SIM); Änderungen an `data/enemies.json`, `data/bosses.json`, `data/biomes/*.json`, `data/sprites.json`, `data/assets.json` (auch keine eigenen Slots je Boss).
- Boss-Kampfablauf: Beschwörung, Flammenspur oder Splitter-Wurf laufen nicht als Simulation ab.
- Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner zeigen (statt als Lücke markieren): Folge-Ticket **B-469**, abhängig von K5.1 (B-132) und WZ11. Ebenso Beispielzustände für Boss, Phase und Warnkreis.
- Zeichnen dieser Anzeigen im Spiel (K5.1, B-132, CLI); Änderungen an `src/scenes/`, `src/model/`, `src/online/`.
- Phaser oder die Bühne einführen (B-460, WZ7.3).
- Monarch-Vorschauen (B-463); Truppen; Entfernen von `figuren.html`/`aufstellung.html` (B-463/AC-13).

## Regeln und Einschränkungen

- `B-463 › Regeln und Einschränkungen` gilt: Domäne DEV (`tools/k3c-studio/`, `src/tools/`), Spiel-Code nur lesend über `@game`, Grenzfall DEV beim Entfernen der Referenzseiten, Firewall, Fensterprüfung nur mit Freigabe von 🧑, Komplexitäts-Budget.
- **Abhängigkeiten:** WZ6 erledigt; Phaser und Bühne aus B-460 (WZ7.3); Abspiel-Bausteine aus B-463 (WZ10.1). K5.1 (B-132) ist keine Abhängigkeit: Läuft WZ11 vor K5 (WZG hat Rang 1, KMP Rang 10), zeigt die Seite die Boss-Effekte als Lücke (AC-12). Die Anbindung an den Zeichencode aus K5.1 (`src/scenes/kampfAnzeige.ts`, `kampfView.ts`, Namen laut K5.1) macht B-469.
- Der Client rechnet nichts (`src/scenes/noSim.test.ts`).
- **Begriffe** nach `docs/glossar.md` (Biom, Tiefe, Miniboss, Endboss). Neue Begriffe (Gegner-Seite, Warnkreis, Boss-Leiste, Event-Banner, Lücke) kommen vor der Freigabe ins Glossar.

## Beispiele

- Gruppe Wald → `greed`, `goblin`, `goblinArcher` (Portal), danach `wolf` (Nacht); `greed` erscheint als Sheet `goblin`, violett getönt (`#c77dff`).
- Gruppe „ohne Biom“ → `alphaWolf` (Sheet `hell-hound`, getönt `#e0e0ff`).
- `mineGhost` › Laufen → Sheet `ghost` hat kein `run`, die Vorschau zeigt `idle` wie das Spiel und „Lücke: run fehlt“; Merkmal `phases`.
- `bat` › Stehen → Figur 130 px über dem Boden (`y`), wie im Spiel.
- Bosse › Tiefe 0 → Goblin-Anführer (Miniboss, `summon`, kein Flächenschlag): Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“ nur für Boss-Leiste und Event-Banner, keine Warnkreis-Lücke, keine Phasen.
- Bosse › Tiefe 4 → Splitter-Titan (Miniboss, `shardThrow`) und Kristallherz-Wächter (Endboss, 3 Phasen mit Schwelle `belowHpPercent` als Text); Grafik wie im Spiel (Rückfall `goblin`) mit der Lücke „keine eigene Boss-Grafik“. Splitter-Titan: Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“ für Boss-Leiste und Event-Banner, kein Warnkreis, kein Phasen-Text. Kristallherz-Wächter: dieselbe Lücke für Warnkreis (ab Phase 2), Boss-Leiste, Phasen-Text und Event-Banner. Nichts davon wird gezeichnet.
- Monarch-Vergleich an → Monarch 0 beritten auf `horse` neben dem Höhlentroll (`hell-gato`, `scale` 1,3) auf derselben Bodenlinie.

## Ausnahme- und Fehlerfälle

- Gegner- oder Boss-ID ohne Eintrag in `sprites.json` › `enemies` → Rückfall `goblin` wie im Spiel, Lücke „keine eigene Grafik“.
- Biom nennt einen Gegner, den `data/enemies.json` nicht kennt → Karte mit Lücke „Gegner fehlt in data/enemies.json“, kein Absturz.
- Boss-Effekte (Warnkreis, Boss-Leiste, Phasen-Text, Event-Banner) → an jedem Boss, für den der Effekt im Spiel vorgesehen ist (Warnkreis nur bei Flächenschlag, Phasen-Text nur beim Endboss mit Phasen, Boss-Leiste und Event-Banner bei jedem Boss), die Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“, kein eigener Ersatz; für nicht zutreffende Effekte (z. B. Warnkreis beim `goblinLeader`) gibt es keine Lücke. Die Seite versucht nicht, den Zeichencode aus K5.1 zu laden (das macht B-469).
- Endboss-Phase ohne `ability` → die Phasen-Karte nennt die Fähigkeit der vorigen Phase (Regel aus `data/bosses.json` › `notes`).
- Figuren-Atlas fehlt, Slot ohne Zuordnung, `clientLog` ohne Spielserver → wie `B-463 › Ausnahme- und Fehlerfälle`.
- Bühne aus WZ7.3 nicht vorhanden → wie `B-463 › Ausnahme- und Fehlerfälle` (Session blockiert).

## Akzeptanzkriterien

- **AC-01** Die Gegner-Seite des Asset-Studios listet alle 17 Gegnertypen aus `data/enemies.json`; ein Vitest-Test prüft, dass jede ID mindestens einmal vorkommt.
- **AC-02** Die Gruppen folgen den Biomen aus `data/biomes/*.json` › `enemies` in Reihenfolge der Tiefe (`forest`, `cave`, `mine`, `ironhold`, `crystal`), darin `portal` vor `night`; Typen ohne Biom (heute `alphaWolf`) stehen in der Gruppe „ohne Biom“ (Vitest-Test).
- **AC-03** Jede Animation des Gegner-Sheets (`idle`, `run`, `attack`, soweit vorhanden) ist einzeln abspielbar mit den Steuerungen der Monarch-Seite; `tint`, `scale`, `alpha` und `y` aus `sprites.json` › `enemies` wirken wie im Spiel (`makeSprite`).
- **AC-04** Fehlt einem Sheet eine Animation (heute `ghost` ohne `run`), zeigt die Vorschau wie `playAnim` `idle` und markiert die Lücke (Vitest-Test).
- **AC-05** Getroffen zeigt den Effekt `hit`, Sterben den Effekt `kill` aus `EFFECT_CONFIG`; die fehlenden eigenen Treffer- und Todesanimationen sind als Lücke markiert.
- **AC-06** Merkmale aus `traits` und die Zusatzfelder `swarmSize`, `aoe`, `phases`, `kiteDistance` stehen an der Karte; ein Merkmal ohne Darstellung im Spiel trägt die Markierung „Lücke“.
- **AC-07** Die Bosse bilden eine eigene Gruppe je Tiefe 0–4 mit allen 5 Einträgen aus `data/bosses.json` › `bosses` und `crystalHeartWarden` aus `endBosses` in Tiefe 4, je mit Name, Art, Fähigkeit und Phasen (mit Schwelle `belowHpPercent` als Text); ein Vitest-Test zählt 6 Bosse.
- **AC-08** Jeder Boss erscheint mit der Grafik, die das Spiel zeichnet (heute Rückfall `goblin`), und der Lücke „keine eigene Boss-Grafik“, solange `sprites.json` › `enemies` keinen Eintrag für seine ID hat.
- **AC-09** Verschoben nach B-469 (dort AC-01): Warnkreis mit Radius `ability.radius` aus dem Zeichencode des Spiels. In B-464 und WZ11 nicht zu erfüllen; die Lücke deckt AC-12 ab.
- **AC-10** Verschoben nach B-469 (dort AC-02 und AC-03): Boss-Leiste und einzeln wählbare Endboss-Phasen mit Phasen-Text. In B-464 und WZ11 nicht zu erfüllen; die Lücke deckt AC-12 ab.
- **AC-11** Verschoben nach B-469 (dort AC-04): Event-Banner für `bossSpawned`, `bossPhase` und `bossDefeated`. In B-464 und WZ11 nicht zu erfüllen; die Lücke deckt AC-12 ab.
- **AC-12** Für jeden Boss zeigt die Gegner-Seite für die zutreffenden der vier Effekte Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner die Lücke „Anzeige im Spiel fehlt (B-132, Anbindung B-469)“ an der Boss-Karte: Warnkreis nur bei Bossen mit Flächenschlag (`aoe`: `trollKing`, `crystalHeartWarden`), Phasen-Text nur beim Endboss mit Phasen (`crystalHeartWarden`), Boss-Leiste und Event-Banner bei jedem der 6 Bosse; das Studio zeichnet keinen eigenen Ersatz. Nachweis: Vitest-Test der Lücken-Liste (je Boss nur die zutreffenden Einträge, zusammen 15: 6 Boss-Leisten, 6 Event-Banner, 2 Warnkreise, 1 Phasen-Text; kein Warnkreis und kein Phasen-Text beim `goblinLeader`), Review des Diffs (kein eigener Zeichencode für diese Anzeigen).
- **AC-13** Größe und Maßstab entsprechen dem Spiel (`UNIT_PX`, `scale`); ein zuschaltbarer Vergleich stellt den Monarchen (Index 0, beritten auf `MONARCH.mount.sprite`) auf derselben Bodenlinie neben jeden Gegner und Boss.
- **AC-14** Die Gegner-Seite nutzt dieselben Abspiel-Bausteine wie die Monarch-Seite (B-463) und die Bühne aus B-460/WZ7.3; Bühne, Steuerung und Lücken-Markierung gibt es im Studio-Frontend je nur einmal (Review).
- **AC-15** Für die Slots je Gegner-ID und `boss` aus `data/assets.json` ist jede Zuordnung als Vorschau wählbar, Präferenz 1 ist voreingestellt; die Auswahl ändert `data/assets.json` nicht.
- **AC-16** Die reine Logik der Gegner-Seite (Gruppen, Lücken, Bosse) hat Vitest-Tests im Studio-Frontend, die in `task check:all` und in der CI laufen.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat):

1. **Bosse:** Name, Art, Fähigkeit und Phasen als Text; Warnkreis, Event-Banner, Phasen-Anzeige und Boss-Leiste sind keine Aufgabe dieses Tickets (siehe 4).
2. **Gruppierung:** Gegner nach Biom (`data/biomes/*.json` › `enemies`), Bosse als eigene Gruppe je Tiefe einschließlich `endBosses`.
3. **Technik und Ersatz:** wie `B-463 › Offene Fragen` 2–3 (Phaser mit den Animationsdefinitionen des Spiels über `@game`/`@data`, gemeinsame Abspiel-Bausteine; `figuren.html` und `aufstellung.html` werden entfernt).

Nachtrag, entschieden von 🧑 am 2026-10-17 (Chat):

4. **Boss-Effekte:** Warnkreis, Boss-Leiste, Phasen und Event-Banner zeigt WZ11 als Lücke; das Folge-Ticket **B-469** (nach K5.1/B-132) bindet sie an. Die früheren AC-09, AC-10 und AC-11 sind dorthin verschoben (hier als „verschoben“ vermerkt), AC-12 bleibt als Lücken-Kriterium. Damit ist die frühere offene Frage 4 (Reihenfolge zu K5) beantwortet.
5. **Phaser und Bühne:** B-460 führt beides mit WZ7.3 ein; die Gegner-Seite nutzt dieselbe Bühne (AC-14).

Offen: keine. Die frühere Frage 5 (Event-Banner nur für Boss-Ereignisse oder auch für Nacht-Events) ist nach `B-469 › Offene Fragen` 2 gewandert.

## Notizen

Das Slot-Register hat für Bosse nur einen Slot `boss` (ohne Zuordnung); eigene Slots je Boss wären eine Änderung des Registers (Nicht-Ziel, B-459). Sheets der Gegner (Stand 2026-10-17): `goblin` (idle 4, run 8, attack 8), `hell-hound` (6/5/6), `cemetery-skeleton` und `cemetery-skeleton-clothed` (1/8/8), `fire-skull` (8/8/8), `hell-gato` (1/4/4), `mushroom` (4/8/8), `ghost` (idle 7, attack 4). K5.1 nennt als geplante Dateien `src/scenes/kampfAnzeige.ts` (reine Funktionen) und `src/scenes/kampfView.ts` (Phaser); maßgeblich sind die Namen im Code nach K5 (relevant erst für B-469).

**Korrektur 2026-10-17 (nach Entscheidungen von 🧑, Spec weiter Entwurf):** Die Boss-Effekte (frühere AC-09, AC-10, AC-11) sind nach B-469 verschoben und hier als Lücke sichtbar (AC-12); Phaser und Bühne kommen aus B-460/WZ7.3 (AC-14); Beispielzustände für Boss, Phase und Warnkreis entfallen hier (B-469).
