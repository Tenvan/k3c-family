# Ist-Stand Monarch, Klassen, Level, Skills und Bürger (Vorbereitung R3.2 und R3.3)

Stand: 2026-10-02 (Session R3.1). Vergleich von `docs/game-design.md` mit dem Go-Code (`engine/sim/`) und `data/*.json`.
Nur Tatsachen aus Code und Daten; was nicht geprüft wurde, steht als **ungeprüft**. Dieses Dokument beschließt nichts und ändert keinen Wert.
Beschlossene Rahmenregeln (hier nicht verhandelbar): Taste X frei für Skills, das Skill-Menü nicht auf View (Vorschlag LB + RB), B unbelegt (`stufen.md` § 6, `wirtschaft.md` § 6);
Spieler sind pro Spieler frei in den Stufen einer Insel (`stufen.md` § 1); Schwierigkeitsgrade ändern nur Wellen und Gegner (`wirtschaft.md` § 4);
Gebäude **Schmiede** (Elite-Upgrades), **Rüstkammer** (Rüstung/Waffen), **Heilplatz**, **Taverne**, **Kaserne** (Truppen-Limit 10, +10) und **Werkstatt** (Bogen, Schwert) sind beschlossen, ihre Werte für Bürger beschließt R3.3 (`materialien-gebaeude.md` § 3).

## 1. Monarch

| Thema | `game-design.md` / `data/monarch.json` | Ist im Go-Code | Abweichung? |
|---|---|---|---|
| Basiswerte Level 1 | HP 100, Schaden 10, Tempo 5 Units/s, Verteidigung 5 | `world.go` › `AddPlayer`: HP 100; Tempo 5 (`economy.go` › `movePlayer`), Sprint ×1,8, Beschleunigung 8; Verteidigung 5 mindert Schaden auf mindestens 1 (`common.go` › `applyDamage`); **Schaden 10 wird nirgends benutzt** | Schaden ohne Wirkung |
| Werte je Level | +10 HP, +2 Schaden, +0,5 Tempo, +1 Verteidigung, höchstens Level 20 (`perLevel`, `maxLevel`) | **kein Level, keine Erfahrung** (`rg -i level engine/sim` ohne Monarch-Treffer) | **ja** |
| Erfahrung | – (GDD schweigt, Skill-Punkte „vor allem versteckt in der Welt, zusätzlich per Level-Up“) | keine | GDD schweigt |
| Tod | Respawn am Hub ohne Strafe, Truppen bleiben | Respawn nach 5 s an der Burg (links bzw. rechts je Spieler), volle HP; bezahlte, nicht fertige Münzen werden erstattet | nein |
| Heilung | – | keine Regeneration, keine Heilung außer Respawn | GDD schweigt |
| Angriff | „kämpft aktiv mit Skills, trägt aber nicht den Hauptschaden“ | **keiner**: `PlayerCommand` hat nur `MoveX`, `Sprint`, `Pay` | **ja** |
| Interagieren, Bau-Menü, Skill-Menü | Tasten Y, View/K | als Aktionen definiert, ohne Funktion (`ist-abgleich.md` § 5) | **ja** |
| Skill-Punkte | versteckt in der Welt, 3–5 je Stufe | Pickup `skillPoint` erhöht den **gemeinsamen** Zähler `World.SkillPoints` (nicht je Spieler); gemessen im Mittel 3,9 je Level (100 Seeds); kein Ausgeben | **ja**: B-007 verlangt einen eigenen Baum je Spieler |
| Skill-Slots | 4 Slots, Belegung nach dem Gamepad-Test | keine | **ja** |

## 2. Klassen, Skill-Baum, Presets

