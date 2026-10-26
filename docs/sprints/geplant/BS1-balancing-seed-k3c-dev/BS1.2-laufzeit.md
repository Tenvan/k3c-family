# BS1.2 · Laufzeit coop2 standard messen und verkürzen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** bs1/2-laufzeit
- **Abhängig von:** BS1.1
- **Tickets:** B-634
- **Kriterien:** AC-03, AC-06

## Ziel

Nach dem Workbench-Neustart läuft `coop2` standard mit Seed `k3c-dev` über das neue `sim_test` deutlich schneller als vorher; die Zahlen stehen in B-634.

## Kontext

- BS1.1 hat `sim_test` erweitert; Vorher-Laufzeit steht in B-634 › Notizen.
- Der Lauf braucht die **neu gestartete** Workbench: zuerst `workbench_status` (Laufzeit/Start nach dem BS1.1-Commit) und das Tool-Schema von `sim_test` (Parameter `seedList`, `world`) prüfen. Fehlt beides → `Status: blockiert`, 🧑 um Neustart bitten.
- Weitere Tempo-Hebel nur ohne Determinismus-Verlust (z. B. Seeds parallel, Abbruch bei klarem Scheitern).

## Erlaubte Dateien

- `tools/k3c-workbench/internal/balance/**`, `tools/k3c-workbench/internal/simrun/**`
- `docs/backlog/B-634-*.md` (Notizen, über `plan_section`)

## Nicht-Ziele

Werteänderungen; neue Bot-Profile.

## Schritte

1. Neustart prüfen (Kontext); sonst blockiert melden.
2. `sim_test start bots=coop2 world=standard` (offline, Seed-Modus dev) → `status` bis fertig, Dauer notieren.
3. Profilieren (`workbench_resources`, Bericht); offensichtliche Hebel umsetzen, wenn deterministisch.
4. Falls Go-Code geändert: erneut **⚠️ Workbench-Neustart nötig** melden (Ergebnis, `notify_ui`, Chat) und die Nachher-Messung erst danach.
5. Vorher/nachher in B-634 › Notizen; `check_run task:check:diff`, Commit.

## Fertig, wenn

- [ ] AC-03: B-634 › Notizen nennt Laufzeit vorher (alle Seeds) und nachher (`k3c-dev`), nachher deutlich kürzer.
- [ ] AC-06: Messlauf lief über das neue `sim_test` nach dem Neustart.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Läufe nur über `sim_test`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
