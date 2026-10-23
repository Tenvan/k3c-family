# B-548 · Entscheidung G3: Der Entwurf der Stil-Bibel (DB32, 16 px, Outline, Licht oben links) gilt

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KR3
- **Projekt:** GRA
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GR8.2 hat eine Stil-Bibel als Entwurf geschrieben: DB32-Palette, 16 px Grundraster, Outline, Licht von oben links (B-331/AC-01). Die Pipeline (`tools/pipeline`) erzwingt nur Palette und Raster, nicht Outline und Licht. Vorlage: GR10 › Anforderungen Punkt 3.

## Ziel

🧑 gibt die Stil-Bibel frei oder ändert sie.

## Beteiligte und Zielgruppen

🧑 entscheidet (Geschmack).

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Geschmacksentscheidung, nur 🧑.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A oder B gewählt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Gilt die Stil-Bibel aus GR8.2 so?

- **A (Default, gilt):** ja, als Leitlinie; Outline und Licht bleiben Empfehlung, die Pipeline prüft nur Palette und Raster.
- **B:** ändern (andere Palette, 32 px, ohne Outline) oder Outline/Licht in der Pipeline erzwingen.
- **Wenn 🧑 B wählt:** Ticket im Projekt GRA für die Änderung von Stil-Bibel und Pipeline; B-331/AC-01 bleibt bis dahin offen. Bei A wird B-331/AC-01 abgehakt.
