# B-691 · Doku: Norton-Ausnahme in arbeitsweise.md, veraltete world/codemap.md

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-31
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gefunden in WZ38.1: `docs/arbeitsweise.md` (Tempo im Lauf) nennt die Norton-Ausnahme für `%USERPROFILE%\.k3c\tmp` nicht, nur `requirements.md` (B-619/AC-04 verlangt beide). `tools/k3c-studio/frontend/src/world/codemap.md` nennt noch `IslandBar` und `StructureTree`, die es nicht mehr gibt.

## Ziel

Doku stimmt mit dem Stand überein.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `docs/arbeitsweise.md` nennt die Norton-Ausnahme oder verweist auf `requirements.md`.
- **AC-02** `world/codemap.md` nennt keine entfernten Komponenten mehr.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
