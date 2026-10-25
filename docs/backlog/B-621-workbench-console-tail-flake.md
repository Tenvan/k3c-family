# B-621 · TestSourcesUndConsoleTail flackert, wenn die Ressourcen-Zeile früh kommt

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-25
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

Kein Sleep als Behebung.

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
