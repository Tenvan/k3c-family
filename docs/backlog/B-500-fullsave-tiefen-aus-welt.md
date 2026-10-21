# B-500 · Voll ausgebauter Spielstand nimmt Tiefen aus den Biomen statt aus der Standard-Welt

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SV2
- **Projekt:** WRT
- **Erstellt:** 2026-10-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gefunden im Review WZ18.7. `fullIsland` (`engine/room/fullsave.go:105`) holt die Stufen über `islandDepths()` (`engine/room/stages.go`, alle Biome), ein neuer Raum seit WZ18.4 aber über die erste Insel der Welt (`engine/room/worlds.go` › `createIn`). Zwei Quellen für dieselben Tiefen; weichen `data/worlds/standard.json` und die Biome ab, hat der voll ausgebaute Stand andere Stufen als ein neues Spiel.

## Ziel

Eine Quelle für die Stufen einer neuen Insel: die Welt-Datei; `islandDepths` entfällt, wenn nichts anderes es braucht.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test in `engine/room`: der voll ausgebaute Stand hat die Tiefen von `sim.StandardWorld().Islands[0].Depths`; `task check:go` grün, Golden byte-gleich.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
