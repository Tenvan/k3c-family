# B-672 · Die Gegner-Seite zeigt Gegner als kompakte Tabelle mit Inspektor, Lücken als Zahl statt langer Badges

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-30 🧑 Chat („specs sind alle freigegeben“)

## Ausgangslage

Gegner-Seite des Studios (`tools/k3c-studio/frontend/src/enemies/EnemiesPage.tsx`). Oben die Leiste mit Gruppe, Animation, Aktion und Monarch-Vergleich, darunter eine eigene Wiedergabe-Leiste, die Bühne und je Gegner eine Karte (`CardRow`) mit Werten als Fließtext, Merkmalen und je Lücke einem langen gelben Badge („Lücke: Merkmal … ohne Darstellung im Spiel“) sowie der Grafik-Auswahl. Bei vier Gegnern ist die Seite 858 px hoch, Goblin Archer allein hat vier Badges über zwei Zeilen. Ein Klick auf eine Figur oder Karte tut nichts. Gesehen am 2026-10-30 im Browser-Pane (1600×1000, `?mock=1`).

Fähigkeiten: Normale Gegner haben keine Skills, nur Merkmale (`ranged`, `kiting`, `prefersTowers` …) und den Angriff (`attack`). Die Bosse haben je Phase eine Fähigkeit in `data/bosses.json` (`aoe` mit Radius und Intervall, `flameTrail`, `shardThrow`, `summon` mit Gegner und Anzahl). Die Seite zeigt sie nur als Text (`abilityText` in `bossLogic.ts`) und in der Gruppe der Bosse die Boss-Anzeigen (`useBossDisplay`, Warnkreis, B-469); abspielen lässt sich keine Fähigkeit.

## Ziel

Gegner einer Gruppe lassen sich auf einen Blick vergleichen, Details und Lücken gibt es pro gewähltem Gegner, die Bühne bleibt groß, und Angriffe und Boss-Fähigkeiten lassen sich so abspielen, wie das Spiel sie zeigt.

## Beteiligte und Zielgruppen

- 🧑 Ralf prüft Gegner-Grafik, Animationen und fehlende Darstellungen im Studio.
- Sessions der Domäne DEV setzen um.

## Anforderungen

- **Eine Werkzeugleiste:** Gruppe, Animation und Aktion zusammen als Segment-Knöpfe (idle, run, attack, Getroffen, Sterben, Fähigkeit …), Chip „Monarch“ für den Vergleich, Wiedergabe kompakt wie in B-671, Chip „Lücken n“ mit allen Lücken der Gruppe im Tooltip oder Panel.
- **Gegner als Tabelle:** je Zeile Name, Tier, Tiefe, HP, Schaden, Tempo, Reichweite und Lückenzahl; Bosse zusätzlich Art (Mini/End).
- **Inspektor rechts:** gewählter Gegner mit Merkmalen, Extras, Lücken (je eine kurze Zeile), Boss-Phasen und der Grafik-Auswahl.
- **Eine Auswahl:** Klick auf eine Zeile oder eine Figur in der Bühne wählt den Gegner, die Figur wird in der Bühne markiert.
- **Fähigkeiten abspielen (Skill-Animationen der Gegner):**
  - Angriff je Gegner wie im Spiel: Nahkampf mit `attack` und Treffer-Effekt, Fernkampf (`ranged`) zusätzlich mit dem Geschoss des Spiels bis zur Reichweite; was das Spiel nicht zeichnet, ist eine Lücke.
  - Bosse: im Inspektor je Phase ein ▶, dazu Modus „Fähigkeit …“ in der Leiste. Gespielt wird die Darstellung des Spiels: `aoe` mit Warnkreis im Radius und Auslösung im Intervall, `flameTrail` als Spur mit Dauer, `shardThrow` als Wurf im Radius, `summon` mit den beschworenen Gegnern (Grafik aus deren Sheet) neben dem Boss. Radius und Intervall kommen aus `data/bosses.json`, in Units auf die Bühne umgerechnet.
  - Intervall und Dauer laufen mit Schleife, Tempo und Einzelbild der Wiedergabe.
  - Fähigkeiten ohne Darstellung im Spiel zeigen die Lücke, kein erfundener Ersatz. Darstellungs-Code wird aus dem Spiel bzw. den bestehenden Boss-Anzeigen (B-469) genutzt, nicht nachgebaut.
- Boss-Anzeigen (`useBossDisplay`) bleiben in der Gruppe der Bosse.

## Nicht-Ziele

- Neue Gegner, Merkmale oder Animationen.
- Änderungen am Slot-Register (bleibt nur lesend).
- Werte bearbeiten (die Seite bleibt Ansicht).

## Regeln und Einschränkungen

- Domäne DEV: `tools/k3c-studio/frontend/src/enemies/`, `src/playback/`, `styles/world.css`, Bausteine in `tools/webkit`.
- Radix nur über `@webkit`, Atomic Design; keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Kompakte Wiedergabe gemeinsam mit B-671; wer zuerst umsetzt, baut sie.

## Beispiele

