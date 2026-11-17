# RU4.10 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru4
- **Abhängig von:** RU4.1, RU4.2, RU4.3, RU4.4, RU4.5, RU4.6, RU4.7, RU4.8, RU4.9
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

Alle Kriterien von RU4 sind geprüft, Befunde behoben oder als Ticket erfasst, die Abnahme in der Sprint-README steht.

## Kontext

Kriterien und Vorentscheidungen: RU4-README. Ergebnisse: RU4.1–RU4.9. Ablauf: `docs/arbeitsweise.md` › Review-Session. Manuelle Abnahme (Pad, Ton, Fenster am TV) nur, wenn 🧑 sie für diesen Lauf freigibt.

## Erlaubte Dateien

- `rust/crates/k3c-platform/**`
- `rust/crates/k3c-game/src/input/**`
- `rust/crates/k3c-game/src/view/**`
- `rust/crates/k3c-game/src/demo/**`

## Nicht-Ziele

Neue Funktionen. Port weiterer Go-Pakete.

## Schritte

1. Jede Session-Ergebnis-Zeile gegen ihr Kriterium prüfen; offene `ignore` mit Sprint-Nummer zählen.
2. `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all --check`, `cargo test --workspace` in `rust/`.
3. Abweichungen gegen die Vorentscheidungen sammeln (Schnittstelle `App`, `Canvas`, `InputState`, `AudioOut`).
4. Abnahme in der Sprint-README (höchstens fünf Zeilen) und Abnahme-Ticket anlegen (`docs/arbeitsweise.md` › Sim, Balance und Gen).

## Fertig, wenn

- [ ] AC-01 bis AC-05: je Kriterium ein Nachweis im Ergebnis.
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
