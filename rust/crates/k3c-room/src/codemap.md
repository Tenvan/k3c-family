# rust/crates/k3c-room/src/

## Responsibility

Einstieg und Fehlertyp des Raum-Crates.

## Design

- `lib.rs`: `pub mod fehler; pub mod room; pub mod store;` und ein Test `crate_baut`.
- `fehler.rs`: `Fehler` (ein `enum` je Crate, Meldungstexte wie in Go): `meldung`, `is` (Ersatz für `errors.Is`), `mit_vorsatz`.
- Untergeordnet: [`room/`](room/codemap.md) (Raummodell), [`store/`](store/codemap.md) (Dateispeicher).

## Flow

`room` ruft den `Store` über das Trait aus `room/env.rs` auf; `store` liefert `Fehler` zurück.

## Integration

- Konsument: `k3c-game`.
