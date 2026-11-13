# WZ40 · INF · check:diff prüft nur den eigenen Diff, betroffene Pakete und parallel

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** INF
- **Reife:** bereit
- **Tickets:** B-730
- **Start-Commit:** 9d8b2353
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (Ralf: "Autonom freigeben", 2026-11-07)

## Ausgangslage

B-730: `check:diff` ist mit PR #509 bereits im Taskfile und in `docs/arbeitsweise.md` › Tempo im Lauf umgesetzt, aber nie gegen die Kriterien belegt; das Ticket steht auf `offen`.

## Ziel

B-730 ist belegt und geschlossen: Doku-Commits prüfen in Sekunden, Go-Änderungen nur betroffene Pakete.

Am Ende sichtbar: `task check:diff` bei reinem Doku-Diff endet nach rund 8 s mit `· nur Doku`.

## Beteiligte und Zielgruppen

🧑 Ralf entscheidet und nimmt ab; Claude belegt.

## Anforderungen

`B-730 › Anforderungen`.

## Nicht-Ziele

Neue Prüfstufen, Review-/CI-Umfang ändern, flackernde Tests (B-704).

## Regeln und Einschränkungen

Nur `Taskfile.yml` (check:diff), `docs/arbeitsweise.md`, Planungsdateien.

## Beispiele

Siehe `B-730 › Beispiele`.

## Ausnahme- und Fehlerfälle

Keine Basis ermittelbar → Abbruch mit Meldung (kein stilles Grün).

## Akzeptanzkriterien

- **AC-01** Doku-Diff: `check:diff` unter 60 s, nur Planungs- und Regel-Tests (`B-730/AC-01`).
- **AC-02** Go-Änderung: nur Paket und Abnehmer, Ausgabe `🚦 check:diff` nennt sie (`B-730/AC-02`).
- **AC-03** `docs/arbeitsweise.md` › Tempo im Lauf beschreibt den Takt (`B-730/AC-03`).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ40.1 | `WZ40.1-belegen-abnahme.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
