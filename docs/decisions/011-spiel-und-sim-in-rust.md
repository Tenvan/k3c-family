# 011 · Spiel und Simulation in Rust, Go bleibt nur für die Werkzeuge

- **Status:** angenommen (🧑, 2026-11-25; vorgeschlagen von Claude in RU41.2)
- **Datum:** 2026-11-25
- **Projekt:** RSU · Tickets: B-1014 (Analyse und Vorschlag), B-1017 (Workbench ohne Engine)
- **Vorgänger:** 008 (natives Spiel in Go + Ebitengine), 009 (nur natives Spiel), 012 (Port-Orakel: Verhalten), 013 (neue Mechaniken nur in Rust)

## Anlass

Projekt RSU hat das Spiel und die Simulation von Go nach Rust portiert (Leitfaden `docs/standards/rust-port.md`). G0 (Umstieg) und G4
(Abnahme am PC) sind von 🧑 entschieden; Stand der übrigen Gates in der Tabelle unten. Danach entfernten RU37 das Go-Spiel `native/`, RU40 die Läufe der Werkzeuge aus der Go-Engine und
RU41 die Go-Engine selbst. 008 beschreibt mit „Go + Ebitengine, Rust entfällt“ den Stand vor dem Port.

## Entscheidung

1. **Ein Rust-Programm:** `rust/crates/k3c-game` ist das Spiel und rechnet die Simulation im selben Prozess (Crates
   `k3c-sim`, `k3c-level`, `k3c-room`, `k3c-data`, `k3c-balance`, `k3c-rng`).
2. **Läufe aus derselben EXE:** Sim-, Level-, Welten-, Hub- und Balancing-Läufe kommen aus `k3c-game sim …`, headless oder
   mit Fenster (`docs/standards/k3c-game-laeufe.md`). Workbench und Studio starten die EXE und rechnen nicht selbst.
3. **Orakel:** Die Golden unter `testdata/golden/` und die Fixtures unter `testdata/rust/` bleiben das Orakel für die
   portierten Regeln (012, 013). Seit RU41 sind sie eingefroren, weil es keinen Go-Schreibweg mehr gibt. Eine Regeländerung,
   die ein Golden ändert, braucht vorher ein Ticket für einen Rust-Schreibweg.
4. **Rückfall:** Der letzte Go-Stand liegt im Tag `go-referenz-final` (Spiel, Engine, Golden-Schreibweg; Bau und Messung in
   `docs/standards/go-referenz.md`). Im Arbeitsbaum gibt es keinen Go-Rückfall mehr.
5. **Werkzeuge bleiben Go und TypeScript:** Workbench, Studio und die CLIs unter `tools/` bleiben Go (Wails). Von
   `engine/` bleiben nur `conlog`, `devrun` und `internal/rules`. Ein Umstieg der Werkzeuge auf Rust ist eine eigene
   Entscheidung (014, RU65).
6. **Aus 008 gilt weiter:** die Punkte zu Rig-Zielbild und Gen-Skript (008 Punkte 4 bis 6). Die Punkte 1 bis 3 (Go +
   Ebitengine, Rust entfällt) löst diese Entscheidung ab.

## Folgen

- 008 trägt im Kopf „überholt durch 011“; ihr Inhalt bleibt.
- `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md` und die Agenten unter `.claude/agents/` beschreiben den Rust-Stand
  (RU41.2). Neue Mechaniken entstehen in `rust/crates/k3c-sim` (013), Determinismus über `k3c-rng`.
- Die Sim-Worktree-Regel B-925 (eigene Workbench-Instanz, weil die Workbench die Engine im Prozess rechnete) verliert
  ihren Grund. Ob sie entfällt, entscheidet Ralf (Ticket aus RU34.3); bis dahin gilt sie für `rust/crates/k3c-sim`.
- `task go:golden`, `go:cover` und der Namensraum `k3c-sim` entfallen (RU41.1).

## Gates

| Gate | Bericht | Ergebnis |
|---|---|---|
| G0 Ruckler-Ursache | RU0.2 › Ergebnis, Projekt RSU | entschieden 🧑 2026-11-16: „Wir steigen definitiv auf Rust um.“ |
| G1 Spike | RU2 (Schwellen aus B-1016) | Sprint erledigt; Bericht unter `docs/messung/` fehlt |
| G2 Ruckler | `docs/messung/rust-ruckler.md` | Gate-Läufe verschoben nach B-1074 (🧑) |
| G3 Sim-Abgleich | `docs/messung/rust-sim-abgleich.md` | Empfehlung bestanden (15 Golden gleich); Entscheidung 🧑 im Bericht offen |
| G4 Abnahme | `docs/messung/rust-abnahme.md` | entschieden 🧑 2026-11-24: weiter (Resttickets B-1162, B-1264) |

Die offenen Punkte bei G1 bis G3 hindern 011 nicht (🧑 hat 011 angenommen), sie halten aber B-1014 › AC-03 und den
Abschluss des Projekts RSU offen.
