# KS4 · SIM · Aktiver Angriff: Gruppen mit Schild und Auslöser, General mit Banner, Variante je Welt

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

- **Bereich:** `engine/sim/**`, `engine/room/**`, `engine/level/**`, `engine/balance/**`, `data/**`, `testdata/golden/**`, `docs/protocol.md`, `docs/rules/kampf.md`, `native/k3c-game/**` (nur Eingabe für Banner/Auslöser)
- **Sperre:** golden, protokoll, spielstand
- Sim-Code im eigenen Worktree (B-925).

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

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
