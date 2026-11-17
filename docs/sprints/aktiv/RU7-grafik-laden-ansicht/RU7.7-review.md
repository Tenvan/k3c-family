# RU7.7 · Review

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru7
- **Abhängig von:** RU7.1, RU7.2, RU7.3, RU7.4, RU7.5, RU7.6
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Alle Kriterien von RU7 sind geprüft, Befunde behoben oder als Ticket erfasst, die Abnahme in der Sprint-README steht.

## Kontext

Kriterien und Vorentscheidungen: RU7-README. Ergebnisse: RU7.1–RU7.6. Ablauf: `docs/arbeitsweise.md` › Review-Session.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/art/**`
- `rust/crates/k3c-game/src/view/**`
- `rust/crates/k3c-game/src/demo/**`

## Nicht-Ziele

Neue Funktionen.

## Schritte

1. Jede Session-Ergebnis-Zeile gegen ihr Kriterium prüfen; kein `ignore` mit RU5 oder RU8 und kein `pending.rs` mehr in `art` und `view`.
2. `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, `cargo test --workspace` in `rust/`; mit gerenderter Grafik (`K3C_GRAFIK_DIR`) lädt der Test alle Gruppen.
3. Abnahme in der Sprint-README (höchstens fünf Zeilen) und Abnahme-Ticket anlegen.

## Fertig, wenn

- [ ] AC-01 bis AC-04: je Kriterium ein Nachweis im Ergebnis.
- [ ] `task check:diff` und `cargo test --workspace` grün.
- [ ] Abnahme in der README, Abnahme-Ticket angelegt.

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

- AC-01 bis AC-04 geprüft: `cargo fmt --all --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (k3c-game 104 grün), `task check` (7 min 48 s) und `task check:go` (2 min 16 s) nacheinander grün. Vorher `origin/sprint/ru8` gemergt (Review-Commit ecbba5587 mit `ItemDrop`); k3c-game hatte keine `Drop`-Referenz.
- Schritt 1: kein `pending.rs` in `art` und `view`, keine `ignore`-Labels mit RU5/RU8, `k3c_rig` nur in `src/art/`.
- `Stage.world: Option<Rc<World>>` (RU7.5): `Rc` passt, kein Thread in k3c-game, die Welt ist nur lesend; kein `Arc`. Kein Fix nötig.
- Nicht geprüft: gerenderte Grafik (`K3C_GRAFIK_DIR`, `gen:paper`) und Race; beides im Abnahme-Ticket B-1048.
- Befunde: keine; keine Fixes. Neues Ticket: B-1048.