- **Presets** in `monarch.json` (Daten, ohne Code): Tank (HP 200, Schaden 8, Verteidigung 10; Taunt, Shield Bash), Zauberer (HP 90, Schaden 12, Verteidigung 4,5; Fireball, Ice Wall), Heiler (HP 100, Schaden 8, Verteidigung 6; Heal, Group Heal), Dieb (HP 90, Schaden 10, Tempo 5,5, Verteidigung 4,5; Backstab, Smoke Bomb).
- **Skill-Baum (GDD):** vier Linien, Punkte frei verteilbar, hybride Builds erlaubt, **Tier-Gating** Tier 2 ab 5, Tier 3 ab 10, Tier 4 (Ultimate) ab 15 Punkten in der Linie; Respec kostenlos im Hub; „Die Skills kommen erst nach dem Vertical Slice.“
- **Skills (GDD, Zahlen):** Tank: Taunt (10 Units, CD 15 s), Shield Bash (Stun 2 s, CD 10 s), Iron Wall (Barriere 500 HP, CD 30 s), Last Stand (überlebt mit 1 HP, CD 120 s). Zauberer: Fireball (30 Schaden, Fläche 3 Units, CD 8 s), Ice Wall (Verlangsamung 50 %, CD 20 s), Lightning Storm (10/s, 5 s, CD 40 s), Meteor (200, Fläche 15 Units, CD 120 s). Heiler: Heal (50 HP, CD 10 s), Group Heal (30 HP, 10 Units, CD 30 s), Divine Shield (100 HP Schild, CD 20 s), Resurrection (Truppen 50 % HP, CD 180 s). Dieb: Backstab (40, von hinten 80, CD 12 s), Smoke Bomb (unsichtbar 5 s, CD 25 s), Poison Blade (10/s, 5 s, CD 15 s), Shadow Strike (Teleport + 150, CD 90 s). Passive je Linie nur namentlich.
- **Ist:** Nichts davon im Code. Voraussetzungen, die fehlen: ein Angriffs- und Skill-Befehl in `PlayerCommand` und im Protokoll, Skill-Zustand je Spieler im Snapshot und im Spielstand (`SAVE_VERSION`), Wirkungen auf Gegner, Truppen und Monarchen.
- `game-design.md` › Offene Fragen: „Eine Klasse pro Spieler als Preset, damit sich die Rollen im Koop ergänzen?“ (B-017) und „Skill-Tasten am Controller“.

## 3. Bürger

| Figur | Daten (`troops.json`) | Ist im Code | Entwicklung |
|---|---|---|---|
| **Landstreicher** | HP 30, Tempo 1,5, Rekrutierung 1 Gold | wandert im Camp (max. 2, Nachwuchs alle 25 s); eine Münze macht ihn zum Bauern (`payVagrant`) | – |
| **Bauer** | HP 40, Tempo 3, kein Angriff | sammelt markierte Ressourcen, baut, holt Bögen; **bei Gefahr bleibt er in der Burg** (`units.go`) | wird mit Bogen zum Bogenschützen |
| **Bogenschütze** | HP 50, Tempo 3,5, Schaden 15, Reichweite 10, 1 Schuss/s; Bogen Holz 50 + Gold 20 | Posten auf Turm (2 Plätze, +3 Reichweite) oder hinter der äußersten Mauer; schießt automatisch (`archer.go`) | Elite in Daten (HP 75, Schaden 23, Reichweite 12, 1,2/s; Stein 100 + Gold 50), **ohne Code** |
| **Krieger** | HP 100, Tempo 3,5, Schaden 20, Reichweite 1; Kosten Holz 50 + Gold 20 | **ohne Code** (kein Schwert, kein Nahkampf-Verhalten) | Elite in Daten (HP 150, Schaden 30, 1,2/s; Kupfer 100 + Gold 50), **ohne Code** |
| Weitere Figuren (Händler, Handwerker u. a.) | – | – | Frage B1 |

