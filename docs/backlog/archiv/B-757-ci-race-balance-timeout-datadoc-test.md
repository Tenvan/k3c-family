# B-757 · CI: Race-Lauf ohne engine/balance, datadoc-Test kennt Balance-Berichtsdateien

- **Domäne:** INF
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ41
- **Projekt:** WZG
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Zwei rote CI-Jobs auf develop nach B-756: (1) Job `go`, Schritt Race-Detector: `engine/balance` läuft unter `-race` in das 10-Minuten-Timeout von `go test` (normal 178 s; `net` braucht 473 s, `sim` 136 s). (2) Job `k3c-workbench`: `TestListeWieLoader` hält `run.json`, `result.json`, `balance.json` (Berichtsdateien von `engine/balance/baselines.go` und `insights_progress.go`, seit #519) für Datendateien, die engine/ lädt.

## Ziel

Beide Jobs sind grün.

## Beteiligte und Zielgruppen

Entwickler (🧑), Release.

## Anforderungen

- `task go:race` nimmt `engine/balance` aus (sie rechnet nur Simulationsläufe, die `engine/sim` unter `-race` selbst prüft; Balance-Läufe gehören nach B-742 ins Abnahme-Ticket) und setzt `-timeout 20m`.
- `TestListeWieLoader` ignoriert die Berichtsdateien `run.json`, `result.json`, `balance.json`.

## Nicht-Ziele

Kein schnelleres Balancing.

## Regeln und Einschränkungen

Entscheidung B-742 (Sim, Balance, Gen nur am Feature); Tasks nur über `task`. Umgesetzt mit #527, Beleg in WZ41.1.

- **Bereich:** `engine/Taskfile.yml`, `tools/k3c-workbench/internal/datadoc/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Push auf develop → `go` und `k3c-workbench` grün.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Prüfkonfiguration.

## Akzeptanzkriterien

- **AC-01** `task go:race` enthält `engine/balance` nicht.
- **AC-02** `go test ./internal/datadoc` im Workbench-Modul ist grün.
- **AC-03** CI auf develop: Jobs `go` und `k3c-workbench` grün.

## Offene Fragen

keine

## Notizen

Gefunden beim Release v0.22.0, nach B-756.
