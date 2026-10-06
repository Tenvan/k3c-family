# B-013 · Restliche Gegner und Elite-KI sind umgesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** K1
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-04, Chat, durch 🧑, mit Sprint K1

## Ausgangslage

`data/enemies.json` beschreibt Verhalten, das die Simulation noch nicht kennt.

## Ziel

Restliche Gegner und Elite-KI sind umgesetzt. Nutzen: Mehr Abwechslung in den Nächten.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen gleichzeitig, lokal und online); Umsetzung durch Entwickler oder Agent in Go.

## Anforderungen

- Gegner-Verhalten aus `data/enemies.json`: Kiting, AoE, `fleesAtHalfHp`, `swarm`.

## Nicht-Ziele

Gegner, die nicht in `enemies.json` stehen.

## Regeln und Einschränkungen

Neue Mechaniken nur in Go (Entscheidung 001, Feature-Stopp in `src/world/`); deterministisch, nur der Seed-RNG, kein `Math.random()`; Werte in `data/`; Komplexitäts-Budget.

## Beispiele

Ein Gegner mit `fleesAtHalfHp` fällt unter die halbe HP → er flieht.

## Ausnahme- und Fehlerfälle

Unbekanntes Verhalten in `enemies.json` → Laden oder Test scheitert, statt es still zu ignorieren.

## Akzeptanzkriterien

- **AC-01** Alle Gegner aus `enemies.json` verhalten sich wie beschrieben.
- **AC-02** Jedes Verhalten hat einen Test.

## Offene Fragen

keine

## Notizen

R4 (2026-10-02): Regelwerk beschlossen (`docs/rules/gegner.md`, `bosse.md`). Umsetzung: B-128 (Traits, Kiting, Angriffsrate), B-129 (neue Gegner und Pools), B-130 (Bosse), B-131 (Events).

–