- Es gibt **kein Level, keine Erfahrung, keine Skills und keine Berufe** für Bürger. Eine Entwicklung existiert nur als Elite-Upgrade in den Daten.
- Es gibt **kein Truppen-Limit** (beschlossen: Basis 10, Kaserne +10; Umsetzung B-116), keine Heilung von Truppen (beschlossen: Heilplatz), kein Verlust-Ereignis (nur der Bestand fällt).
- Truppen sterben ohne Rückerstattung; beim Fall einer Burg gehen alle Truppen verloren außer Landstreichern (`castleFallen`, als „Stufenverlust“ beschlossen).
- Wirkung der R2-Gebäude auf Bürger: **Werkstatt** liefert Waffen (Bogen im Code, Schwert beschlossen), **Schmiede** (Kupfer) Elite-Upgrades, **Rüstkammer** (Eisen) Rüstung/Waffen für alle Truppen, **Heilplatz** Heilung, **Taverne** 1 Landstreicher je Tag, **Kaserne** Limit +10; ihre Zahlen fehlen, sie sind Teil von R3.3.
- **Adern** (R2.3) brauchen je Ader bis zu 2 Bauern gleichzeitig: Die Zahl der Bauern begrenzt den Materialfluss (`materialien-gebaeude.md` § 1).

## 4. Koop und Stufen

- Jeder Spieler hat einen eigenen Monarchen und Gold; der Skill-Zähler ist im Code **gemeinsam** (`World.SkillPoints`), nicht je Spieler.
- Spieler sind pro Spieler in den Stufen einer Insel unterwegs und tragen Fortschritt (Level, Skills) mit; ein Spieler in einer anderen Stufe als die Burg-Verteidiger kann nicht alle Skills am Hub einsetzen. Ob Fortschritt je Spieler, je Insel oder je Raum zählt, ist offen (Frage K2).
- **Respec** „kostenlos im Hub“ gilt jetzt je Hub (jede Stufe hat einen Hub); ob überall oder nur im Hub der ersten Stufe, ist offen.

## 5. Kennzahlen für Zielkorridore

Aus `ist-abgleich.md` § 6 verfügbar: `playerDown` (Monarch gefallen), `World.Troops` (Bestand je Art), `recruited`, `armed`, `chest`, `skillPoint`, Gold je Spieler. **Fehlen** (Vorschlag für B-099): Level und Erfahrung je Spieler (kein Level im Code), Skill-Punkte je Spieler, Skill-Einsatz und -Wirkung, Verluste an Truppen je Welle (nur Differenz, kein Ereignis `troopLost`), Heilung, Zeit bis zum ersten Elite-Upgrade.

| Beschlussgröße | Kennzahl für den Zielkorridor | Verfügbar? |
|---|---|---|
| Level-Kurve des Monarchen | Level am Ende von Tag 5/10 | nein (kein Level) |
| Skill-Punkte je Stufe | `skillPoint`-Ereignisse je Level und Spieler | Summe ja, je Spieler nein |
| Monarch fällt | `playerDown` je Welle | ja |
| Truppen je Tag | Bestand `World.Troops` zu Tagesbeginn | ja |
| Verluste je Welle | Differenz des Bestands, `troopLost` | Ereignis fehlt |
| Zeit bis Elite | Tick des ersten Elite-Upgrades | nein (kein Code) |

## 6. Fragen für die Workshops

Je Frage mit Optionen und Empfehlung; 🧑 entscheidet einzeln.

**R3.2 Monarch**

