# B-526 · Entscheidung: Hintergrund-Höhe je Biom gegen WORLD_TOP

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Felder zeigen die Welt ab `WORLD_TOP` (420, `src/scenes/layout.ts`), der Himmel darueber entfaellt. Parallax-Hintergruende einzelner Biome koennten bei dieser Hoehe Randkanten oder Himmelsanteile verlieren. Offen aus dem POC zu B-413.

## Ziel

🧑 entscheidet, ob `WORLD_TOP` fuer alle Biome gilt oder je Biom abweicht.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Default (von Claude autonom gewählt, BD4): ein fester `WORLD_TOP` fuer alle Biome, Hintergruende werden nicht angepasst.
- Alternative: `worldTop` je Biom in `data/` (SIM/REG-Aenderung) oder Anpassung der Hintergrund-Ebenen (Projekt GRA).

## Nicht-Ziele

Neue Grafiken.

## Regeln und Einschränkungen

Werte gehoeren nach `data/`; Client rechnet nichts.

## Beispiele

Wueste und Wald haben unterschiedliche Horizonthoehen.

## Ausnahme- und Fehlerfälle

nicht relevant.

## Akzeptanzkriterien

- **AC-01** 🧑 prueft alle Biome am Geraet und entscheidet.

## Offene Fragen

Fester Wert oder je Biom?

## Notizen

Aus B-413 › Notizen (BD4, 2026-10-22). Autonom entschieden, Claude; Pruefung am Geraet in BD4.4.
