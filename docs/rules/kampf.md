# Regelwerk: Kampf (Angriff, Portale, Krone)

Beschlossen von 🧑 im Interview am 2026-11-15 (Kampfsystem nach Kingdom Two Crowns; Spec: B-926, Umsetzung B-927 bis B-934, Projekt KSY, Sprints KS1 bis KS5; Rahmen: [`monarch.md`](monarch.md), [`buerger.md`](buerger.md), [`gegner.md`](gegner.md), [`bosse.md`](bosse.md), [`stufen.md`](stufen.md), [`welten.md`](welten.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. **Alle Werte sind Platzhalter** (Startwerte „vorläufig“, `provisional`); das Feintuning folgt mit dem Balancing (B-099), solange es gesperrt ist, ohne Läufe. Jede Regel gilt für 2+ Spieler. Neue Begriffe stehen im [Glossar](../glossar.md) (Krone, Kronenraub, Portal-HP, Gruppe, Gruppenführer, Angriffs-Auslöser, Banner, Sichere Stufe, Kampfvariante).
Wo diese Datei einer älteren Regel widerspricht, gilt diese Datei; die anderen Regeldateien verweisen hierher.

## 1. Grundsatz

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Kein Tod, keine Lebenspunkte beim Monarchen.** Kämpfer verlieren bei Treffern die Waffe und werden Bürger (§ 4), der Monarch zahlt Treffer mit Gold und verliert zuletzt die Krone (§ 5). Spielende gibt es nur durch **Kronenraub** (§ 6). | Wie im Vorbild: Verlust hat Gewicht und lässt sich zurückholen. | – | – |
| **Aktiver Angriff statt reiner Verteidigung.** Truppen rücken aus und zerstören die **Portale** (§ 2); wie, bestimmt die **Kampfvariante** der Welt (§ 3). | Die Spieler entscheiden, wann sie angreifen; ohne Portale endet der Druck einer Stufe dauerhaft. | `data/worlds/*.json` › `combat` | – |

## 2. Portale

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Portale haben HP**, je Tiefe der Stufe ein Wert (Startwerte, vorläufig: Tiefe 0 = 500, 1 = 800, 2 = 1200, 3 = 1800, 4 = 2500). Heute sind Portale nur X-Koordinaten (`engine/sim/types.go`). | Ein Portal ist ein Ziel; tiefere Stufen verlangen mehr Einsatz. | `data/biomes/<biom>.json` › `portals.hp` (neben `portals.count`) | Zeit bis zum ersten zerstörten Portal je Stufe (Kennzahl fehlt, B-099) |
| **Nur ausrückende Truppen beschädigen ein Portal** (Gruppen im Angriff, Trupp hinter dem Banner). Gegner, Türme, Mauern, Bogenschützen auf Posten und Skills des Monarchen richten dort keinen Schaden an (Monarch-Schlag: Annahme, ungeprüft, siehe § 9). | Das Portal fällt nur durch aktiven Angriff, nie nebenbei. | – | – |
| **Verteidiger-Wellen:** Solange Truppen ein Portal angreifen, spawnt es **laufend** Gegner in Wellen (Takt und Menge je Tiefe, Startwerte vorläufig: alle 20 s je 4 Gegner in Tiefe 0, steigend). Das ist zusätzlich zur normalen Nachtwelle bzw. zum Aggressionspool. | Ein Angriff darf nicht gratis sein; das Portal wehrt sich. | `data/biomes/<biom>.json` › `portals.defense` (`intervalSeconds`, `size`) | – |
| **Zerstörtes Portal:** Es spawnt nichts mehr und ist dauerhaft weg (im Spielstand). Gegner fliehen zum nächsten noch stehenden Portal der Stufe (sonst bleiben sie bis zum Tod). | Fortschritt bleibt erhalten. | Spielstand (`SAVE_VERSION`, Sperre spielstand) | – |
| **Sichere Stufe:** Sind **alle Portale einer Stufe** zerstört, ist die Stufe **dauerhaft sicher**, auch nach dem Laden: keine Wellen mehr (weder Nacht noch Aggressionspool), der **Miniboss entfällt** (er kommt mit einer Welle, `bosse.md` § 1); wer das **letzte Portal** zerstört, erhält seinen **Skill-Punkt**. Der **Endboss im Bau** bleibt angreifbar, damit der Inselwechsel möglich bleibt. In einer sicheren Stufe gibt es **keinen Raub** (§ 5, § 6). | Wer alle Portale zerstört, hat die Stufe gewonnen; der Endboss bleibt der Meilenstein der Insel. | Spielstand je Stufe | – |

## 3. Kampfvariante je Welt (`combat.mode`)

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Eine Variante je Welt:** `groups` (**Standard**) oder `general`. Fehlt das Feld, gilt `groups`. Zusätzlich `combat.groupSize` (Standard 5, nur `groups`). Das Feld ist ein optionales Feld der Welt-Datei (`welten.md` § 2). | Jede Welt kann anders spielen; der Standard bleibt einfach. | `data/worlds/<id>.json` › `combat` `{mode, groupSize}` | – |

### 3.1 Variante `groups` (Standard)

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Schild an der Burg:** Ein Zahlziel an der Burg verkauft ein Schild für **10 Gold** (Startwert). Ein **Soldat** (Krieger, auch Elite), der sich das Schild holt, wird **Gruppenführer**. Bogenschützen und Bürger werden nie Führer. | Der Spieler baut Gruppen gezielt auf, mit Gold als Preis. | `data/troops.json` › `combat.shieldCost` | – |
| **Gruppe** = Gruppenführer + **5 Soldaten** (`combat.groupSize` je Welt). Die Soldaten stammen aus den Kriegern des Hubs und zählen weiter zum Truppen-Limit (`buerger.md` § 3). | Kleine, überschaubare Trupps. | `data/worlds/<id>.json` › `combat.groupSize` | – |
| **Angriffs-Auslöser:** Links und rechts am **äußersten Tor** (`outerWall`) steht je ein Auslöser; ihn zu bezahlen kostet **2 Gold** (Startwert) und schickt **alle Gruppen** zum Angriff, zum nächsten Portal oder zu Gegnern auf dem Weg (Richtung = Seite des Auslösers, Annahme, ungeprüft). | Ein Knopf für den Angriff, wie im Vorbild. | `data/troops.json` › `combat.triggerCost` | – |
| **Kampf:** Die Gruppe kämpft, bis das Portal zerstört oder die Gruppe aufgerieben ist. Danach oder ohne Ziel **kehrt sie ans Tor zurück**. **Nur Soldaten** rücken aus; Bogenschützen bleiben an Mauer und Turm. | Die Verteidigung bleibt stehen, während die Soldaten angreifen. | – | – |
| **Schilde lassen sich nicht aufsammeln.** Der Führer verliert sein Schild **erst als Letzter seiner Gruppe**; dann ist er Bürger wie jeder getroffene Kämpfer (§ 4). | Der Verlust des Führers beendet die Gruppe. | – | – |

### 3.2 Variante `general`

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Banner:** Jeder Monarch hat **ein Banner**, das an **jedem Tor** zur Verfügung steht (am äußersten Tor je Seite, wenn es zurückgestellt ist). Der Monarch nimmt es auf; ihm folgt **genau ein Trupp** aus **bis zu 5 Soldaten und 5 Bogenschützen** (Startwerte), der Rest bleibt auf Posten. Je Monarch ein Banner, also ein Trupp je Monarch. | Der Monarch führt selbst, wie ein General. | `data/troops.json` › `combat.troopSoldiers`, `combat.troopArchers` | – |
| **Zurückstellen:** Geht der Monarch **hinter das äußerste Tor** zurück, wird das Banner dort wieder aufgestellt, der Trupp kehrt zurück. | Der Angriff endet, wo der General ihn beendet. | – | – |
| **Nachbesetzung:** Fällt ein Kämpfer des Trupps, ersetzt ihn ein neuer, der **vom letzten Tor** losgeht (vermutet: äußerstes Tor der Seite), solange Truppen da sind. | Der Trupp hält seine Stärke ohne Zutun. | – | – |
| **Mit Banner darf der Monarch bauen und zahlen** (Default, nicht gesondert bestätigt). | Kein Sperrzustand. | – | – |

## 4. Kämpfer und Bürger

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Getroffene Kämpfer verlieren die Waffe und werden Bürger (Bauer)**; die Waffe fällt als Drop (Elite-Waffen mit ihrer Stufe, `buerger.md` § 3). Das gilt auch für den Gruppenführer (Schild fällt weg, nicht als Drop) und für Berufe mit Werkzeug. | Wie bisher die erste Stufe der Verlust-Kaskade, nun die einzige. | `engine/sim/unit_warrior.go` (SIM) | Verluste je Welle: Median höchstens 25 % der Kämpfer (`buerger.md` § 3, B-099) |
| **Keine Kaskade bis zum Landstreicher:** Ein Treffer macht nie einen Landstreicher. Ein **getroffener Bürger** (ohne Waffe) **flieht zur Burg**, ohne Statusverlust; seine Münze bleibt bei ihm. **Landstreicher werden weiter nicht angegriffen.** | Der Bürger bleibt rekrutiert; nichts geht dauerhaft verloren. | – | – |
| **Bürger holen das nächste Werkzeug oder die nächste Waffe** (Händler bzw. Werkstatt, oder fallen gelassen am Boden), **nur wenn kein Gegner innerhalb von 8 Units** ist; sonst wartet er. Das ersetzt für diesen Fall die globale Nacht-Gefahr (`isDangerous`). Bei gleicher Distanz zählt der Boden vor dem Händler. Das Truppen-Limit prüft weiter beim Waffe-Holen (`buerger.md` § 3). | Wer entwaffnet ist, rüstet sich neu, aber nicht mitten im Kampf. | `data/troops.json` › `peasant.fetchEnemyDistance` (8 Units) | – |

## 5. Monarch: Gold statt HP, Krone

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Keine HP.** Jeder Treffer kostet **1 Münze** (Startwert); sie fliegt **sichtbar aus dem Sack** und fällt als Münze auf den Boden, **aufsammelbar** für alle. Der Geldsack ist der Puffer, im HUD ist das Gold die Lebensanzeige (B-934). | Der Treffer kostet Gold statt Leben. | `data/monarch.json` › `hitCoins` (löst `base.hp` und die Verteidigung ab) | – |
| **Gegner können dieses Gold rauben** (wie der Dieb, `stealsGold`) und **ins Portal tragen**; dort ist es verloren. **Nur wenn die Stufe noch Portale hat;** sonst lassen sie es fallen. Wird der Träger getötet, fällt das Gold. | Verlorenes Gold lässt sich zurückholen. | `engine/sim/` (SIM) | – |
| **Ohne Gold fällt die Krone:** Trifft ein Gegner den Monarchen ohne Gold, springt die **Krone in hohem Bogen nach hinten** (vom Gegner weg). **Eine Krone je Monarch.** | Das letzte Pfand; wie im Vorbild. | `data/monarch.json` › `crown` (Flugbahn, Startwerte) | – |
| **Lava** schadet dem Monarchen wie ein Treffer: **1 Münze je Sekunde**, ohne Gold fällt die Krone (🧑 2026-11-15, `stufen.md` § 1). | Gleiche Regel wie Gegnertreffer. | `data/biomes/ironhold.json`, `crystal.json` | – |
| **Ohne Krone** kann der Monarch **nur laufen und eine Krone aufheben**: kein Schlag, keine Skills, kein Bauen oder Zahlen, kein Banner. | Die Krone ist die Handlungsfähigkeit. | – | – |
| **Aufheben:** Der Monarch hebt seine Krone auf. **Ein Mitspieler darf eine fremde Krone aufheben und dem Besitzer zurücktragen.** Es gibt keinen Grabstein, kein Wiederbeleben, keinen Respawn (§ 7). | Koop: Der Mitspieler rettet. | – | Anteil zurückgeholter Kronen an gefallenen (Kennzahl fehlt, B-099) |
| **Gegner heben die Krone auf und tragen sie ins Portal:** das ist **Kronenraub** (§ 6). Wird der Träger vorher getötet, fällt die Krone wieder. **Ohne Portale** in der Stufe gibt es **keinen Raub**: Gegner lassen Krone und Gold fallen. | Der Verlust lässt sich abwenden (wie bei Ausrüstung, `gegner.md` § 4). | – | – |
| **Skills umgedeutet** (`monarch.md` § 3): **Schild** (Tank, Divine Shield) fängt Treffer ohne Goldverlust ab; **Last Stand** = die **letzte Münze hält einen Treffer länger** (Abklingzeit bleibt). **HP-Skills auf Gold umgedeutet (🧑 2026-11-15):** Thick Skin = jeder n-te Treffer kostet nichts; Regeneration = Gold-Rinnsal tagsüber (Münzen fließen in den Sack, Werte vorläufig); die **Heiler-Linie heilt Truppen statt den Monarchen** (Heal, Group Heal); Resurrection bringt eine gefallene Krone sofort zu ihrem Monarchen zurück (Umdeutung, Details KS3); Divine Shield = Schild fängt mehrere Treffer ab. Weitere Passive: KS3 (B-930). | Die Skills behalten ihren Zweck, ohne HP. | `data/monarch.json` › `skills` | – |

## 6. Niederlage: Kronenraub statt Burgfall

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Kronenraub ist die Niederlage:** Trägt ein Gegner **eine** Krone ins Portal, ist das Spiel **verloren** (Game Over: der Raum endet, der Spielstand bleibt wie zuletzt gespeichert), **in jedem Schwierigkeitsgrad**. Bei 2+ Spielern genügt eine geraubte Krone. | Klares, sichtbares Spielende. | – | Kronenraub je Seed und Grad (Kennzahl fehlt, B-099) |
| **Burgfall bleibt Schaden ohne Spielende:** Fällt die Burg, wirken nur noch **Gold/Material-Verlust** oder **Stufenverlust** (`stufen.md` § 4); der Modus „Komplett verloren“ gilt für den Burgfall nicht mehr. **Ultra: Burgfall = Stufenverlust**, Leicht/Normal: nur Schaden (Gold/Material-Verlust); Kronenraub ist überall Game Over. | Der Hub hat Gewicht, ohne das Spiel zu beenden. | `data/difficulty.json` › `defeat`, `engine/sim/island_defeat.go` | – |

## 7. Entfallene Regeln

Folgende Regeln aus früheren Dateien **entfallen** (je mit Verweis in der jeweiligen Datei): Monarch-HP und Verteidigung, Respawn nach Wartezeit, Grabstein, Wiederbeleben (A halten) samt Ereignissen `revive` und `revived`, Heiler-Skill „Resurrection“, die Verlust-Kaskade bis zum Landstreicher (Bauer lässt Münze fallen), der Burgfall als Spielende.

## 8. Werte (Platzhalter, vorläufig)

| Wert | Startwert | Ziel-Datei |
|---|---|---|
| Portal-HP je Tiefe 0 bis 4 | 500 / 800 / 1200 / 1800 / 2500 | `data/biomes/*.json` › `portals.hp` |
| Verteidiger-Welle je Portal (Takt, Menge) | 20 s, 4 Gegner (Tiefe 0) | `data/biomes/*.json` › `portals.defense` |
| Kampfvariante, Gruppengröße | `groups`, 5 | `data/worlds/*.json` › `combat` |
| Schild (Gruppenführer) | 10 Gold | `data/troops.json` › `combat.shieldCost` |
| Angriffs-Auslöser | 2 Gold | `data/troops.json` › `combat.triggerCost` |
| Trupp hinter dem Banner | 5 Soldaten, 5 Bogenschützen | `data/troops.json` › `combat.troopSoldiers`, `combat.troopArchers` |
| Gegner-Abstand zum Werkzeug-Holen | 8 Units | `data/troops.json` › `peasant.fetchEnemyDistance` |
| Münzen je Treffer auf den Monarchen | 1 | `data/monarch.json` › `hitCoins` |
| Kronen-Flugbahn | offen (Bogen nach hinten) | `data/monarch.json` › `crown` |

## 9. Offen und Annahmen

- **Entschieden 🧑 2026-11-15:** alles in §§ 1 bis 6 außer den hier genannten Annahmen; **Werte** sind Platzhalter (Portal-HP ausdrücklich `provisional`).
- Annahme (ungeprüft): der Schlag des Monarchen und seine Skills beschädigen Portale nicht; Richtung des Auslösers = seine Seite; „vom letzten Tor“ = äußerstes Tor der Seite; Soldaten füllen die Gruppe aus den freien Kriegern des Hubs.
- Annahme (Ralf: bleibt Annahme): Kämpfer behalten die HP-Regel bis zur Entwaffnung. Offen/ungeprüft: ob **jeder** Treffer einen Kämpfer entwaffnet oder die HP-Regel (Rüstung, Heilplatz) bestehen bleibt (Default: HP-Regel bleibt, bei HP 0 fällt die Waffe, `buerger.md` § 3).
- **Entschieden 🧑 2026-11-15 (Nachtrag):** Ultra: Burgfall = Stufenverlust; Kronenraub bleibt in jedem Grad Game Over; Leicht und Normal: Burgfall nur Schaden (Gold/Material-Verlust). Lava: 1 Münze je Sekunde. HP-Skills auf Gold umgedeutet (§ 5). **Sichere Stufe:** Wer das **letzte Portal** einer Stufe zerstört, erhält den **Skill-Punkt des Miniboss**; die **Vollmond-Belohnung** gibt es nur in Stufen mit Portalen.
- Offen: Krone und Banner zugleich (Vorschlag: fällt die Krone, wird das Banner am Tor zurückgestellt); Geste der Übergabe einer fremden Krone; Krone eines Mitspielers während der Träger selbst getroffen wird; Wirkung von Treffern auf den Gruppenführer, solange noch Soldaten der Gruppe stehen (Vorschlag: keine).
- Offen: Tier-Gating-Widerspruch (`game-design.md` 5/10/15 gegen `monarch.md` 2/4/6), nicht Teil von B-926.
- Offen: Boss-Belohnung (Gold, Material) des entfallenden Minibosses und Händler-Überfall in einer sicheren Stufe.
- **Umgesetzt KS3 (B-930, B-931), Auslegungen (ungeprüft, Feintuning B-099):** Ein Schild fängt den Treffer ganz ab (auch den Überschuss). Thick Skin: jeder 4. Treffer gratis. Last Stand: einmalig, nur bei der letzten Münze. Regeneration = Gold-Rinnsal tagsüber (0,2 Münzen/s); Heal, Group Heal, Healing Aura, Holy Ground heilen Truppen. Resurrection setzt nur Kronen zurück, die kein Gegner trägt. Fortified ist eine Truppen-Aura (+3 Verteidigung, Umkreis 4). Ein Mitspieler trägt eine fremde Krone zum Besitzer und setzt sie ihm bei Ankunft auf. In geschützten Nächten gibt es keinen Raub; die Krone eines abwesenden (Free) Monarchen sitzt wieder auf.
- Ereignisse für Portal, Krone und sichere Stufe legen KS2 bis KS4 mit dem Protokoll an (`protocol.md`); Anzeige: B-934.
