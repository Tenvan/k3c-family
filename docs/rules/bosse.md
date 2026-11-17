# Regelwerk: Bosse und Events

Beschlossen von 🧑 im Workshop R4.3 am 2026-10-02 (Grundlage: [`archiv/ist-gegner-bosse.md`](archiv/ist-gegner-bosse.md); Rahmen: [`gegner.md`](gegner.md), [`stufen.md`](stufen.md), [`monarch.md`](monarch.md), [`wirtschaft.md`](wirtschaft.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte sind **Startwerte**, Feintuning mit dem Balancing-Tester (B-099).
Zielkorridore gelten im Standardszenario **Normal, Wald-Start, 2 Spieler, Bot „sparsam“, je 100 Seeds**. Jede Regel gilt für 2+ Spieler.
**Kampf (B-926, 🧑 2026-11-15):** In einer **sicheren Stufe** (alle Portale zerstört) entfällt der Miniboss mit den Wellen; der Endboss im Bau bleibt angreifbar; das letzte Portal gibt den Skill-Punkt des Minibosses, die Vollmond-Belohnung gibt es nur in Stufen mit Portalen (`kampf.md` § 2).

## 1. Bosse

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Je Stufe ein Miniboss, je Insel ein Endboss** in der tiefsten Stufe (Kristallhöhle in Insel 1). Die Insel wählt ihren Endboss mit `endBoss` (`welten.md` § 2), sonst gilt der der tiefsten Stufe; `endBosses` hat je einen für die Tiefen 0 bis 4 (Tiefen 0–3 vorläufig, B-981). Der **Bau** liegt immer in der tiefsten Stufe, in jedem Biom (`level.Options.Lair`; das Biom-Feld `lair` gilt nur ohne Insel). Jeder Boss ist eine **eigene Figur** im Thema der Stufe. | Klare Meilensteine; Bosse sind Höhepunkte, keine stärkeren Standardgegner. | `data/bosses.json` (SIM legt an) | – |
| **Auslöser Miniboss:** Er kommt mit einer bestimmten Welle über ein Portal (entfällt, wenn alle Portale der Stufe zerstört sind, `kampf.md` § 2): **Welle 5** der Stufe im Wald (Oberwelt), **Welle 3** in Höhle, Mine, Eisenstollen und Kristallhöhle (unten kommen Wellen seltener). | Der Boss ist Teil des Wellenrhythmus und kommt zum Hub. | `data/bosses.json` › `wave` | Miniboss besiegt: Wald bis Tag 8 in 60–85 %; Höhle bis Tag 14, Mine bis Tag 20 in 50–80 % (Startziele) |
| **Auslöser Endboss:** Er liegt in seinem **Bau** im Level (am Ende vor dem Eingang) und wird **ausgelöst, wenn ein Spieler dort ankommt**. Er **wartet ohne Zeitdruck**. | Spieler bestimmen den Zeitpunkt und können sich vorbereiten. | `data/bosses.json` › `lair` | Endboss besiegt bis Tag 25 in 30–55 % |
| **Werte:** Miniboss etwa **8× HP und 2× Schaden** eines Standardgegners der Stufe, **1 Fähigkeit**; Endboss etwa **30× HP**, **3 Phasen** mit je einer neuen Fähigkeit. Zusätzlich skaliert nach der Insel-Tabelle (`stufen.md`) und mit der **Spieleranzahl der Insel** (HP × (1 + `bossHpPerExtraPlayer` je Zusatzspieler, eigenes Feld in `data/waves.json`, getrennt von `perExtraPlayer` der Wellen). | Bosse bleiben ein Gruppenkampf, auch bei 4 Spielern. | `data/bosses.json` | Boss-Kampfdauer Median 60–180 s (Kennzahl fehlt, B-099) |
| **Gold und Material:** Miniboss **100 Gold + 50 Material** der Stufe, Endboss **500 Gold + 250 Material** der Stufe, als Haufen am Ort für alle; dazu die **Skill-Punkte** (Miniboss 1, Endboss 3, `monarch.md` § 3). | Bosse sind lohnend. | `data/bosses.json` › `reward` | – |
| **Camp und Bau:** In einem Biom mit Bau liegt das Landstreicher-Camp (Anwerben) **nie näher als 80 Units** (`lair.noticeUnits` 40 plus 40 Rand) am Bau. **Hinter einem Portal** (vom Hub aus gesehen) liegt es nur, wenn kein anderer Platz weit genug vom Bau und außerhalb der Verteidigungszone frei ist. Der Generator ordnet die Plätze so: 1. mindestens 80 Units vom Bau (B-981), 2. außerhalb der Verteidigungszone (`sitePlacement.outsideUnits`, B-261), 3. nicht hinter einem Portal, 4. nah am Hub; der Abstand der Plätze zum Camp (B-261) bleibt in allen 500 Seeds je Biom gewahrt. **Vorläufig, Entscheidung 🧑 offen (Review WE5.7):** Die Grenze „höchstens 5 % der Seeds hinter einem Portal“ ist nur für die Kristallhöhle geprüft (15 von 500). Als tiefste Stufe einer Insel liegt das Camp im Wald in 40, in der Höhle in 39, im Eisenstollen in 77 und in der Mine in 147 von 500 Seeds hinter einem Portal, weil Punkt 2 vor Punkt 3 gilt. | Hinter dem Portal und vor dem Bau laufen Portal-Gegner und die Beschwörung des Endbosses; der Weg zum Camp wäre tödlich, und ohne Bauern gibt es kein Heer (BS3.4, B-645). | `engine/level/level.go` › `safeCampSlots`, `farFromLair` (Reihenfolge in `placeEvents`: Bau, Verteidigungszone, Portal, Hub-Nähe; die Quote hinter einem Portal ist nur in der Kristallhöhle eine Grenze); `level.CheckLairFit` (Platz für Bau und Camp, Prüfung beim Laden); `data/biomes/crystal.json` › `lengthUnits` (600–700, damit außerhalb der Linien Platz bleibt) | Test `camps_test.go`: 500 Seeds, Abstand zum Bau immer, hinter einem Portal höchstens 5 %; `lair_test.go`: 500 Seeds je Biom mit Bau, Abstand zum Bau und zu den Linien immer, die Quote hinter einem Portal außerhalb der Kristallhöhle nur als Ausgabe (keine Grenze) |
| Ein besiegter Boss **kehrt nie zurück**; der Sieg steht im Spielstand, auch bei Stufenverlust. | Bosse sind Meilensteine, keine Wiederholung. | Spielstand | – |
| Der **Sieg über den Endboss** erfüllt die Endboss-Bedingung des Inselwechsels am Hafen (dazu fertiges Schiff, `stufen.md` § 1) und löst bei Ziel „Endboss“ den Kampagnensieg aus (`stufen.md`). **Mit dem Sieg beginnt die friedliche Insel** (`kampf.md` § 10, B-973): alle Angriffe enden, ein Fest beginnt. | Beschlossene Rahmenregeln. | – | – |
| Im Bosskampf gilt der Monarch-Treffer wie sonst: 1 Münze, ohne Gold fällt die Krone (`kampf.md` § 5); Wiederbeleben und Respawn entfallen. | Keine Sonderregel. | – | – |

### 1.1 Bosse der Insel 1 (Startwerte, Namen vorläufig)

| Stufe | Miniboss | Fähigkeit | Endboss |
|---|---|---|---|
| Wald | Goblin-Anführer | ruft Goblins (2 je 10 s) | – |
| Höhle | Troll-König | Flächenschlag (Radius 3) | – |
| Mine | Ratten-Königin | ruft Rattenschwärme | – |
| Eisenstollen | Lava-Golem | Flammenspur (Fläche am Boden) | – |
| Kristallhöhle | Splitter-Titan | Splitter-Wurf (Fernkampf, Fläche) | **Kristallherz-Wächter** |

**Endboss (Kristallhöhle):** Phase 1 **Beschwörung** (ruft Kristallspinnen), Phase 2 **Flächenschlag** (Radius 5, alle 6 s), Phase 3 **Wut** bei 25 % HP: +50 % Tempo. HP etwa 30× eines Standardgegners der Kristallhöhle (Startwert, B-099).

## 2. Events

| Event | Auslöser | Wirkung | Belohnung |
|---|---|---|---|
| **Vollmond** | jede **7. Nacht** (globaler Zyklus) | verstärkte Wolfswelle (+50 % Wolfsanteil), dazu ein **Alpha-Wolf** (Elite) | 50 Gold je bespielter Stufe, deren Burg die Nacht hielt, an ihrem Hub (höchstens einmal je Stufe und Nacht; bespielt = beim Morgengrauen steht dort ein gesteuerter Monarch, B-379) |
| **Blutmond** | jede **13. Nacht** | alle Gegner +30 % Schaden in dieser Nacht, auch Flächenangriff und Splitter der Bosse; die Bodenfläche (Hazard) bleibt ohne Faktor (B-380) | doppelter Drop der Gegner dieser Nacht |
| **Händler-Überfall** | jeder **4. Händler-Besuch** (`buerger.md`) | Gegner greifen den Händler an; er flieht, wenn er stirbt | Wird er geschützt, schenkt er Material oder einen Rabatt beim Tausch (Startwert: 100 Material der Insel) |

Begründung: Abwechslung im Wellenrhythmus und Gewicht für den Händler. Daten: `data/events.json` (SIM legt an). **Je Welt übersteuerbar** (`overrides › events`, `welten.md` § 1.1, B-982): Abstand (`everyNights`, `everyVisits`), Stärke (`wolfShare`, `damageFactor`, `dropFactor`) und Lohn (`rewardGold`, `rewardMaterial`) je Event; Abstand **0 schaltet das Event aus**, Abstände und Mengen ≥ 0, Faktoren > 0, Gegnerarten bekannt; neue Events stehen künftig in der Basis mit Abstand 0 und werden je Welt eingeschaltet. Der Raum rechnet mit dem Wertesatz der Welt, alle Stufen einer Insel mit demselben. Zielkorridor: Burg hält Vollmond und Blutmond in ≥ 70 % der Seeds (Startziel).
**Friedliche Insel (B-973):** Ab dem Sieg über den Endboss der Insel entfallen die Events mit Gegnern (Vollmond, Blutmond, Händler-Überfall) bis zum Inselwechsel; Händler-Besuche finden ohne Überfall statt (`kampf.md` § 10).

## 3. Offen und Annahmen

- **Alle Zahlen** (Welle 5/3, 8×/30× HP, 2× Schaden, Belohnungen, Phasenwerte, Event-Rhythmen) und die Namen der Bosse sind Vorschläge des Agenten; 🧑 hat den Satz „Vorschlag“ bestätigt, einzelne Zahlen nicht gesondert.
- Händler-Überfall: Rhythmus „jeder 4. Besuch“ und die Belohnung sind Startwerte (B-099, B-121).
- Blutmond und Vollmond gelten für die Nacht des globalen Zyklus in allen Stufen (KB1, B-379, B-380). Stichtag für „bespielt“ ist das Morgengrauen: Wer kurz davor die Stufe wechselt, verschiebt die Belohnung (bewusst einfach gehalten, Bestätigung durch 🧑 über KR6).
- Kennzahlen, die fehlen: `bossSpawned`, `bossDefeated`, Kampfdauer, Event-Ereignisse (B-099).
