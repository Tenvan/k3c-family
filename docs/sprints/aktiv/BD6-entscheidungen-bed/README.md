# BD6 · REG · Autonome Entscheidungen zu BED prüfen und festschreiben

- **Status:** aktiv
- **Projekt:** BED
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-525, B-526, B-527, B-528, B-529, B-533, B-534, B-535, B-536, B-537
- **Start-Commit:** df21c2c2
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

BD2 bis BD5 wurden auf Ralfs Anweisung „autonom abarbeiten“ umgesetzt (PRs 431, 432, 436, 440). Dabei hat Claude offene Fragen mit Defaults entschieden und als Tickets vom Typ Frage abgelegt. Diese Entscheidungen sind nicht von Ralf bestätigt.

## Ziel

Ralf prüft jede autonome Entscheidung (bestätigen oder ändern); bestätigte Defaults werden festgeschrieben, geänderte als Folge-Tickets umgesetzt. Am Ende sichtbar: alle genannten Frage-Tickets sind erledigt oder verworfen, abweichende Folgearbeit steht als Ticket im Backlog.

## Beteiligte und Zielgruppen

Entscheidet: 🧑 Ralf. Vorbereitet: Claude. Betroffen: Spielende (Pause, HUD, Spielmenü), Betrieb (Spielstand-Löschen).

## Anforderungen

Entscheidungs-Tickets (Default und Alternative stehen je im Ticket unter Notizen): B-525 Goldtopf, B-526 fester WORLD_TOP, B-527 Abmelden mit zwei Tastatur-Spielern, B-528 Regeldetails der Raum-Pause, B-529 Protokollform pause/resume, B-533 Löschen ohne Token mit Sicherung, B-534 Spielmenü gleich Lobby, B-535 Pause nach Wiederverbinden erneut senden (BD7), B-536 Versionen v0.21.0 (BD3) und v0.21.1 (BD7), B-537 vierter Punkt der Meldung aus WZ18.8 (BD5). B-524 und B-530 sind in BD7 umgesetzt.

Die Tickets sind dem KRT-Sprint KR1 zugeordnet (Sammelstelle aller kritischen Fragen zu BED); BD6.1 und KR1.1 sind dieselbe Durchsicht.

## Nicht-Ziele

Neue Funktionen; die Client-Anbindung von pause/resume (eigener CLI-Sprint); Abnahmen am Gerät (BD4.4, BD5.4 bleiben in ihren Sprints).

## Regeln und Einschränkungen

Nur Ralf bestätigt Entscheidungen. Bis dahin gelten die Defaults. Keine Rücksicht auf alte Spielstände.

## Beispiele

B-533 bestätigt → Ticket erledigt, nichts weiter. B-533 geändert (z. B. Löschen nur mit Token) → neues Umsetzungs-Ticket mit dem geänderten Verhalten.

## Ausnahme- und Fehlerfälle

Eine Entscheidung wird abgelehnt und berührt bereits gemergten Code → Folge-Ticket mit Umbau, kein stilles Zurücknehmen.

## Akzeptanzkriterien

- **AC-01** Ralf hat jede Entscheidung B-525 bis B-529 und B-533 bis B-537 bestätigt oder geändert, vermerkt im jeweiligen Ticket unter Notizen.
- **AC-02** Zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket im Projekt BED; bestätigte Frage-Tickets sind erledigt.
- **AC-03** Die Versionsnummern der Sprints BD3 und BD7 sind gesetzt oder verworfen (B-536).

## Offene Fragen

Keine blockierende. Alle Entscheidungen sind mit Default umgesetzt (autonom, Claude, 2026-10-22) und gelten, bis 🧑 Ralf sie in BD6.1 bestätigt oder ändert.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD6.1 | `BD6.1-entscheidungen.md` | Workshop | Mensch | offen |

BD6.1: Ralf geht die Frage-Tickets in KR1 durch (AC-01, AC-02, AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
