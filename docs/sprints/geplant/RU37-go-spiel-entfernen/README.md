# RU37 · INF · Go-Spiel entfernen, Doku und Regeln nachziehen

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** INF
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

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

1. Tag `go-final` setzen, `native/k3c-game` und `native/k3c-rig` entfernen (AC-01)
2. CI, Taskfile und Repo-Regeln ohne Go-Spiel (AC-02)
3. `CLAUDE.md`, `codemap.md`, Arbeitsweise, Agenten auf das Rust-Spiel (AC-03)
4. Review

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Stand vor diesem Sprint bleibt über Git erreichbar (B-1015 › Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `native/**`, `Taskfile.yml`, `.github/workflows/ci.yml`, `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md`, `tests/projectRules.test.ts`, `.claude/agents/**`
- **Abhängig von:** RU36
- **Braucht 🧑:** Entscheidung
- **Sperre:** deps

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Der Tag `go-final` zeigt auf den letzten Stand mit Go-Spiel; `native/k3c-game` und `native/k3c-rig` sind entfernt, `engine/` ist unverändert.
- **AC-02** `task check:all` ist grün; CI und Taskfile bauen nur noch das Rust-Spiel.
- **AC-03** `CLAUDE.md`, `codemap.md` und `docs/arbeitsweise.md` nennen das Rust-Spiel und sagen, dass die Go-Engine nur noch den Läufen der Workbench dient (bis RU41).

## Offene Fragen

Entscheidung 🧑 am Gate bzw. vor dem Start (B-1014 › Offene Fragen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
