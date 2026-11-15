# B-934 · Das Spiel zeigt Portal-HP, Krone, Münzverlust, Schild, Banner und Auslöser

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** KS5
- **Projekt:** KSY
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die neuen Kampfregeln (B-927 bis B-933) brauchen Anzeige im nativen Spiel.

## Ziel

Portal-HP, Krone (am Kopf, fallend, getragen), Münzverlust bei Treffer, Schild des Gruppenführers, Banner und Auslöser sind sichtbar; Assets über `assets_*` (005).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `native/k3c-game/**`, `data/paper.json`
- **Abhängig von:** B-931, B-932, B-933
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Jedes neue Element hat eine Darstellung und, wo nötig, ein Papierschnitt-Asset.
- **AC-02** HUD: Gold ist die Lebensanzeige (Geldsack), Healthbar bleibt weg.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
