# B-1125 · Rust-HUD rechnet Beutel-Obergrenze und Tausch-Hinweis ohne Insel (Abweichung von Go)

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU52
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review RU53.5: `hud/state.rs` (`purse_max_of`) und `hud/hints.rs` (`trade_at`, `train_at`) rufen die Sim mit `None` als Insel, weil `Frame`/`Stage` keine Insel-Daten tragen. In Go bekommt das HUD die lebende Stufen-Welt mit gesetztem `w.island` (`native/k3c-game/local/step.go:244`): Dev-Bonus (`stock_bonus`) zählt zur Beutel-Obergrenze, `resource_full` kennt die Lagerkapazität. In Rust zeigt die Spielerkarte bei Dev-Bonus ein zu kleines Maximum, und der Hinweis „Kaufen“ erscheint bei vollem Lager, obwohl die Sim nichts tut. Der Kommentar im Code „wie Go bei w.island == nil“ in RU53.3 war falsch.

## Ziel

Das HUD rechnet Beutel-Obergrenze und Tausch-/Ausbilden-Hinweis mit derselben Insel-Sicht wie die Sim (Go-gleich).

## Beteiligte und Zielgruppen

Spieler am PC; Second Worker (RU52, `k3c-sim`, `local/peer.rs`).

## Anforderungen

Go ist das Orakel. Eine lesende Insel-Sicht (`stock_bonus`, Stufenzahl, Lager anderer Stufen) über `Stage` oder `Frame` ins HUD, ohne `stage_ref` (nimmt Zustände heraus); Änderung in `k3c-sim` gegebenenfalls nötig (`storages_except` ist privat).

## Nicht-Ziele

Spielregeln, Protokoll, Spielstand-Format.

## Regeln und Einschränkungen

Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen.

## Beispiele

Dev-Bonus Lager gesetzt: Spielerkarte zeigt das erhöhte Maximum. Volles Lager am Händler: kein Hinweis „Kaufen“.

## Ausnahme- und Fehlerfälle

Ohne Insel (Standbild, Replay): `None` bleibt der Fallback.

## Akzeptanzkriterien

- **AC-01** Test: Welt mit `stock_bonus > 0` zeigt das erhöhte Beutel-Maximum im HUD.
- **AC-02** Test: volles Lager am Händler ergibt keinen Kaufen-Hinweis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
