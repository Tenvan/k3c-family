# B-675 · DoTs, Buffs und Kontrolleffekte der Skills sind im Spiel sichtbar

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KB5
- **Projekt:** KMP
- **Erstellt:** 2026-10-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Viele Skills wirken über eine Dauer, im Spiel ist davon fast nichts zu sehen (geprüft am 2026-10-30):
- Kontrolle auf Gegnern: shieldBash `stun` 2 s, iceWall `slow` 0,5 für 5 s, taunt 4 s.
- DoT: lightningStorm legt einen Sturm (`w.Storms`, 10/s, 5 s, Radius 5) an.
- Buffs auf Monarch/Truppen: ironWall und divineShield `shield`, lastStand, frostArmor, Auren (armorAura, healingAura, blessings, guardian), regeneration, holyGround.
- Sichtbar ist heute nur Smoke Bomb (`stealthFor`, Monarch fast durchsichtig, `worldRenderer.ts`).
Der Snapshot an den Client trägt Sturm, Verlangsamung, Betäubung, Schild und Auren nicht (`src/model/types.ts` kennt nur `stealthFor`).

## Ziel

Spieler sehen, welcher Gegner betäubt, verlangsamt oder verspottet ist, wo ein Sturm Schaden macht und welcher Monarch oder Trupp Schild, Aura oder Regeneration hat.

## Beteiligte und Zielgruppen

- 🧑 Ralf und Familie spielen.
- Sessions der Domänen SRV (Snapshot) und CLI (Darstellung) setzen um.

## Anforderungen

- Snapshot ergänzt je Gegner die laufenden Kontrolleffekte (stun, slow, taunt) mit Restdauer, je Spieler Schild (Wert, Restdauer), lastStand, frostArmor, und die Liste der Stürme (x, Radius, Restdauer); Auren als Radius am Monarchen. Nur, was die Simulation schon hat; Protokollversion erhöhen.
- Darstellung je Effekt, klein und lesbar im Split-Screen: Symbol über dem Gegner (Sterne für Betäubung, Schneeflocke/Blautönung für Verlangsamung, Ausrufezeichen für Spott), Bodenfläche mit Blitzen für den Sturm, Schild-Blase und Balken am Monarchen, Aura-Ring am Boden.
- Werte (Farben, Größen) in `src/scenes/effects/` nach Muster `skillLooks.ts`, damit das Studio sie importieren kann.

## Nicht-Ziele

- Neue Skills oder Werte.
- Ton für Status-Effekte.
- Studio-Darstellung (B-676).

## Regeln und Einschränkungen

- Logik in der Simulation, der Client zeichnet nur.
- 2 Spieler, Split-Screen; Leistung: keine Objekte je Takt neu anlegen (Pool wie bei Effekten).
- Protokolländerung: Review vor dem Commit.

## Beispiele

- shieldBash trifft Goblin → 2 s Sterne über dem Goblin.
- lightningStorm → 5 s Fläche mit Radius 5 am Wirkort.
- ironWall → Schild-Blase am Monarchen, verschwindet bei 0 HP Schild oder nach 10 s.

## Ausnahme- und Fehlerfälle

- Effekt endet, während der Gegner stirbt → Anzeige verschwindet mit ihm.
- Viele Gegner gleichzeitig verlangsamt → Leistung im Rahmen (`perfProbe`).

## Akzeptanzkriterien

- **AC-01** Go-Test: Snapshot enthält Kontrolleffekte, Schild und Stürme mit Restdauer.
- **AC-02** Protokoll und Golden-Daten nachgezogen.
- **AC-03** Vitest: jede Effektart wird gezeichnet und wieder entfernt.
- **AC-04** Abnahme durch 🧑 im Spiel mit zwei Spielern.

## Offene Fragen

- Ein Ticket oder getrennt in Snapshot (SRV) und Darstellung (CLI)? Default: zwei Sessions eines Sprints. Entscheidet 🧑 beim Planen.
- Symbole als Grafik-Slots (Asset-Studio) oder gezeichnet? Default: gezeichnet, Slots später. Entscheidet 🧑.

## Notizen

- Aus der Rückfrage von 🧑 am 2026-10-30 zur Monarch-Seite des Studios (B-671).
- Hängt mit B-674 zusammen (Wirkort des Sturms). Folge-Ticket fürs Studio: B-676.
