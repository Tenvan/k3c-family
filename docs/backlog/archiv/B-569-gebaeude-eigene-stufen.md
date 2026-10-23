# B-569 · Gebäude bekommen eigene Stufen (levels je Gebäude)

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** GEB13
- **Projekt:** GEB
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 🧑 zu B-544 (D7, KR2.1): Gebäude sollen nicht mehr nur der Hub-Stufe folgen (GEB9-Default), sondern eigene Stufen haben.

## Ziel

Jedes Gebäude kann eigene Stufen (`levels`) in `data/buildings.json` haben; die Grafik folgt der Gebäudestufe.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `levels` je Gebäude in `data/buildings.json` (REG, ggf. SIM), Werte nur dort.
- Verhältnis zur Regel „Linie k ab Hub-Stufe k“ (`docs/rules/stufen.md`) und zu B-538 (Hub-Wachstum bleibt Grafik) klären.
- Protokoll, Spielstand und Golden prüfen; Entscheidung über Umfang vor der Spec durch 🧑.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Spec mit Umfang, Regeln und Kriterien von 🧑 freigegeben; Umsetzung erst danach.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Aus B-544 (KR2.1). Umfang offen, deshalb Spec Entwurf und Rückfragen an 🧑 vor der Umsetzung.
