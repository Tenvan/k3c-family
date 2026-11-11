# NG10 · INF · Rust und Bevy entfernen

- **Status:** geplant
- **Projekt:** NAT
- **Domäne:** INF
- **Reife:** Entwurf
- **Tickets:** B-779
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit Entscheidung 008 entfällt Rust; im Repo liegen noch `rust/`, `native/k3c-proto`, die Rust-Brücke in `engine/core` und ihre Tests, Rust-Schritte in Taskfiles und CI, Projekt RST. Einzelheiten: B-779.

## Ziel

Das Repo ist rein Go. Am Ende sichtbar: `task check:all` grün ohne installiertes Rust, die Suche aus `B-779 › Beispiele` ist außerhalb der Archiv-Ordner leer.

## Beteiligte und Zielgruppen

Entwickler (🧑, Claude), 🧑 Ralf (Freigabe, Entscheidung zum Zeitpunkt).

## Anforderungen

`B-779 › Anforderungen`.

## Nicht-Ziele

`B-779 › Nicht-Ziele`; `native/k3c-ebiten` und `native/k3c-rig` (bleiben als Referenz).

## Regeln und Einschränkungen

`B-779 › Regeln und Einschränkungen`. Die Goldens bleiben grün (kein Neuschrieb). `check:all` läuft hier ausnahmsweise, weil das Ticket es verlangt.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/**`, `native/k3c-proto/**`, `engine/core/**`, `engine/sim/coins_spike_test.go`, `engine/level/rust_test.go`, `go.mod`, `go.sum`, `Taskfile.yml`, `native/Taskfile.yml`, `native/measure.ps1`, `.github/workflows/**`, `Dockerfile`, `.gitignore`, `requirements.md`, `CLAUDE.md`, `docs/arbeitsweise.md`, `docs/decisions/006-rust-engine.md`, `docs/decisions/007-rust-server.md`, `**/codemap.md`
- **Abhängig von:** NV3
- **Braucht 🧑:** Freigabe, Entscheidung
- **Sperre:** taskfile, deps

## Beispiele

`B-779 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-779 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Suche leer außerhalb der Archiv-Ordner, `rust/` und `native/k3c-proto` fehlen (`B-779/AC-01`).
- **AC-02** `task check:all` grün ohne Rust (`B-779/AC-02`).
- **AC-03** 006 und 007 überholt, Projekt RST geschlossen, übrige RST-Tickets umgehängt (`B-779/AC-03`).

## Offene Fragen

- Nicht auf den Mini-PC warten (`B-779 › Offene Fragen`). Vorschlag: nicht warten.
- Umhängen der RST-Tickets nach WZG bzw. LST (`B-779 › Offene Fragen`). Vorschlag wie dort.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG10.1 | `NG10.1-rust-loeschen.md` | Umsetzung | autonom | offen |
| NG10.2 | `NG10.2-doku-rst.md` | Umsetzung | autonom | offen |
| NG10.3 | `NG10.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
