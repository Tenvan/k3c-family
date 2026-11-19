# WE8.6 · Review WE8 und Abnahme-Ticket

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/we8
- **Abhängig von:** WE8.1, WE8.7, WE8.8, WE8.5
- **Tickets:** B-991, B-992, B-995, B-996
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06

## Ziel

Der Sprint WE8 ist geprüft, Befunde sind behoben oder als Ticket angelegt, das Abnahme-Ticket steht.

## Kontext

Sprint-README `docs/sprints/*/WE8-welten-events-zuordnung/README.md` mit AC-01 bis AC-06, Tickets B-991, B-992, B-995, B-996. Reviewer getrennt vom Schreiben: `k3c-reviewer` (Engine, Protokoll), `k3c-hud` (Anzeige), `k3c-welten` (Weltdateien), `k3c-balancing` nur lesend für die vorläufigen Werte. Sim, Balance, Gen und `check:race` nur im Abnahme-Ticket (B-742); Simulationen sind gesperrt.

## Erlaubte Dateien

- `docs/rules/bosse.md`
- `docs/protocol.md`
- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-room/**`
- `data/events.json`
- `rust/crates/k3c-game/src/**`

## Nicht-Ziele

Neue Funktionen, Simulationen, Balancing.

## Schritte

1. Diff des Sprints lesen, Reviewer-Agenten parallel starten.
2. AC-01 bis AC-06 mit Nachweis prüfen.
3. Schwere Befunde beheben, Rest als Ticket mit Projekt und Sprint.
4. `task check`, `task check:go` nacheinander.
5. Abnahme-Ticket anlegen, Abschnitt Abnahme füllen, Glossar (Überfall, Sandsturm, Nebel, Skin) im letzten Commit; B-959 und B-960 als abgelöst melden.

## Fertig, wenn

- [ ] AC-01 bis AC-06 mit Nachweis in der Sprint-README › Abnahme.
- [ ] `task check` und `task check:go` grün; genau ein Abnahme-Ticket angelegt.

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
