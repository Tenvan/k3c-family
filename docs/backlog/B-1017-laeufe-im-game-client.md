# B-1017 · Sim und Balancing laufen im Game-Client, headless oder mit Fenster per Parameter

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU33
- **Projekt:** RSU
- **Erstellt:** 2026-11-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Sim-, Balancing- und Level-Läufe rechnet heute die Workbench im eigenen Prozess (`engine/simcli`, WE1, B-924); `cmd/k3c-sim` ist die dünne CLI dazu. Sim-Änderungen brauchen deshalb einen eigenen Worktree mit eigener Workbench (B-925). 🧑 am 2026-11-16: Simulation und Balancing sollen mit dem Game-Client laufen, per Parameter headless oder mit Fenster; die Workbench ruft das in ihren MCP-Tools auf.

## Ziel

Eine EXE für Spielen und für Läufe: Was ein Lauf rechnet, ist genau das Spiel. Die Workbench startet die Spiel-EXE; mit Fenster sieht man dem Lauf zu.

## Beteiligte und Zielgruppen

🧑 und Agenten starten Läufe über die MCP-Tools der Workbench. 🧑 sieht Bots mit Fenster beim Spielen zu.

## Anforderungen

- `k3c-game sim <run|level|balance|world|hub> [--input DATEI|-]` mit `--headless` (Standard bei `sim`) oder `--window`. Unterbefehle, JSON-Zeilen und Exit-Codes bleiben, wie sie der Kopfkommentar von `cmd/k3c-sim/main.go` beschreibt; ohne `sim` startet die EXE das Spiel.
- Headless läuft ohne Fenster, ohne Grafikkontext und ohne Ton, auch in der CI und als Kindprozess.
- Gleicher Seed und gleiche Parameter ergeben in beiden Modi denselben Bericht.
- Fortschritt und Ergebnis als JSON-Zeilen auf stdout (`progress`, `result`, `report`, `error`, `aborted`), Berichte als Dateien wie bisher unter `reports/`, Exit-Codes 0, 1, 2, 130; Abbruch per CTRL_BREAK.
- Mit Fenster spielen die Bots sichtbar in zwei Bildhälften; Zeitraffer ist wählbar.
- Die MCP-Tools `sim_run`, `sim_test`, `replay_run`, `level_generate` und das Balancing bekommen den Parameter `head` (Standard: headless) und starten die EXE des gewählten Checkouts als Kindprozess im Job-Objekt.
- Die Workbench importiert danach keine Engine mehr.

## Nicht-Ziele

Die Sim-Sperre aufheben. Neue Bots oder Kennzahlen. Läufe über das Netz.

## Regeln und Einschränkungen

Sim-Sperre (Beschluss 🧑 2026-11-11): Bau und Tests der Läufe ja, echte Sim- und Balancing-Läufe erst nach Widerruf. EXE mit festem Pfad, Proben nur auf Loopback. Umsetzung nach G4 (🧑 2026-11-16: Sim und Balancing so weit hinten wie möglich): RU33 (Sim, Level, Welten, Hub im Game-Client), RU34 (Workbench dazu), zuletzt RU40 (Balancing im Client und in der Workbench). Berücksichtigt wird es von Anfang an: RU4 trennt die Schleife vom Fenster.

- **Bereich:** `rust/crates/k3c-game/src/runs/**`, `tools/k3c-workbench/**`
- **Abhängig von:** RU36
- **Braucht 🧑:** –
- **Größe:** L
- **Sperre:** exe

## Beispiele

`k3c-game sim run --headless --input lauf.json` (JSON `{"biome","seed","ticks","inputs"}`) → `{"event":"result",…}`, Code 0, kein Fenster. `sim_run {head: true}` in der Workbench → das Spiel öffnet sich, der Lauf ist sichtbar, der Bericht ist derselbe wie headless.

## Ausnahme- und Fehlerfälle

Die EXE fehlt oder ist älter als der Checkout → das MCP-Tool nennt den Bau-Befehl und rechnet nicht. `--window` ohne Bildschirm → Fehler mit Code ≠ 0, kein stiller Wechsel auf headless. Ein Lauf hängt → `stop` beendet den ganzen Prozessbaum.

## Akzeptanzkriterien

- **AC-01** Die Schnittstelle ist in `docs/standards/k3c-game-laeufe.md` beschrieben (Unterbefehle, Parameter, Fortschritt, Bericht, Exit-Codes).
- **AC-02** Sim, Wiedergabe, Level und Balancing laufen headless ohne Grafikkontext und schreiben Berichte im bisherigen Format.
- **AC-03** Derselbe Lauf mit Fenster liefert bei gleichem Seed denselben Bericht.
- **AC-04** Die MCP-Tools der Workbench starten die Spiel-EXE; `head` wählt den Modus; nach dem Ende bleibt kein Prozess übrig.
- **AC-05** `tools/k3c-workbench` importiert keine Engine mehr; die Regel B-925 (eigene Workbench je Sim-Worktree) ist angepasst.

## Offene Fragen

1. **Schon im Go-Spiel (🧑)?** Die Anforderung hängt nicht an Rust: `native/k3c-game` um `run` zu erweitern und WE1 zurückzubauen kostet geschätzt 8–10 Sessions (ungeprüft). Vorschlag: als eigener Sprint in WZG vorziehen, unabhängig von RSU.
2. Standard bei `sim_test`: headless (Vorschlag).
4. Größe L: Vor der Freigabe wird das Ticket je Sprint geteilt (RU33, RU34, RU40).
3. Mit Fenster: Echtzeit oder Zeitraffer als Standard (🧑)?

## Notizen

Folgen für den Plan: Es gibt kein eigenes `k3c-sim` in Rust. RU4 baut die Spielschleife von Anfang an unabhängig vom Fenster (RU4/AC-02); RU2/AC-04 prüft das schon im Spike.
