# RU4.10 · Review

- **Status:** fertig
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

- AC-01 geprüft: Runner-, Batch-, Affine- und Formen-Tests grün, Probe auf Vulkan mit 59–60 Bildern/s.
- AC-02 geprüft: Test `runner_tests` (1000 Takte, simulierte Uhr = headless); Probe `--two --headless` mit Skript → Code 0.
- AC-03 geprüft: Eingabe-Port mit portierten Go-Tests grün, Review verhaltensgleich. Pad an Hardware ungeprüft → B-1028.
- AC-04 geprüft: Dekodier- und Rückfall-Tests grün; Hören ungeprüft → B-1028.
- AC-05 geprüft: TTF-Tests, Screenshot mit Umlauten, `log_emoji` rot/grün probiert.
- `task check` (5 min 34 s) und `task check:go` grün; clippy, fmt, `cargo test --workspace`: 129 bestanden, 1 ignoriert (wartet auf RU8).
- Neue Tickets: B-1025 (Settings ohne Stress, RU7), B-1028 (Abnahme).
