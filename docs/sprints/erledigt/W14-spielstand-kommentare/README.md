# W14 · SIM · Spielstand-Kommentare nach W7.2 nachziehen

- **Status:** erledigt
- **Projekt:** WRT
- **Domäne:** SIM
- **Reife:** bereit
- **Tickets:** B-418
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Kommentare in `island_save_v3.go` und `island_save_v5.go` nennen nach W7.2 veraltete Versionen (B-418).

## Ziel

Die Kommentare nennen die gültigen Spielstand-Versionen. Am Ende sichtbar: Diff nur in Kommentaren, Tests grün.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-418 › Anforderungen.

## Nicht-Ziele

Änderungen am Spielstand-Format.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Die Kommentare in `island_save_v3.go` und `island_save_v5.go` stimmen mit dem Code überein (B-418/AC).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W14.1 | `W14.1-kommentare-pruefen.md` | Umsetzung | autonom | fertig |

Stichpunkte (Reife Entwurf): W14.1 Kommentare (AC-01), ein Commit ohne Review (nur Kommentare).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
