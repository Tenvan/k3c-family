# B-251 · Figuren werden ganzzahlig skaliert und flimmern nicht

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** GR7
- **Projekt:** GRA
- **Erstellt:** 2026-10-04
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-12, Chat (Ralf), Revision 1

## Ausgangslage

19 von 21 Figuren-Packs laufen mit nicht ganzzahliger Skalierung (×1,25 bis ×3,25, Faktor `scale` in `data/sprites.json`), abgestimmt auf gleiche Figurgröße. Q13 verlangt ×2 bis ×3. 🧑 hat die Figuren im Workshop GR1.1 trotzdem als „passt“ bestätigt, mit Vermerk auf dieses Ticket (`docs/assets/zuordnung.md`).

## Ziel

Figuren zeigen gleich große Pixel ohne Flimmern beim Laufen (NEAREST-Filter mit krummem Faktor), soweit es am TV auffällt.

## Beteiligte und Zielgruppen

Spieler am TV; 🧑 beurteilt, ob das Flimmern stört.

## Anforderungen

- Erst am TV beobachten, ob das Flimmern sichtbar ist; nur dann umstellen.
- Gleiche Figurgröße bleibt erhalten (z. B. Frames zuschneiden oder auf ×2/×3 runden).

## Nicht-Ziele

Neue Figuren (B-193), Umgebungsgrafik (GR3).

## Regeln und Einschränkungen

Stilbeschluss Q13 (`docs/fragenkatalog.md`); Werte in `data/sprites.json`, nicht im Code.

## Beispiele

`eliteWarrior` ×1,25 → Pixel unterschiedlich breit beim Laufen → nach Umstellung ×2 mit zugeschnittenem Frame.

## Ausnahme- und Fehlerfälle

Am TV kein Flimmern sichtbar → Ticket verwerfen.

## Akzeptanzkriterien

- **AC-01** Am TV beobachtet: Figuren flimmern beim Laufen nicht, oder das Ticket ist mit Begründung verworfen.

## Offene Fragen

Stört das Flimmern am TV überhaupt? Entscheidet 🧑.

## Notizen

**Verworfen (🧑, Chat, 2026-10-24):** Die bisherigen Umsetzungen sind ausreichend; Grafik wird künftig mit der Integration des AI-Chats anders und besser gelöst. Keine ganzzahlige Umstellung (ersetzt die Entscheidung B-547 B vom 2026-10-23).
