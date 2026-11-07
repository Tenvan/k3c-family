# B-718 · Truppen greifen sichtbar zum Gegner an und haben keine Lesbarkeits-Reste aus MG3

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** MG9
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

PS1.3: Der Schwertarm des Kriegers verdeckt im Angriff das Gesicht. D-MG3-08: Helm-Nasal unter dem Querbalken, Schild nur in lauf/angriff, Waffe schwingt in `lauf` voll mit, Stabknauf vor dem Bart, Strohhut so gelb wie die Krone.

## Ziel

Truppen sind in allen Animationen lesbar.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Krieger-Angriff zum Gegner hin; alle Punkte aus D-MG3-08 beheben.

## Nicht-Ziele

Pfeil (B-714).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Krieger `angriff`: Klinge vor dem Körper Richtung Gegner, Gesicht frei.
- **AC-02** Jeder Punkt aus D-MG3-08 erledigt (Liste im Ergebnis, Sichtprüfung).

## Offene Fragen

keine

## Notizen

Quelle: PS1.3 Punkt 3, D-MG3-08. Nach B-712, B-714. Lane `truppen`.
