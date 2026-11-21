# rust/crates/k3c-rig/src/

## Responsibility

Einstieg des Crates: teilt es in Rechnung (`rig`) und Zeichnen (`paint`).

## Design

- `lib.rs` (4 Zeilen): `pub mod paint; pub mod rig;`.
- Untergeordnete Karten: [`rig/`](rig/codemap.md) (reine Rechnung), [`paint/`](paint/codemap.md) (Laden und Zeichnen).

## Flow

`paint` ruft `rig` für Posen, Matrizen und Bänder und gibt das Ergebnis an `Canvas` weiter.

## Integration

- `paint` hängt von `k3c-platform` ab, `rig` nicht.
- Konsument: `k3c-game`.
