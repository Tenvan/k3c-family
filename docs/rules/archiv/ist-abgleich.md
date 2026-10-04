# Ist-Abgleich Regelwerk I (Vorbereitung R1.2 und R1.3)

Stand: 2026-10-02 (Session R1.1). Vergleich von `docs/game-design.md` mit dem Go-Code (`engine/sim/`, `engine/level/`) und `data/*.json`.
Nur Tatsachen aus dem Code; was nicht gelesen oder getestet wurde, steht als **ungeprüft**. Beschlüsse trifft 🧑 in den Workshops,
dieses Dokument ändert keine Regel und keinen Wert.

> **Nachtrag R1.3 (2026-10-02):** Die Spielstruktur ist anders geplant als im Ist: **1 Spielstand → n Inseln → n Stufen je Insel, Stufen pro Spieler frei begehbar, alle Stufen laufen weiter** (`stufen.md`, Entscheidung `003`). Die Abschnitte 3 und 4 beschreiben den **Ist-Stand** (eine Stufe = eine Welt, gemeinsame Reise); sie gelten nicht als Soll.

## 1. Kern-Loop und Wirtschaft

| Regel in game-design.md | Ist im Go-Code / `data/` | Abweichung? |
|---|---|---|
| Gold pro Spieler, Baumaterial (Holz/Stein/Kupfer) gemeinsam für den Hub | `Player.Gold`, `World.Stock` (`engine/sim/types.go`) | nein |
| – (Beutel nicht erwähnt) | Startgold 100 je Spieler, Beutel maximal 100 (`data/economy.json` › `purse`) | **GDD schweigt** |
| Münzen geben: halten | `payIntervalSeconds` 0,25 s je Münze, Reichweite 2 Units; abgebrochenes Zahlen erstattet die Münzen (`economy.go`, `refundPending`) | nein |
| Fallen gelassene Münzen kann der andere Spieler aufheben | `pickupRangeUnits` 0,8, `dropPickupDelaySeconds` 1,5 (`economy.json`) | nein |
| – (Tageseinkommen nicht erwähnt) | `dawnGoldPerPlayer` 5 je lebendem Monarch bei Tagesanbruch (`payDawnIncome`, `cycle.go`) | **GDD schweigt** |
| Gold überall: Truhen, Gegner-Drops | Truhen 10–25 (`chestGold`); Drops je Gegner in `enemies.json` (`gold`), Landstreicher-Münze 1 Gold | siehe Gegner |
| Drops: Standard 5–15, Elite 20–50 Gold, 10 % Stufen-Ressource | je Gegnerart eigener Bereich (z. B. Wolf 3–5, Höhlentroll 50–80); Ressourcen-Drop 10 %, Menge 5 (`enemyResourceDrop`) | **ja** (Zahlen weichen ab, GDD nennt Klassenwerte) |
| Landstreicher → Münze → Bauer, Bauer sammelt und baut | Camps: max. 2 Landstreicher, Nachwuchs 25 s (`recruitCamp`); Sammeln: Baum 10 Holz/4 s, Fels 10 Stein/6 s, Kupfer 10/8 s, Markierung kostet 1–2 Münzen | nein |
| Werkstatt: Bauer + Bogen → Bogenschütze; Bauer + Schwert → Krieger | Werkstatt hält max. `bowRack` 3 Bögen (Bogen: Holz 50 + Gold 20); Warrior-Daten in `troops.json`, **kein Code für Krieger oder Schwerter** (`rg warrior engine/sim` ohne Treffer) | **ja** |
| Elite-Upgrades mit Stein/Kupfer (+50 % HP/Schaden, +20 % Tempo) | `eliteArcher`/`eliteWarrior` nur in `troops.json`, **ohne Code** | **ja** |
| Truppen-Limit hängt von den Kasernen ab | **kein Limit im Code**; Kaserne („+10 Truppen-Limit“) nur in `buildings.json` | **ja** |
| Gebäude: Burg, Mauer, Turm, Tor, Werkstatt, Farm, Kaserne, Treppen | Bauplätze nur: Mauer ×2, Turm ×2, Werkstatt, Treppe hoch (ab Tiefe 1), Treppe runter (wenn es tiefer geht) (`data/hub.json`). **Tor, Farm, Kaserne haben keinen Bauplatz** | **ja** |
| HP/Kosten außer Turm und Treppen sind Platzhalter | `buildings.json` unverändert; Balancing offen (B-015) | bekannt |
| Start | Startbesetzung des Hubs: 1 Bauer, 2 Bogenschützen (`hub.json` › `startTroops`, „vorläufiges Balancing“) | **GDD schweigt** |

