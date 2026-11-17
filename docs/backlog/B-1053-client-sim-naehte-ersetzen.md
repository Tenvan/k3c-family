# B-1053 · Sim-Nähte im Rust-Client sind durch die portierte Sim ersetzt

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU30
- **Projekt:** RSU
- **Erstellt:** 2026-11-17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Client-Sprints RU16, RU17, RU20 und RU23 laufen vor den Sim-Sprints RU14, RU15, RU18, RU19. Fehlende Sim-Funktionen stehen deshalb hinter Nähten: `trait SimView` mit `PendingSimView`/`FixedSimView` in `props` (RU16.1, auch von `hud` genutzt), Platzhalter in `props/pending.rs`, `hud/pending.rs`, `fx/pending.rs` (z. B. `aura_of`, `can_learn`, `can_respec`, `learn_skill`, `respec`, `story_*`) und Tests mit `ignore = "wartet auf RU1x"`.

## Ziel

Nach dem Sim-Port ruft der Client die echten Funktionen aus `k3c-sim`; keine Naht und kein Platzhalter bleibt im Client.

## Beteiligte und Zielgruppen

Agenten von RU30 setzen um; das Review RU30 prüft.

## Anforderungen

- `SimView`, `PendingSimView` entfernen; `FixedSimView` bleibt nur, wenn das Standbild-Werkzeug `still` es für Bildvergleiche braucht.
- `props/pending.rs`, `hud/pending.rs`, `fx/pending.rs` leer und gelöscht.
- Alle Client-Tests mit `ignore = "wartet auf RU10|RU14|RU15|RU18|RU19"` laufen ohne `ignore`.

## Nicht-Ziele

Regeländerungen. Änderungen in `k3c-sim`.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/fx/**`, `rust/crates/k3c-game/src/still/**`
- **Abhängig von:** RU18, RU19
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

nicht relevant

## Ausnahme- und Fehlerfälle

Fehlt eine Sim-Funktion nach RU22 noch, bleibt die Naht mit Ticket im Projekt RSU.

## Akzeptanzkriterien

- **AC-01** `grep -rn -E "PendingSimView|pending\.rs|wartet auf RU1[0-9]" rust/crates/k3c-game/src` findet nichts; `cargo test -p k3c-game` grün.

## Offene Fragen

keine

## Notizen

Aus der Planung RSU 2026-11-17 (Rückmeldungen der Planer zu RU16, RU20, RU23). Umsetzung als Teil von RU30, sinnvoll in RU30.1 oder als eigene Session vor dem Review.
