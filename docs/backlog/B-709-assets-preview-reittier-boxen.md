# B-709 · assets_preview nennt bei Reit-Animationen keine Teil-Boxen des Reittiers

- **Domäne:** PAP
- **Typ:** Idee
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

MG1.1: `assets_preview monarch {animation: galopp}` liefert Teil-Boxen nur für den Reiter; die Teile des Pferds (rumpf, beine, kopf) fehlen. Außerdem fehlen dort `oberschenkel-h`/`unterschenkel-h` (verdeckt). Für MG2 (Reittiere im Reit-Bild) wäre eine Box je Teil mit Präfix `reittier:` hilfreich. Daneben (klein): `assets_guide` zeigt „55.00000000000001 %“ (Gleitkomma ungerundet).

## Ziel

`frame` bzw. `assets_preview` nennen bei Reit-Animationen auch die Teil-Boxen des Reittiers; `assets_guide` rundet Prozentwerte.

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

- **AC-01** `paper frame monarch --animation galopp` liefert Boxen mit Präfix `reittier:`.
- **AC-02** `assets_guide` zeigt `55 %`.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
