# BS7.1 · Werte-Gruppen und Einfrieren in den Balancing-Zielen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** bs7/1-werte-gruppen
- **Abhängig von:** BS1.4
- **Tickets:** B-640
- **Kriterien:** AC-01

## Ziel

Jedes Balancing-Ziel und jeder Tuning-Hebel gehört zu genau einer Werte-Gruppe (`basis`, `holz`, `schiff`); Gruppen lassen sich einfrieren.

## Kontext

- Ziele: `data/balance-targets.json`, Hebel: `data/balance-hints.json`, Laden/Schema: `tools/k3c-workbench/internal/balance/targets.go`, `hints.go`.
- Spec und Gruppen-Vorschlag: B-640 › Anforderungen.
- `seeds`/`seedMode` aus BS1 bleiben unberührt.

## Erlaubte Dateien

- `data/balance-targets.json`, `data/balance-hints.json`
- `tools/k3c-workbench/internal/balance/**`

## Nicht-Ziele

Baseline, Delta-Bericht (BS7.2), Editor (BS7.3).

## Schritte

1. Schema: Feld `group` je Ziel und Hebel, Liste `frozen` (Standard leer).
2. Zuordnung eintragen.
3. Schema-Test: jedes Ziel/jeder Hebel genau eine bekannte Gruppe.
4. `check_run task:check:diff`, Commit.
5. **⚠️ Workbench-Neustart nötig** melden, falls Go-Code geändert (Ergebnis Zeile 1, `notify_ui` warn, Chat).

## Fertig, wenn

- [ ] AC-01: Schema-Test grün, `frozen` wird gelesen.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
