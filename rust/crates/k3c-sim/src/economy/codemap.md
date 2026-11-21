# rust/crates/k3c-sim/src/economy/

## Responsibility

Monarchen, Münzen und alles, was man bezahlt (Port von `engine/sim/economy*.go`): Zahlziel suchen, Münzen zahlen und
erstatten, Spielerbewegung, Einsammeln, Bauplätze, Morgengold, Gold-Fontäne und Wirtschaftsstand für das Protokoll.

## Design

- `mod.rs`: `PayTarget`, `find_pay_target` (Bauplatz vor Landstreicher vor Ressource …), `site_payable`, `offer_target`,
  `key_of` (Schlüssel wie `"site:3"`); Re-Exporte der Untermodule.
- `players.rs`: `step_players` (Spieler 0, 1, …), `move_player`, `to_hub`, `collect_coins`, `collect_pickups`.
- `pay.rs`: `pay_one_coin`, `pay_site`, `pay_to_target`, `refund_pending`, `refund_site`.
- `sites.rs`: `give_gold`, `scatter_coins`, `add_stock`, `step_sites` (bezahlte Plätze ziehen Material, ruft `step_crafts`).
- `dawn.rs`: `dawn_gold`, `pay_dawn_income`; `fountain.rs`: `fountain_coins`, `scaled_gold`; `util.rs`: `nearest`,
  `can_afford`, `spend`, `clear_pending`.
- `view.rs`: `Economy`, `HubUpgrade`, `economy_of`, `site_gold_left`, `hub_wait_state`, `WAITING_NEXT_ISLAND`; liest nur ab.
- `links.rs`: Import-Stelle (`merchant`, `monarch`, `skills`, `hub`, `events`).

## Flow

`world::step::step` ruft `step_players` (Bewegung, Zahlen, Einsammeln) früh im Tick und `step_sites` danach. Eine gehaltene
Zahltaste gibt im Takt Münzen an das nächste bezahlbare Ziel in Reichweite; ohne Ziel fällt die Münze auf den Boden.
Kampfziele (`combat::spots`), Händler und Hub-Ausbau hängen als Zahlziele an `pay_to_target`.

## Integration

- Abhängigkeiten: `state`, `building` (`upgrades`), `combat`, `merchant`, `hub`, `monarch`, `skills`, `events`, `coop` (Truhen).
- Konsumenten: `links.rs` (`scaled_gold`, `scatter_coins`), `world`, `enemies` (Drops), `k3c-game` (HUD liest `economy_of`).
- Daten: `data/economy.json`, `data/buildings.json`.
