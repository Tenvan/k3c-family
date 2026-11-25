# RU70 · DEV · Studio-Backend in Rust: Welten, Hub, Assets, Spiel-EXE-Anbindung, MCP

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-25

## Ausgangslage

Entwurf. Abhängig von RU66, RU67 und RU40. Studio-Backend: rund 19.500 Zeilen Go, 11.800 Zeilen Tests; Frontend bleibt React. Nach RU40 entfallen die Importe der Go-Engine im Studio (rund 1.900 Zeilen Verbraucher-Code in `goengine`, `simrun/offline`, `world/override`, `tools_replay`).

## Ziel

Das Backend des Studios läuft in Rust hinter dem unveränderten Frontend: Welten, Inseln, Hub, Assets, Aufruf der Spiel-EXE, MCP-Server `k3c-studio` (5181).

## Beteiligte und Zielgruppen

Agenten setzen um; 🧑 nimmt das Studio in Rust ab (Welten-Editor, Assets, MCP).

## Anforderungen

Backend von `tools/k3c-studio` in Rust, Frontend unverändert. Teile: 1. Gerüst: Fenster, MCP-Server `k3c-studio` auf Port 5181 (UI-API 8193). 2. Welten, Inseln, Chunks, Hub (`world_*`, `hub_*`, `level_generate` über die Spiel-EXE). 3. Assets (`assets_*`, auf dem Crate aus RU67). 4. Sim und Balancing (`sim_*`, `replay_run`, `reports_list`, `saves_list`) über die Spiel-EXE. Die Go-Fassung bleibt bis zur Abnahme lauffähig.

## Nicht-Ziele

Workbench (RU69). Entfernen von Go (RU71). Neue Studio-Funktionen.

## Regeln und Einschränkungen

Tool-Namen, Parameter und Antworten des MCP bleiben gleich. Balancing schreibt nur unter `data/worlds/` (Beschluss 2026-11-22). Datei ≤ 400, Funktion ≤ 60 Zeilen. Abnahme nur durch 🧑 am PC.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-studio/**`, `rust/Cargo.toml`, `rust/Cargo.lock`, `tools/k3c-studio/**`, `Taskfile.yml`
- **Abhängig von:** RU66, RU67, RU40
- **Braucht 🧑:** Abnahme
- **Sperre:** deps, taskfile, exe

## Beispiele

`world_check` in der Rust-Studio-Fassung meldet für dieselbe Welt dieselben Befunde wie die Go-Fassung.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go. Fehlende MCP-Funktion: Erweiterung vorschlagen.

## Akzeptanzkriterien

- **AC-01** Die Rust-Studio-App startet als Fenster und im Dev-Betrieb und bedient den MCP auf 5181 mit allen bisherigen Tools.
- **AC-02** `task check:all` ist grün; 🧑 hat das Studio am PC abgenommen.

## Offene Fragen

Abhängig von Entscheidung 014 (🧑).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Stichpunkte (Reife Entwurf): 1. Gerüst (AC-01). 2. Welten, Hub, Level (AC-01). 3. Assets (AC-01). 4. Sim, Balancing, Berichte (AC-01). 5. Review und Abnahme 🧑 (AC-02).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
