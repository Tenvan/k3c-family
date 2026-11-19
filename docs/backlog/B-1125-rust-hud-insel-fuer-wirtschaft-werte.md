# B-1125 · Rust-HUD rechnet Beutel-Obergrenze und Tausch-Hinweis ohne Insel (Abweichung von Go)

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** RU63
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review RU53.5: `hud/state.rs` (`purse_max_of`) und `hud/hints.rs` (`trade_at`, `train_at`) rufen die Sim mit `None` als Insel, weil `Frame`/`Stage` keine Insel-Daten tragen. In Go bekommt das HUD die lebende Stufen-Welt mit gesetztem `w.island` (`native/k3c-game/local/step.go:244`): Dev-Bonus (`stock_bonus`) zählt zur Beutel-Obergrenze, `resource_full` kennt die Lagerkapazität. In Rust zeigt die Spielerkarte bei Dev-Bonus ein zu kleines Maximum, und der Hinweis „Kaufen“ erscheint bei vollem Lager, obwohl die Sim nichts tut. Der Kommentar im Code „wie Go bei w.island == nil“ in RU53.3 war falsch.

## Ziel

Das HUD rechnet Beutel-Obergrenze und Tausch-/Ausbilden-Hinweis mit derselben Insel-Sicht wie die Sim (Go-gleich).

## Beteiligte und Zielgruppen

Spieler am PC; Second Worker (RU52, `k3c-sim`, `local/peer.rs`).

## Anforderungen

Go ist das Orakel. Die Sim spiegelt die lesbaren Teile der Insel in jede Stufen-Welt (`World::island_view`, `#[serde(skip)]`, gesetzt in `Island::sync_stock`, Vorbild `island_events`): Stufenzahl, Lager der anderen Stufen, `stock_bonus`, `cheap_build`, Lair-Sperre. `World::island_ref_view()` baut daraus allokationsfrei einen `IslandRef` (nur lesende Felder). Das HUD (`hud/state.rs`, `hud/hints.rs`) nutzt ihn statt `None`. Kein Eingriff in Spielregeln, Protokoll oder Spielstand (Spiegel ohne JSON).

## Nicht-Ziele

Spielregeln, Protokoll, Spielstand-Format; Anzeige-Naht `props/simview.rs` (B-1107).

## Regeln und Einschränkungen

Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen; RU42 (kein unwrap/expect/panic außerhalb Tests, keine Allokation je Bild); keine neue Abhängigkeit.

Planungsangaben:

- **Bereich:** `rust/crates/k3c-sim/src/state/island_ref.rs`, `rust/crates/k3c-sim/src/state/types_world.rs`, `rust/crates/k3c-sim/src/island/mod.rs`, `rust/crates/k3c-game/src/hud/state.rs`, `rust/crates/k3c-game/src/hud/hints.rs`, `rust/crates/k3c-game/src/hud/purse_tests.rs`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Dev-Bonus Lager gesetzt: Spielerkarte zeigt das erhöhte Maximum. Volles Lager am Händler: kein Hinweis „Kaufen“.

## Ausnahme- und Fehlerfälle

Ohne Insel (Standbild, Replay): `None` bleibt der Fallback.

## Akzeptanzkriterien

- **AC-01** Test: Welt mit `stock_bonus > 0` zeigt das erhöhte Beutel-Maximum im HUD.
- **AC-02** Test: volles Lager am Händler ergibt keinen Kaufen-Hinweis.
- **AC-03** Test in `k3c-sim`: `sync_stock` setzt den Spiegel je Stufe (Lager der anderen Stufen, Bonus, Lair-Sperre); `island_ref_view()` ist `None` ohne Insel.
- **AC-04** Ralf sieht bei Dev-Bonus Lager das erhöhte Maximum auf der Spielerkarte (manuelle Abnahme).

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
