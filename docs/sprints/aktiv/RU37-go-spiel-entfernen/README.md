# RU37 · INF · Go-Spiel entfernen, Doku und Regeln nachziehen

- **Status:** aktiv
- **Projekt:** RSU
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-1143
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-11-19 (Ralf: ausdrücklich)

## Ausgangslage

**Vorrang vor RU51/RU52 (Ralf 2026-11-19).** Umgebaut am 2026-11-19: Ralf hat ausdrücklich entschieden, alle nicht mehr gebrauchten Go-Builds jetzt zu entfernen (Ausnahme: die Workbench `tools/k3c-workbench` samt allem, was sie im Prozess braucht), alle drei Schritte sofort und ohne das Gate G4 (RU36) abzuwarten. Dieser Sprint ist Schritt 2: das Go-Spiel (`native/k3c-game`, `native/k3c-rig`) entfernen. Schritt 1 (Referenz sichern, Build und CI, Lizenzliste) ist RU58 und läuft vorher; Schritt 3 (ungenutzte Go-Engine) ist RU59 danach.

Das Go-Spiel bleibt nach dem Löschen als Tag `go-referenz-final` baubar (`docs/standards/go-referenz.md`); die Go-Seite der Bildvergleiche und Messungen liegt gesichert im Repo (RU58.2, RU58.3). `engine/` und `cmd/` bleiben: Die Workbench rechnet die Engine im Prozess (`engine/simcli`, `store`, `room`, `conlog`, `devrun` und deren Abhängigkeiten `sim`, `level`, `balance`, `rng`, `enginetools`).

Achtung Kollision: Offene Sprints und Tickets nennen `native/k3c-game` in ihrem Bereich (aktiv: GS1, GS2, KS3, KS4, KS5, KS6, KS7, WE4, WI1; geplant: GS3, GS4, WE6, WE8; dazu 42 offene Tickets). Sie ändern Spielfunktionen im Go-Spiel; das Go-Spiel fällt weg. Das ist ein Befund für Ralf und den Hauptrechner (Ticket B-1143), kein Hindernis für diesen Sprint.

## Ziel

Das Repo enthält das Spiel nur noch in Rust. Am Ende sichtbar: `native/` existiert nicht mehr, `task check:all` ist grün, `CLAUDE.md` nennt das Rust-Spiel; der Job `Go · Tests arm64` ist entfernt (B-1124 AC-02).

## Beteiligte und Zielgruppen

Second Worker setzt um (alle Sessions Lane `zweit`), die Review-Session prüft. Ralf (🧑) hat das Löschen am 2026-11-19 entschieden; es gibt keinen Haltepunkt mehr, nur den Nachweis der Referenz (RU58).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler), soweit sie das Löschen betreffen. Ralf-Entscheidung vom 2026-11-19 statt G4: kein Warten auf RU36.

4 Sessions (B-1015 gilt nicht mehr für die Reihenfolge; Welle 20 entfällt, Vorrang vor RU51 und RU52).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell: `sonnet`.

### Sessions

1. Umsüge vor dem Löschen: Rig-Teile nach `public/rig/` (Kopie), Rig-Export nach `tools/rigexport/`, `cmd/sync` nach `tools/sync/`, Messskript nach `tools/measure/`, Workbench-Start auf die Rust-EXE (AC-01, Vorbereitung)
2. `native/` entfernen (Tag `go-referenz-final` steht seit RU58.1), Taskfile, CI (inklusive Job `Go · Tests arm64`), `checkdiff` und Repo-Regeln ohne Go-Spiel (AC-01, AC-02)
3. `CLAUDE.md`, `codemap.md`, Arbeitsweise, `requirements.md`, Agenten auf das Rust-Spiel (AC-03)
4. Review

### Vorentscheidungen

