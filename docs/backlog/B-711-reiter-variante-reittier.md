# B-711 · Variante des Reiters tauscht die Materialien des Reittiers nicht

- **Domäne:** PAP
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** MG9
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

MG1.2: Die Varianten `spieler2`–`spieler4` des Monarchen tauschen `koenigsblau`, die Satteldecke des Pferds (`reittiere/pferd`, Teil `sattel`, `koenigsblau`) bleibt in Reit-Animationen aber blau, weil der Tausch nur auf die Spec des Reiters wirkt (`tools/paper/rig/reiten.ts`).

## Ziel

Entscheiden, ob der Tausch einer Reiter-Variante auch auf das Reittier wirkt (Satteldecke in Spielerfarbe) oder ob das Reittier eigene Varianten bekommt.

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

- **AC-01** Entscheidung in `paper-spec.md` › Varianten; bei „wirkt mit“: `galopp@spieler2.png` zeigt die Satteldecke in `smaragd`.

## Offene Fragen

Tausch mitwirken lassen (einfach) oder Reittier-Varianten (flexibler, MG2)?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
