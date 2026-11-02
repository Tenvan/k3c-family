# BS7.1 · Werte-Gruppen und Einfrieren in den Balancing-Zielen

- **Status:** fertig
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

**⚠️ Workbench-Neustart nötig** (Go-Code in `tools/k3c-workbench/internal/balance` geändert; ohne Neustart lädt die laufende Workbench `group`/`frozen` nicht).

- AC-01 umgesetzt und geprüft: `groups.go` (Gruppen `basis`, `holz`, `schiff`, `Targets.Frozen`, `IsFrozen`); `group` je Ziel (Pflicht in `DefaultTargets`/`checkRoles`) und je Hebel (Pflicht in `DefaultHints`/`checkGroups`); `frozen: []` in `data/balance-targets.json`. Tests in `groups_test.go` (Gruppen vollständig, `frozen` gelesen und geprüft, Ziel ohne Gruppe scheitert), `go:test` für `internal/balance/...` grün.
- Zuordnung: Ziel `woodAtDawnFrom3` und Hebel `markCost` → `holz`; `endbossDefeated`, `endbossDay` und Hebel `endBosses.*` → `schiff`; alles andere `basis`.
- `task check:diff`: einziger Rot-Grund waren drei 5-s-Timeouts unter Last (`projectRules`, `planningProjects`); einzeln wiederholt grün.
- Keine Simulation gefahren (Regel 2026-10-28).
