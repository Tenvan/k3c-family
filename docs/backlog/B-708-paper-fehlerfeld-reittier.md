# B-708 · Ablehnung reittier-unbekannt nennt das Feld ohne Animationsnamen

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

MG1.1: `assets_set monarch {animationen: {schritt: {reittier: esel}}}` lehnt richtig ab, meldet aber `feld: animationen.reittier` statt `animationen.schritt.reittier` (`tools/paper/rig/pruefen.ts` bzw. `rig/reiten.ts`). Bei mehreren Reit-Animationen ist nicht erkennbar, welche gemeint ist.

## Ziel

Das Fehlerfeld nennt den vollen Pfad `animationen.<name>.reittier`.

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

- **AC-01** `bin/paper.test.ts`: Reittier fehlt → `fehler[0].feld` ist `animationen.schritt.reittier`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
