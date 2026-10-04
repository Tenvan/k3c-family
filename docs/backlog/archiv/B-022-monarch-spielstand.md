# B-022 · Monarch-Level und Skills stehen im Spielstand

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Status:** erledigt
- **Sprint:** S1
- **Erstellt:** 2026-09-29
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S1

## Ausgangslage

Rest von S2.1: Der Spielstand enthält noch keine Monarch-Level und Skills.

## Ziel

Monarch-Level und Skills stehen im Spielstand. Nutzen: Fortschritt darf beim Laden nicht verloren gehen.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen gleichzeitig, lokal und online); Umsetzung durch Entwickler oder Agent in Go.

## Anforderungen

- Der Spielstand enthält Monarch-Level und Skills je Spieler.
- Alte Spielstände bleiben lesbar.

## Nicht-Ziele

Die Skill-Mechanik selbst (B-007).

## Regeln und Einschränkungen

Neue Mechaniken nur in Go (Entscheidung 001, Feature-Stopp in `src/world/`); deterministisch, nur der Seed-RNG, kein `Math.random()`; Werte in `data/`; Komplexitäts-Budget. Spielstand-Format mit Version (SP06.2).

## Beispiele

Monarch mit Level 3 und Taunt speichern und laden → Level 3 und Taunt sind da.

## Ausnahme- und Fehlerfälle

Alter Spielstand ohne Level und Skills → lädt mit Startwerten.

## Akzeptanzkriterien

- **AC-01** Speichern und Laden in Go erhalten Level und Skills.
- **AC-02** Ein Test belegt es.

## Offene Fragen

keine

## Notizen

Zusammen mit B-007.