## 2. Besitz und Koop (2–4+ Spieler)

| Regel in game-design.md | Ist | Abweichung? |
|---|---|---|
| Gold pro Spieler, Material gemeinsam; „im Playtest prüfen“ | wie oben; Münzen anderer Spieler aufheben möglich | offen (Playtest B-008) |
| Jeder Spieler eigener Monarch, Beitritt jederzeit | `AddPlayer`; Spieler links/rechts der Burg abwechselnd (`world.go`) | nein |
| Jede Regel gilt für 2+ Spieler | Wellenstärke (`waves.json`) hängt **nicht** von der Spieleranzahl ab; Tageseinkommen je Spieler; Reise verlangt alle | **Frage W1** |
| – | Ein „freier“ Monarch (`Player.Free`, B-059) zählt für die Reise nicht mit (`travel.go`) | GDD schweigt |

## 3. Tag/Nacht und Wellen

| Regel in game-design.md | Ist | Abweichung? |
|---|---|---|
| Wald: Tag 10 min, Nacht 5 min, 1 min Dämmerung | `biomes/forest.json` › `cycle` (10/5/1); Dämmerung zählt als eigene Phase vor der Nacht (`cycle.go`) | nein |
| Globaler Zyklus läuft auch unter Tage weiter | `cycleAt(globalDayNight, …)` (Zyklus des Waldes) | nein |
| Welle beginnt nachts | Oberwelt: eine Welle je Nacht (`startWave` bei Phasenwechsel zu `night`); morgens ziehen Gegner heim (`sendEnemiesHome`) | nein |
| Wellen 1–5: 5–10 Standard; 6–10: 10–15 + 1–2 Elite; ab 11: 15–20 + 3–5 Elite | `waves.json` › `table` identisch; Gegnerzahl ist Zufall in den Grenzen (`engine/rng`) | nein |
| Skalierung pro Stufe: HP +50 %, Schaden +30 %, Speed +10 % | `depthScaling` 1,5 / 1,3 / 1,1, **potenziert** je Tiefe (`enemies.go`, `math.Pow`) | Auslegung: „pro Stufe“ = multiplikativ je Tiefe (ungeprüft gegen Absicht) |
| Gegner laufen geradeaus auf den Hub zu | Zustände und Ziel-Präferenzen über Traits (`enemies.go`) | nein |
| Wolf bei Vollmond | **kein Mond im Code** (`rg -i mond\|moon` ohne Treffer); Wölfe kommen nachts | **ja** |
| Greed klaut Gold | `stealsGold`, `waves.stealGold` 5 Gold je Kontakt (`enemies.go`) | nein |
| Wellen unter Tage über den Aggressionspool | `aggressionPool`: +1 %/min, +5 % je Kill, +1 % je gesammelter Ressource (`biomes/cave.json`, `mine.json`; `cycle.go`, `waves.go`, `units.go`); bei 100 % Welle, Reset auf 0 | nein |

## 4. Stufen und Niederlage

