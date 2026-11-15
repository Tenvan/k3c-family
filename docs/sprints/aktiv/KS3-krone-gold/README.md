# KS3 · SIM · Monarch: Gold statt HP, Krone fällt, Raub statt Burgfall

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

- **Bereich:** `engine/sim/**`, `engine/room/**`, `data/**`, `testdata/golden/**`, `docs/protocol.md`, `native/k3c-game/**` (nur Anpassung an entfernte HP-Felder)
- **Sperre:** golden, protokoll, spielstand
- Sim-Code im eigenen Worktree (B-925).

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
| KS3.1 | `KS3.1-krone-gold.md` | Umsetzung | autonom | in Arbeit |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
