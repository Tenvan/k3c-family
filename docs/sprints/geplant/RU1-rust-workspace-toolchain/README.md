# RU1 · INF · Rust-Workspace, Toolchain, Taskfile und CI

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-1014, B-1015, B-1018
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-16

## Ausgangslage

Im Repo gibt es seit B-779 kein Rust mehr; lokal ist `cargo 1.99.1` (MSVC) installiert. Go-Tests, `check:diff` und CI kennen nur Go und TypeScript.

## Ziel

Ein leerer, baubarer Rust-Workspace mit allen Crates der Umstellung, eigenen Tasks und CI-Job. Am Ende sichtbar: `task rust:test` und der CI-Job „Rust“ sind grün, Go-Jobs unverändert.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Komplexitäts-Budget, Freigabe je Crate), `B-1015 › Anforderungen` (Wellen).

Welle 2 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Code bleibt unverändert lauffähig (Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `rust/Cargo.toml`, `rust/Cargo.lock`, `rust/rust-toolchain.toml`, `rust/clippy.toml`, `rust/rustfmt.toml`, `rust/Taskfile.yml`, `rust/crates/*/Cargo.toml`, `rust/crates/*/src/lib.rs`, `rust/crates/k3c-game/src/main.rs`, `.gitignore`, `docs/standards/rust-port.md`, `tools/checkdiff/**`, `.github/workflows/ci.yml`, `tests/projectRules.test.ts`, `Taskfile.yml`
- **Abhängig von:** RU0
- **Braucht 🧑:** Entscheidung
- **Sperre:** deps

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `rust/Cargo.toml` ist ein Workspace mit den Crates `k3c-rng`, `k3c-det`, `k3c-data`, `k3c-level`, `k3c-sim`, `k3c-room`, `k3c-balance`, `k3c-rig`, `k3c-game`; `cargo build --workspace` läuft unter Windows (MSVC) durch.
- **AC-02** Abhängigkeiten stehen nur unter `[workspace.dependencies]`, jede mit Freigabe 🧑 (Liste im Ziel-Ticket › Notizen); `Cargo.lock` ist eingecheckt.
- **AC-03** `task rust:build`, `rust:test`, `rust:lint` (clippy mit `-D warnings`) und `rust:fmt` existieren im Namensraum `rust`; `task check:diff` prüft nur geänderte Rust-Crates.
- **AC-04** Der CI-Job „Rust · fmt · clippy · Tests (Windows)“ ist grün; die Go-Jobs sind unverändert.
- **AC-05** `tests/projectRules.test.ts` prüft Rust-Dateien auf ≤ 400 Zeilen; clippy `too_many_lines` steht auf 60.
- **AC-06** `docs/standards/rust-port.md` enthält den Port-Leitfaden aus `B-1018 › Anforderungen` wörtlich; jede spätere Session verweist darauf.

## Offene Fragen

Entscheidung 🧑 am Gate bzw. vor dem Start (B-1014 › Offene Fragen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU1.1 | `RU1.1-workspace-und-leitfaden.md` | Umsetzung | autonom | offen |
| RU1.2 | `RU1.2-tasks-und-check-diff.md` | Umsetzung | autonom | offen |
| RU1.3 | `RU1.3-ci-und-repo-regeln.md` | Umsetzung | autonom | offen |
| RU1.4 | `RU1.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
