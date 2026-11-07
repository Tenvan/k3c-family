# B-719 · Gegner sind unterscheidbar und richtig groß, ohne Lesbarkeits-Reste aus MG4

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** MG9
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-07 (auf Ralfs Zustimmung im Chat)

## Ausgangslage

D-MG4-07: Kristallwächter zu nah am Troll, Goblin-Pfeil ragt über den Bogen, Bogen schleift, Feuerball klein, Flammenkrone ähnelt der Monarch-Krone, Troll-Keule verdeckt den Kopf. PS1.3: Troll wirkt im Angriff einbeinig, Kristallspinne mit 97 × 59 px zu klein.

## Ziel

Gegner sind auf einen Blick zu unterscheiden.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Alle Punkte aus D-MG4-07 und PS1.3 Punkt 4 beheben.

## Nicht-Ziele

Bosse (B-720).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Troll `angriff` mit zwei sichtbaren Beinen.
- **AC-02** Kristallspinne mindestens so hoch wie der Goblin (Messwert im Ergebnis).
- **AC-03** Jeder Punkt aus D-MG4-07 erledigt (Sichtprüfung).

## Offene Fragen

keine

## Notizen

Quelle: PS1.3 Punkt 4, D-MG4-07. Nach B-712, B-714, B-715. Lane `gegner`.
