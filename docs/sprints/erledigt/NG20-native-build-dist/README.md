# NG20 · CLI · Natives Spiel komplett nach dist (B-827)

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-14, Chat, durch 🧑

## Ausgangslage

Stand vor dem Sprint, kurz. Details stehen in den Tickets.

## Ziel

`task native:game:build` legt das komplette Spiel nach `dist/k3c-game` und kopiert geänderte Grafik neu.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

- **Tickets:** B-827

## Nicht-Ziele

Was ausdrücklich nicht dazugehört (verhindert Ausufern), mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `native/Taskfile.yml`, `native/k3c-game/cmd/sync/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Sperre:** taskfile

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `task native:game:build` legt das komplette Spiel nach `dist/k3c-game` und kopiert geänderte Grafik neu (`B-827`).
- **AC-02** Review ohne offenen schweren Befund.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`. Blockierende Fragen verhindern die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| NG20.1 | `NG20.1-build-nach-dist.md` | Umsetzung | autonom | fertig |
| NG20.2 | `NG20.2-review.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
