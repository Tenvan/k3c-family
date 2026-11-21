# rust/crates/k3c-sim/src/hermit/

## Responsibility

Einsiedler (RU62.10, B-972): Eine Welt mit Eintrag hat einen besonderen Bürger. Wer an seiner Hütte `price` Münzen einwirft, löst den Bau
seines Bauwerks auf dem Hub-Platz aus; solange es steht, wirkt sein Bonus auf der ganzen Insel. Neue Mechanik nur in Rust.

## Design

- `mod.rs`: `Hermit` (Zustand), `BUILDING_STATE` (`hermitBuilding`), `hut_x` (Hütte aus Insel-Seed über `k3c-rng`), `Island::enable_hermit`,
  `hall_site`, `stage_at_depth`. Hütte in Tiefe 1 auf Inseln mit mindestens zwei Stufen, Platz in Tiefe 0.
- `pay.rs`: `step_hut` (Einwurf, Spielerschritt aus `economy/players.rs`); abgelehnt bei zu wenig Münzen, schon bezahlt oder Stufe nicht erreicht.
- `step.rs`: `step_hermit` (Bau starten, Bauzeit `buildSeconds`, Wiederaufbau zerstörter Bauwerke bei Tag).
- `bonus.rs`: `damage_factor` (Dojo), `heal_walls` (Schrein bei Tagesanbruch), `hall_seconds`, `is_hall`.
- `validate.rs`: `hub_with_hall`, `check_hall_site`: hängt den Platz an die hub.json und lässt die Hub-Prüfung laufen.
- Gespeichert werden Platz und `Island::hermit.paid` (`IslandSave::hermit_paid`, Version 12).

## Flow

Der Einwurf (`pay.rs`) markiert die Zahlung; `step_island` ruft `step_hermit` nach den Stufen und startet den Bau. Je Takt veröffentlicht
`prepare_hermit` Faktor und Heilanteil in `World::hermit`, die Kampf- und Heilcode liest.

## Integration

- Abhängigkeiten: `state` (`HermitState`, `Site`), `island`, `hub` (Prüfung), `k3c-rng`, `k3c-data` (`hermits.rs`).
- Daten: `data/hermits.json` (`worlds`, Hütte, Preis, `hubSite`).
- Konsumenten: `economy`, `island`, `combat` (Schadensfaktor); Anzeige über `state/hermit_view.rs`.
