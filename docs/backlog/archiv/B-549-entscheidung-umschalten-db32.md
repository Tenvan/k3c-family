# B-549 · Entscheidung G4: Das Spiel lädt weiter die Quellgrafik, nicht public/grafik-db32/

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

Die Pipeline aus GR8 schreibt 57 Assets nach `public/grafik-db32/` (Auswahl `docs/assets/pipeline-liste.json`, Vorher/Nachher auf `pipeline.html`). Das Spiel lädt weiter die Quellen. Problemfälle: `greentree` (10× hochskaliert, schrumpft auf 16×16), `castle-tower` (512 px, keine Hochskalierung erkannt), `woodwall` (weiche Kanten). Regel „Hochskalierung zählt erst ab 8 px Restkante“ ist autonom gesetzt. Vorlage: GR10 › Anforderungen Punkt 4 (B-331/AC-04).

## Ziel

🧑 entscheidet, ob das Spiel auf die DB32-Fassung umschaltet, und was mit den drei Problemfällen geschieht.

## Beteiligte und Zielgruppen

🧑 entscheidet und nimmt am TV ab.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Umschalten ändert das ganze Bild am TV: nur mit TV-Abnahme durch 🧑. Atlas-Budget Xbox (GR4).

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A, B oder C gewählt und die Problemfälle entschieden.

## Offene Fragen

Siehe Notizen.

## Notizen

**Entscheidung 🧑 (KR3.1, 2026-10-23):** A, nicht umschalten. Das Spiel lädt weiter die Quellgrafik, `public/grafik-db32/` bleibt Vorschau, die Problemfälle (greentree, castle-tower, woodwall) bleiben unangetastet. B-331/AC-04 ist als „Default: nicht umschalten“ abgeschlossen.
