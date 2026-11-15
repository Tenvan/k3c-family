# B-931 · Die gefallene Krone fliegt im Bogen nach hinten; Gegner rauben sie ins Portal (Niederlage), Monarchen sammeln sie auf

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** KS3
- **Projekt:** KSY
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Eine Krone gibt es im Code nicht; Niederlage kommt heute vom Burgfall (`engine/sim/island_defeat.go`, Modi resources/stage/lost je Grad).

## Ziel

Die Krone fällt in hohem Bogen nach hinten (vom Gegner weg). Gegner können sie rauben und ins Portal tragen: dann ist das Spiel verloren. Monarchen können sie aufsammeln; der Mitspieler darf eine fremde Krone zurücktragen. Eine Krone je Monarch. Kronenraub ersetzt den Burgfall als Niederlage; Burgfall bleibt Schaden ohne Spielende.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `engine/sim/**`, `engine/room/**`, `data/**`
- **Abhängig von:** B-930
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Krone als Objekt mit Flugbahn, deterministisch (Test).
- **AC-02** Gegner trägt Krone ins Portal → Niederlage (Test); Monarch ohne Krone kann sie aufnehmen (Test).
- **AC-03** Burgfall beendet das Spiel nicht mehr (Test), Defeat-Modi angepasst.
- **AC-04** 2 Spieler: zwei Kronen, Mitspieler trägt fremde Krone zurück (Test).

## Offene Fragen

Entschieden 🧑 2026-11-15: Ohne Krone kann der Monarch nur laufen und eine Krone aufheben. Kronenraub ist in jedem Grad Game Over. In einer Stufe ohne Portale gibt es keinen Raub: Gegner lassen Krone und Gold fallen.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
