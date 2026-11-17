# RU5.5 · Review Rig in Rust

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru5
- **Abhängig von:** RU5.4
- **Tickets:** B-1014
- **Kriterien:** AC-01, AC-02, AC-03

## Ziel

Der Sprint ist abgenommen, der PR gegen `main` ist offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Zusätzlich: kein `unsafe`, kein `HashMap` in `k3c-rig`; produktiver Go-Code in `native/k3c-rig` unverändert (nur `rustref_test.go`).

## Erlaubte Dateien

- `rust/crates/k3c-rig/**`
- `native/k3c-rig/**/rustref_test.go`
- `testdata/rust/rig/**`

## Nicht-Ziele

Neue Funktionen. Befunde über 30 Zeilen Fix werden Tickets.

## Schritte

1. Jedes Kriterium mit dem Befehl der jeweiligen Session nachprüfen.
2. `git diff main -- native/k3c-rig` prüfen: nur Testdateien.
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

AC-01 geprüft: `task check` und `task check:go` grün, Referenzen 1e-9. AC-02 geprüft: Netz über `Canvas::mesh`, Tests grün. AC-03 geprüft: Schwelle in `RIG.md` (Rust-Port: Toleranzen). Abweichung vom Plan: Umhang beim Reiten 1e-6 (Entscheidung Hauptagent). Tickets: B-1038, B-1039. Kein manuelles Prüfen.
