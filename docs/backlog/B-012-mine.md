# B-012 · Mine (Tiefe 2) ist vollständig

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** W2
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-05, Chat, durch 🧑, Revision 1, mit Sprint W2

## Ausgangslage

Die Mine (Tiefe 2) ist heute nur angelegt.

## Ziel

Mine (Tiefe 2) ist vollständig. Nutzen: Die dritte Stufe ist heute nur angelegt.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen gleichzeitig, lokal und online); Umsetzung durch Entwickler oder Agent in Go.

## Anforderungen

- Kupfer als Ressource.
- Eigene Gegner: Zombie, Rattenschwarm, Minengeist.

## Nicht-Ziele

Tiefe 3 und 4 (B-024).

## Regeln und Einschränkungen

Neue Mechaniken nur in Go (Entscheidung 001, Feature-Stopp in `src/world/`); deterministisch, nur der Seed-RNG, kein `Math.random()`; Werte in `data/`; Komplexitäts-Budget. Erst nach Regelwerk III; Feature-Kette REG → SIM → CLI.

## Beispiele

Das Team steigt in die Mine → Kupfer ist abbaubar, nachts kommen Minen-Gegner.

## Ausnahme- und Fehlerfälle

Wird im Regelwerk III festgelegt (siehe Offene Fragen).

## Akzeptanzkriterien

- **AC-01** Die Mine ist spielbar mit eigenen Ressourcen und Gegnern.
- **AC-02** Tests in Go decken Ressourcen und Gegner ab.

## Offene Fragen

Regeln der Mine (Regelwerk III, 🧑).

## Notizen

Nach Regelwerk III.
