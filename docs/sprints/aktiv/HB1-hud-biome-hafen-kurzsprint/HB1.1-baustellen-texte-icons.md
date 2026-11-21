# HB1.1 · HUD: Baustellen-Texte kleiner, Kosten als Icons, Ausbau x/n

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/hb1
- **Abhängig von:** –
- **Tickets:** B-1186
- **Kriterien:** AC-01

## Ziel

Baustellen im Spiel zeigen kleineren Text, Kosten als Ressourcen-Icons und den Ausbau als x/n.

## Kontext

HUD-Texte der Baustellen liegen in `rust/crates/k3c-game/src/hud/` (Text-Slots in `hud/paint.rs`, Zustand `hud/state.rs`, Zeichnen `hud/drawworld.rs`); Ressourcen-Icons gibt es als Papierschnitt-Icons (Gruppe `icons`). Ticket B-1186. Fallstricke: keine Allokation je Bild, Split-Screen mit zwei Spielern, Dateien ≤ 400 Zeilen. Zuerst die Stelle suchen, die den Baustellen-Text zeichnet (grep nach Kosten/Ausbau).

## Erlaubte Dateien

- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/props/**`

## Nicht-Ziele

Gebäudename (HB1.2), Händler (HB1.2), neue Icons (vorhandene nutzen).

## Schritte

1. Stelle des Baustellen-Texts finden, Schriftgröße verkleinern.
2. Kosten als Icon plus Zahl statt Wort zeichnen (Rückfall Kurztext ohne Icon-Sheet).
3. Ausbau-Text auf x/n kürzen.
4. Tests für Text und Icon-Wahl.

## Fertig, wenn

- [ ] AC-01: Baustellen-Texte kleiner, Kosten als Icons, Ausbau als x/n; Tests grün.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-01 geprüft: Baustellen-Anzeige neu in `props/plan_cost.rs` (`price_cost`, `coin_rows`, `cost_row`): Kosten als Rohstoff-Icons (`icons/icon-holz` usw., 0,55 Unit) mit Zahl, Name klein (`SMALL` 0,55 statt 1,0 Unit) darüber, ohne Icon-Sheets Kurztext („20 Holz, 5 Stein“). Bauplatz-, Bogen- und Burg-Preis nutzen es; Wartegrund- und „erst roden“-Texte ebenfalls klein (`label_sized`). Ausbau-Titel der Burg ist nur noch „x/n“ (`hub_title`). `paint.rs`: `Item.w` skaliert Sprites gleichmäßig. `site_text` liefert für Preis und Bogen nur noch den Namen. Tests `plan_cost_tests.rs` (Icons, Fallback), `sites_tests.rs` angepasst; `cargo test -p k3c-game --lib` und clippy grün. `task check:diff`: nur die bekannten Planungstests rot (zuletzt kamen Umgebung/Größe der drei neuen HB1-Tickets dazu, behoben).

Nicht geprüft: Aussehen am PC (Icon- und Textgröße vermutet). Abweichung: `props/` darf `hud` nicht importieren, darum steht die Icon-Tabelle als Kopie in `plan_cost.rs`.
