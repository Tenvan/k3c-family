# KS3 · SIM · Monarch: Gold statt HP, Krone fällt, Raub statt Burgfall

- **Status:** erledigt
- **Projekt:** KSY
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

KS2 gemergt (#611): Portale mit HP, sichere Stufe, keine Kaskade.

## Ziel

Monarch ohne HP: Gold je Treffer, Krone fällt, Kronenraub statt Burgfall.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- Hinweis (B-1143, 2026-11-19): Alle Sessions sind fertig. Die Sim liegt seit RU10 bis RU22 in Rust, das Go-Spiel entfällt mit RU37; die Parität prüfen RU60.2 und RU60.4, der Abschluss folgt in RU60.5. Die Pfade `engine/*` und `native/k3c-game/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-sim`, `k3c-room`, `rust/crates/k3c-game/src` (Entscheidung 013).
- Sim-Code lief im eigenen Worktree (B-925).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `data/**`, `testdata/golden/**`, `docs/protocol.md`, `rust/crates/k3c-game/src/**` (nur Anpassung an entfernte HP-Felder)
- **Sperre:** golden, protokoll, spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Gold je Treffer, Münzen aufsammelbar und raubbar, Lava 1 Münze/s, HP-Skills auf Gold umgedeutet (`B-930`).
- **AC-02** Krone fällt, Raub ins Portal = Game Over, Aufheben, zwei Kronen, Burgfall nur Schaden bzw. ultra Stufenverlust (`B-931`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KS3.1 | `KS3.1-krone-gold.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-19 (Second Worker, RU60.5): Alle Sessions fertig; Go-Abnahme entfällt mit RU37. Monarch zahlt Treffer mit Gold, Krone fällt und wird geraubt: alle 15 Go-Krone-Tests haben Rust-Gegenstücke (`k3c-sim/src/crown/`, Beleg RU60.4).
Befund: B-1008 (laufender Kronenraub nach Verlassen und Weiter) betrifft auch Rust, `ignore`-Test `island/save_crown_tests.rs`; Behebung braucht Format-Entscheidung 🧑 (Ticket bleibt offen).
Sichtprüfung am TV: Rust-Abnahme-Liste.
