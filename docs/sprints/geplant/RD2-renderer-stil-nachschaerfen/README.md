# RD2 · PAP · Renderer und Stil nach der Sichtprüfung: Schatten, Materialien, Sichtbarkeit, Rig-Proportionen

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** Entwurf
- **Tickets:** B-712, B-713, B-714, B-715
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

PS1 hat gezeigt: Papier-Look schwächer als im Prototyp, fehlende Materialien, keine Sichtbarkeit je Keyframe, Rigs ohne Proportionen je Figur, renderKey ohne Reittier.

## Ziel

Renderer und Stil liefern alles, was die Asset-Sprints MG9–MG11 brauchen.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-712, B-713, B-714, B-715.

## Nicht-Ziele

Assets außer den in B-712 genannten Material-Umstellungen.

## Regeln und Einschränkungen

Welle 4, erster Sprint: vor MG9, MG10, MG11. Lanes `stil` und `renderer`.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** B-712/AC-01 bis AC-04
- **AC-02** B-713/AC-01, AC-02
- **AC-03** B-714/AC-01 bis AC-03
- **AC-04** B-715/AC-01 bis AC-03

## Offene Fragen

keine

## Sessions

Reife Entwurf:

- RD2.1 Stil: Schatten wie Prototyp (B-712)
- RD2.2 Stil: neue Materialien und Umstellung (B-712)
- RD2.3 Renderer: renderKey, Keyframe `sichtbar` (B-713, B-714)
- RD2.4 Renderer: Rig-Proportionen (B-715)
- RD2.5 Review

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
