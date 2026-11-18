# RU42 · CLI · Leitplanken für Leistung und Stabilität

- **Status:** erledigt
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1075
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18

## Ausgangslage

Der Rust-Workspace verbietet nur `unsafe` und zu lange Funktionen. Panics (`unwrap`, `expect`, `panic!`) und Allokationen im Bild- und Takt-Pfad sind weder durch Lints noch durch Tests begrenzt (B-1075, RU39: Allokationen nicht messbar).

## Ziel

Der Workspace erzwingt Stabilität per Lint (kein `unwrap`/`expect`/`panic!`/`todo!`/`unimplemented!` im Produktivcode) und Leistung per Test (Allokationen je Bild mit Obergrenze); der Port-Leitfaden nennt die Konvention. Am Ende sichtbar: `task rust:lint` schlägt bei neuem `unwrap` fehl, ein Test in `k3c-game` nennt die Allokationen je Bild.

## Beteiligte und Zielgruppen

Agenten setzen um und halten die Konvention in jedem Rust-Sprint ein; Ralf (🧑) hat die Leitplanken delegiert (2026-11-18).

## Anforderungen

- B-1075 (Allokationen je Bild messen), ohne `unsafe` im Spiel-Crate: eigenes Test-Hilfscrate `k3c-alloc-probe` ohne externe Abhängigkeiten.
- Workspace-Lints `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented` auf `deny`, in Tests erlaubt (`clippy.toml`).
- Abschnitt „Leistung und Stabilität“ in `docs/standards/rust-port.md`.

## Nicht-Ziele

Optimierungen selbst und Abbau markierter Schwachstellen (Sprint RU43). Keine Sim, kein Balancing, kein gen:paper, kein check:race.

## Regeln und Einschränkungen

`unsafe_code = forbid` bleibt für den Workspace; `k3c-alloc-probe` setzt selbst `deny` und erlaubt `unsafe` nur an der `GlobalAlloc`-Implementierung mit `// SAFETY:`. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Messung und Feinoptimierung am Ende (Ralf 2026-11-18).

- **Bereich:** `rust/Cargo.toml`, `rust/clippy.toml`, `rust/crates/**`, `docs/standards/rust-port.md`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** deps

## Beispiele

Ein neues `x.unwrap()` im Produktivcode → `task rust:lint` rot. Ein Bild mit mehr Allokationen als die Obergrenze → Test rot.

## Ausnahme- und Fehlerfälle

Läuft der Allokationstest ohne gerenderte Grafik nicht, wird er wie die `still_*`-Tests übersprungen. Bewusst belassene Stellen tragen `// STAB(B-xxxx)` oder `// PERF(B-xxxx)` mit Ticket in RU43.

## Akzeptanzkriterien

- **AC-01** `clippy::unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented` stehen im Workspace auf `deny`; Tests sind per `clippy.toml` ausgenommen; `task rust:lint` ist grün.
- **AC-02** Bestehende Verstöße sind behoben oder je Datei mit `STAB(B-xxxx)` markiert und als Ticket in RU43 erfasst.
- **AC-03** `k3c-alloc-probe` zählt Allokationen je Thread; `unsafe` nur an der `GlobalAlloc`-Implementierung.
- **AC-04** Ein Test in `k3c-game` zählt die Allokationen je Bild der Wiedergabe (nach 120 Aufwärmbildern) und schreibt das Maximum als Obergrenze fest (B-1075/AC-01).
- **AC-05** Je Hotspot existiert ein PERF-Ticket in RU43 (B-1075/AC-02).
- **AC-06** `docs/standards/rust-port.md` enthält den Abschnitt „Leistung und Stabilität“.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU42.1 | `RU42.1-lints-und-allokationsmessung.md` | Umsetzung | autonom | fertig |
| RU42.2 | `RU42.2-review.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-18 (autonom, Claude): AC-01 bis AC-06 erfüllt (RU42.1 › Ergebnis). Review ohne schwere Befunde, `task check` und `task check:go` grün. Neue Tickets: B-1076 (STAB Sim), B-1077 und B-1078 (PERF step, draw) in RU43; Allokationen je Bild heute Maximum 805. Kein Abnahme-Ticket für Sim, Balance, Gen (nicht berührt).
