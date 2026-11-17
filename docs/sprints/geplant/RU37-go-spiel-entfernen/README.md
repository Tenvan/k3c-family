# RU37 · INF · Go-Spiel entfernen, Doku und Regeln nachziehen

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-17

## Ausgangslage

Nach G4 liegt das Spiel doppelt im Repo. Die Go-Engine bleibt vorerst, weil Workbench und Balancing sie noch brauchen (bis RU40).

## Ziel

Das Repo enthält das Spiel nur noch in Rust. Am Ende sichtbar: `task check:all` grün ohne `native/k3c-game`.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 20 von 26, 4 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. Umzüge vor dem Löschen: Rig-Teile nach `public/rig/` (Kopie), Rig-Export nach `tools/rigexport/`, `cmd/sync` nach `tools/sync/`, Messskript nach `tools/measure/`, Workbench-Start auf die Rust-EXE (AC-01, Vorbereitung)
2. Haltepunkt 🧑, Tag `go-final`, `native/` entfernen, Taskfile, CI, `checkdiff` und Repo-Regeln ohne Go-Spiel (AC-01, AC-02)
3. `CLAUDE.md`, `codemap.md`, Arbeitsweise, `requirements.md`, Agenten auf das Rust-Spiel (AC-03)
4. Review

### Vorentscheidungen

- Das Rust-Spiel lädt heute die Rig-Teile aus `native/k3c-rig/assets`, `rust:game:build` kopiert mit `native/k3c-game/cmd/sync`, die Workbench startet `native/k3c-game/bin/k3c-game.exe`: Diese Abhängigkeiten werden in Session 1 gelöst, bevor Session 2 löscht. So bleibt jeder Commit grün.
- Gelöscht wird erst nach ausdrücklichem Ja von 🧑 in Session 2; G4 „weiter“ muss in der Abnahme von RU36 stehen.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt. `engine/` bleibt unverändert (Workbench und Balancing brauchen die Go-Engine bis RU41). Tag und Löschen nur nach Ja von 🧑 (RU37.2 hält an).

- **Bereich:** `native/**`, `Taskfile.yml`, `tools/Taskfile.yml`, `rust/Taskfile.yml`, `.github/workflows/ci.yml`, `.gitignore`, `CLAUDE.md`, `codemap.md`, `requirements.md`, `docs/arbeitsweise.md`, `tests/projectRules.test.ts`, `.claude/agents/**`, `public/rig/**`, `tools/rigexport/**`, `tools/sync/**`, `tools/measure/**`, `tools/checkdiff/**`, `tools/k3c-workbench/internal/studioview/**`, `rust/crates/k3c-game/src/art/library.rs`, `rust/crates/k3c-game/src/art/rigs_tests.rs`, `rust/crates/k3c-rig/src/rig/testutil.rs`, `rust/crates/k3c-rig/src/paint/assets.rs`
- **Abhängig von:** RU36
- **Braucht 🧑:** Entscheidung
- **Sperre:** deps, taskfile

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Der Tag `go-final` zeigt auf den letzten Stand mit Go-Spiel; `native/k3c-game` und `native/k3c-rig` sind entfernt, `engine/` ist unverändert.
- **AC-02** `task check:all` ist grün; CI und Taskfile bauen nur noch das Rust-Spiel.
- **AC-03** `CLAUDE.md`, `codemap.md` und `docs/arbeitsweise.md` nennen das Rust-Spiel und sagen, dass die Go-Engine nur noch den Läufen der Workbench dient (bis RU41).

## Offene Fragen

Entscheidung 🧑 vor dem Löschen (RU37.2 hält an und fragt).

- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Schnitt geändert: Session 1 zieht zuerst alles aus `native/` um, was Rust-Spiel, Paket und Workbench brauchen (Rig-Teile, Rig-Export, `cmd/sync`, Messskript, Workbench-Start); Tag und Löschen kommen zusammen mit CI, Taskfile und Regeln in Session 2. Grund: Sonst bricht das Rust-Spiel nach dem Löschen (`DEFAULT_RIG_DIR` = `native/k3c-rig/assets`).
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Neue Orte `public/rig/` (Rig-Teile), `tools/rigexport/` (TypeScript-Export, Tasks `gen:rig`, `gen:rig:check`), `tools/sync/` (Wurzel-Modul), `tools/measure/` (Messskript).
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Sperre `deps, taskfile`, weil der Sprint Taskfiles umbaut; Bereich um die Umzugsziele, `.gitignore`, `requirements.md`, `tools/checkdiff` und `studioview` erweitert.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU37.1 | `RU37.1-umzuege-vor-dem-loeschen.md` | Umsetzung | autonom | offen |
| RU37.2 | `RU37.2-tag-go-final-und-entfernen.md` | Umsetzung | autonom | offen |
| RU37.3 | `RU37.3-doku-und-agenten-auf-rust.md` | Umsetzung | autonom | offen |
| RU37.4 | `RU37.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