- **K1 · Preset oder freie Wahl (B-017):** (a) freie Skillung, die Presets sind nur Startverteilungen (GDD, Empfehlung), (b) feste Klasse beim Beitritt, (c) Klasse wählbar, Respec im Hub kostenlos.
- **K2 · Skill-Baum persönlich oder gemeinsam:** (a) persönlicher Baum je Spieler, Fortschritt gehört dem Spieler (B-007, Empfehlung), (b) gemeinsamer Pool je Raum, (c) je Insel getrennt.
- **K3 · Erfahrung und Level:** Quelle (Kills, Bosse, Bauen und Münzen, Zeit/Tage, Entdeckungen), Kurve, Höchststufe 20? (a) Erfahrung aus Kills und Bossen plus Entdeckungen, (b) Level aus Tagen, (c) kein Level, nur Skill-Punkte.
- **K4 · Skill-Punkte:** versteckt in der Welt (3–5 je Stufe, heute 3,9), zusätzlich Level-Up? Wem gehört ein Fund (Finder, alle im Raum)? Empfehlung: dem Finder, Level-Up zusätzlich.
- **K5 · Tier-Gating und Respec:** Tier 2/3/4 ab 5/10/15 Punkten in der Linie, Respec kostenlos im Hub (bei mehreren Hubs: in jedem Hub)? Empfehlung: wie GDD, in jedem Hub.
- **K6 · Monarch im Kampf:** (a) rein Skills (Hauptschaden bleibt bei den Truppen, GDD), (b) zusätzlich ein einfacher Nahkampfschlag (Schaden 10) mit Taste, (c) nur automatischer Schlag in Reichweite. Empfehlung (a) für den Start.
- **K7 · Skill-Slots und Tasten:** 4 Slots; Controller X = Slot 1 (frei), weitere z. B. LB, RB, LT? Skill-Menü auf LB + RB (Vorschlag aus R1.3)? Tastatur E, Q, R, T (F ist Vollbild, B Bau)? Empfehlung: Slot 1 auf X/E, Slots 2–4 auf LB/RB/LT bzw. Q/R/T, Menü LB+RB bzw. K.
- **K8 · Linien in Insel 1:** alle vier oder zuerst Tank und Zauberer (B-007)? Empfehlung: Tank und Zauberer zuerst, Heiler und Dieb folgen.
- **K9 · Werte je Level:** +10 HP, +2 Schaden, +0,5 Tempo, +1 Verteidigung bis Level 20 übernehmen? Empfehlung: ja als Startwerte.
- **K10 · Tod im Koop:** Respawn 5 s ohne Strafe wie heute oder Wiederbeleben durch Mitspieler? Empfehlung: wie heute (Heiler-Skill „Resurrection“ betrifft nur Truppen).

**R3.3 Bürger**

- **B1 · Wer sind Bürger?** Angenommen Landstreicher, Bauer, Bogenschütze, Krieger, Elite. Weitere Figuren (Händler, Handwerker, Bergmann, Holzfäller, Heiler)? Empfehlung: Berufe als Spezialisierung des Bauern (B2), keine neuen Figuren vorab.
- **B2 · Berufe:** Heute wird der Bauer mit Waffe zum Kämpfer. Zusätzlich Berufe für Arbeiten (Holzfäller, Bergmann an den Adern, Baumeister)? (a) nein, ein Bauer kann alles, (b) Bergmann und Baumeister als Beruf mit Bonus, (c) volle Berufe.
- **B3 · Level und Erfahrung der Bürger:** (a) nur Upgrades über Schmiede und Rüstkammer (Elite), (b) Truppen leveln durch Kämpfe, (c) beides. Empfehlung (a).
- **B4 · Skills der Bürger:** keine, oder wenige passive Verbesserungen (Rüstkammer: Rüstung, Waffen)? Empfehlung: passive Upgrades über Gebäude, keine aktiven Skills.
- **B5 · Kosten:** Bogen/Schwert Holz 50 + Gold 20, Elite Stein/Kupfer 100 + Gold 50 gelten weiter? Material kommt aus dem Insel-Vorrat. Empfehlung: ja als Startwerte.
- **B6 · Truppen-Limit:** Basis 10, Kaserne +10: zählt es alle Bürger oder nur Kämpfer (Bogenschütze, Krieger)? Empfehlung: nur Kämpfer, Bauern frei (Adern brauchen viele Bauern).
- **B7 · Verhalten:** Bauern bleiben bei Gefahr in der Burg (Ist) auch mit Adern in tiefen Stufen? Kämpfer: Posten (Turm, hinter der Mauer) oder aktiver Vorstoß? Empfehlung: wie heute.
- **B8 · Heilung und Verlust:** Heilplatz heilt Truppen und Spieler; sonst keine Regeneration? Verlorene Truppen ersetzen über Taverne und Kaserne? Empfehlung: ja.

## 7. Gliederung für `monarch.md` und `buerger.md`

1. Monarch: Basiswerte, Level und Erfahrung, Skill-Punkte, Baum (Linien, Tiers, Respec), Slots und Tasten, Kampf, Tod, Koop. 2. Bürger: Figuren, Berufe, Level, Upgrades (Schmiede, Rüstkammer), Kosten, Limit, Verhalten, Heilung, Verlust. 3. Zielkorridore je Beschlussgröße. 4. Offen und Annahmen.
