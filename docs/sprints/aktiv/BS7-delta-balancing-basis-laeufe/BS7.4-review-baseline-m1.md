# BS7.4 · Review BS7 und Baseline M1 sichern

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** bs7/4-review-baseline-m1
- **Abhängig von:** BS3.4, BS7.5
- **Tickets:** B-640
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

BS7 ist abgenommen; der Meilenstein-1-Lauf aus BS3.4 ist als Baseline `M1` gesichert, Gruppe `basis` eingefroren, und er steht im Editor unter „Basis-Läufe“.

## Kontext

- Vorher prüfen, dass die Workbench nach BS7.3 neu gestartet ist (`workbench_status`, Tool-Schema `sim_test` mit `baseline`); sonst `Status: blockiert`, 🧑 deutlich um Neustart bitten.
- Bericht-ID des Meilenstein-Laufs: B-635 › Notizen (aus BS3.4). Fehlt der Bericht oder passt er nicht mehr zu den Daten, den Lauf über `sim_test start bots=coop2 world=standard` wiederholen.
- Review nach `docs/arbeitsweise.md` › Review-Session.

## Erlaubte Dateien

- Dateien aus BS7.1–BS7.3 (Befunde beheben)
- `data/balance-targets.json` (`frozen`)
- `reports/baseline/**`
- `docs/sprints/**/BS7-*/**`, `docs/backlog/B-640-*.md`

## Nicht-Ziele

Werteänderungen.

## Schritte

1. Neustart prüfen.
2. Baseline `M1` sichern (`sim_test`), `frozen: ["basis"]` setzen.
3. Kontroll-Delta-Lauf mit `baseline=M1` → alle Gruppen ✅ unverändert.
4. Editor: Basis-Lauf `M1` sichtbar (Browser-Pane der Workbench oder API).
5. `check_run task:check`, `task:check:go`, `task:check:race`; Abnahme, Sprint abschließen, PR.

## Fertig, wenn

- [ ] AC-05: Baseline `M1` gesichert, Kontroll-Lauf ohne Abweichung, im Editor sichtbar.
- [ ] Alle Kriterien geprüft, Checks grün, PR offen.

## Prüfen

```bash
task check
task check:go
task check:race
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
