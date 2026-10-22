# B-546 · Entscheidung G1: Die sechs neuen Gegner behalten die GR8-Figuren, die GR7.3-Kandidaten werden nicht importiert

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
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

**Frage:** Welche Figur bekommen lavaSlime, ironBeetle, fireSpirit, crystalSpider, shardling, crystalGuardian?

- **A (Default, gilt):** GR8-Figuren bleiben; GR7.3 schließt je Lücke mit „kein Treffer“, keine Fremddatei wird importiert. Begründung: Lizenzen der Kandidaten sind ungeprüft, `fireSpirit` hat nur Effekt-Treffer, `ironBeetle` nur zwei Kandidaten.
- **B:** je Gegner ein Kandidat aus `docs/funde/gr7-figuren-funde.html`; Claude prüft die Lizenz, kopiert nach `public/sprites/<sheet>/`, Credit, `data/sprites.json` (`enemies.<id>`).
- **C:** Bild-KI oder neu gezeichnet für ironBeetle/crystalSpider.
- **Wenn 🧑 B oder C wählt:** GR7.3 Schritt 4 (Einbau) durch Claude, danach Sichtprüfung; bei A setzt Claude GR7.3 auf `fertig` mit „kein Treffer, GR8-Figur“ und B-193, B-329 auf `erledigt`.
