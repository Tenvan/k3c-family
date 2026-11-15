# B-985 · world_set hängt eine Insel an und setzt den Hafen der Insel davor im selben Aufruf

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE5
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Praxistest Welten-MCP (WE3): Jede Insel außer der einzigen der letzten Etappe braucht einen Hafen in Tiefe 0 (`engine/sim/island_worlds.go`). `world_set` hängt Inseln an, den Hafen setzt aber nur `world_chunks_set`. Wer eine Insel anhängt, braucht deshalb zwei Aufrufe in fester Reihenfolge (erst Hafen der bisher letzten Insel, dann anhängen); sonst lehnt die Prüfung ab. Code: `tools/k3c-workbench/internal/world/edit.go`, `edit_island.go`, `patch.go`, `internal/mcpsrv/tools_world.go`.

## Ziel

Eine Insel anhängen ist ein Aufruf: `world_set` setzt Hafen und neue Insel zusammen und prüft erst das Ergebnis.

## Beteiligte und Zielgruppen

Claude und andere Agenten, die Welten über MCP bauen; 🧑 im Welten-Editor (nutzt denselben Code).

## Anforderungen

- Ein Insel-Eintrag im `patch` von `world_set` nimmt `plannedChunks` und `chunkCounts` je Tiefe an (gleiches Format wie `world_chunks_set`), auch für bestehende Inseln.
- Der ganze Patch wird angewendet und dann einmal geprüft (wie `world_check`); bei Fehler wird nichts geschrieben.
- Die Tool-Beschreibung nennt den Weg mit Beispiel; die Ablehnung „kein Hafen“ nennt Insel und den Weg zur Lösung.
- `world_chunks_set` bleibt unverändert.

## Nicht-Ziele

Automatisch einen Hafen erfinden, Änderung der Hafen-Regel in der Engine, Oberfläche des Editors.

## Regeln und Einschränkungen

Klasse A (Problem, DEV, keine Spielregel, kein Format). Radix nur über `@webkit` (hier keine Oberfläche).

- **Bereich:** `tools/k3c-workbench/internal/world/edit.go`, `tools/k3c-workbench/internal/world/edit_island.go`, `tools/k3c-workbench/internal/world/edit_test.go`, `tools/k3c-workbench/internal/world/patch.go`, `tools/k3c-workbench/internal/mcpsrv/tools_world.go`, `tools/k3c-workbench/internal/mcpsrv/tools_world_test.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`world_set` mit `islands: [{id: island2, plannedChunks: {"0": [… harbor …]}}, {id: island3, name: …, depths: [0, 4]}]` → eine Antwort, Welt gültig.

## Ausnahme- und Fehlerfälle

Patch hängt eine Insel an, ohne der bisher letzten einen Hafen zu geben → Ablehnung nennt die Insel und das Feld `plannedChunks`; die Datei bleibt unverändert. Ungültiger Chunk-Plan → Meldung der Engine mit Tiefe und Platz.

## Akzeptanzkriterien

- **AC-01** Test in `tools/k3c-workbench/internal/world`: ein `world_set`-Patch setzt Hafen und hängt eine Insel an; die Datei ist gültig.
- **AC-02** Test: fehlender Hafen → Ablehnung mit Insel und Hinweis, Datei unverändert.
- **AC-03** `check_run task:check:dev` grün.

## Offene Fragen

- Form → Default: `plannedChunks`/`chunkCounts` im Insel-Eintrag statt eines Kurzfelds `harbor: true`, weil der Platz des Hafens sonst geraten wäre (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-959 (Punkt Hafen im selben `world_set`).
