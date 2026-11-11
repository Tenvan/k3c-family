# B-779 · Rust und der Bevy-Prototyp sind aus dem Repo entfernt, das Projekt ist rein Go

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Entscheidung 008 (2026-11-11): Das native Spiel entsteht in Go + Ebitengine, Rust entfällt komplett. Im Repo liegen noch `rust/` (Workspace `k3c-core`: rng, Münzen, Level per FFI, 2,6k Zeilen, nur von Tests aufgerufen: `engine/sim/coins_spike_test.go`, `engine/level/rust_test.go`), der Bevy-Prototyp `native/k3c-proto`, Rust-Aufgaben in `Taskfile.yml`, `rust/Taskfile.yml` und `native/Taskfile.yml`, dazu Projekt RST mit den Entscheidungen 006 und 007.

## Ziel

Das Repo enthält keinen Rust-Code und keine Rust-Schritte mehr; Doku, Projekte und Entscheidungen nennen Go + Ebitengine als Weg.

## Beteiligte und Zielgruppen

Entwickler (🧑, Claude): kein Rust-Toolchain mehr nötig, kürzere Prüfläufe.

## Anforderungen

- `rust/` und `native/k3c-proto` sind gelöscht, ebenso die FFI-Brücke und ihre Tests in `engine/`.
- Taskfiles, CI, Dockerfile, `.gitignore`, `requirements.md` und das Messskript `native/measure.ps1` kennen kein Rust und kein Bevy mehr.
- `CLAUDE.md`, `docs/arbeitsweise.md`, Glossar und Codemaps nennen 008 statt 006.
- Entscheidungen 006 und 007 tragen den Status „überholt durch 008“; Projekt RST ist erledigt oder verworfen.

## Nicht-Ziele

Änderungen an der Go-Simulation, am Ebitengine-Prototyp oder an Messwerten; offene RST-Tickets inhaltlich bearbeiten.

## Regeln und Einschränkungen

Erst suchen, dann löschen (Befehl und Ergebnis ins Ergebnis der Session). Die Messwerte des Bevy-Prototyps bleiben in B-776 und B-777 erhalten. Tickets verwirft nur 🧑.

## Beispiele

`rg -l -i "cargo|rustc|bevy|k3c-core"` liefert nach dem Sprint nur noch Treffer in `docs/decisions/`, `docs/backlog/archiv/`, `docs/sprints/erledigt/` und in den Tickets B-775 bis B-779.

## Ausnahme- und Fehlerfälle

Ein Go-Test hängt noch an der Rust-Bibliothek → Test löschen, wenn er nur den Spike prüft; sonst stoppen und melden.

## Akzeptanzkriterien

- **AC-01** Die Suche aus den Beispielen ist außerhalb der genannten Ordner leer; `rust/` und `native/k3c-proto` fehlen.
- **AC-02** `task check:all` ist grün, ohne installiertes Rust.
- **AC-03** 006 und 007 sind als überholt markiert, Projekt RST ist geschlossen (Sprints RS1 und RS2 abgeschlossen, übrige Tickets einem anderen Projekt zugeordnet).

## Offene Fragen

- Die übrigen Tickets aus RST betreffen Go und die Workbench, nicht Rust (B-726, B-731, B-743, B-744, B-746, B-747, B-753, B-754). Default: nach WZG bzw. LST umhängen; B-731, B-753 und B-754 prüfen, ob sie mit dem Monolithen ohne Server entfallen.

## Notizen

Angelegt 2026-11-11 mit Entscheidung 008. Sinnvoll erst nach der Messung am Mini-PC (B-777/AC-06), damit der Bevy-Prototyp dort noch als Vergleich laufen kann.

2026-11-11 (🧑, Chat): Rust-Tickets B-725 und B-736 verworfen, Pi-Sessions RS1.2 und RS2.5 verworfen; „wir gehen wieder komplett auf Go“.
