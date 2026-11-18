# RU43 · CLI · Optimierung nach Messung

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** Entwurf
- **Tickets:** B-1076
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

RU42 führt die Marker `// PERF(B-xxxx)` und `// STAB(B-xxxx)` ein; Tickets mit `PERF` oder `STAB` landen in diesem Sprint.

## Ziel

Die PERF- und STAB-Markierungen sind nach Messung abgearbeitet (Allokationen je Bild, Draw-Calls, Panics).

## Beteiligte und Zielgruppen

Agenten setzen um; Messung und Feinoptimierung am Ende (Ralf 2026-11-18).

## Anforderungen

Entwurf: Sessions werden vor dem Aktivieren aus den eingeplanten PERF- und STAB-Tickets abgeleitet.

## Nicht-Ziele

Optimierung ohne Messung.

## Regeln und Einschränkungen

Erst messen (`k3c-alloc-probe`, `messung_cpu`), dann ändern.

- **Bereich:** `rust/crates/**`
- **Abhängig von:** RU42
- **Braucht 🧑:** –
- **Sperre:** –

## Beispiele

nicht relevant: Entwurf.

## Ausnahme- und Fehlerfälle

nicht relevant: Entwurf.

## Akzeptanzkriterien

- **AC-01** Jede `PERF(B-xxxx)`- und `STAB(B-xxxx)`-Markierung ist entfernt oder durch eine Messung als unkritisch begründet.

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
