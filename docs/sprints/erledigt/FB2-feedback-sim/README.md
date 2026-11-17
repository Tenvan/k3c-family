# FB2 · SIM · Live-Test-Feedback: Gegnerwellen, Stressmodus, Geldsack-Skill

- **Status:** erledigt
- **Projekt:** NAT
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Live-Test 🧑 2026-11-15: Der Stresstest bringt keine Gegnerwellen, ein reiner Stressmodus fehlt, der Geldsack soll per Skill wachsen.

## Ziel

Stresstest mit Wellen, Stressmodus an der Burg und Geldsack-Größen in der Engine.

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

- **AC-01** Der Stresstest erzeugt Gegnerwellen (`B-920/AC-01`).
- **AC-02** Reiner Stressmodus: Gegner an der Burg, nur eigene Bevölkerung, deterministisch (`B-921/AC-01` bis `AC-03`).
- **AC-03** Geldsack in drei Größen als Skill-Stufen mit Münz-Obergrenze je Spieler (`B-922/AC-01`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| FB2.1 | `FB2.1-wellen-stressmodus-geldsack.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
