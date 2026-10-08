# B-286 · TestTickReiheJeRaum schlägt im Gesamtlauf gelegentlich fehl

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** NT1
- **Projekt:** –
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`cmd/k3c-load/load_test.go` › `TestTickReiheJeRaum` schlug am 2026-10-05 im Gesamtlauf `task check:all` einmal fehl („test-load-reihe-0: 61 Ticks, Code "GMFS", 1 Fehler“). Einzeln lief der Test dreimal grün, der zweite Gesamtlauf auch. Nebenher liefen Vite, Spielserver und k3c-dev; vermutet wird ein Zeitproblem unter Last (ungeprüft).

## Ziel

Der Test ist unter Last stabil, damit die Release-Checkliste nicht durch einen Zufall rot wird.

## Beteiligte und Zielgruppen

Entwickler und CI; Release-Checkliste (`docs/arbeitsweise.md` › Release).

## Anforderungen

- Fehlerquelle im Lastlauf erkennen und beheben; den Test nicht einfach lockern.

## Nicht-Ziele

Änderungen am Lastwerkzeug über den Test hinaus.

## Regeln und Einschränkungen

Domäne SRV; Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

`task check:all` auf einem ausgelasteten Rechner → grün.

## Ausnahme- und Fehlerfälle

nicht relevant: Es geht nur um die Stabilität eines Tests.

## Akzeptanzkriterien

- **AC-01** Die Ursache des einen Fehlers im Lauf ist benannt (Log oder Fehlerart in `r.Errors`).
- **AC-02** `go test ./cmd/k3c-load -run TestTickReiheJeRaum -count=20` ist grün, während parallel `task check` läuft.

## Offene Fragen

keine

## Notizen

Gefunden beim Release v0.7.0.
