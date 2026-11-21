# rust/crates/k3c-game/src/props/

## Responsibility

Zeichnet Bauten, Natur und Objekte aus dem Spielzustand (Port von `native/k3c-game/props/`): Burg mit Wachstumsreihe, Bauplätze mit Stufe, Schaden und Ruine, Mauern, Baustellen, Preis-Anzeigen und Wartegründe, Bäume, Felsen, Adern, Plantage, Portal, Hafen, Fracht, Truhen, Münzen, Händler und Einsiedler. Fackeln zeichnet `world`.

## Design

- Zustand: `listen.rs` (`Infos`, `HubState`, `StageInfo`, `listen`: abgeleiteter Zustand je Update), `simview.rs` (Trait `SimView`: Naht zu Sim-Funktionen), `pending.rs` (`LiveSimView`, `Economy`, `CombatSpot` als Platzhalter), `data.rs` (`Values` aus `data/`: Namen, Kosten).
- Auswahl-Regeln: `buildings.rs` (`Look`, `site_look`, `stage_anim`), `nature.rs`, `sites.rs` (`nearest_tags`, Regale, Preis-Tafeln), `assets.rs` (`level_assets`: Papier-Assets je Level).
- Zeichenliste (rein, getestet): `plan.rs` (`Planner`, `Item`, `Shape`, Trait `Sheets`) mit Teilen `plan_castle`, `plan_sites` (+ `_cargo`, `_gate`, `_marks`), `plan_entities`, `plan_objects`, `plan_combat`, `plan_cost`, `plan_hermit`, `plan_merchant`; `decor.rs`/`decor_glow.rs` (Natur-Eigenschaften), `coin_fountain.rs`.
- Zeichnen: `paint.rs` (`Painter` über `k3c_platform`).

## Flow

`listen` rechnet je Update die `Infos`; je Zelle baut `Planner` die Zeichenliste (liegt auf `Layer::BUILDINGS`), `Painter::paint` zeichnet sie. Assets des Levels meldet `level_assets` über die `view`-Rolle `AssetSource`.

## Integration

- Abhängigkeiten: `k3c_sim` (Zustand, stark), `k3c_data`, `k3c_level`, `k3c_platform`, `view`, `art`, `actors`.
- Konsumenten: `game::packages`/`parts`, `still::compose` (`build_props`, `props_library`), `hud` (Preis- und Asset-Hilfen), `replay::neutral` (`NeutralSimView`).
