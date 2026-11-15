# B-920 · Der Stresstest startet keine Gegnerwellen

- **Domäne:** SIM
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** FB2
- **Projekt:** NAT
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Live-Test 🧑 2026-11-15: Der Stresstest (ST2) startet nur ein Spiel mit der eingestellten Anzahl Soldaten und Geschwindigkeit; Gegnerwellen kommen nicht.

## Ziel

Der Stresstest erzeugt Gegnerwellen nach seinen Einstellungen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Kein Sim- oder Balancing-Lauf (Sperre 🧑 2026-11-11); nur Unit-Tests.

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Ein Stresstest erzeugt innerhalb der ersten Nacht Gegner (Test in `engine/sim`).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
