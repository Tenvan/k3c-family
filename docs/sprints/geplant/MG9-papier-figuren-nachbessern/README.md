# MG9 · PAP · Papierschnitt nachbessern: Monarch, Reittiere, Truppen, Gegner

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** PAP
- **Reife:** Entwurf
- **Tickets:** B-716, B-717, B-718, B-719
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

PS1.3 und PS1.4: Spielerfarben, Reit-Idle, Reittier-Silhouetten, Truppen- und Gegner-Befunde.

## Ziel

Figuren, Reittiere, Truppen und Gegner bestehen die Sichtprüfung.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-716, B-717, B-718, B-719.

## Nicht-Ziele

Bosse (MG10), Spiel-Einbau (US3).

## Regeln und Einschränkungen

Nach RD2. Lanes `reittiere`, `monarch`, `truppen`, `gegner` parallel (max. 4); `monarch` nach `reittiere`.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** B-716/AC-01 bis AC-03
- **AC-02** B-717/AC-01 bis AC-03
- **AC-03** B-718/AC-01, AC-02
- **AC-04** B-719/AC-01 bis AC-03

## Offene Fragen

keine

## Sessions

Reife Entwurf:

- MG9.1 Reittiere (B-716)
- MG9.2 Monarch (B-717)
- MG9.3 Truppen (B-718)
- MG9.4 Gegner (B-719)
- MG9.5 Review

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
