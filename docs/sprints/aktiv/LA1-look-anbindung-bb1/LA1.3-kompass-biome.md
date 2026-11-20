# LA1.3 · Der Kompass im Hub zeigt die Biome der Insel

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** zweit
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/la1
- **Abhängig von:** –
- **Tickets:** B-1167
- **Kriterien:** AC-03

## Ziel

Der Kompass im Hub zeigt die Biome der Insel an.

## Kontext

Ticket B-1167 (Ralf 2026-11-20). Offen im Ticket: Lage des Kompasses im Hub, Anzeige als Papierschnitt-Icon oder Text, Biome der Insel oder je Richtung. Die Session entscheidet nach Go-Verhalten (falls der Kompass dort existiert) und `docs/rules/`; bleibt etwas offen, mit Default entscheiden und im Ergebnis melden. Sichtprüfung 🧑 am PC.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/hud/**`, `props/**`, `world/**` (Hub, Kompass)
- `data/hub.json`, `data/paper.json`

## Nicht-Ziele

Neue Biome, Regeln, Seekarte.

## Schritte

1. Kompass im Hub finden, Datenquelle der Biome der Insel klären.
2. Anzeige umsetzen mit Test.
3. `cargo test -p k3c-game`, `task rust:lint`, einmal `task check:diff`.

## Fertig, wenn

- [ ] AC-03: Kompass zeigt die Biome der Insel (Test).
- [ ] Nachprüfung 🧑: Anzeige im Spiel.
- [ ] Tests und `task rust:lint` grün.

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
