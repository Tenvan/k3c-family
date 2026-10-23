# B-547 · Entscheidung G2: Krumme Skalierungsfaktoren bleiben bis zur TV-Beobachtung GR7.4

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

19 von 21 Figuren-Packs laufen mit krummem Faktor (×1,25 bis ×3,25, `scale` in `data/sprites.json`); dazu lavaSlime ×1,75 und crystalGuardian ×2,5 (2×1,25) aus GR8. B-251 verlangt, erst am TV zu beobachten, ob das Flimmern stört (GR7.4). Vorlage: GR10 › Anforderungen Punkt 2.

## Ziel

🧑 entscheidet nach GR7.4, ob ganzzahlig skaliert wird.

## Beteiligte und Zielgruppen

🧑 beobachtet am TV und entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Werte nur in `data/sprites.json`; Sattelpunkte (`mounts.*.saddle`, `riderWaist`) hängen am Faktor.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A oder B gewählt (nach der Beobachtung in GR7.4).

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Stört das Flimmern krumm skalierter Figuren am TV?

- **A (Default, gilt):** so lassen; B-251 wird nach GR7.4 mit Begründung verworfen, wenn kein Flimmern auffällt.
- **B:** je Pack `scale` auf ×2 oder ×3 runden, Frames zuschneiden, Sattelpunkte nachziehen (GR7.4 Schritt 3, Claude), erneute TV-Prüfung durch 🧑.
- **Wenn 🧑 B wählt:** Claude setzt GR7.4 Schritt 3 um (nur `data/sprites.json`, `public/sprites/`, `docs/assets/zuordnung.md`).
