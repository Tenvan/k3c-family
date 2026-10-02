# B-115 · Die Stufen sind nach unten schmaler und dichter, Eisenstollen und Kristallhöhle sind als Stufen angelegt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Es gibt drei Biome (Wald 900–1100, Höhle 700–900, Mine 550–700 Units); Eisen und Kristall haben keine Stufe, das GDD nennt Tiefe 3 (Eisen, Lava) und Tiefe 4 (Kristall).

## Ziel

Insel 1 hat fünf Stufen; die Breiten sind nach unten schmaler und dichter; Eisenstollen und Kristallhöhle sind als Biome mit Adern angelegt. Nutzen: Jede Tiefe erschließt das nächste Material (`docs/rules/materialien-gebaeude.md` § 1, `stufen.md` § 1).

## Beteiligte und Zielgruppen

Spieler; REG liefert die Gegner- und Boss-Werte der neuen Stufen (Regelwerk III).

## Anforderungen

- `data/biomes/ironhold.json` (Eisenstollen: Länge 480–560, Eisen-Adern, Lava-Hindernisse) und `crystal.json` (Kristallhöhle: Länge 400–480, Kristall-Adern); Paletten und Gegner-Pools mit vorläufigen Werten bis Regelwerk III.
- Dichter: Ressourcen und Ereignisse je Chunk in Höhle, Mine, Eisenstollen, Kristallhöhle so, dass die Level spielbar bleiben (`validateLevel`-Regeln).
- Insel-Daten (`data/islands.json`, B-103) führen die fünf Stufen; Tiefen-Eingänge und Treppen verbinden sie.
- Skalierung der Gegner je Tiefe wie `docs/rules/stufen.md` (Insel-Tabelle).

## Nicht-Ziele

Gegner- und Boss-Werte (Regelwerk III), Grafiken (B-010), weitere Inseln.

## Regeln und Einschränkungen

`docs/rules/stufen.md`, `materialien-gebaeude.md`; Level-Tests mit 500 Seeds je Biom. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Spieler gehen aus der Mine über den Tiefen-Eingang in den Eisenstollen: kürzeres Level, 2 Eisen-Adern, Lava, stärkere Gegner.

## Ausnahme- und Fehlerfälle

Biom ohne Gegner-Pool → Fehler beim Laden, nicht beim Spielen.

## Akzeptanzkriterien

- **AC-01** Test: Beide neuen Biome erzeugen mit 500 Seeds gültige Level (`validateLevel`); Breiten liegen in den Bereichen.
- **AC-02** Test: Die Breiten von Höhle und Mine sind nach Beschluss angepasst, die Ressourcendichte erfüllt die Zielwerte aus B-099 (Messung im Ticket).
- **AC-03** Die Insel führt fünf Stufen; Eingang und Treppen verbinden benachbarte Stufen.
- **AC-04** Golden-Level-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Gegner-Pools und Hindernisse (Lava) der neuen Stufen (Regelwerk III).

## Notizen

Aus R2.2 und R2.3. Abhängig von B-100, B-103, B-114 und Regelwerk III.

R4 (2026-10-02): Gegner und Pools der beiden neuen Stufen stehen in `docs/rules/gegner.md`; Umsetzung in B-129 (Daten), Portale ab Tiefe 3 drei.
