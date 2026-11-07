# B-715 · Rigs vierbeiner und figur erlauben Proportionen, Füße und Gangarten je Tier oder Figur

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RD2
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

D-MG2-08: Wölfe, Höllenhund und Elefant teilen die Proportionen des Pferds. PS1.3: Reittiere brauchen Pfoten bzw. Füße je Tier; Bosse brauchen eine eigene Silhouette statt der Krieger-Grundform.

## Ziel

Rigs tragen Proportionen je Figur, damit B-716 und B-720 umsetzbar sind.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `vierbeiner`: Beinlänge, Rumpf und Fußform (Huf, Pfote, Elefantenfuß) je Tier; Gangarten je Tier wählbar (Elefant ohne `galopp`).
- `figur`: Rig-Proportionen (Rumpf, Arme, Beine, Kopf) je Figur.

## Nicht-Ziele

Assets umstellen (B-716, B-720).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Spec-Vertrag und Schema kennen die neuen Felder; Tests für Pfote, Huf, Elefantenfuß.
- **AC-02** Elefant-Spec ohne `galopp` ist gültig, Pferd ohne `galopp` nicht.
- **AC-03** Figur mit langen Armen und kurzem Rumpf rendert ohne Gelenkfehler (Golden-Fixture).

## Offene Fragen

keine

## Notizen

Quelle: PS1.1 ✏️ D-MG2-08, PS1.3 Punkte 2 und 5. Lane `renderer`.
