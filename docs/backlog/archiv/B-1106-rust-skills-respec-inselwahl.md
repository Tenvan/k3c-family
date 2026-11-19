# B-1106 · Skills lernen, Respec und Zielinsel-Wahl fehlen im Rust-Spiel

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** RU53
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-19
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Im Rust-Spiel lässt sich kein Skill lernen. Stand 2026-11-19 (Planer-Prüfung): `k3c-room` hat `learn`, `respec`, `choose_island` (`room/skills.rs`) und `local/actions.rs` ruft sie bereits (RU21/RU24/RU50). Es fehlen die HUD-Shims in `rust/crates/k3c-game/src/hud/pending.rs`: `available_points`, `can_learn`, `can_respec` liefern 0/false (Skill-Menü zeigt nichts Lernbares), dazu `purse_max_of`, `train_at`, `trade_at` als neutrale Werte (Go: `sim.*`). Folge: `e2e_flow` weicht in 6 Stationen vom Go-Welt-Hash ab (`KNOWN_DIVERGENT`) und verfehlt `KNOWN_DEVIATIONS` (Spieler 2 lernt keinen Skill). Vermuteter Soft-Lock bei der Zielinsel-Wahl nach einem Inselsieg (Review RU48.4, ungeprüft). Die Story-Shims (`story_*`, `WorldDef`, RU19) sind nicht Teil dieses Tickets.

## Ziel

Die HUD-Shims rufen `k3c_sim::monarch::{available_points, can_learn, can_respec}` und `k3c_sim::state::{purse_max_of, train_at, trade_at}`; Lernen, Respec und Zielinsel-Wahl funktionieren im lokalen Rust-Spiel wie in Go (`engine/room/skills.go`, `native/k3c-game/local/actions.go`); `e2e_flow` läuft ohne `KNOWN_DIVERGENT` und `KNOWN_DEVIATIONS`.

## Beteiligte und Zielgruppen

Spieler am PC mit Pad (zwei Spieler am Sofa); Claude (Hauptrechner) setzt um. 🧑 liest das Ergebnis.

## Anforderungen

- Verhalten wie Go; Go ist das Orakel, bei Abweichung gilt Go. Keine Änderung an Spielregeln, Protokoll, Spielstand-Format.
- Jede Mechanik mit 2 Spielern gleichzeitig (Skill-Menü je Spieler).
- Deterministisch; Goldens und die Go-Referenz (`testdata/rust/e2e/flow.json`) werden nie geändert.
- Dateien <= 400 Zeilen, Funktionen <= 60 Zeilen, keine neuen Abhängigkeiten.

## Nicht-Ziele

Story-Shims (RU19), Dev-Rolle (B-1111, RU52), Änderungen in `k3c-sim` (läuft in RU52; ist dort etwas nötig, als Ticket melden), Go-Code.

## Regeln und Einschränkungen

CLAUDE.md, Entscheidung 008. Sim und Balancing gesperrt. Planungsangaben:

- **Bereich:** `rust/crates/k3c-room/src/room/skills.rs`, `rust/crates/k3c-room/src/room/skills_tests.rs`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/local/actions.rs`, `rust/crates/k3c-game/src/local/actions_tests.rs`, `rust/crates/k3c-game/tests/e2e_flow.rs`, `docs/messung/rust-bildvergleich.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

Spieler 2 hat einen Skill-Punkt und öffnet das Skill-Menü: der Skill ist wählbar und wird gelernt (`monarch.skills.len() == 1`). Respec an der Burg am Tag: Punkte zurück. Inselsieg mit offenem Wechselpunkt: Seekarte lässt eine Zielinsel wählen, der Raum wechselt.

## Ausnahme- und Fehlerfälle

Nachts oder fern der Burg: Respec abgelehnt wie in Go. Zu wenig Punkte oder Voraussetzung fehlt: Lernen abgelehnt mit Meldung. Zielinsel ohne Krone oder ungültig: abgelehnt.

## Akzeptanzkriterien

- **AC-01** `hud/pending.rs` enthält keine Shims für `available_points`, `can_learn`, `can_respec`, `purse_max_of`, `train_at`, `trade_at` mehr; `state_labels.rs`, `state.rs`, `hints.rs` rufen die Sim.
- **AC-02** Ein Test lernt je Spieler (2 Spieler) einen Skill über das HUD/`local` und prüft Respec; er stimmt mit den Go-Tests (`engine/room/skills_test.go`) überein.
- **AC-03** Zielinsel-Wahl über Seekarte und `actions_choose_island` ist in einem Test belegt; Soft-Lock nach Inselsieg ist geprüft (behoben oder als nicht vorhanden belegt).
- **AC-04** `KNOWN_DIVERGENT` und `KNOWN_DEVIATIONS` in `e2e_flow.rs` sind leer bzw. entfernt, `cargo test -p k3c-game --test e2e_flow` grün, Referenz unverändert.
- **AC-05** `task rust:lint` grün; Review-Session ohne offene schwere Befunde.

## Offene Fragen

keine (Default autonom entschieden: Room-Aktionen existieren schon, Umfang ist die HUD-Anbindung plus Test; 2026-11-19)

## Notizen

Go: `engine/room/skills.go`, `sim.LearnSkill`, `sim.CanLearn`, `sim.CanRespec`. Messung: `docs/messung/rust-bildvergleich.md`.
