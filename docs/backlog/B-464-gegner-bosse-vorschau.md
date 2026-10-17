# B-464 · Eine Gegner-Seite im Asset-Studio zeigt alle Gegnertypen und Bosse mit ihren Vorschauen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ10
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

17 Gegnertypen in `data/enemies.json` (`greed`, `wolf`, `alphaWolf`, `goblin`, `goblinArcher`, `skeleton`, `bat`, `caveTroll`, `zombie`, `ratSwarm`, `mineGhost`, `lavaSlime`, `ironBeetle`, `fireSpirit`, `crystalSpider`, `shardling`, `crystalGuardian`) mit Grafik aus `data/sprites.json` › `enemies`; Bosse in `data/bosses.json` (je Tiefe 0–4, dazu `endBosses` mit `crystalHeartWarden`). `figuren.html`/`aufstellung.html` zeigen sie statisch; Laufen, Angriff, Tod und Boss-Effekte sieht man nur im Spiel.

## Ziel

Eine Gegner-Seite im Asset-Studio (B-459) zeigt alle Gegnertypen und Bosse mit ihren Animationen (laufen, angreifen, getroffen, sterben, Sonderfähigkeiten), gruppiert nach Biom bzw. Tiefe, im Spielmaßstab und mit Grafik-Auswahl aus dem Slot-Register.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Neue Gegner oder Bosse, Werte und Wellen (REG/SIM); Boss-Kampfablauf; Monarch (B-463).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Die Gegner-Seite listet alle Gegnertypen aus `data/enemies.json`, gruppiert nach Entscheidung zu Frage 2, und spielt jede ihrer Animationen ab.
- **AC-02** Die Bosse aus `data/bosses.json` (einschließlich `endBosses`) haben eine eigene Gruppe mit Animationen und Effekten, soweit Grafik vorhanden; Lücken sind markiert.
- **AC-03** Größe und Maßstab entsprechen dem Spiel; ein Vergleich mit Monarch-Größe ist zuschaltbar.
- **AC-04** Die Seite folgt dem Aufbau der Monarch-Seite (B-463), gemeinsame Abspiel-Bausteine.

## Offene Fragen

1. Welche Boss-Effekte gehören hinein (Warnkreis, Event-Banner, Phasen)? Entscheidet 🧑.
2. Gruppierung nach Biom (aus `data/biomes/*.json` › `enemies`) oder nach Art (Bodentruppe, Flieger, Fernkampf)?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
