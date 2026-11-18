# RU6.6 · Review Level-Generator in Rust

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru6
- **Abhängig von:** RU6.5
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Sprint ist abgenommen, der PR gegen `main` ist offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Zusätzlich: Jede `_test.go` aus `engine/level/` (19 Dateien, 18 mit Tests) ist in `rust/crates/k3c-level` portiert oder mit Grund im Ergebnis einer Session genannt (`bench_test.go`: nur Benchmarks); die Zahl der Go-Testfunktionen (`grep -c "^func Test" engine/level/*_test.go`) entspricht der Zahl der Rust-Tests samt `#[ignore]`; kein `#[ignore]` mit Marke `RU6` mehr; `pending.rs` ist leer oder gelöscht; `engine/`, `native/` und `testdata/golden` unverändert (`git diff --stat main -- engine native testdata`); kein `HashMap`, `unsafe`, `Rc`, `RefCell`, `powf`, `exp`, `sin`, `cos`, `f64::max`, `f64::min` in `rust/crates/k3c-level/src` (`grep`); jede Datei ≤ 400, jede Funktion ≤ 60 Zeilen. AC-03: aus den Ergebnissen der Sessions RU6.1 bis RU6.5 die Zeilen Rust-Code je Session (ohne Tests) in die Abnahme schreiben; Plan: 450 je Session. Liegt eine Session bei mehr als 675 (1,5-Faches), empfiehlt die Abnahme, die Schätzung für RU10 vor dessen Start zu erneuern. Reviewer-Agent `k3c-reviewer` (Sonnet) läuft getrennt vom Schreiben.

## Erlaubte Dateien

- `rust/crates/k3c-level/**` (nur Befunde unter etwa 30 Zeilen Fix)

## Nicht-Ziele

Neue Funktionen. Befunde über 30 Zeilen Fix werden Tickets.

## Schritte

1. Jedes Kriterium (AC-01 bis AC-03) gegen die Ergebnisse der Sessions RU6.1 bis RU6.5 nachprüfen; `cargo test -p k3c-level` einmal selbst laufen lassen (alle sechs Level-Golden und `nature.json`).
2. Dateigrößen, Funktionslänge, verbotene Muster (siehe Kontext) mit `grep` auswerten, Test-Zählung abgleichen.
3. Reviewer-Agent `k3c-reviewer` im Hintergrund starten, parallel `task check` und danach `task check:go` laufen lassen.
4. Abnahme der Sprint-README füllen, das Abnahme-Ticket für Sim, Balance und Gen anlegen (genau eins), PR gegen `main` öffnen.

## Fertig, wenn

- [ ] Jedes Kriterium (AC-01, AC-02, AC-03) ist im Abschnitt Abnahme mit Nachweis genannt.
- [ ] `task check` und `task check:go` grün.
- [ ] PR offen.

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

AC-01 geprüft: `cargo test -p k3c-level` 87 grün, Golden bitgleich. AC-02 geprüft: 82 Go-Tests, 84 Rust-Tests, kein `#[ignore]`, kein `pending.rs`. AC-03 geprüft: 583, 320, 532, 545, 374 Zeilen je Session, alle unter 675.

Dateien ≤ 400 Zeilen, keine verbotenen Muster (`HashMap`, `unsafe`, `Rc`, `RefCell`, `powf`, `exp`, `sin`, `cos`, `f64::max`, `f64::min`: kein Treffer). Reviewer `k3c-reviewer`: kein schwerer Befund, vier mittlere und leichte in B-1079 (Marker mit Ticketnummer versehen). `task check` und `task check:go` grün.

Abweichungen vom Plan: keine. Neue Tickets: B-1079, B-1080 (Abnahme). Manuell geprüft hat niemand.
