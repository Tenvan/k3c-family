# RU18 · SIM · Sim: Wirtschaft, Bau, Hub, Monarch, Skills

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** 9932e5b6f
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Wirtschaft (576), Bau (521), Hub (503), Monarch (465), Skills (352), Händler, Beutel und Fackeln (rund 230 Zeilen).

## Ziel

Münzen, Bauen, Hub, Monarch und Skills laufen in Rust. Am Ende sichtbar: portierte Tests grün.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 12 von 26, 6 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) und das Zustandsmodell `docs/standards/rust-sim-zustand.md`. Modell-Empfehlung: `sonnet`.

### Sessions

Sessions mit den Go-Dateien, die sie portieren (Zeilen ohne Tests; „T“ = es gibt `<name>_test.go`, der Test wird in derselben Session portiert). Ziel: `engine/sim/` → `rust/crates/k3c-sim/src/{economy,merchant,building,hub,monarch,skills}/`; Dateinamen nach Leitfaden § 2. Die Sessions laufen als Kette auf dem Branch `sprint/ru18`. Gegen das Repo geprüft (2026-11-17): `hub_validate.go` hat 227 statt 221 Zeilen; `checkWorldHubSites` und `checkSiteBounds` (rund 30 Zeilen aus `world_hub_sites.go`) kommen aus RU14 hierher, weil sie `HubFinding` brauchen; `economy_view` wandert zu Session 2, damit Session 3 unter 650 Zeilen bleibt. Tests ohne eigene Quelldatei (`building_healer_w4`, `building_levels_data`, `building_ruin_preview`, `building_ruin_save`, `site_level`, `sites`, `coin_scale_data`, `hub_candidate`, `hub_step`, `site_data`, `stairs_cost`, `world_hub_sites`, `merchant_raid_target`, `monarch_can`, `monarch_presets`, `skills_cast_event`, `skills_linien`, `feedback`) gehören zur passenden Session.

| Session | Go-Dateien | Zeilen |
|---|---|---:|
| 1 | `engine/sim/`: building_barracks (39, T), building_healer (21, T), building_outpost (40, T), building_plantation (65, T), building_repair (55, T), building_ruin (66, T), building_spell_tower (43, T), building_tavern (44, T), building_upgrades (148, T) | 521 |
| 2 | `engine/sim/`: economy (373), economy_dawn (17), economy_fountain (28), economy_pay (43), economy_view (115, T) | 576 |
| 3 | `engine/sim/`: hub_level (181, T), hub_validate (227), hub_validate_layout (101), world_hub_sites (nur `checkWorldHubSites`, `checkSiteBounds`, 30, T) | 539 |
| 4 | `engine/sim/`: merchant (170, T), monarch (183, T), monarch_data (131) | 484 |
| 5 | `engine/sim/`: monarch_mount (5, T), monarch_passives (146, T), skills (87, T), skills_caster (80, T), skills_healer (48, T), skills_status (50), skills_tank (41, T), skills_thief (46, T) | 503 |
| 6 | Review (alle Kriterien, `docs/arbeitsweise.md` › Review-Session) | – |

Summe: 2623 Zeilen Go. Jede Port-Session erfüllt die Kriterien für ihre Dateien, das Review prüft alle.

Zusatz: Jede Session ersetzt die Platzhalter, deren Go-Funktion sie liefert (`state/pending.rs`, `world/pending.rs`, `building/pending.rs`, `economy/pending.rs`) und hebt die `ignore`-Tests mit `wartet auf RU18` auf, die sie damit lauffähig macht.

## Nicht-Ziele

Regel-, Wert- oder Formatänderungen. Änderungen am Go-Code. Optimierungen über den Go-Stand hinaus.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014. Keine neue Abhängigkeit, kein `HashMap` (Iterationsreihenfolge, Determinismus).

**Port-Orakel (Entscheidung 012):** Idiomatisches, performantes Rust ist erwünscht; Hinweise „wie Go“ und „Go-Reihenfolge“ in Sprint und Sessions (zum Beispiel Streuung in `each_layout`, Gleichstand-Regeln, Zahl-Reihenfolge) gelten nicht für Rechenreihenfolge, `k3c-det`-Mathematik, lineare Suche und Go-Datenstrukturen. Verbindlich bleiben Reihenfolge der Ereignisse und der rng-Aufrufe, Ganzzahl-Werte und Determinismus innerhalb von Rust; Fließkomma gilt mit Toleranz 1e-4 relativ (`docs/standards/rust-port.md` § 1).