| Regel in game-design.md | Ist | Abweichung? |
|---|---|---|
| Stufen: Wald 900–1100, Höhle 700–900, Mine 550–700 Units | `data/biomes/*.json` › `lengthUnits` | nein |
| Eigener Hub je Stufe, alle bleiben bestehen | `Campaign.worlds`, Hubs im Spielstand (`campaign.go`, `save.go`) | nein |
| Tiefen-Eingang am Ende, zurück nur über Treppen | `travelPoints`: Eingang (`exit`), gebaute Treppen (`travel.go`) | nein |
| Wechsel, wenn die Spieler am Eingang stehen | **alle lebenden, gesteuerten Monarchen** innerhalb 3 Units für 2 s (`hub.json` › `travel`); tote zählen nicht; ohne Entscheider kein Wechsel | **Frage W2** (Ausfall/AFK von Online-Geräten) |
| Hub-Kern zerstört: Respawn, Gebäude zerstört, 50 % der Ressourcen, alle Truppen verloren | `castleFallen` (`world.go`): Burg steht wieder, **alle Bauplätze zurück auf „unbezahlt“**, Material halbiert, **Gold je Spieler halbiert**, alle Truppen weg **außer Landstreichern**, Gegner/Wellen/Geschosse gelöscht; **die Startbesetzung kommt nicht zurück**; kein Game Over | **ja** (Gold, Landstreicher, Startbesetzung, kein Spielende nicht im GDD) |
| Monarch tot: Respawn am Hub, keine Strafe | Respawn nach `respawnSeconds` 5 s an der Burg; unterwegs bezahlte Münzen werden erstattet; Gold bleibt | nein |
| Monarch kämpft mit Skills, Stats Level 1/Level 20 | Basiswerte nur als Verteidigung beim Schaden (`applyDamage`); **kein Monarch-Angriff, kein Level, keine Skills im Code** | **ja** (Regelwerk II) |
| Siegbedingung | **keine** (`Campaign` hat kein Ende) | **Frage W3** (B-025) |

## 5. Steuerung

| Aktion | game-design.md | Ist (`src/input/playerInput.ts`) | Abweichung? |
|---|---|---|---|
| Laufen, Sprint, Beitreten/Münzen, Vollbild | wie Tabelle | A/D bzw. ←/→, Shift/RT, Leertaste/A, F/RS | nein |
| Interagieren | X / E | Taste existiert, **ohne Funktion** | **Frage W4** (B-021) |
| Bau-Menü / Skill-Menü / Pause | Y·B / View·K / Menu·Esc | als Aktionen definiert, **ohne Funktion**; View + Menu gemeinsam = zurück zur Landingpage (reserviert) | **ja**: View ist laut `CLAUDE.md` mit Menu reserviert |

## 6. Messgrößen (für den Balancing-Tester B-099)

Was sich headless aus `World` und `World.Events` ablesen lässt (`sim.CreateWorld`, `sim.Step`, `sim_run` in k3c-dev):

| Kennzahl | Quelle | Vorhanden? |
|---|---|---|
| Zeit, Tag, Phase | `World.Time`, `World.Cycle` | ja |
| Gold je Spieler, Material | `Player.Gold`, `World.Stock` | ja |
| Welle, Gegneranzahl, Wartende | `World.Wave`, `World.Enemies`, `World.SpawnQueue`; Ereignis `wave` (`count`) | ja |
| Bauplatz gebaut/zerstört, Zeitpunkt | Ereignisse `built`, `destroyed`; `Site.State` | ja (Zeit aus dem Tick) |
| Truppen je Art, Verluste | `World.Troops` (Art, HP) | Bestand ja, **Verlust-Ereignis nein** (nur Differenz) |
| Spieler gefallen | Ereignis `playerDown` | ja |
| Gold gestohlen | Ereignis `goldStolen` | ja |
| Burg gefallen (Niederlage) | Ereignis `castleFallen` | ja |
| Aggressionspool | `World.Aggression` | ja |
| Tageseinkommen, Truhen, Rekrutierung | Ereignisse `chest`, `recruited`, `gathered` | ja |
| Wirtschaftsfluss (Einnahmen/Ausgaben je Tag) | – | **nein** (Summen aus Ereignissen zu bilden; Ausgaben für Bauen sind nicht als Ereignis erfasst, ungeprüft) |
| Schaden an Burg/Gebäuden je Welle | – | **nein** (nur Ergebnis als HP) |
| Sieg | – | nicht definiert (W3) |
| Spielverhalten (Bots) | – | **nein**: `PlayerCommand` (`MoveX`, `Sprint`, `Pay`) ist die einzige Eingabe; Bots müssen Geben/Bauen über Position und Zahlen steuern |

