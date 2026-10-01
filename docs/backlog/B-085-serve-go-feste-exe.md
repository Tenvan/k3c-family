# B-085 · `task serve:go` startet eine EXE mit festem Pfad

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-01
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`task serve:go` ruft `go run ./cmd/k3c-server` (`Taskfile.yml`). `go run` baut die EXE bei jedem Lauf in einen neuen Ordner unter
`%TEMP%\go-build…`. Die Windows-Firewall behandelt jede dieser EXE als neue App und fragt bei jedem Start erneut nach der Freigabe
(`k3c-server.exe` für öffentliche, private oder Domänennetzwerke). Für Xbox und Handy im Heimnetz ist die Freigabe nötig, jedes Mal neu zu bestätigen ist unpraktisch.

## Ziel

Der Go-Server startet immer aus derselben EXE, damit die Firewall-Freigabe einmal erteilt wird und bleibt. Nutzen: Server für Xbox und Handy ohne Dialog bei jedem Start.

## Beteiligte und Zielgruppen

🧑 betreibt den Server im Heimnetz (PC, später Pi); Entwickler und Agenten.

## Anforderungen

- `task serve:go` baut `k3c-server.exe` nach `bin/` (oder ähnlich, nicht eingecheckt) und startet diese Datei.
- Die Firewall-Freigabe selbst erteilt weiter 🧑 (Systemeinstellung), die Task setzt keine Regeln.

## Nicht-Ziele

Firewall-Regeln per Skript anlegen; Docker-Image (`Dockerfile`).

## Regeln und Einschränkungen

Domäne INF (`Taskfile.yml`, `.gitignore`); Befehle nur über `task`.

## Beispiele

Zweiter Start von `task serve:go` am selben Tag → keine neue Firewall-Abfrage.

## Ausnahme- und Fehlerfälle

Build scheitert → die Task bricht ab und startet keine alte EXE.

## Akzeptanzkriterien

- **AC-01** Zwei Läufe von `task serve:go` starten dieselbe EXE (gleicher Pfad), `bin/` ist nicht im Repo (`.gitignore`).

## Offene Fragen

keine

## Notizen

Gefunden beim Browser-Lauf zu T1.1 (Firewall-Dialog bei jedem Start).
