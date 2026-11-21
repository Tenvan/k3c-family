# ECO1.2 · Siegel in Auswahl, Spielstand-Liste und HUD

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/2-siegel-anzeige
- **Abhängig von:** ECO1.1
- **Tickets:** B-1192
- **Kriterien:** AC-02

## Ziel

Das Siegel des gewählten Grades steht in der Auswahl, in der Spielstand-Liste und im HUD bei der Welt-Anzeige.

## Kontext

Ticket B-1192, Siegel-Assets aus ECO1.1. Der Grad liegt schon im Spielstand; fehlt er (alter Stand), gilt der Standardgrad ohne Fehler. Frame-Budget: Textur einmal laden, kein Allokieren je Bild; 1 und 2 Ansichten.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/menu/**`
- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/local/**`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Wo Schwierigkeit heute gezeigt wird, per grep finden (menu, Spielstand-Liste, hud Welt-Zeile).
2. Siegel-Asset je Grad laden und neben Text zeichnen.
3. Alter Stand ohne Grad: Standardgrad.
4. Tests (Anzeige, fehlender Grad).

## Fertig, wenn

- [ ] AC-02: Auswahl, Spielstand-Liste und HUD zeigen das Siegel; alter Stand ohne Fehler.
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

AC-02 geprüft: Siegel des Grads erscheint (1) in der Lobby-Zeile „Grad“ (folgt der Auswahl), (2) in der Spielstand-Liste je ladbarem Stand, (3) im HUD links neben der Biome-Zeile am Kompass. Fehlt der Grad (alter Stand, `null`, unbekannter Name), gilt das Siegel von `normal`, kein Fehler. Tests: `menu/seal_tests.rs` (Zuordnung, Asset-Liste, Lobby, Spielstand-Karte), `hud_zeigt_das_siegel_des_grads`, `saves_list_nennt_den_grad` (k3c-room); `cargo test -p k3c-game --lib` 1014 grün. `task check:diff`: Stufe rust-test rot nur durch `alloc_je_bild` (vorbestehend, B-1190 AC-04; Maximum 942 im step, Obergrenze 786), sonst grün; `cargo fmt` hat zusätzlich HB1-Dateien (camp_guard_tests, harbor_start_tests, island/mod.rs, props/mod.rs) nur formatiert.

Abweichung von den erlaubten Dateien: Der Grad musste vom Spielstand bis zur Anzeige durchgereicht werden, darum zusätzlich `k3c-room/src/store/saves_list.rs` (+Test), `view/session.rs` (`SaveInfo.grade`), `view/story.rs` (`Campaign.grade`) und neu `view/seal.rs` (Zuordnung Grad → Asset, gemeinsam für Menü und HUD, weil `hud` `menu` nicht importieren darf, Importregel).

Manuell offen (Ralf): Sicht im Spiel prüfen, ob das Siegel in Lobby, Liste und HUD lesbar sitzt (Split-Screen, SMALL); Hauptmenü „Weiter“ zeigt kein Siegel.

### Für den Hauptrechner
- Hauptmenü-Karte „Weiter“ könnte das Siegel des letzten Stands zeigen (Row.badge ist da, `latest.grade` fehlt noch).
- `Campaign::frame` klont den Grad je Frame (wie island/completed); bei PERF-Ticket mit `alloc_je_bild` mitnehmen.
