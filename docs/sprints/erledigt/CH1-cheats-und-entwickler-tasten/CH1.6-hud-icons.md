# CH1.6 · HUD-Icons statt Ressourcen-Namen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/ch1
- **Abhängig von:** –
- **Tickets:** B-890
- **Kriterien:** AC-07

## Ziel

Das HUD zeigt Ressourcen als Papierschnitt-Icon mit Zahl statt mit Namen.

## Kontext

- HUD: `native/k3c-game/hud/` (Vorrat, Spielerkarte).
- Icons: `public/grafik/icons/`, Zuordnung `data/paper.json`; keine neuen Assets zeichnen.

## Erlaubte Dateien

- `native/k3c-game/hud/**`
- `native/k3c-game/art/**`
- `data/paper.json`

## Nicht-Ziele

- Neue Icons zeichnen (fehlende als Ticket).

## Schritte

1. Zuordnung Ressource → Icon mit Test (jede Ressource hat eins).
2. HUD zeichnet Icon + Zahl.
3. Aufnahme als Nachweis.

## Fertig, wenn

- [ ] AC-07: Go-Test der Zuordnung grün, Aufnahme zeigt Icons.
- [ ] `check_run task:check:diff` grün.

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

- AC-07 geprüft: `hud/icons.go` (Zuordnung Rohstoff → Icon aus der Gruppe icons, HUD als Asset-Quelle, `stockParts`), `hud/drawstock.go` (Icon + Zahl, voll in Orange, ohne Grafik Name statt Icon); Spielerkarte zeigt das Münz-Icon statt des gezeichneten Goldtopfs. Tests `TestEveryResourceHasIcon` (jedes Feld von `sim.Stock` hat ein Icon, jedes Icon steht in `data/paper.json`), `TestStockParts` grün.
- Nachweis: Aufnahme `task native:game:shots -- --seed wald --tick 300 --count 1` (2 Spieler, Split-Screen) zeigt Holz-Icon mit 100/1500 und Münz-Icon mit Gold. Für die Aufnahme lief `task gen:paper` (am Feature); die dabei geänderten `golden.json`/`index.paper.json` sind nicht eingecheckt (Abweichung vorbestehend, Abnahme-Ticket).
- Kein Icon fehlte, kein neues Ticket.
- `check_run task:check:diff:go` grün.