- Gruppe „Oberwelt (Wald)“ → Tabelle mit Greed, Goblin, Goblin Archer, Wolf; Goblin Archer zeigt Lücken 4.
- Klick auf Goblin Archer in der Bühne → Zeile markiert, Inspektor zeigt ranged, prefersTowers, kiting, kiteDistance 8 mit je einer Lückenzeile.
- Goblin Archer, Aktion „attack“ → Geschoss fliegt bis Reichweite 10, mit Monarch-Vergleich auf den Monarchen zu.
- Gruppe „Bosse“ → Tabelle mit Spalte Art, Inspektor zeigt Phasen und Fähigkeit.
- Boss mit `aoe` Radius 3, Intervall 4 s → ▶ zeigt den Warnkreis mit 3 Units, alle 4 s ausgelöst; Tempo 2× halbiert den Abstand.
- Boss mit `summon` goblin ×2 → zwei Goblins erscheinen neben dem Boss.

## Ausnahme- und Fehlerfälle

- Gegner ohne Daten (`def` fehlt) → Zeile mit ID, Werte „–“.
- Keine Slots für einen Gegner → keine Grafik-Auswahl im Inspektor.
- Monarch-Vergleich an → die Zuordnung Figur ↔ Zeile berücksichtigt den Versatz (heute `offset`).
- Phase ohne eigene Fähigkeit → ▶ spielt die Fähigkeit der vorigen Phase (wie `abilityText` sie nennt) oder ist ausgeblendet, wenn es keine gibt.
- Beschworener Gegner ohne Sheet → Lücke, Platzhalter-Rahmen an seiner Stelle.
- Unbekannter Fähigkeits-Typ in `data/bosses.json` → Lücke „Fähigkeit <typ> ohne Darstellung“.

## Akzeptanzkriterien

- **AC-01** Bei vier Gegnern ist die Seite ohne Scrollen vollständig sichtbar bei 1600×1000 (Browser-Pane, Port 5196, `?mock=1`).
- **AC-02** Lücken stehen in der Tabelle als Zahl und im Inspektor je als eine Zeile; die langen Badges je Karte gibt es nicht mehr (Vitest).
- **AC-03** Klick auf Zeile oder Figur wählt denselben Gegner, auch mit Monarch-Vergleich (Vitest mit Bühnen-Attrappe).
- **AC-04** Gruppe der Bosse zeigt Art, Phasen und Boss-Anzeigen wie heute (Vitest).
- **AC-05** Bestehende Tests von `EnemiesPage` und `enemyLogic` sind grün oder ohne Verlust an geprüftem Verhalten angepasst.
- **AC-06** Jede Fähigkeit aus `data/bosses.json` lässt sich abspielen oder zeigt ihre Lücke; Radius und Intervall stammen aus den Daten (Vitest auf der Logik).
- **AC-07** Fernkämpfer (`ranged`) zeigen beim Angriff das Geschoss des Spiels bis zur Reichweite oder die Lücke (Vitest).
- **AC-08** Schleife, Tempo und Einzelbild wirken auf Fähigkeiten (Vitest); Browser-Pane: `aoe`, `summon` und ein Fernkämpfer sind auf der Bühne sichtbar (Screenshot in der Abnahme).

## Offene Fragen

- Gruppen als Auswahlliste (heute) oder als Liste links? Default: Auswahlliste in der Leiste (Mock). Entscheidet 🧑.
- Braucht `FigureStage` für den Klick auf eine Figur eine Erweiterung (Treffer-Test)? Default: ja, klein in `PreviewStage`; prüft die umsetzende Session.
- Wie zeichnet das Spiel `flameTrail` und `shardThrow` heute (vermutet: nur Warnkreis bzw. gar nicht)? Default: was das Spiel zeichnet, sonst Lücke; prüft die umsetzende Session vor dem Bau.
- Sollen überhaupt Gegner-Skills jenseits der Bosse kommen? Default: nein, dazu wäre ein eigenes Ticket in SIM nötig. Entscheidet 🧑.

## Notizen

- Mock aus dem Chat vom 2026-10-30 (Widget „studio_planer_monarch_gegner_mock“, Reiter Gegner).
- Geschwister-Tickets: B-670 (Gebäude-Planer), B-671 (Monarch).
- Fähigkeiten (Skill-Animationen) auf Wunsch von 🧑 am 2026-10-30 ergänzt (Anforderungen, AC-06 bis AC-08).
- **Umgesetzt 2026-10-30:** `enemies/abilityLogic.ts`, `useEnemies.ts`, `EnemyTable.tsx`, `EnemyInspector.tsx` (neu), `EnemiesPage.tsx`, `enemyLogic.ts`, `bossLogic.ts` (Exporte); Treffer-Test und Markierung in `stage/figureLayer.ts`; gemeinsame Wiedergabe mit B-671.
- AC-01 Browser-Pane 5196 `?mock=1` 1600×1000: Seite endet bei 921 px, kein Scrollen. AC-02 bis AC-07 Vitest (`EnemiesPage.test.tsx`, `abilityLogic.test.ts`, `timeline.test.ts`). AC-08: Vitest-Teil grün; Screenshots offen, weil im Mock die Gegner-Grafiken nicht laden (schon vor dem Umbau leer).
- Entschieden nach Default: Gruppe bleibt Auswahlliste; Treffer-Test klein in der Bühne; keine Skills für normale Gegner.
- Lücken, die das Spiel nicht zeichnet: Bodenfläche von `flameTrail`, Splitter-Radius von `shardThrow`, eigener Effekt bei Auslösung von `aoe`. Ungeprüft: Blickrichtung der Gegner-Sheets beim Angriff mit Monarch-Vergleich.
- Offen: Abnahme durch 🧑 im echten Studio (AC-08 Screenshots).
