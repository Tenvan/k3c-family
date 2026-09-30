# B-072 · depguard prüft die Schichtgrenze auch für engine/rng

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

SP04 hat `engine/rng` angelegt. Die Regel `sim-ohne-netz` in `.golangci.yml` gilt nur für `engine/sim` und
`engine/level`. Ein Import von `engine/room` oder `engine/net` in `engine/rng` bliebe also unbemerkt. Heute nutzt
`engine/rng` nur die Standardbibliothek.

## Ziel

Die Automatik meldet es, wenn die Simulations-Bausteine (`engine/sim`, `engine/level`, `engine/rng`) Räume oder Netz importieren.

## Beteiligte und Zielgruppen

Entwickler oder Agent.

## Anforderungen

- `sim-ohne-netz` in `.golangci.yml` gilt auch für `**/engine/rng/**`.

## Nicht-Ziele

Neue Schichtregeln.

## Regeln und Einschränkungen

`docs/arbeitsweise.md` › Komplexitäts-Budget (Schichtgrenzen); Domäne INF.

## Beispiele

`import "k3c/engine/net"` in `engine/rng` → `golangci-lint run` meldet depguard.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Konfiguration.

## Akzeptanzkriterien

- **AC-01** Ein Probe-Import von `k3c/engine/net` in `engine/rng` lässt `npm run check:go` scheitern.

## Offene Fragen

keine

## Notizen

Entstanden in SP04.2 (2026-09-30).