- Das Rust-Spiel lädt heute die Rig-Teile aus `native/k3c-rig/assets`, `rust:game:build` kopiert mit `native/k3c-game/cmd/sync`, die Workbench startet `native/k3c-game/bin/k3c-game.exe`: Diese Abhängigkeiten werden in Session 1 gelöst, bevor Session 2 löscht. So bleibt jeder Commit grün.
- Gelöscht wird erst, wenn RU58 gemergt ist (Tag und Referenz auf `origin`); Session 2 prüft das (`git ls-remote --tags origin go-referenz-final`, Dateien der Referenz im Arbeitsbaum), sonst `blockiert`.
- `measure.ps1` (aus RU58.3: `-Engine game` gibt ohne Go-EXE die gesicherten Werte aus) zieht in Session 1 nach `tools/measure/`; der Zweig `game` bleibt dort ein reiner Referenz-Ausgabezweig ohne EXE-Suche unter `native/`.
- Go-Tests, die Testdaten für Rust schreiben (`rustref_test.go` in `native/k3c-game/...` und `engine/...`): Die in `native/` verschwinden mit dem Spiel; `testdata/rust/**` ist damit eingefroren (wie die Golden), bis ein Rust-Schreibweg existiert. Session 2 listet im Ergebnis, welche Dateien in `testdata/rust/` nur ein gelöschter Go-Test schreiben konnte.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. `engine/`, `cmd/` und die Workbench-Engine-Importe ändern (RU59, RU40, RU41). Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt für den Hauptrechner; der Second Worker darf Mess- und Gen-Läufe (2026-11-18). `engine/` bleibt unverändert (Workbench rechnet sie im Prozess). Tag und Löschen: Ralf 2026-11-19 ausdrücklich, kein Haltepunkt; Vorbedingung nur RU58 gemergt.

**Vorrang:** Alle Sessions Lane `zweit`, im Projekt RSU nach RU58 und vor RU51/RU52 (Ralf 2026-11-19). "Ralf 2026-11-19: alle drei Schritte sofort": Die frühere Abhängigkeit von RU36 entfällt bewusst.

- **Bereich:** `native/**`, `Taskfile.yml`, `tools/Taskfile.yml`, `rust/Taskfile.yml`, `.github/workflows/ci.yml`, `.gitignore`, `CLAUDE.md`, `codemap.md`, `requirements.md`, `docs/arbeitsweise.md`, `tests/projectRules.test.ts`, `.claude/agents/**`, `public/rig/**`, `tools/rigexport/**`, `tools/sync/**`, `tools/measure/**`, `tools/checkdiff/**`, `tools/k3c-workbench/internal/studioview/**`, `rust/crates/k3c-game/src/art/library.rs`, `rust/crates/k3c-game/src/art/rigs_tests.rs`, `rust/crates/k3c-rig/src/rig/testutil.rs`, `rust/crates/k3c-rig/src/paint/assets.rs`
- **Abhängig von:** RU58 (Ralf 2026-11-19: nicht mehr RU36; alle drei Schritte sofort)
- **Braucht 🧑:** –
- **Sperre:** deps, taskfile

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** `native/k3c-game` und `native/k3c-rig` sind entfernt (der Tag `go-referenz-final` aus RU58 zeigt auf den Stand davor), `engine/` ist unverändert.
- **AC-02** `task check:all` ist grün; CI und Taskfile bauen nur noch das Rust-Spiel; der Job `Go · Tests arm64` ist entfernt (B-1124/AC-02).
- **AC-03** `CLAUDE.md`, `codemap.md` und `docs/arbeitsweise.md` nennen das Rust-Spiel und sagen, dass die Go-Engine nur noch der Workbench dient (und bis RU59/RU41 gekürzt wird).

## Offene Fragen

keine. Mit Default entschieden (autonom, Claude, 2026-11-19, Ralf ausdrücklich: alle drei Schritte jetzt): Haltepunkt 🧑 und G4 vor dem Löschen entfallen; der Tag heißt `go-referenz-final` und entsteht in RU58.1 (nicht mehr `go-final` in RU37.2). Offene Tickets und Sprints mit `native/`-Bereich: B-1143 (Entscheidung Ralf), blockiert diesen Sprint nicht.

- Mit Default entschieden (autonom, Claude, 2026-11-17): Neue Orte `public/rig/` (Rig-Teile), `tools/rigexport/` (TypeScript-Export, Tasks `gen:rig`, `gen:rig:check`), `tools/sync/` (Wurzel-Modul), `tools/measure/` (Messskript).
- Mit Default entschieden (autonom, Claude, 2026-11-17): Sperre `deps, taskfile`, weil der Sprint Taskfiles umbaut.

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
