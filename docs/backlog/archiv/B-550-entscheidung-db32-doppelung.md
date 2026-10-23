# B-550 · Entscheidung G5: Die DB32-Palette bleibt doppelt in pipeline und buildings

- **Domäne:** REG
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** KR3
- **Projekt:** GRA
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die DB32-Palette steht zweimal: `tools/pipeline/palette.go` und `tools/buildings/db32.go`. Vorlage: GR10 › Anforderungen Punkt 5.

## Ziel

🧑 entscheidet, ob ein gemeinsames Paket entsteht.

## Beteiligte und Zielgruppen

🧑 entscheidet; Claude setzt um.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

`engine/` importiert nichts aus `tools/`; ein gemeinsames Paket gehört nach `tools/assetkit`.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A oder B gewählt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Entscheidung 🧑 (KR3.1, 2026-10-23):** A, die DB32-Palette bleibt doppelt in `tools/pipeline` und `tools/buildings`; kein gemeinsames Paket.
