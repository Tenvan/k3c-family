# B-709 · assets_preview nennt bei Reit-Animationen keine Teil-Boxen des Reittiers

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ43
- **Projekt:** WZG
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

MG1.1: `assets_preview monarch {animation: galopp}` liefert Teil-Boxen nur für den Reiter; die Teile des Pferds (rumpf, beine, kopf) fehlen. Außerdem fehlen dort `oberschenkel-h`/`unterschenkel-h` (verdeckt). Für MG2 (Reittiere im Reit-Bild) wäre eine Box je Teil mit Präfix `reittier:` hilfreich. Daneben (klein): `assets_guide` zeigt „55.00000000000001 %“ (Gleitkomma ungerundet).

## Ziel

`frame` bzw. `assets_preview` nennen bei Reit-Animationen auch die Teil-Boxen des Reittiers; `assets_guide` rundet Prozentwerte.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `paper frame` liefert bei Reit-Animationen je Reittier-Teil eine Box mit Präfix `reittier:` (auch verdeckte Teile); CLI-Teil in PT2.1.
- `assets_preview` reicht diese Boxen durch, `assets_guide` rundet Prozentwerte; Workbench-Teil in WZ43.2.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Domäne PAP (CLI) und DEV (MCP-Tool); Vertrag `docs/standards/paper-spec.md` › 8 für das Box-Präfix.

- **Bereich:** `tools/paper/cli/frame*.ts`, `tools/paper/rig/**`, `tools/k3c-workbench/internal/mcpsrv/tools_assets*.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `paper frame monarch --animation galopp` liefert Boxen mit Präfix `reittier:`.
- **AC-02** `assets_guide` zeigt `55 %`.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
