# B-562 · Die Monarch-Seite des Studios zeigt den Skill-Ring des Spiels statt der Lücke

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SK3
- **Projekt:** SKL
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Monarch-Seite des Studios (`tools/k3c-studio/frontend/src/monarch/actions.ts`, `skillRows`) markiert jeden Skill mit „Lücke: keine Darstellung im Spiel“. Seit SK2.3 hat jede Linie im Spiel einen Ring (Effekt-Art `skill`, `LINE_COLORS` in `src/scenes/effects/effects.ts`).

## Ziel

Die Monarch-Seite zeigt je Skill den Ring seiner Linie statt der Lücke.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `skillRows` nutzt `LINE_COLORS`/`EFFECT_CONFIG.skill` des Spiels; Lücke nur bei Linien ohne Farbe.

## Nicht-Ziele

Neue Effekte.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Vitest der Studio-Seite: kein Skill mit Lücke „keine Darstellung“, solange jede Linie eine Farbe hat.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
