# RU41 · INF · Go-Engine entfernen, Entscheidung 011 abschließen

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

Nach RU40 braucht nichts mehr die Go-Sim; `engine/sim`, `level`, `room`, `store`, `balance`, `rng` liegen doppelt im Repo.

**Stand 2026-11-19 (Ralf):** Der Go-Abbau läuft vorgezogen in RU58 (Referenz, Build und CI, Lizenzliste), RU37 (Go-Spiel entfernen) und RU59 (ungenutzte Go-Engine entfernen). RU41 ist der **späte Rest**: Was RU59 wegen der Workbench stehen lässt (Balancing: `engine/balance`, `simcli`, `enginetools`, `cmd/k3c-sim`; alles, was sie transitiv brauchen), entfernt RU41 erst, wenn RU40 die Workbench von der Engine gelöst hat. Die Vorbedingung (Workbench importiert keine der Sim-Pakete mehr) bleibt.

## Ziel

Sim und Balancing gibt es nur noch in Rust; Go bleibt für Workbench und Werkzeuge. Am Ende sichtbar: `task check:all` grün ohne `engine/sim`.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 26 von 26, 3 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`, Entscheidung 011 und Doku `opus`.

### Sessions

1. Haltepunkt 🧑, dann `engine/sim`, `level`, `room`, `store`, `balance`, `rng`, `enginetools`, `simcli`, `internal/golden`, `internal/detvec`, `internal/trace` und `cmd/k3c-sim` entfernen; `conlog`, `devrun`, `internal/rules` behalten; Taskfile, CI, `checkdiff`, `go.mod` (AC-01, AC-02)
2. `CLAUDE.md`, `codemap.md`, Arbeitsweise (Domänen, Sperren, Agenten), Entscheidung 011 mit Annahme durch 🧑 (AC-03)
3. Review, Abschluss des Projekts vorschlagen

### Vorentscheidungen

- Voraussetzung fürs Löschen: Workbench und Werkzeuge importieren keine der Sim-Pakete mehr (RU34, RU40; Prüfung mit `go list -deps`); sonst hält RU41.1 an.
- `testdata/` bleibt ganz; ungelesene Dateien werden nur genannt.
- Nach RU41 sind die Golden eingefroren, bis es einen Rust-Schreibweg gibt; die Arbeitsweise sagt das.

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt. Löschen nur nach Ja von 🧑 (RU41.1 hält an); Entscheidung 011 nimmt nur 🧑 an (RU41.2 hält an).

Vom 2026-11-19: RU59 hat den Bestand schon belegt und Ungenutztes entfernt (Belegtabelle in `docs/arbeitsweise.md` › Go-Engine-Bestand); RU41.1 geht davon aus und entfernt den verbliebenen Rest samt `engine/simcli`/`cmd/k3c-sim`, falls RU40.4 sie nicht schon entfernt hat.

- **Bereich:** `engine/**`, `cmd/**`, `go.mod`, `go.sum`, `testdata/**`, `Taskfile.yml`, `.github/workflows/ci.yml`, `tools/checkdiff/**`, `tools/k3c-workbench/go.mod`, `tools/k3c-workbench/go.sum`, `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md`, `docs/decisions/**`, `tests/projectRules.test.ts`, `.claude/agents/**`
- **Abhängig von:** RU37, RU40, RU59
- **Braucht 🧑:** Entscheidung
- **Sperre:** deps, taskfile

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die genannten Go-Pakete sind entfernt; die Golden unter `testdata/golden/` bleiben und werden von den Rust-Tests gelesen.
- **AC-02** `task check:all` ist grün; die Workbench baut ohne die entfernten Pakete.
- **AC-03** `CLAUDE.md`, `codemap.md` und `docs/arbeitsweise.md` beschreiben den Rust-Stand; Entscheidung 011 ist von 🧑 angenommen, 008 trägt „überholt durch 011“.

## Offene Fragen

Entscheidung 🧑 vor dem Löschen (RU41.1) und zur Annahme von 011 (RU41.2).

- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Zusätzlich entfernt werden `engine/simcli`, `engine/internal/golden`, `engine/internal/detvec`, `engine/internal/trace` und `cmd/k3c-sim`, weil sie nur der Go-Sim dienen; `engine/internal/rules` bleibt für die verbliebenen Go-Pakete.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): Bereich um `Taskfile.yml`, `ci.yml`, `tools/checkdiff` und die `go.mod` der Workbench erweitert, Sperre `deps, taskfile`.
- Mit Default entschieden (autonom, Claude, 2026-11-17; 🧑 kann widersprechen): `testdata/` wird nicht ausgedünnt; Golden sind nach RU41 eingefroren, bis ein Ticket einen Rust-Schreibweg schafft.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU41.1 | `RU41.1-go-engine-pakete-entfernen.md` | Umsetzung | autonom | offen |
| RU41.2 | `RU41.2-doku-und-entscheidung-011.md` | Umsetzung | autonom | offen |
| RU41.3 | `RU41.3-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
