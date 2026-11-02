# BS7.2 · Benannte Baseline und Delta-Bericht über sim_test

- **Status:** fertig
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

**⚠️ Workbench-Neustart nötig** (Go-Code in balance, simrun und mcpsrv geändert; ohne Neustart kennt sim_test weder `baseline` noch `saveBaseline`, und die Berichte haben kein Delta).

- AC-02 umgesetzt und geprüft: `sim_test` nimmt `baseline` (Name; offline; der Bericht unter `reports/baseline/<name>/balance.json` ersetzt die Repo-Baseline im CLI-Aufruf) und `action: baseline` (`id` + `saveBaseline` sichert einen durchgelaufenen Lauf samt `run.json` mit Bots, Welten, Seeds, Seed-Modus und `frozen`; ohne Namen listet es die gesicherten). Gesichert wird nur bei `Pass`, ein vergebener Name wird nie überschrieben. Tests: `balance/baselines_test.go`, `simrun/spec_baseline_test.go`, `mcpsrv/simtest_baseline_test.go`. Tool-Beschreibung und `instructions.md` erklären die Delta-Runde.
- AC-03 umgesetzt und geprüft: `Insights.Delta` (`delta.go`) je Gruppe mit Ampel, Trend und Anzahl besser/schlechter; eingefrorene Gruppen erscheinen nur bei kippendem Ziel, mit Warnung; Hebel eingefrorener Gruppen fehlen in Zielen und Hinweisen. Der Markdown-Bericht hat den Abschnitt „Delta je Werte-Gruppe“. Tests: `balance/delta_test.go` (frei, eingefroren, Hinweise, ohne Baseline).
- Für BS7.3 bereit: `balance.ListBaselines(root)` liefert die Basis-Läufe; die RPC dafür gehört zu BS7.3.
- `task check:diff` grün (erster Lauf: gocyclo-Befund im eigenen Test, behoben). Keine Simulation gefahren (Regel 2026-10-28); die Pfade sind nur mit Fake-Prozessen getestet.
