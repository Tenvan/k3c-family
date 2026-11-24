# GS4.7 · Flaschenpost im Rust-Spiel zeichnen (ersetzt GS4.4)

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
