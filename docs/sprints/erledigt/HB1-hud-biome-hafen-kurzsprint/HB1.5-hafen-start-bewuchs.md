# HB1.5 · Hafen als Start, 1 bis 2 Biome vom Lager, Hafen vom Wald umgeben

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/hb1
- **Abhängig von:** HB1.4
- **Tickets:** B-1188
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Neue Spiele starten am Hafen; der Seed legt 1 bis 2 Biome zwischen Lager und Hafen; der Hafen ist von Bäumen umgeben (Dichte einstellbar) und muss freigeräumt werden.

## Kontext

Ticket B-1188. Inselaufbau, Start und Hafen in `rust/crates/k3c-sim`, `k3c-room`, `k3c-level`; Regeln `docs/rules/stufen.md`, `docs/rules/welten.md`. Zuerst grep nach Hafen/harbor/start. Dichte als Datenwert in `data/`. Keine Rücksicht auf alte Spielstände. Go-Engine eingefroren (Entscheidung 013).

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-room/**`
- `rust/crates/k3c-level/**`
- `data/**`
- `testdata/**`

## Nicht-Ziele

Neue Inseln, Seekarte.

## Schritte

1. Startpunkt neuer Spiele auf den Hafen setzen.
2. Zahl der Biome zwischen Lager und Hafen (1 bis 2) aus dem Seed.
3. Hafen mit Bäumen umgeben, Dichte als Datenwert, Test.
4. Tests über viele Seeds.

## Fertig, wenn

- [ ] AC-01: Neue Spiele starten am Hafen.
- [ ] AC-02: 1 bis 2 Biome zwischen Lager und Hafen (Test über viele Seeds).
- [ ] AC-03: Hafen von Bäumen umgeben, Dichte einstellbar (Test).
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

AC-01 geprüft: `add_island_player` stellt neue Monarchen an den Steg (`island/harbor_start.rs` `start_x`, ±3 Units); Test `neue_spiele_starten_am_hafen`. Beim Laden stehen sie wie bisher an der Burg. AC-02 geprüft: `near_harbor` setzt den Hafen-Slot aus dem Seed auf −1 oder −2 (Seite wie in den Weltdaten, vorher −7); Test über 40 Seeds (beide Abstände kommen vor). AC-03 geprüft: `plant_harbor_trees` bewachsen den Hafen-Chunk mit Dichte `TREES_PER_UNIT` 0,8 je Unit bis auf einen Startstreifen ±5 Units; Test `hafen_ist_bewachsen_bis_auf_den_startstreifen`. Alles hinter dem nativen Schalter `WorldValues.native_rules` (vorher `camp_guards`, jetzt auch für Hafen). Angepasste Tests (Hafen nicht mehr bei −7, Start am Hafen statt an der Burg): `ship_tests`, `save_tests`, `views_tests`, `local/actions_tests` (Respec nur an der Burg). k3c-sim, k3c-room, k3c-game grün außer den schon roten `eingebettete_welten_gueltig` und `insel_nach_1000_schritten_wie_go`; clippy sauber. Abweichungen: Dichte ist eine Konstante im Code, kein Datenwert (Go-Datenbaum-Vergleich würde ein neues Feld ablehnen); „muss freigeräumt werden“ heißt hier nur: Winkelbäume stehen, Bäume lassen sich fällen, kein Bau-Sperre am Schiff. Weltdaten (`slot: -7`) unverändert. Für Ralf im Spiel prüfen: Abstand Hafen-Burg, Startgefühl am Steg, ob der Steg frei genug ist.
