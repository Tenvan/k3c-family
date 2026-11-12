# B-621 · TestSourcesUndConsoleTail flackert, wenn die Ressourcen-Zeile früh kommt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ42
- **Projekt:** WZG
- **Erstellt:** 2026-10-25
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

`dev:test` (2026-10-25, W12.4): `TestSourcesUndConsoleTail` (`tools/k3c-workbench/app_logs_test.go:54`) war einmal rot. `ConsoleTail` lieferte zusätzlich die Zeile `💓 ressourcen: …` des Ressourcen-Monitors. Mit `-count=5` ist der Test grün, er flackert also je nach Zeitpunkt.

## Ziel

Der Test ist unabhängig vom Zeitpunkt der Ressourcen-Zeile grün.

## Beteiligte und Zielgruppen

Agent DEV.

## Anforderungen

- Den Test so fassen, dass er nur die erwarteten Zeilen prüft (z. B. die Ressourcen-Zeile herausfiltern) oder den Monitor im Test abschalten.

## Nicht-Ziele

Änderung des Ressourcen-Monitors selbst.

## Regeln und Einschränkungen

Kein Sleep als Behebung. Domäne DEV, Klasse A (Problem).

- **Bereich:** `tools/k3c-workbench/app_logs*.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`go test ./tools/k3c-workbench -run TestSourcesUndConsoleTail -count=50` grün.

## Ausnahme- und Fehlerfälle

nicht relevant

## Akzeptanzkriterien

- **AC-01** Der Test ist mit `-count=50` unter Last grün.

## Offene Fragen

keine

## Notizen

Gefunden in W12.4.

2026-10-31 (Claude, Team WZG): Nicht mehr reproduzierbar. `go test . -run TestSourcesUndConsoleTail -count=50` in `tools/k3c-workbench`: ok (5,1 s), nichts geändert.
