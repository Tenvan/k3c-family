# BS7.2 · Benannte Baseline und Delta-Bericht über sim_test

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs7/2-baseline-delta-bericht
- **Abhängig von:** BS7.1
- **Tickets:** B-640
- **Kriterien:** AC-02, AC-03

## Ziel

`sim_test` nimmt eine benannte Baseline, sichert durchgelaufene Läufe als Baseline mit Konfiguration und liefert einen Delta-Bericht je Werte-Gruppe.

## Kontext

- Baseline heute: `tools/k3c-workbench/internal/balance/insights.go` (`BaselineView`, Trend), Dateien `insights_files.go`.
- MCP-Tool: `tools/k3c-workbench/internal/mcpsrv/simtest.go`, Instructions `mcpsrv/instructions.md`; Lauf-Ausführung `internal/simrun/`.
- Ablage-Vorschlag: `reports/baseline/<name>/` mit Bericht und `run.json` (Bots, Welt, Seeds, Seed-Modus, Datum, Ergebnis).
- Spec: B-640.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/balance/**`, `tools/k3c-workbench/internal/simrun/**`
- `tools/k3c-workbench/internal/mcpsrv/simtest*.go`, `mcpsrv/instructions.md`
- `tools/k3c-workbench/app_*.go` (nur API für Basis-Läufe, falls BS7.3 sie braucht)

## Nicht-Ziele

Editor-Oberfläche (BS7.3).

## Schritte

1. `sim_test`: Parameter `baseline` (Name oder Lauf-ID) und Aktion bzw. Parameter zum Sichern (`saveBaseline=<name>`, nur bei durchgelaufenem Lauf).
2. Delta-Bericht je Gruppe: Ampel, Trend; eingefrorene Gruppen nur beim Kippen mit Warnung; Hinweise nur für freie Gruppen.
3. Liste der Basis-Läufe als Funktion/API für BS7.3.
4. Unit-Tests (Delta, Kippen, Sichern), `mcpsrv`-Test der Parameter; Beschreibung und `instructions.md` (Delta-Runde: „Baseline des letzten Meilensteins, nur freie Gruppe justieren“).
5. `check_run task:check:diff`, Commit.
6. **⚠️ Workbench-Neustart nötig** melden (Ergebnis Zeile 1, `notify_ui` warn, Chat).

## Fertig, wenn

- [ ] AC-02: Tests für `baseline`/Sichern grün, Instructions ergänzt.
- [ ] AC-03: Delta-Test mit eingefrorener und freier Gruppe grün.
- [ ] Neustart gemeldet; `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
