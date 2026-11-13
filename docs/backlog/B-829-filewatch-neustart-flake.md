# B-829 · TestDateiAenderungStartetNeu bleibt unter Last stabil

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`tools/k3c-workbench/internal/services/filewatch_test.go` › `TestDateiAenderungStartetNeu` scheitert gelegentlich, wenn parallel andere Agenten testen (WZ42.1: im vollen `k3c-workbench:gotest` rot, einzeln `-count=10` grün). Vermutet (ungeprüft): Die zwei `touch` auf `a.go` und `b.go` fallen unter Last nicht in dasselbe Entprell-Fenster, dann gibt es drei Starts und `waitFor(count()==2)` läuft ab; die festen `time.Sleep(60/80 ms)` der Negativ-Prüfungen sind zusätzlich lastabhängig.

## Ziel

`check:dev` wird durch diesen Test nicht mehr rot, wenn andere Agenten parallel testen.

## Beteiligte und Zielgruppen

Claude (Umsetzung); alle parallel arbeitenden Agenten.

## Anforderungen

- Keine Behebung per längerem Sleep; die Aussage (eine Sammeländerung = ein Neustart, andere Endung und Stop = kein Neustart) bleibt.

## Nicht-Ziele

Umbau des Datei-Watch der Workbench.

## Regeln und Einschränkungen

Domäne DEV, Klasse A.

- **Bereich:** `tools/k3c-workbench/internal/services/filewatch*.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`go test -run TestDateiAenderungStartetNeu -count=50 ./internal/services` grün, während ein zweites `go test ./...` läuft.

## Ausnahme- und Fehlerfälle

Ursache liegt im Watcher selbst (zwei Neustarts für eine Sammeländerung) → Fix im Watcher mit Test.

## Akzeptanzkriterien

- **AC-01** `TestDateiAenderungStartetNeu -count=50` grün unter paralleler Last (zweites `go test ./...`), ohne verlängerte Sleeps.

## Offene Fragen

keine

## Notizen

Gefunden in WZ42.1 (im vollen `k3c-workbench:gotest` rot). Review WZ42.4: nicht reproduziert, `-count=30` allein und `-count=30` neben parallelem `go test -short -count=1 ./...` grün, in `check:dev` Lauf 1 grün; Prio deshalb niedrig. Bei nächstem Auftreten Ausgabe (Zahl der Starts) hier notieren.
