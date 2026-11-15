# B-921 · Reiner Stressmodus: Gegner spawnen bei der Burg, nur eigene Bevölkerung nach Einstellung

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
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

Live-Test 🧑 2026-11-15: Es fehlt ein reiner Stressmodus.

## Ziel

Ein Stressmodus, in dem Gegner direkt bei der Burg spawnen und nur die eigene Bevölkerung nach Einstellung lebt, um Last und Kampf gezielt zu prüfen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Abhängig von:** B-920
- Kein Sim- oder Balancing-Lauf (Sperre 🧑 2026-11-11).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Im Stressmodus spawnen Gegner bei der Burg (Test).
- **AC-02** Die eigene Bevölkerung entspricht der Einstellung; sonst entsteht nichts.
- **AC-03** Der Modus ist im Testing-Menü wählbar und deterministisch (`engine/rng`).

## Offene Fragen

Wellenstärke und Takt im Stressmodus: aus den Stresstest-Einstellungen oder fest? (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
