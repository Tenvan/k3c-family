# LA3.2 · Review und Abnahme-Ticket LA3

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** hauptrechner
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la3
- **Abhängig von:** LA3.1
- **Tickets:** B-1182
- **Kriterien:** AC-01, AC-02

## Ziel

Sprint LA3 ist geprüft; Abnahme-Ticket angelegt.

## Kontext

Review von LA3.1 gegen AC-01 und AC-02 der Sprint-README (Spielstand-Verhalten, Golden).

## Erlaubte Dateien

- `docs/sprints/aktiv/LA3-*/**`

## Nicht-Ziele

Keine neuen Funktionen.

## Schritte

1. Diff lesen, k3c-reviewer laufen lassen.
2. `task check` und `task check:go` nacheinander.
3. Abnahme-Ticket anlegen (Sichtprüfung am PC durch 🧑).

## Fertig, wenn

- [ ] AC-02: Review ohne offene schwere Befunde, alte Spielstände laden wie festgelegt.
- [ ] `task check` und `task check:go` grün.

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

AC-01, AC-02 geprüft. `task check` grün (nach dem Merge von origin/main). `task check:go`: alle Go-Tests grün, `golangci-lint` fehlt lokal, Lint nicht gelaufen (B-1189). Rust: `k3c-data`, `k3c-sim` Hub-Tests, Platz-Regeln grün; rot bleiben nur `alloc_je_bild`, `e2e_flow`, `insel_nach_1000_schritten_wie_go` (auf main rot) und zwei Tests wegen lokaler, nicht eingecheckter `data/worlds/rom.json`. Review: Reviewer-Agent in LA3.1 (zwei Befunde, behoben); Diff `origin/main...sprint/la3` und Merge gelesen: Go-Golden und `hub.json` unverändert, `data-tree.json.gz` nur wegen `sizes.json` neu geschrieben, Lint-Fixes im Workbench-Frontend ohne Verhaltensänderung. Abnahme-Ticket: B-1189 (B-1185 ist das Abnahme-Ticket von LA2).
