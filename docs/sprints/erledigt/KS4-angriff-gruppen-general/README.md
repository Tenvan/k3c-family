# KS4 · SIM · Aktiver Angriff: Gruppen mit Schild und Auslöser, General mit Banner, Variante je Welt

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

KS2 (#611) und KS3 (#613) gemergt: Portale mit HP und Schadens-Schnittstelle `Troop.attacking`, Krone statt HP.

## Ziel

Aktiver Angriff je Welt: Gruppen (Schild, Auslöser) oder General (Banner).

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- Hinweis (B-1143, 2026-11-19): Alle Sessions sind fertig. Die Sim liegt seit RU10 bis RU22 in Rust, das Go-Spiel entfällt mit RU37; die Parität prüfen RU60.2 und RU60.4, der Abschluss folgt in RU60.5. Die Pfade `engine/*` und `native/k3c-game/*` stehen nur noch im Tag `go-referenz-final` und entsprechen `rust/crates/k3c-sim`, `k3c-level`, `k3c-room`, `k3c-balance`, `rust/crates/k3c-game/src` (Entscheidung 013).
- Sim-Code lief im eigenen Worktree (B-925).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-sim/**`, `rust/crates/k3c-room/**`, `rust/crates/k3c-level/**`, `rust/crates/k3c-balance/**`, `data/**`, `testdata/golden/**`, `docs/protocol.md`, `docs/rules/kampf.md`, `rust/crates/k3c-game/src/**` (nur Eingabe für Banner/Auslöser)
- **Sperre:** golden, protokoll, spielstand

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Gruppen-Angriff mit Schild (10 Gold), Gruppenführer, Auslöser (2 Gold) am Tor, Rückkehr, Variante je Welt (`B-932`).
- **AC-02** General-Angriff mit Banner je Monarch, Trupp 5+5, Nachbesetzen (`B-933`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KS4.1 | `KS4.1-angriff.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

2026-11-19 (Second Worker, RU60.5): Alle Sessions fertig; Go-Abnahme entfällt mit RU37. Aktiver Angriff (Gruppen mit Schild und Auslöser, General mit Banner, B-932, B-933): Regeln in `k3c-sim/src/combat/` mit Tests (`gate_tests.rs`: `gateOutUnits`, `shieldMinHubLevel`); Anzeige in `props/plan_combat.rs` (Beleg RU60.2).
Sichtprüfung am TV: Rust-Abnahme-Liste. Keine neuen Tickets.