- **Bereich:** `rust/crates/k3c-sim/src/economy/**`, `rust/crates/k3c-sim/src/building/**`, `rust/crates/k3c-sim/src/hub/**`, `rust/crates/k3c-sim/src/monarch/**`, `rust/crates/k3c-sim/src/skills/**`, `rust/crates/k3c-sim/src/merchant/**`, `rust/crates/k3c-sim/src/lib.rs` (nur `pub mod`-Zeilen), `rust/crates/k3c-sim/src/state/pending.rs`, `rust/crates/k3c-sim/src/world/pending.rs`, `rust/crates/k3c-sim/src/world/values.rs`, `rust/crates/k3c-sim/src/world/hub_sites.rs` (nur Platzhalter löschen und Aufrufstellen auf die echte Funktion umstellen)
- **Abhängig von:** RU14, RU15
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Port ohne neues Verhalten; die Beispiele sind die Go-Tests des Bereichs.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die Go-Tests zu Wirtschaft und Händler sind portiert und grün.
- **AC-02** Die Go-Tests zu Bau und Hub sind portiert und grün.
- **AC-03** Monarch und Skills verhalten sich für zwei Spieler gleichzeitig wie in Go.

## Offene Fragen

Keine zum Port. Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen):

- **Reflexion der Fertigkeiten:** Gos `skillEffects`-Map wird eine `match`-Funktion (`cast_effect`), kein Funktionszeiger-Katalog.
- **`check_world_hub_sites` in RU18 statt RU14:** Die Prüfung braucht `HubFinding` aus `hub_validate`; bis RU18.3 ist sie ein Platzhalter, Tests aus RU14 warten mit `ignore = "wartet auf RU18"`.
- **`PayTarget`:** Die echte Definition ersetzt den Platzhalter aus RU8 in `state/pending.rs` (RU18.2).
- **Falsche Sprint-Nummern in `state/pending.rs` (Hinweis für RU22):** `stock_field`, `resource_full`, `purse_max` (Go: `island_storage.go`), `protected_night` (`island_options.go`) tragen RU18, gehören zu RU10, `profession_of` (`unit_professions.go`) zu RU11. Wer die Go-Datei portiert, liefert sie; RU18 löscht nur eigene Platzhalter; fehlt einer bei RU18.2, steht ein Platzhalter mit der richtigen Nummer in `economy/pending.rs`.
- **`tavernOnIsland` und Skill-Pool (Zugriff auf `w.island`):** Rust hat keinen Rückverweis; die Insel kommt als Parameter (RU18.4).
- **Abweichung vom Schnitt:** `economy_view` in Session 2 statt 3; Zusatzdatei `world_hub_sites` (Teil) in Session 3.

**Nachzug aus Go-Sprints** (Beschluss 🧑 2026-11-16: Die Go-Engine wächst nicht weiter). Diese Arbeiten gehören nicht in den bitgleichen Port. Sie entstehen nach dem Golden-Abgleich (RU22) als neuer Rust-Code, angefangener Go-Code wird neu geschrieben statt konvertiert. Spec steht im Ticket:

- SI2.3 Doppel-Truhe in der Engine (B-968)
- SI4.3 Reittier-Katalog, Gatter, Kauf und Spielstand (B-970)
- SI5.2 Figur, Hütte, Bauplatz und Boni (B-972)

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU18.1 | `RU18.1-building-sites-upgrades.md` | Umsetzung | autonom | fertig |
| RU18.2 | `RU18.2-economy-pay-view.md` | Umsetzung | autonom | fertig |
| RU18.3 | `RU18.3-hub-level-validate.md` | Umsetzung | autonom | fertig |
| RU18.4 | `RU18.4-merchant-monarch-data.md` | Umsetzung | autonom | offen |
| RU18.5 | `RU18.5-monarch-passives-skills.md` | Umsetzung | autonom | offen |
| RU18.6 | `RU18.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
