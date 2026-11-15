# KS7 · SIM · Kampf-Feedback aus der Sichtprüfung: Auslöser am Pfahl vor dem Tor, Schild-Gestell an der Burg

- **Status:** aktiv
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

- **Bereich:** `engine/sim/**`, `data/**`, `public/grafik/**`, `native/k3c-game/props/**`, `native/k3c-game/actors/**`
- **Sperre:** golden
- Sim-Code im eigenen Worktree (B-925).

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

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
