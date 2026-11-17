# WE8.2 · Engine: Event raid (Überfall) (B-991)

- **Status:** verworfen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/we8
- **Abhängig von:** WE8.1
- **Tickets:** B-991
- **Kriterien:** AC-02, AC-06

## Ziel

Die Engine kennt das Event `raid` (Überfall): jede n-te Nacht eine zusätzliche schnelle Gruppe einer Gegnerart, Lohn bei gehaltener Burg (B-991).

## Kontext

Eigener Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Vorbild Vollmond: `engine/sim/events.go`, `events_moon.go` (zusätzliche Gegner der Art `kind`, Lohn bei Tagesanbruch), Tests `events_moon_test.go`, `events_test.go`, `events_budget_test.go`. Events kommen seit WE5.3 (B-982) aus dem Wertesatz der Welt. Daten `data/events.json`; Zustand und Ereignisse in `engine/room/`, beschrieben in `docs/protocol.md` (Sperre `protokoll`). Regeltext aus WE8.1 in `docs/rules/bosse.md` § 2. Portale der Oberfläche: `docs/rules/kampf.md` § 2. Nur `engine/rng`. Golden dürfen sich nicht ändern (Basis: `everyNights: 0`).

## Erlaubte Dateien

- `engine/sim/**`
- `engine/room/**`
- `data/events.json`
- `docs/rules/bosse.md`
- `docs/protocol.md`

## Nicht-Ziele

Anzeige (WE8.4), Einschalten in Sengoku (WE8.5), Sonderverhalten der Ninja, Simulation, Balancing, `task go:golden`.

## Schritte

1. Tests zuerst: Zusatzgruppe in der n-ten Nacht (Zahl, Tempo), nicht in anderen Nächten; Lohn; keine Portale → kein Überfall; Überlagerung mit Vollmond; gleicher Seed → gleicher Ablauf.
2. `raid` in `data/events.json` (aus, Werte vorläufig im `notes`-Feld), Laden und Prüfung.
3. Logik nach dem Muster des Vollmonds; Ereignis im Zustand, `docs/protocol.md` nachziehen.
4. Braucht es ein Spielstand-Feld: stoppen und melden (Sperre `spielstand` fehlt).
5. Review `k3c-reviewer` vor dem Commit.

## Fertig, wenn

- [ ] AC-02: B-991/AC-01, AC-02 als Tests grün; B-991/AC-04 nachgezogen.
- [ ] AC-06: gleicher Seed → gleicher Ablauf, `testdata/golden/` unverändert (B-991/AC-03).

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
