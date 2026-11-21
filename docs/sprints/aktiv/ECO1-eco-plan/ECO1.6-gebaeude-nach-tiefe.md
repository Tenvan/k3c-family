# ECO1.6 · Gebäude nach Tiefe: unten Verteidigung, Produktion oben

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** zweit
- **Modell:** opus
- **Umgebung:** ?
- **Branch:** eco1/6-gebaeude-nach-tiefe
- **Abhängig von:** –
- **Tickets:** B-1196
- **Kriterien:** AC-06

## Ziel

Je Gebäudeart ist in den Daten festgelegt, auf welchen Ebenen sie stehen darf: Kampfgebäude (Mauer, Turm, Tor, Kaserne, Bogner, Heilplatz) überall, Produktion nur ganz oben.

## Kontext

Ticket B-1196. Heute baut jede Ebene, was die Hub-Stufe freigibt (`data/hub.json`, `data/buildings.json`). Zuordnung als Datenwert, Rust-Sim (`k3c-sim/src/hub`, `building`). Spielstand-Version erhöhen, alte Stände egal; Go-Golden nur mit `native_rules` aus.

## Erlaubte Dateien

- `rust/crates/k3c-sim/src/hub/**`
- `rust/crates/k3c-sim/src/building/**`
- `rust/crates/k3c-data/src/**`
- `data/hub.json`
- `data/buildings.json`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Wo Plätze je Ebene entstehen (hub sites) finden.
2. Datenfeld je Art (z. B. `levels`/`topOnly`), Sim filtert Plätze nach Tiefe.
3. Tests: unten nur erlaubte Arten, oben alle.

## Fertig, wenn

- [ ] AC-06: Unten nur erlaubte Arten, oben alle; Zuordnung in den Daten.
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
