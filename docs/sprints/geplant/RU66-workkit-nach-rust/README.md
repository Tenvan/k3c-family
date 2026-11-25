# RU66 · DEV · workkit nach Rust: mcpkit, applog, console, proc, usage, winicon, conlog, devrun

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

Entwurf. Abhängig von RU65 (Entscheidung 014 angenommen). `tools/workkit` (mcpkit, applog, console, proc, usage, uikit, claude, winicon, rund 2.900 Zeilen) sowie `engine/conlog` und `engine/devrun` (rund 800 Zeilen) sind der gemeinsame Code beider Apps.

## Ziel

Der gemeinsame Code liegt als Rust-Crates im Workspace (`rust/`), mit portierten Tests; Workbench und Studio laufen weiter auf Go, bis ihre Backends folgen (RU69, RU70). Kleinster Schritt zuerst: mcpkit und applog.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft; 🧑 gibt neue Crates frei (Teil von Entscheidung 014).

## Anforderungen

Port von `tools/workkit` (`mcpkit`, `applog`, `console`, `proc`, `usage`, `winicon`; `uikit` und `claude` nur, soweit das Backend sie braucht) sowie `engine/conlog` und `engine/devrun` nach Rust-Crates unter `rust/crates/`, Verhalten wie Go, Tests mitportiert. Reihenfolge: 1. `mcpkit` und `applog`, 2. `console`, `proc` (Prozessbäume, Job-Objekt, CTRL_BREAK), 3. `usage`, `winicon`, `conlog`, 4. `devrun` (Supervisor).

## Nicht-Ziele

Backends von Workbench und Studio (RU69, RU70). Entfernen des Go-Codes (RU71).

## Regeln und Einschränkungen

Port ohne Verhaltensänderung; Go bleibt lauffähig und ist das Orakel. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur nach Annahme von Entscheidung 014.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-kit*/**`, `rust/Cargo.toml`, `rust/Cargo.lock`
- **Abhängig von:** RU65
- **Braucht 🧑:** –
- **Sperre:** deps

## Beispiele

`mcpkit` in Rust beantwortet `initialize`, `tools/list` und `tools/call` wie die Go-Fassung; ein portierter Test mit denselben Eingaben gibt dieselben Ausgaben.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Abweichung beheben oder als Ticket erfassen, nie den Test anpassen.

## Akzeptanzkriterien

- **AC-01** Die genannten Pakete liegen als Rust-Crates vor, ihre portierten Tests sind grün (`cargo test`, `task rust:lint`).
- **AC-02** Kein Go-Code geändert; `task check:diff` grün.

## Offene Fragen

Mit Default entschieden (autonom, Claude, 2026-11-25; 🧑 kann widersprechen): Ein Crate `k3c-kit` mit Modulen statt eines Crates je Paket.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Stichpunkte (Reife Entwurf): 1. mcpkit, applog (AC-01). 2. console, proc (AC-01). 3. usage, winicon, conlog (AC-01). 4. devrun (AC-01). 5. Review (AC-02).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
