# B-129 · Eisenstollen und Kristallhöhle haben ihre Gegner und Pools

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** K1
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Eisenstollen und Kristallhöhle haben noch keine Gegner (`docs/rules/ist-gegner-bosse.md` § 1); B-115 legt die Biome an.

## Ziel

Die neuen Gegnerarten (Lavaschleim, Eisenkäfer, Feuergeist; Kristallspinne, Splitterwicht, Kristallwächter) sind in den Daten und in den Pools der beiden Stufen. Nutzen: Die tiefen Stufen sind spielbar (`docs/rules/gegner.md` § 1).

## Beteiligte und Zielgruppen

Spieler; Werte pflegt REG.

## Anforderungen

- `data/enemies.json`: sechs neue Arten mit Startwerten aus `gegner.md` § 1.
- `data/biomes/ironhold.json` und `crystal.json`: `enemies.portal` mit je 2 Standard + 1 Elite; Portale ab Tiefe 3 drei.
- Gold-Drops und Skalierung nach Insel-Tabelle.

## Nicht-Ziele

Biome und Level (B-115), Traits (B-128), Bosse (B-130), Sprites (B-010).

## Regeln und Einschränkungen

`docs/rules/gegner.md`; Werte nur in `data/`. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Eisenstollen, Nacht: Lavaschleim und Eisenkäfer kommen aus drei Portalen, der Feuergeist schießt mit Flammen-Fläche.

## Ausnahme- und Fehlerfälle

Biom ohne Pool → Fehler beim Laden.

## Akzeptanzkriterien

- **AC-01** Test: Beide Biome laden mit gültigen Pools (Standard und Elite); Wellen planen Gegner aus dem Pool.
- **AC-02** Test: Skalierung nach Insel-Tabelle für die neuen Arten.
- **AC-03** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Namen und Aussehen vorläufig (B-010).

## Notizen

Aus R4.2. Abhängig von B-115 und B-128.
