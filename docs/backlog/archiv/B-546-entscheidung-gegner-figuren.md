# B-546 · Entscheidung G1: Die sechs neuen Gegner behalten die GR8-Figuren, die GR7.3-Kandidaten werden nicht importiert

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** KR3
- **Projekt:** GRA
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GR7.3 (B-193) hat für die sechs Gegner ohne eigene Figur je bis zu drei Fremd-Kandidaten auf `docs/funde/gr7-figuren-funde.html` gesammelt (Lizenzen ungeprüft, teils nur Suchlinks). GR8.1 (B-329) hat die Lücken inzwischen autonom gefüllt: lavaSlime=fire-worm, fireSpirit=cemetery-ghost, shardling=flying-eye, crystalGuardian=skeleton (je getönt), ironBeetle und crystalSpider selbst erzeugt (`tools/spritegen`, 32×24, DB32). Vorlage: GR10 › Anforderungen Punkt 1, GR7.3 › Ergebnis.

## Ziel

🧑 bestätigt die GR8-Zuordnung (= „kein Treffer“ je Lücke in GR7.3) oder wählt je Gegner einen Kandidaten.

## Beteiligte und Zielgruppen

🧑 entscheidet (Auswahl nach Geschmack, Q14); Claude baut danach ein.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage). Boss-Figur bleibt Lücke (B-130).

## Regeln und Einschränkungen

Lizenzregel Q70 (CC0, CC-BY, OGA-BY, CC-BY-SA); Credits sofort in `public/sprites/CREDITS.md`. Ein Fremd-Kandidat wird erst nach Lizenzprüfung importiert.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A oder je Gegner einen Kandidaten (B) gewählt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Entscheidung 🧑 (KR3.1, 2026-10-23):** B, je Gegner ein Kandidat aus `docs/funde/gr7-figuren-funde.html`, wo es einen guten gibt. Lizenzen sind ungeprüft, Claude prüft sie auf der Quellseite vor dem Import (Q70).

| Gegner | Wahl |
|---|---|
| lavaSlime | Fire Slime Monster Variations DB32 (ImogiaGames, CC0) |
| ironBeetle | LPC Beetle (Redshrike, CC-BY 3.0 / OGA-BY 3.0, Credit Pflicht) |
| fireSpirit | GR8-Figur bleibt (cemetery-ghost), kein Import |
| crystalSpider | Spider Attack, Idle, Shoot, Walk (ethanf30games, CC0) |
| shardling | Animated Walk-Cycle Monsters (Night Blade, CC0) |
| crystalGuardian | Golem animations (irmirx, CC-BY 3.0, Credit Pflicht) |

Umsetzung: GR7.3 Schritt 4 (Einbau durch Claude: Lizenz prüfen, kopieren nach `public/sprites/<sheet>/`, `CREDITS.md`, `data/sprites.json`, Zuordnung), danach Sichtprüfung durch 🧑. Fehlt bei einem Kandidaten das Sheet oder die Lizenz nicht, bleibt die GR8-Figur.
