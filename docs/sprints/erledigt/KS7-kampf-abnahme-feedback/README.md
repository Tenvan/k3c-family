# KS7 · SIM · Kampf-Feedback aus der Sichtprüfung: Auslöser am Pfahl vor dem Tor, Schild-Gestell an der Burg

- **Status:** erledigt
- **Projekt:** KSY
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Sichtprüfung 🧑 2026-11-15 zu KS5/KS6.

## Ziel

Auslöser am Pfahl vor dem Tor, Schild-Gestell ab höherer Burgstufe.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- Hinweis (B-1143, 2026-11-19): Alle Sessions sind fertig. Sim in Rust seit RU10 bis RU22, das Go-Spiel entfällt mit RU37; die Parität prüft RU60.2, der Abschluss folgt in RU60.5. Die Pfade `engine/sim` und `native/k3c-game/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-sim`, `rust/crates/k3c-game/src/*` (Entscheidung 013).
- Sim-Code lief im eigenen Worktree (B-925).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-sim/**`, `data/**`, `public/grafik/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-game/src/actors/**`
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Auslöser/Banner als Pfahl vor dem Tor (`B-941`).
- **AC-02** Schild-Gestell ab Burgstufe, Stapel sichtbar (`B-942`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KS7.1 | `KS7.1-pfahl-gestell.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-19 (Second Worker, RU60.5): Alle Sessions fertig; Go-Abnahme entfällt mit RU37. Auslöser am Pfahl vor dem Tor (B-941) und Schild-Gestell ab Burgstufe (B-942): Tests `combat/gate_tests.rs` (Abstand und Mindest-Burgstufe aus `data/economy.json`), Anzeige `TRIGGER_LOOK` und `RACK_LOOK` in `props/plan_combat.rs` (RU60.2).
Offen: Sichtprüfung am TV (AC-03 beider Tickets, Rust-Abnahme-Liste); Gestell-Stapelhöhe nicht belegt (B-942/AC-02).
