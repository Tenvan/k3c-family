# RU1.4 · Review Rust-Workspace

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/ru1
- **Abhängig von:** RU1.3
- **Tickets:** B-1014, B-1015, B-1018
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint ist abgenommen, der PR gegen `main` ist offen.

## Kontext

Ablauf nach `docs/arbeitsweise.md` › Review-Session. Diff des Sprints: `git diff origin/main...sprint/ru1`. Zusätzlich prüfen: Nichts außerhalb von `rust/`, `docs/standards/rust-port.md`, `tools/checkdiff`, `Taskfile.yml`, `ci.yml`, `tests/projectRules.test.ts`, `.gitignore` ist geändert; kein Go-Test ist angefasst.

## Erlaubte Dateien

- `rust/**`
- `docs/standards/rust-port.md`
- `tools/checkdiff/**`
- `Taskfile.yml`
- `.github/workflows/ci.yml`
- `tests/projectRules.test.ts`

## Nicht-Ziele

Neue Funktionen. Befunde über 30 Zeilen Fix werden Tickets.

## Schritte

1. Jedes Kriterium AC-01 bis AC-06 mit dem Befehl aus der jeweiligen Session nachprüfen.
2. `task check` und `task check:go` nacheinander.
3. Ist-Werte für B-1015/AC-02 notieren: Dauer und Token je Session (aus den Ergebnissen der Sessions).
4. Abschnitt Abnahme der Sprint-README füllen, PR öffnen.

## Fertig, wenn

- [ ] Jedes Kriterium ist im Abschnitt Abnahme mit Nachweis genannt.
- [ ] `task check` und `task check:go` sind grün.

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

- AC-01 bis AC-06 nachgeprüft: zehn Crates in `rust/crates`, `cargo build/test/clippy -D warnings/fmt --check` grün, `Cargo.lock` eingecheckt, alle Crates mit `version.workspace`, `task rust:*` und `check:diff` mit Rust-Bahn (`TestRustSteps`), Job `rust` in `ci.yml`, 400-Zeilen-Test mit 401-Zeilen-Probe rot, Leitfaden § 1 bis § 10.
- `task check` grün (5 min 42 s), `task check:go` grün (2 min 34 s), `task check:diff` grün.
- Diff gegen den Stand vor RU1: außerhalb des erlaubten Bereichs nur Planungsdateien (Sessions, Sprint, Backlog-Index); kein Go-Test geändert, außer dem neuen Fall in `tools/checkdiff/select_test.go`.
- Befunde: keine. Abweichung: Kein PR (Auftrag der Orchestrierung: der Sprint-PR folgt später). Abnahme-Ticket: B-1021.
- Manuell geprüft: nichts.
