# WI1 · SIM · Handwerker mit Hammer bauen, Bogner-Shop, Gold beim Holzfällen, Truhen ×10 mit Fontäne

- **Status:** geplant
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

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `pfad/**` (Vereinigung der Erlaubten Dateien der Sessions ohne Sammeldateien; ein Sprint = eine Dateimenge)
- **Abhängig von:** – (nur IDs: SP00, B-000, SP00.1; erst starten, wenn erledigt und gemergt)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; Abnahmen am Gerät sind keine Abhängigkeit)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

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

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
