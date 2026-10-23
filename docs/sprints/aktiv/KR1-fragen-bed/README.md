# KR1 · REG · Kritische Fragen zu BED

- **Status:** aktiv
- **Projekt:** KRT
- **Domäne:** REG
- **Reife:** bereit
- **Tickets:** B-525, B-526, B-527, B-528, B-529, B-533, B-534, B-535, B-536, B-537
- **Start-Commit:** df21c2c2
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude

## Ausgangslage

Das Projekt BED (Bedienung & HUD) wurde in BD2 bis BD7 autonom umgesetzt. Alles, was Ralf entscheiden muss, liegt als Frage-Ticket mit umgesetztem Default vor (B-525 bis B-529, B-533 bis B-537). Offen sind außerdem die Geräte-Abnahmen BD4.4 (HUD-Leiste, Fluss, Austreten) und BD5.4 (Spielmenü am PC); sie bleiben in ihren Sprints.

## Ziel

Ralf hat alle kritischen Fragen zu BED an einer Stelle entschieden. Am Ende sichtbar: kein offenes Frage-Ticket im Projekt BED, Abweichungen als Umsetzungs-Ticket mit Projekt BED.

## Beteiligte und Zielgruppen

Entscheidet: 🧑 Ralf. Vorbereitet: Claude. Betroffen: Spielende (HUD, Pause, Spielmenü), Betrieb (Spielstand löschen, Versionen).

## Anforderungen

Je Ticket `B-5NN › Notizen` (Frage, Default, Alternative, Folgen):

| Ticket | Thema | Herkunft | Default gilt |
|---|---|---|---|
| B-525 | Goldtopf: Wortlaut und Symbol | BD4 | ja |
| B-526 | Hintergrund-Höhe gegen WORLD_TOP | BD4 | ja |
| B-527 | Abmelden mit zwei Tastatur-Spielern | BD4 | ja |
| B-528 | Regeldetails der Raum-Pause | BD3 | ja |
| B-529 | Protokoll v8 pause/resume | BD3 | ja |
| B-533 | Spielstand löschen ohne Token, mit Sicherung | BD5 | ja |
| B-534 | Spielmenü = Lobby, keine Welt-Auswahl | BD5 | ja |
| B-535 | Pause nach Wiederverbinden erneut senden | BD7 | ja |
| B-536 | Versionen v0.21.0 / v0.21.1 | BD3, BD7 | kein Tag |
| B-537 | vierter Punkt der Meldung WZ18.8 | BD5 | „keiner“ |

## Nicht-Ziele

Umsetzung geänderter Entscheidungen (eigene Tickets im Projekt BED); Geräte-Abnahmen BD4.4 und BD5.4; neue Funktionen.

## Regeln und Einschränkungen

Nur Ralf bestätigt. Bis dahin gelten die Defaults. Geänderte Entscheidungen auf gemergtem Code → Folge-Ticket, kein stilles Zurücknehmen. Keine Rücksicht auf alte Spielstände. BD6.1 ist dieselbe Durchsicht aus Sicht des Projekts BED.

## Beispiele

B-533 bestätigt → Ticket erledigt. B-533 geändert (Löschen nur mit Token) → neues SRV-Ticket im Projekt BED, B-533 erledigt mit Verweis.

## Ausnahme- und Fehlerfälle

Ralf kann eine Frage noch nicht beantworten (z. B. erst nach BD4.4) → Ticket bleibt eingeplant, Default gilt weiter, KR1.1 bleibt offen.

## Akzeptanzkriterien

- **AC-01** Jedes der Tickets B-525 bis B-529 und B-533 bis B-537 trägt unter Notizen Ralfs Entscheidung (bestätigt oder Alternative).
- **AC-02** Bestätigte Tickets sind erledigt; zu jeder geänderten Entscheidung existiert ein Umsetzungs-Ticket mit Projekt BED.

## Offene Fragen

keine (die Fragen sind der Inhalt der Tickets und blockieren die Freigabe nicht, weil Defaults gelten).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| KR1.1 | `KR1.1-fragen-bed.md` | Workshop | Mensch | fertig |

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
