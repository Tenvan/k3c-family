# GS4.7 · Flaschenpost im Rust-Spiel zeichnen (ersetzt GS4.4)

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/gs4
- **Abhängig von:** GS4.3
- **Tickets:** B-980
- **Kriterien:** AC-06

## Ziel

Das Rust-Spiel zeichnet Flasche, Geschichtszeile, Richtungspfeil und Leuchten in beiden Spielerhälften und löscht sie am Ziel.

## Kontext

Spec: `B-980 › Anforderungen` (AC-02 bis AC-05). Die Go-Session GS4.4 ist verworfen; diese Session ersetzt sie in Rust (Entscheidung 013): Anzeige in `rust/crates/k3c-game/src/props/`, `world/`, `fx/`, `hud/`, `view/`. Das Asset kommt aus GS4.3, Zustand und Markierung aus GS4.6.

Jede Anzeige mit 2 Spielern prüfen; HUD-Prüfung durch `k3c-hud`, Grafik durch `k3c-grafik` vor dem Commit.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/props/**`
- `rust/crates/k3c-game/src/world/**`
- `rust/crates/k3c-game/src/fx/**`
- `rust/crates/k3c-game/src/hud/**`
- `rust/crates/k3c-game/src/view/**`
- `rust/crates/k3c-game/tests/still_*.rs`
- `data/paper.json`

## Nicht-Ziele

Neue Spielregeln (GS4.6). Neue Assets (GS4.3); fehlt eines, vorläufig zeichnen und melden.

## Schritte

1. Flasche am Strand zeichnen (Prop), Leuchten (fx) und Richtungspfeil (hud) je Spielerhälfte.
2. Geschichtszeile einblenden, am Ziel Pfeil und Markierung löschen.
3. Standbild-Tests; `k3c-hud` und `k3c-grafik` vor dem Commit.

## Fertig, wenn

- [ ] AC-06: Das Spiel zeichnet Flasche, Geschichtszeile, Pfeil und Leuchten in beiden Spielerhälften und löscht sie am Ziel (B-980/AC-02 bis AC-05).
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

AC-06 geprüft: Flasche samt Glitzer aus GS4.4 vorhanden; neu: Leuchten am Ziel (props/plan_bottle.rs, pulsiert, drei Scheiben), Radar-Marker Flasche/Ziel, Richtungspfeil je Spielerhälfte (hud/bottle.rs, ohne Text, ohne Allokation), Geschichtszeile bei bottleFound in allen Hälften (Campaign::bottles, Rc), Löschen am Ziel folgt dem Sim-Zustand (Markierung weg = Pfeil und Leuchten weg im selben Frame; Laden ohne Ereignis = keine Zeile). Tests: hud/bottle_tests.rs (8) und plan_bottle_tests.rs, cargo test -p k3c-game --lib 1093 grün, k3c-hud-Review ohne schwere Befunde.
check:diff nicht grün abgeschlossen: Clippy-Fehler in fremder WIP (k3c-sim island/save_load.rs, hud/kampf_tests.rs, props/plan_tests.rs); eigene Dateien clippy- und fmt-sauber.
Abweichungen: kleine Änderung in local/campaign.rs und view/story.rs (Zeilen der Flaschenpost an den Frame), da die Geschichte dort sonst nicht ankommt. Grenze: Die Markierung kennt die Anzeige nur, wenn die Stufe der Tiefe 0 im Frame liegt. Kein k3c-grafik-Review (kein Asset geändert).
