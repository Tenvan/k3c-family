# Regelwerk: Bosse und Events

Beschlossen von 🧑 im Workshop R4.3 am 2026-10-02 (Grundlage: [`archiv/ist-gegner-bosse.md`](archiv/ist-gegner-bosse.md); Rahmen: [`gegner.md`](gegner.md), [`stufen.md`](stufen.md), [`monarch.md`](monarch.md), [`wirtschaft.md`](wirtschaft.md)).
Je Regel: **Regel · Begründung · Verweis auf `data/` · Zielkorridor**. Werte sind **Startwerte**, Feintuning mit dem Balancing-Tester (B-099).
Zielkorridore gelten im Standardszenario **Normal, Wald-Start, 2 Spieler, Bot „sparsam“, je 100 Seeds**. Jede Regel gilt für 2+ Spieler.

## 1. Bosse

| Regel | Begründung | Daten | Zielkorridor |
|---|---|---|---|
| **Je Stufe ein Miniboss, je Insel ein Endboss** in der tiefsten Stufe (Kristallhöhle in Insel 1). Jeder Boss ist eine **eigene Figur** im Thema der Stufe. | Klare Meilensteine; Bosse sind Höhepunkte, keine stärkeren Standardgegner. | `data/bosses.json` (SIM legt an) | – |
| **Auslöser Miniboss:** Er kommt mit einer bestimmten Welle über ein Portal: **Welle 5** der Stufe im Wald (Oberwelt), **Welle 3** in Höhle, Mine, Eisenstollen und Kristallhöhle (unten kommen Wellen seltener). | Der Boss ist Teil des Wellenrhythmus und kommt zum Hub. | `data/bosses.json` › `wave` | Miniboss besiegt: Wald bis Tag 8 in 60–85 %; Höhle bis Tag 14, Mine bis Tag 20 in 50–80 % (Startziele) |
| **Auslöser Endboss:** Er liegt in seinem **Bau** im Level (am Ende vor dem Eingang) und wird **ausgelöst, wenn ein Spieler dort ankommt**. Er **wartet ohne Zeitdruck**. | Spieler bestimmen den Zeitpunkt und können sich vorbereiten. | `data/bosses.json` › `lair` | Endboss besiegt bis Tag 25 in 30–55 % |
| **Werte:** Miniboss etwa **8× HP und 2× Schaden** eines Standardgegners der Stufe, **1 Fähigkeit**; Endboss etwa **30× HP**, **3 Phasen** mit je einer neuen Fähigkeit. Zusätzlich skaliert nach der Insel-Tabelle (`stufen.md`) und mit der **Spieleranzahl der Insel** (HP × (1 + 0,5 je Zusatzspieler), wie die Wellen). | Bosse bleiben ein Gruppenkampf, auch bei 4 Spielern. | `data/bosses.json` | Boss-Kampfdauer Median 60–180 s (Kennzahl fehlt, B-099) |
| **Gold und Material:** Miniboss **100 Gold + 50 Material** der Stufe, Endboss **500 Gold + 250 Material** der Stufe, als Haufen am Ort für alle; dazu die **Skill-Punkte** (Miniboss 1, Endboss 3, `monarch.md` § 3). | Bosse sind lohnend. | `data/bosses.json` › `reward` | – |
| Ein besiegter Boss **kehrt nie zurück**; der Sieg steht im Spielstand, auch bei Stufenverlust. | Bosse sind Meilensteine, keine Wiederholung. | Spielstand | – |
| Der **Sieg über den Endboss** öffnet Boot/Portal zur nächsten Insel (gemeinsamer Wechsel) und löst bei Ziel „Endboss“ den Kampagnensieg aus (`stufen.md`). | Beschlossene Rahmenregeln. | – | – |
| Fällt ein Spieler im Bosskampf, wird er wie üblich wiederbelebt (A halten, 3 s) oder respawnt nach 15 s (`monarch.md` § 5). | Keine Sonderregel. | – | – |

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
| **Vollmond** | jede **7. Nacht** (Oberwelt) | verstärkte Wolfswelle (+50 % Wolfsanteil), dazu ein **Alpha-Wolf** (Elite) | 50 Gold |
| **Blutmond** | jede **13. Nacht** | alle Gegner +30 % Schaden in dieser Nacht | doppelter Drop der Gegner dieser Nacht |
| **Händler-Überfall** | jeder **4. Händler-Besuch** (`buerger.md`) | Gegner greifen den Händler an; er flieht, wenn er stirbt | Wird er geschützt, schenkt er Material oder einen Rabatt beim Tausch (Startwert: 100 Material der Insel) |

Begründung: Abwechslung im Wellenrhythmus und Gewicht für den Händler. Daten: `data/events.json` (SIM legt an). Zielkorridor: Burg hält Vollmond und Blutmond in ≥ 70 % der Seeds (Startziel).

## 3. Offen und Annahmen

- **Alle Zahlen** (Welle 5/3, 8×/30× HP, 2× Schaden, Belohnungen, Phasenwerte, Event-Rhythmen) und die Namen der Bosse sind Vorschläge des Agenten; 🧑 hat den Satz „Vorschlag“ bestätigt, einzelne Zahlen nicht gesondert.
- Händler-Überfall: Rhythmus „jeder 4. Besuch“ und die Belohnung sind Startwerte (B-099, B-121).
- Blutmond und Vollmond treten in der Oberwelt ein; wie sie unten wirken (Aggressionspool), legt das SIM-Ticket fest (Annahme: gelten für die Nacht des globalen Zyklus).
- Kennzahlen, die fehlen: `bossSpawned`, `bossDefeated`, Kampfdauer, Event-Ereignisse (B-099).
