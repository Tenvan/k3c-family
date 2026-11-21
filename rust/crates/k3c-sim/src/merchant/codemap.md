# rust/crates/k3c-sim/src/merchant/

## Responsibility

Händler (Port von `engine/sim/merchant.go`): einer je Insel, nur im Hub der Tiefe `hub.merchant.onlyDepth`. Er handelt ein Material; Spieler kaufen
dafür mit Gold oder verkaufen aus dem Vorrat.

## Design

- `mod.rs` (einzige Datei): `MERCHANT_ID` (-1), `merchant_dawn` (Ankunft an jedem `everyDays`-ten Tag, mit Taverne `everyDaysWithTavern`; Abreise
  nach `stayDays`), `merchant_arrive`, `merchant_leaves`, `damage_merchant`, `tavern_on_island`, `merchant_x`, `merchant_at`, `pay_merchant`,
  `refund_merchant`.
- Material per `w.rng` aus den freigeschalteten (Hub-Stufe ≥ Stufe in `economy.merchant.materials`). Zwei Zahlziele: Kaufen (`+buyOffsetUnits`) und
  Verkaufen (`+sellOffsetUnits`, A halten).
- Zustand: `Merchant` in `state/shared.rs`, `MerchantSave` im Insel-Spielstand, `MerchantData` in `k3c_data::economy`; hier nichts doppelt.

## Flow

Bei `dawn` entscheidet `merchant_dawn` über Ankunft oder Abreise; `pay_merchant` wird vom Spielerschritt aufgerufen, bucht Gold und Vorrat
(`add_stock`, `give_gold`, `resource_full`) und emittiert Ereignisse. Überfälle laufen über `merchant_raid_start`/`_end` (`links`).

## Integration

- Abhängigkeiten: `economy`, `island::storage`, `links`, `state`, `world` (`hub_of`, `vals`).
- Daten: `data/economy.json › merchant`, `data/hub.json`.
- Konsumenten: `world/cycle.rs`, `economy/players.rs`, `events/merchant.rs`, `island/save_v5.rs`.
