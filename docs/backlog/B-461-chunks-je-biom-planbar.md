# B-461 · Chunks eines Bioms lassen sich individuell planen, per Seed oder mit festen Gebäuden und Objekten

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ8
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Level-Generator (`engine/level/level.go`) reiht Chunks links und rechts vom Hub zufällig nach `chunkWeights` des Bioms (`data/biomes/<biom>.json`, z. B. Wald `forest 5, clearing 3, rocks 1, stream 1`), dazu den Hub, Sonder-Chunks (`portal`, `exit`, `edge`) und `eventChunks` (`chest`, `recruitCamp`). Alle Würfe kommen aus einem Strom `<biom>:<seed>`; Adern und Linien haben eigene Ströme. Gespeichert wird nur der Seed. Feste Gegenden (ein bestimmter Chunk mit festen Gebäuden oder Objekten an fester Stelle) lassen sich nicht planen.

## Ziel

Chunks eines Bioms lassen sich im Welt-Betrachter (B-460) individuell planen: entweder per Seed (zufällig wie heute, aber festgehalten) oder mit festen Gebäuden und Objekten für spezielle Gegenden. Der Generator übernimmt geplante Chunks deterministisch.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Neue Chunk-Arten oder neue Objekte mit eigener Regel (REG); Balancing der Chunk-Gewichte; ganze Level von Hand bauen (nur einzelne Chunks).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ein Datenformat für geplante Chunks (Platz in der Reihe, Chunk-Art, Seed oder feste Objekt-/Gebäudeliste) ist als Entscheidung von 🧑 festgelegt und im Glossar eingetragen.
- **AC-02** Der Generator setzt geplante Chunks an ihre Stelle, die übrigen bleiben zufällig; jeder geplante Chunk würfelt aus einem eigenen Strom, damit er die übrigen Würfe nicht verschiebt. Ohne Planung bleiben `testdata/golden/level-*.json` byte-gleich; mit Planung gleiche Eingaben → gleiches Level (Golden-Test).
- **AC-03** Der Welt-Betrachter (B-460) bearbeitet geplante Chunks eines Bioms und zeigt das Ergebnis sofort in Chunk- und Live-Vorschau.
- **AC-04** Ungültige Planung (Chunk außerhalb der Länge, Konflikt mit Hub, Sonder-Chunk oder Mauerlinien) wird abgelehnt und benannt.

## Offene Fragen

1. Wo liegt die Planung: im Biom (`data/biomes/<biom>.json`, gilt für jedes Level des Bioms) oder je Insel/Level (`data/islands.json`)? Annahme: je Biom, optional je Insel überschreibbar.
2. Feste Gebäude: echte Bauten (gehören dann der Insel, z. B. Ruine) oder nur Objekte (Truhe, Camp, Ressourcen)? Entscheidet 🧑, ggf. REG-Ticket.
3. Domänen: Datenformat und Generator SIM, Editor DEV – zwei Sprints oder einer mit zwei Domänen?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
