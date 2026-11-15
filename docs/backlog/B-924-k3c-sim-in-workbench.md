# B-924 · Die Workbench nutzt den k3c-sim-Code direkt statt über den Kindprozess

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WE1
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

B-726 hat die Workbench von der Engine getrennt: Simulation, Balancing, Level, Welt- und Hub-Prüfung laufen über den Kindprozess `bin/k3c-sim` (`internal/simexe`, depguard-Listen in `tools/k3c-workbench/.golangci.yml`). Grund war die geplante Rust-Engine (Entscheidung 006); mit 008 ist sie entfallen. Jede neue Workbench-Funktion braucht seitdem einen Unterbefehl in `cmd/k3c-sim`, JSON hin und zurück und einen Fake im Test (zuletzt B-923). Hinweis 🧑 2026-11-15: Der Code soll wieder in die Workbench, das macht die Weiterentwicklung deutlich einfacher.

## Ziel

Die Workbench ruft die Engine-Logik von `k3c-sim` direkt im Prozess auf; neue Funktionen brauchen keinen Unterbefehl und keinen JSON-Umweg mehr.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Die Logik der Unterbefehle von `cmd/k3c-sim` (run, balance, level, world, hub, replay) liegt in einem Go-Paket, das Workbench und `cmd/k3c-sim` gemeinsam nutzen; `cmd/k3c-sim` bleibt als dünne CLI für Taskfile und CI.
- Die Workbench ruft dieses Paket direkt; `internal/simexe` und die Fakes (`fakesim_test.go`, `fakehub_test.go`, `fakeengine_test.go`) entfallen, Tests laufen gegen die echte Engine.
- Ein Absturz (panic) in der Engine-Logik reisst die Workbench nicht mit: Aufrufe laufen mit `recover` und melden einen Fehler.
- Lange Läufe (`sim_test`) blockieren die Oberfläche nicht und bleiben abbrechbar.

## Nicht-Ziele

Keine Simulationen oder Balancing-Läufe zum Prüfen (Sperre 🧑 2026-11-11). Kein Umbau der Engine selbst.

## Regeln und Einschränkungen

- Entscheidung 006 ist überholt (008); B-726 wird zurückgenommen, die depguard-Regeln `workbench-ohne-engine` und `workbench-tests-ohne-sim` entfallen bzw. werden angepasst.
- `engine/` importiert weiter nichts aus `tools/`.
- **Bereich:** `cmd/k3c-sim/**`, `engine/enginetools/**`, `tools/k3c-workbench/**`
- **Abhängig von:** B-923
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** exe

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `tools/k3c-workbench` importiert die Engine-Logik direkt; `internal/simexe` ist entfernt, kein Workbench-Code startet `bin/k3c-sim`.
- **AC-02** `cmd/k3c-sim` ist eine dünne CLI über dasselbe Paket; `go test ./cmd/...` grün.
- **AC-03** Eine panic in der Engine-Logik wird zu einem Fehler im Tool bzw. auf der Seite (Test).
- **AC-04** `task check:dev` und `task check:go` grün.

## Offene Fragen

Soll `cmd/k3c-sim` ganz entfallen (Taskfile/CI dann über die Workbench-EXE)? Default: bleibt als dünne CLI, weil CI und `task balance` ohne Wails-Build auskommen (🧑).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
