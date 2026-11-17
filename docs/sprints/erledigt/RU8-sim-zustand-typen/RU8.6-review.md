# RU8.6 · Review Sim-Zustand

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru8
- **Abhängig von:** RU8.5
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03, AC-04

## Ziel

Der Sprint ist abgenommen, der PR gegen `main` ist offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Zusätzlich: kein `HashMap`, `Rc`, `RefCell`, `unsafe`, kein Lebenszeit-Parameter an Zustands-Typen; `engine/` und `testdata/golden` im Diff unverändert.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-level/src/types.rs`
- `docs/standards/rust-sim-zustand.md`

## Nicht-Ziele

Neue Funktionen. Befunde über 30 Zeilen Fix werden Tickets.

## Schritte

1. Jedes Kriterium nachprüfen.
2. `grep -rn -E "HashMap|Rc<|RefCell|unsafe" rust/crates/k3c-sim` auswerten.
3. `task check` und `task check:go` nacheinander.
4. Abnahme der Sprint-README füllen, PR öffnen.

## Fertig, wenn

- [ ] Jedes Kriterium ist im Abschnitt Abnahme mit Nachweis genannt.
- [ ] `task check` grün.

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

AC-01 bis AC-04 geprüft: `task check` und `task check:go` grün, `cargo test -p k3c-sim` grün. Fixes: `Drop` zu `ItemDrop`, 4 Ignore-Labels RU14 zu RU18. Neue Tickets: B-1049, B-1050. Keine manuelle Prüfung.
