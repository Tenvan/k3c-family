# BD8 · CLI · Lobby bietet beim Anlegen eines Spielstands eine Welt-Auswahl

- **Status:** erledigt
- **Projekt:** BED
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-566
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-23, Revision 1

## Ausgangslage

Die Lobby legt Spielstände ohne Welt-Auswahl an (B-566).

## Ziel

Beim Anlegen eines Spielstands wählt man die Welt. Am Ende sichtbar: Lobby mit Auswahl, per Gamepad bedienbar.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

B-566 › Anforderungen.

## Nicht-Ziele

Neue Welten.

## Regeln und Einschränkungen

Entscheidungen, Domänen-Grenzen, Budget, Verträge, die für den ganzen Sprint gelten.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Die Lobby bietet eine Welt-Auswahl beim Anlegen, mit Gamepad und Tastatur bedienbar (B-566/AC).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD8.1 | `BD8.1-welt-auswahl.md` | Umsetzung | autonom | fertig |

Stichpunkte (Reife Entwurf): BD8.1 Auswahl (AC-01); BD8.2 Review.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
