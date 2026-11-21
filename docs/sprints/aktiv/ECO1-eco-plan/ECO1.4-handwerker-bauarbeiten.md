# ECO1.4 · Handwerker mit Gürtel macht alle Bauarbeiten

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/4-handwerker-bauarbeiten
- **Abhängig von:** ECO1.3
- **Tickets:** B-1194
- **Kriterien:** AC-04

## Ziel

Ein Bürger mit Werkzeuggürtel ist Handwerker und macht Bau, Ausbau, Reparatur, Rodung und Fels; ohne Gürtel macht er davon nichts.

## Kontext

Ticket B-1194, Gürtel aus ECO1.3. Berufe Bergmann und Baumeister gehen im Handwerker auf (Default). Fällt der Handwerker, bleibt der Gürtel liegen; ein Bürger ohne Gürtel holt ihn nur ohne Gegner in 8 Units (B-929). Deterministisch, 2+ Spieler, Spielstand-Version erhöhen.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/unit/**`
- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-sim/src/state/**`
- `data/troops.json`
- `docs/rules/buerger.md`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Job-Zuteilung (`unit/jobs.rs`, Hammer-Arbeit) lesen.
2. Gürtel als Voraussetzung für Bau, Reparatur, Rodung, Fels.
3. Liegenbleiben und Aufheben des Gürtels.
4. Tests je Arbeit mit und ohne Gürtel.

## Fertig, wenn

- [ ] AC-04: Ein Bürger mit Gürtel macht Bau, Reparatur, Rodung und Fels; ohne Gürtel nichts davon.
- [ ] `task check:diff` grün.

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
