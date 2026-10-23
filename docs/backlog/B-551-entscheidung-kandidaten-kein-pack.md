# B-551 · Entscheidung G6: public/grafik/kandidaten/ ist kein Grafik-Pack

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

GR9.1 (B-420, erledigt) nimmt `public/grafik/kandidaten/` aus dem Pack-Test `src/tools/grafikPacks.test.ts` aus; Kandidaten brauchen nur eine Credit-Zeile. Vorlage: GR10 › Anforderungen Punkt 6.

## Ziel

🧑 bestätigt die Ausnahme oder wünscht einen eigenen Pack-Eintrag.

## Beteiligte und Zielgruppen

🧑 entscheidet.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

Lizenzen in den CREDITS-Dateien bleiben Pflicht.

## Beispiele

nicht relevant (Frage).

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat unter Notizen A oder B gewählt.

## Offene Fragen

Siehe Notizen.

## Notizen

**Frage:** Ist `kandidaten/` ein Pack?

- **A (Default, gilt, umgesetzt):** kein Pack, Test nimmt den Ordner aus.
- **B:** eigener Eintrag in `GRAFIK_PACKS` (kleine Änderung in `src/tools/grafikPacks.ts` und Test).
- **Wenn 🧑 B wählt:** Ticket CLI, eine kleine Session.
