# B-1187 · Biome: dichter Bewuchs, Biome am Radar, Landstreicher-Lager zwischen Bäumen und Taverne an der Lagerstelle

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** HB1
- **Projekt:** RSU
- **Erstellt:** 2026-11-21
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (2026-11-21, Auftrag von 🧑)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

RU36.7 hat nur den Wald (Standard-Seed) verdichtet. Die Biome stehen nur in der Hub-Kompasszeile (LA1.3), Landstreicher-Lager sind frei im Gelände.

## Ziel

Alle Biome starten dicht bewachsen; Landstreicher-Lager liegen zwischen 2 bis 3 Bäumen und verschwinden, wenn diese gerodet werden; an der Stelle ist eine Taverne baubar, die bis zu 5 Landstreicher anlockt; Biome sind am Radar sichtbar.

## Beteiligte und Zielgruppen

Spieler; 🧑 Ralf nimmt am PC ab.

## Anforderungen

- Alle Biome: dichter Baumbewuchs zum Start, per Seed gesteuerte kleine Lichtungen und Lücken (wie RU36.7, Werte je Biom einstellbar).
- Biome im oder über dem Radar anzeigen (nicht nur im Hub).
- Landstreicher-Lager haben immer 2 bis 3 Bäume direkt um sich; sind sie gerodet, wird das Lager zerstört.
- An dieser Stelle ist danach eine Taverne baubar; sie lockt bis zu 5 Landstreicher an, einen je Tag.

## Nicht-Ziele

Neue Einheiten; Balancing der Kosten (Abnahme-Ticket).

## Regeln und Einschränkungen

Neue Mechanik nur in Rust (`k3c-sim`, `k3c-level`), Go-Engine eingefroren (Entscheidung 013). Deterministisch über den vorhandenen Rng. Keine Rücksicht auf alte Spielstände. Golden nur bei gewollter Wertänderung.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-level/**`, `rust/crates/k3c-sim/**`, `data/biomes/**`, `data/**`, `rust/crates/k3c-game/src/hud/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** L
- **Sperre:** golden

## Beispiele

Wald: ~90 % der Fläche Bäume, 1 bis 2 Lichtungen je Insel; Landstreicher-Lager von 3 Bäumen umgeben; Bäume gefällt → Lager weg → Taverne baubar → ein Landstreicher je Tag, höchstens 5.

## Ausnahme- und Fehlerfälle

Nur 2 Bäume vorhanden: Lager trotzdem gesetzt. Taverne gebaut, Lager nie existiert: nicht möglich. Alle 5 Landstreicher da: kein weiterer.

## Akzeptanzkriterien

- **AC-01** Alle Biome zum Start dicht bewachsen, Lichtungen per Seed, Test je Biom.
- **AC-02** Biome sind am Radar sichtbar.
- **AC-03** Landstreicher-Lager von 2 bis 3 Bäumen umgeben, bei Rodung zerstört (Sim-Test).
- **AC-04** Taverne an der Lagerstelle baubar, lockt höchstens 5 Landstreicher, einen je Tag (Sim-Test).
- **AC-05** Abnahme am PC durch 🧑.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