Ziele für Zielkorridore (R1.2, R1.3) sollten Kennzahlen aus dieser Tabelle nutzen; fehlende Kennzahlen werden Teil des Tickets B-099.

## 7. Fragen für die Workshops

**R1.2 (Wirtschaft, Taste X)**

- **W1 · Wellenstärke und Spieleranzahl:** Die Welle hängt nicht von der Zahl der Spieler ab. Optionen: (a) bleibt so, (b) Gegnerzahl ×(1 + 0,5 je Zusatzspieler), (c) pro Spieler eine Stärkestufe. Auswirkung: Balance bei 2 gegen 4 Spielern.
- **Gold-Beutel (100) und Startgold (100):** bleiben, ändern oder Gold begrenzt ganz anders? Das GDD schweigt.
- **Tageseinkommen 5 Gold je Spieler:** Absicht oder Platzhalter?
- **Fehlende Truppen/Gebäude im Code** (Krieger, Elite, Tor, Farm, Kaserne, Truppen-Limit): vorerst streichen, später oder in R2 (SIM)?
- **Drops:** Klassenwerte aus dem GDD oder die Werte je Gegner aus `enemies.json`?
- **W4 · Taste X:** Optionen: (a) bleibt frei für Skills (Regelwerk II), (b) „Interagieren“ (Truhe, Camp, Eingang), (c) Bau-Menü hierher. Y ist heute Bau-Menü, View + Menu sind reserviert.

**R1.3 (Stufen, Niederlage, Ziel)**

- **W3 · Ziel und Siegbedingung:** Optionen: (a) tiefste Stufe halten (z. B. Tag N in Tiefe 2), (b) Endboss, (c) offen ohne Sieg. Kein Wert im Code, Entscheidung offen (B-025).
- **W2 · Reise-Regel mit Ausfall:** Tote Spieler zählen nicht, ein AFK-Gerät blockiert. Optionen: alle lebenden, Mehrheit, Zeitlimit.
- **Niederlage:** Gold halbiert, Landstreicher bleiben, Startbesetzung kommt nicht zurück, Burg steht sofort wieder (kein Game Over): als Regel beschließen oder ändern?
- **Wolf bei Vollmond:** streichen oder umsetzen?
- **Skalierung „pro Stufe“:** multiplikativ je Tiefe (wie im Code) oder anders?
- **Steuerung View:** „Skill-Menü“ kollidiert mit der reservierten Kombination View + Menu. Weiter so (View allein)?

## 8. Gliederung für `docs/rules/`

| Datei | Zweck | Sprint |
|---|---|---|
| `wirtschaft.md` | Gold, Material, Besitz im Koop, Tag/Nacht, Wellen, Taste X, je Regel mit Zielkorridor | R1.2 |
| `stufen.md` | Tiefen, Aggressionspool, Niederlage, Ziel der Kampagne, Reise im Koop, je Regel mit Zielkorridor | R1.3 |
| `monarch.md` | Monarch, Werte, Skills und Bindung an Tasten | Regelwerk II |
| `gegner-truppen.md` | Gegner-, Truppen- und Gebäudewerte, Elite, Truppen-Limit | Regelwerk III |
| `ist-abgleich.md` | dieses Dokument; wird nach R1.4 nicht fortgeschrieben | R1.1 |
