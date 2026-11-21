# FG1.2 · Waffenschmied, Rüstungsschmied, Bogner, Kombinationen und Elite in der Sim

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Lane:** erst
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** fg1/2-meister-kombinationen
- **Abhängig von:** FG1.1
- **Tickets:** B-1200
- **Kriterien:** AC-02

## Ziel

Bogner, Waffenschmied und Rüstungsschmied fertigen Bogen, Schwert und Rüstung ins Regal; Bürger werden Bogenschütze oder Soldat, diese mit Rüstung zu Elite; der Hub-HP-Bonus der Rüstkammer entfällt.

## Kontext

Ticket B-1200. Waffenschmied ersetzt die Schmiede, Rüstungsschmied die Rüstkammer, Handwerksmeister (ECO1.3) macht Gürtel. Genau ein Beruf je Figur, der Bürger holt Ausrüstung selbst aus dem Regal. Produktion nur ganz oben (ECO1.6). Spielstand-Version erhöhen.

## Erlaubte Dateien

- `rust/crates/k3c-sim/**`
- `rust/crates/k3c-data/**`
- `data/buildings.json`
- `data/troops.json`
- `data/hub.json`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Schmiede/Rüstkammer-Logik (`building/upgrades.rs`) lesen.
2. Waffenschmied und Rüstungsschmied mit Regal, Gegenstände Schwert und Rüstung.
3. Bürger + Schwert = Soldat, Soldat/Bogenschütze + Rüstung = Elite; Handwerker nimmt nichts.
4. Hub-HP-Bonus entfernen.
5. Test je Kombination.

## Fertig, wenn

- [ ] AC-02: Fünf Kombinationen mit Test je Kombination, Hub-HP-Bonus entfällt.
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
