# B-986 · world_override_set nennt bei einer Ablehnung die erlaubten Dateien und gesperrten Felder

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WE5
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Praxistest Welten-MCP (WE3): `world_override_set` lehnte `events` und `hub › sites` ab, ohne zu sagen, welche Dateien erlaubt und welche Felder gesperrt sind. Die Regel steht in `docs/rules/welten.md` § 1.1 und § 1.3, der Code in `tools/k3c-workbench/internal/world/override.go`, `values.go` und `engine/sim/world_values_locked.go`.

## Ziel

Wer einen Override setzt, liest aus der Ablehnung, was erlaubt ist, ohne im Regelwerk zu suchen.

## Beteiligte und Zielgruppen

Claude und andere Agenten über MCP, 🧑 im Welten-Editor.

## Anforderungen

- Unbekannte Datei → Meldung nennt die erlaubten Dateien (aus der Engine gelesen, keine zweite Liste in der Workbench).
- Gesperrtes Feld → Meldung nennt den Grund und den Verweis `welten.md § 1.3`.
- Unbekanntes Feld → Meldung nennt die Felder der Ebene (höchstens 12, danach `…`).
- Die Tool-Beschreibung von `world_override_set` nennt die erlaubten Dateien.

## Nicht-Ziele

Neue erlaubte Dateien oder entsperrte Felder (B-982, B-983), Texte der Engine-Prüfung ändern.

## Regeln und Einschränkungen

Klasse A (Problem, DEV, keine Spielregel, kein Format).

- **Bereich:** `tools/k3c-workbench/internal/world/override.go`, `tools/k3c-workbench/internal/world/override_test.go`, `tools/k3c-workbench/internal/world/values.go`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`world_override_set file: biomes` → `unbekannte Datei "biomes" (erlaubt: buildings, economy, enemies, goals, hub, monarch, ship, troops, waves)`.

## Ausnahme- und Fehlerfälle

Die Liste der erlaubten Dateien wächst in der Engine (B-982) → die Meldung folgt ohne Änderung in der Workbench (Test liest die Liste aus der Engine).

## Akzeptanzkriterien

- **AC-01** Test: unbekannte Datei → Meldung enthält alle erlaubten Dateien.
- **AC-02** Test: gesperrtes Feld und unbekanntes Feld → Meldung mit Grund bzw. Feldern der Ebene.
- **AC-03** `check_run task:check:dev` grün.

## Offene Fragen

keine

## Notizen

Zerlegt aus B-959 (Punkt Fehlermeldung mit erlaubten Dateien).
