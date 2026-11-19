# KS2 · SIM · Portale zerstörbar, Stufe sicher, Kämpfer werden Bürger

- **Status:** aktiv
- **Projekt:** KSY
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Kampfregeln stehen in `docs/rules/kampf.md` (KS1).

## Ziel

Portale zerstörbar, sichere Stufe, Kämpfer werden Bürger, Werkzeug holen nur ohne Gegner in der Nähe.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- Hinweis (B-1143, 2026-11-19): Alle Sessions sind fertig. Die Sim liegt seit RU10 bis RU22 in Rust; die Parität prüft RU60.1 und RU60.4, der Abschluss folgt in RU60.5. Die Pfade `engine/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-sim`, `k3c-level`, `k3c-room` (Entscheidung 013).
- Sim-Code lief im eigenen Worktree (B-925), keine Sim-/Balancing-Läufe.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-level/**`, `rust/crates/k3c-room/**`, `data/**`, `testdata/golden/**`, `docs/protocol.md`
- **Sperre:** golden, protokoll, spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Portale mit HP, Verteidiger in Wellen, sichere Stufe dauerhaft, Skill-Punkt fürs letzte Portal (`B-927/AC-01` bis `AC-04`).
- **AC-02** Kämpfer werden Bürger, keine Kaskade, Bürger fliehen (`B-928`).
- **AC-03** Werkzeug holen mit 8 Units Gegner-Abstand (`B-929`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KS2.1 | `KS2.1-portale-buerger.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
