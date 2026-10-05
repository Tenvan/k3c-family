# B-282 · TestGleicherSeedGleicheEingaben scheitert nicht, wenn task check:go parallel läuft

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`cmd/k3c-load/load_test.go` › `TestGleicherSeedGleicheEingaben` verlangt je Bot mindestens 20 Eingaben in einem Lauf von
2 s Wanduhr. Im Review S2.5 scheiterte er einmal in `task check:go` (alle Pakete parallel, Windows): „zu wenige
Eingaben (59, 60)“, der dritte Lauf mit anderem Seed hatte also unter 20. Allein dreimal hintereinander grün
(`go test -count=3 -run TestGleicherSeedGleicheEingaben ./cmd/k3c-load/`).

## Ziel

`task check:go` ist ohne Wiederholung verlässlich grün.

## Beteiligte und Zielgruppen

Entwickler, CI.

## Anforderungen

- Der Test hängt nicht von der Auslastung des Rechners ab (z. B. Mindestzahl aus der Laufzeit ableiten oder Lauf bis n Eingaben statt fester Dauer).

## Nicht-Ziele

Verhalten des Lasttest-Werkzeugs selbst.

## Regeln und Einschränkungen

Deterministisch (Seed), keine längere Laufzeit als heute ohne Grund.

## Beispiele

`task check:go` auf einem ausgelasteten Rechner → Test grün.

## Ausnahme- und Fehlerfälle

nicht relevant (Testcode).

## Akzeptanzkriterien

- **AC-01** `go test -count=20 ./cmd/k3c-load/` parallel zu `go test ./engine/...` ist grün.

## Offene Fragen

keine

## Notizen

–
