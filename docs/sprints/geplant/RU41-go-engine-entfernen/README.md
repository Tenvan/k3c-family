# RU41 · INF · Go-Engine entfernen, Entscheidung 011 abschließen

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

Nach RU40 braucht nichts mehr die Go-Sim; `engine/sim`, `level`, `room`, `store`, `balance`, `rng` liegen doppelt im Repo.

## Ziel

Sim und Balancing gibt es nur noch in Rust; Go bleibt für Workbench und Werkzeuge. Am Ende sichtbar: `task check:all` grün ohne `engine/sim`.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

`B-1014 › Anforderungen` (Port ohne Regeländerung, Golden als Orakel, zwei Spieler).

Welle 26 von 26, 3 Sessions (B-1015).

Für jede Session gilt der Port-Leitfaden `docs/standards/rust-port.md` (B-1018). Modell-Empfehlung: `sonnet`.

### Sessions

1. `engine/sim`, `level`, `room`, `store`, `balance`, `rng`, `enginetools` entfernen; `conlog` und `devrun` für die Workbench behalten (AC-01, AC-02)
2. `CLAUDE.md`, `codemap.md`, Arbeitsweise (Domänen, Sperren, Agenten), Entscheidung 011 (AC-03)
3. Review

## Nicht-Ziele

Regel- oder Wertänderungen. Die Workbench in Rust. Alles, was ein anderer Sprint des Projekts RSU nennt.

## Regeln und Einschränkungen

Entscheidung 008 gilt, bis 🧑 Entscheidung 011 annimmt (B-1014). Der Go-Stand vor diesem Sprint bleibt über Git erreichbar (B-1015 › Rückfall). Golden werden nie zugunsten von Rust neu geschrieben. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Neue Crates nur mit Freigabe 🧑. Die Sim-Sperre vom 2026-11-11 gilt; Golden-Vergleiche laufen nur als Tests und nur mit der Ausnahme aus B-1014.

- **Bereich:** `engine/**`, `cmd/**`, `go.mod`, `testdata/**`, `CLAUDE.md`, `codemap.md`, `docs/arbeitsweise.md`, `docs/decisions/**`, `tests/projectRules.test.ts`, `.claude/agents/**`
- **Abhängig von:** RU37, RU40
- **Braucht 🧑:** Entscheidung
- **Sperre:** deps

## Beispiele

Siehe Akzeptanzkriterien; jedes nennt den Befehl oder die Datei, an der man das Ergebnis sieht.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go: Die Abweichung wird behoben oder als Ticket erfasst, nie durch ein geändertes Golden verdeckt. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Die genannten Go-Pakete sind entfernt; die Golden unter `testdata/golden/` bleiben und werden von den Rust-Tests gelesen.
- **AC-02** `task check:all` ist grün; die Workbench baut ohne die entfernten Pakete.
- **AC-03** `CLAUDE.md`, `codemap.md` und `docs/arbeitsweise.md` beschreiben den Rust-Stand; Entscheidung 011 ist von 🧑 angenommen, 008 trägt „überholt durch 011“.

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
