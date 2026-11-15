# B-024 · Tiefe 3 und 4 sind beschrieben

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RG2
- **Projekt:** BAL
- **Erstellt:** 2026-09-29
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Tiefe 3 (Eisen, Lava) und Tiefe 4 (Kristall) sind nur als Idee geplant, noch ohne Regelwerk und Biom-Daten.

## Ziel

Tiefe 3 und 4 sind beschrieben. Nutzen: Langzeitmotivation.

## Beteiligte und Zielgruppen

Familie als Spieler; 🧑 entscheidet in Workshops, der Agent bereitet vor.

## Anforderungen

- Tiefe 3 (Eisen, Lava) und Tiefe 4 (Kristall) sind im Regelwerk beschrieben.
- Biom-Daten für beide liegen in `data/biomes/`.

## Nicht-Ziele

Umsetzung in Go.

## Regeln und Einschränkungen

Werte stehen in `data/*.json`, Regeln in `docs/rules/` verweisen darauf; `game-design.md` bleibt die Übersicht. Jede Regel gilt für 2+ Spieler.

## Beispiele

nicht relevant – Beschreibung, keine Mechanik.

## Ausnahme- und Fehlerfälle

nicht relevant – Beschreibung, keine Mechanik.

## Akzeptanzkriterien

- **AC-01** Das Regelwerk beschreibt Tiefe 3 und 4.
- **AC-02** Biom-Daten für beide Tiefen liegen in `data/biomes/`.

## Offene Fragen

keine

## Notizen

Stand geprüft 2026-11-15: **beide Kriterien sind erfüllt, Vorschlag Status erledigt** (setzt 🧑 bzw. der Sprint-Abschluss).

- AC-01: `docs/rules/stufen.md`:22 (Eisenstollen und Kristallhöhle: Ausgang, 3 Portale, Lava), `docs/rules/materialien-gebaeude.md`:17, :21, :29 (Kristall, fünf Stufen der Insel 1, Breiten), `docs/rules/gegner.md`:17–18 (Gegner beider Stufen), `docs/rules/bosse.md`:29–30 (Lava-Golem, Splitter-Titan, Kristallherz-Wächter), `docs/rules/kampf.md`:64 (Lava und Monarch).
- AC-02: `data/biomes/ironhold.json` („Tiefe 3 (Eisenstollen)“, `depth` 3) und `data/biomes/crystal.json` („Tiefe 4 (Kristallhöhle)“, `depth` 4).
