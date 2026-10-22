# B-542 · Entscheidung D5: Plantage-Pappel steht hinter der Farm

- **Domäne:** DEV
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** KR2
- **Projekt:** GEB
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit GEB10 ist die Plantage-Pappel so hoch wie alle Bäume und steht deshalb hinter statt vor der Farm; der Setzling bleibt 1 × 1 davor.

## Ziel

🧑 bestätigt die Anordnung oder wählt eine andere.

## Beteiligte und Zielgruppen

🧑 entscheidet (Geschmack, am Gerät in GEB10.5).

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

nicht relevant (Frage).

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen „bestätigt“ oder die Alternative vermerkt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Darf die hohe Pappel hinter der Farm stehen?

- **Default (gilt):** ja, hinter der Farm.
- **Alternative:** eigene niedrige Plantagen-Pappel vor der Farm.
- **Wenn 🧑 die Alternative wählt:** DEV-Ticket (neue JSON/PNG in `tools/buildings`) plus CLI-Anpassung der Ebene in `src/scenes/render/plantation.ts`.
