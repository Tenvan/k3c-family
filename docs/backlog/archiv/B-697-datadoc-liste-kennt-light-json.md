# B-697 · Datei-Liste der Workbench kennt `light.json` nicht (TestListeWieLoader rot)

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`go test ./internal/datadoc` in `tools/k3c-workbench` scheitert auf `develop` (ohne weitere Änderungen): `TestListeWieLoader` meldet „engine/ lädt light.json, die Liste kennt die Datei nicht“ (`datadoc_test.go:35`). `task check:diff` bricht dadurch bei jedem Diff ab, der die Workbench-Tests einschließt.

## Ziel

Die Datei-Liste der Workbench und der Lader der Engine kennen dieselben Daten-Dateien; `task k3c-workbench:gotest` ist grün.

## Beteiligte und Zielgruppen

Entwickler (🧑 Ralf, Sessions DEV).

## Anforderungen

- `light.json` in die Liste aufnehmen oder aus dem Lader entfernen, je nachdem, was gewollt ist.

## Nicht-Ziele

Weitere Daten-Dokumentation.

## Regeln und Einschränkungen

Domäne DEV: `tools/k3c-workbench/`.

## Beispiele

nicht relevant: einzelner roter Test.

## Ausnahme- und Fehlerfälle

nicht relevant: einzelner roter Test.

## Akzeptanzkriterien

- **AC-01** `go test ./internal/datadoc` in `tools/k3c-workbench` ist grün.

## Offene Fragen

keine

## Notizen

Gefunden in der Session KB6.2 (2026-11-05), dort unabhängig von den Änderungen (rot auch auf unverändertem `develop`).
