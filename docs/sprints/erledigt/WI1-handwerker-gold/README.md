# WI1 · SIM · Handwerker mit Hammer bauen, Bogner-Shop, Gold beim Holzfällen, Truhen ×10 mit Fontäne

- **Status:** erledigt
- **Projekt:** WIR
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Hinweis 🧑 2026-11-15: Wirtschaft weicht vom Vorbild ab (Handwerker, Gold aus Holz, Truhen). Prio nach KS7 und FB4, für einen autonomen Lauf.

## Ziel

Handwerker mit Hammer, Bogner-Shop, Gold beim Holzfällen, ergiebige Truhen mit Fontaine.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- Hinweis (B-1143, 2026-11-19): Alle Sessions sind fertig. Sim in Rust seit RU10 bis RU22, das Go-Spiel entfällt mit RU37; die Parität prüft RU60.2, der Abschluss folgt in RU60.5. Die Pfade `engine/*` und `native/k3c-game/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-sim`, `k3c-balance`, `rust/crates/k3c-game/src/*` (Entscheidung 013).
- Sim-Code lief im eigenen Worktree (B-925); keine Sim-/Balancing-Läufe.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-balance/**`, `data/**`, `docs/rules/**`, `docs/data/**`, `testdata/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/hud/**`, `tools/ruletable/**`
- **Sperre:** golden, spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Handwerker und Start-Handwerker (`B-950`).
- **AC-02** Hämmer und Bogner (`B-951`).
- **AC-03** Gold aus Holz, Gegner-Drop gesenkt (`B-952`).
- **AC-04** Truhen ×10 mit Fontaine (`B-953`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WI1.1 | `WI1.1-handwerker-gold.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-19 (Second Worker, RU60.5): Alle Sessions fertig; Go-Abnahme entfällt mit RU37. Handwerker mit Hammer, Bogner, Gold aus Holz, Truhen-Fontäne (B-950 bis B-953): Truhenwert und Fontäne belegt (`data/economy.json › chestGold`, `economy/players_tests.rs`, `economy/gold_tests.rs`), Handwerker und Berufe in `unit/` mit mehr Tests als in Go (RU60.4).
Offen: Sichtprüfung am TV (AC-03 von B-953 und Handwerker-Anzeige, Rust-Abnahme-Liste).
