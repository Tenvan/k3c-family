# B-625 · Balance-Baseline nach BAL6 neu schreiben

- **Domäne:** REG
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** –
- **Projekt:** BAL
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BAL6 (2026-10-25) hat Bot, Ziele und den Zielschlüssel (mit Grad) geändert. `testdata/balance/baseline.json` ist noch der alte Stand, der Vergleich meldet daher alles als neu.

## Ziel

Die Baseline passt zum Stand nach BAL6.

## Beteiligte und Zielgruppen

Agent REG.

## Anforderungen

- Voller Lauf über `sim_test` (100 Seeds, geschätzt 35–40 min), danach `balance:baseline` mit Begründung im Commit.

## Nicht-Ziele

Wertänderungen.

## Regeln und Einschränkungen

Testläufe nur über `sim_test` (B-348).

## Beispiele

–

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** Der Baseline-Vergleich nach BAL6 meldet keine falschen Abweichungen.

## Offene Fragen

keine

## Notizen

2026-10-29 (Claude, Sichtung): verworfen. Die Vergleichsbasis ist jetzt die benannte Baseline `M1` (B-640), nicht mehr `testdata/balance/baseline.json`.
