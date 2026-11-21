# rust/crates/k3c-game/src/

## Responsibility

Wurzel des Crates `k3c-game`: Bibliothek (`lib.rs`) und Binary-Einstieg (`main.rs`). Beide sind schlank, die Arbeit
liegt in den Unterordnern.

## Design

- `lib.rs`: nur die Modulliste (`actors`, `art`, `audio`, `demo`, `devtools`, `devwin`, `fx`, `game`, `hud`, `input`,
  `local`, `menu`, `props`, `replay`, `still`, `view`, `world`) und ein Smoke-Test (`crate_baut`).
- `main.rs`: `main() -> ExitCode` verteilt nach dem ersten Argument. `demo` (`demo::parse_args`, `demo::run`), `still`
  (`still::parse_args`, `still::run`), `replay` (`replay::cli::parse_args`, `replay::run::run`, Exit 2 bei Aufruffehler),
  `licenses` (druckt `menu::licenses::license_text()`), `play` oder kein Argument (`game::run::play`, davor
  `game::workdir::anchor_to_package`). Beginnt das erste Argument mit `-`, geht alles an `play`; ein unbekanntes Wort
  ergibt `USAGE`.
- Unterordner: [actors](actors/codemap.md), [art](art/codemap.md), [audio](audio/codemap.md), [demo](demo/codemap.md),
  [devtools](devtools/codemap.md), [devwin](devwin/codemap.md), [fx](fx/codemap.md), [game](game/codemap.md),
  [hud](hud/codemap.md), [input](input/codemap.md), [local](local/codemap.md), [menu](menu/codemap.md),
  [props](props/codemap.md), [replay](replay/codemap.md), [still](still/codemap.md), [view](view/codemap.md),
  [world](world/codemap.md).

## Flow

Prozessstart → `main` → Unterbefehl → `Ok` (Exit 0) oder `Err` (Meldung „Fehler: …“, Exit 1); `play` liefert den
`ExitCode` selbst.

## Integration

- Importiert nur aus der eigenen Bibliothek (`k3c_game::{demo, game, menu, replay, still}`).
- Der Build setzt optional `K3C_VERSION` (Task `rust:game:exe`).
