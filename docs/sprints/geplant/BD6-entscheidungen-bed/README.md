# BD6 · REG · Autonome Entscheidungen zu BED prüfen und festschreiben

- **Status:** geplant
- **Projekt:** BED
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BD2 bis BD5 wurden auf Ralfs Anweisung „autonom abarbeiten“ umgesetzt (PRs 431, 432, 436, 440). Dabei hat Claude offene Fragen mit Defaults entschieden und als Tickets vom Typ Frage abgelegt. Diese Entscheidungen sind nicht von Ralf bestätigt.

## Ziel

Ralf prüft jede autonome Entscheidung (bestätigen oder ändern); bestätigte Defaults werden festgeschrieben, geänderte als Folge-Tickets umgesetzt. Am Ende sichtbar: alle genannten Frage-Tickets sind erledigt oder verworfen, abweichende Folgearbeit steht als Ticket im Backlog.

## Beteiligte und Zielgruppen

Entscheidet: 🧑 Ralf. Vorbereitet: Claude. Betroffen: Spielende (Pause, HUD, Spielmenü), Betrieb (Spielstand-Löschen).

## Anforderungen

Entscheidungs-Tickets (Default und Alternative stehen je im Ticket): B-525 Goldtopf, B-526 fester WORLD_TOP, B-527 Abmelden mit zwei Tastatur-Spielern, B-528 Regeldetails der Raum-Pause, B-529 Protokollform pause/resume, B-533 Löschen ohne Token mit Sicherung, B-534 Spielmenü gleich Lobby, B-535 Pause nach Wiederverbinden erneut senden (BD7). B-524 und B-530 sind in BD7 umgesetzt. Zusätzlich: Versionen v0.21.0 (BD3) und v0.21.1 (BD7, vorgeschlagen) bestätigen.

## Nicht-Ziele

Neue Funktionen; die Client-Anbindung von pause/resume (eigener CLI-Sprint); Abnahmen am Gerät (BD4.4, BD5.4 bleiben in ihren Sprints).

## Regeln und Einschränkungen

Nur Ralf bestätigt Entscheidungen. Bis dahin gelten die Defaults. Keine Rücksicht auf alte Spielstände.

## Beispiele

B-533 bestätigt → Ticket erledigt, nichts weiter. B-533 geändert (z. B. Löschen nur mit Token) → neues Umsetzungs-Ticket mit dem geänderten Verhalten.

## Ausnahme- und Fehlerfälle

Eine Entscheidung wird abgelehnt und berührt bereits gemergten Code → Folge-Ticket mit Umbau, kein stilles Zurücknehmen.

## Akzeptanzkriterien

- **AC-01** Ralf hat jede Entscheidung B-525 bis B-529, B-533, B-534 und B-535 bestätigt oder geändert, vermerkt im jeweiligen Ticket.
- **AC-02** Zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket; bestätigte Frage-Tickets sind erledigt.
- **AC-03** Die Versionsnummern der Sprints BD3 und BD7 sind gesetzt oder verworfen.

## Offene Fragen

Alle Entscheidungen sind die offenen Fragen dieses Sprints; sie entscheidet 🧑 Ralf.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BD6.1 | – (Entwurf) | Entscheidung (🧑) | Mensch | offen |

BD6.1: Ralf geht die Frage-Tickets durch (AC-01, AC-02, AC-03). Die Session-Datei entsteht beim Aktivieren.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
