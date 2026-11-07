# B-713 · renderKey des Reiters enthält den Hash der Reittier-Spec

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RD2
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-07, Chat (mit Sprint RD2)

## Ausgangslage

D-RD1-06: Der renderKey des Monarchen ändert sich nicht, wenn sich nur die Reittier-Spec ändert; die Reit-PNGs gelten dann fälschlich als aktuell.

## Ziel

Eine Änderung am Reittier rendert den Reiter neu.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

renderKey eines Assets mit `reittier` enthält den Hash der Reittier-Spec.

## Nicht-Ziele

Änderungen am Golden-Format.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test in `tools/paper`: Pferd-Spec ändern → renderKey des Monarchen ändert sich, ohne Reittier nicht.
- **AC-02** `gen:paper` rendert nach einer Pferd-Änderung auch `figuren/monarch`.

## Offene Fragen

keine

## Notizen

Quelle: PS1.1 ✏️ D-RD1-06. Lane `renderer`.
