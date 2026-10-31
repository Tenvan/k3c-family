# WZ38 · DEV · Umgesetzte Werkzeug-Tickets abnehmen: Welten-Editor, Studio-Seiten, Git-Seite, check:diff

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-663, B-664, B-665, B-666, B-667, B-670, B-671, B-672, B-580, B-619
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 2: B-673 entfernt)

## Ausgangslage

Zehn Tickets sind umgesetzt, stehen aber ohne Sprint auf `offen`/`eingeplant`, weil 🧑 die Beobachtung noch nicht gemacht hat: Welten-Editor (B-663 bis B-667), Studio-Seiten (B-670 bis B-672), Git-Seite (B-580) und Tempo von `check:diff` (B-619, Norton-Ausnahme und Workbench-Neustart offen). B-673 (Studio-Quellen) ist bereits erledigt und nicht Teil dieses Sprints (Revision 2).

## Ziel

🧑 nimmt die umgesetzten Werkzeug-Tickets in einem Durchgang ab, danach sind sie `erledigt`. Am Ende sichtbar: Welten-Editor, Studio-Seiten und Git-Seite am PC.

## Beteiligte und Zielgruppen

🧑 nimmt ab; Agent DEV prüft vorher je Ticket Kriterien gegen Code und Tests.

## Anforderungen

Kriterien der Tickets B-663 bis B-667, B-670 bis B-672, B-580, B-619.

## Nicht-Ziele

Neue Funktionen; Umbauten.

## Regeln und Einschränkungen

Spec „rückwirkend“, Freigabe durch 🧑; Abnahmen nur durch 🧑; Tests auf Port 5196.

## Beispiele

Welten-Editor mit und ohne Chat bei 800 px → keine waagrechte Scrollleiste.

## Ausnahme- und Fehlerfälle

Abnahme findet einen Fehler → Ticket bleibt offen, neues Ticket.

## Akzeptanzkriterien

- **AC-01** Welten-Editor erfüllt `B-663/AC-01` bis `B-667/AC-05`.
- **AC-02** Studio-Seiten Planer, Monarch, Gegner erfüllen `B-670` bis `B-672`.
- **AC-03** Git-Seite erfüllt `B-580/AC-01` bis `AC-05`.
- **AC-04** `check:diff` ist wie in `B-619` beschrieben schnell (Norton-Ausnahme gesetzt, Workbench neu gestartet).
- **AC-05** 🧑 hat AC-01 bis AC-04 abgenommen.

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ38.1 | `WZ38.1-kriterien-belegen.md` | Umsetzung | autonom | offen |
| WZ38.2 | `WZ38.2-abnahme-am-pc.md` | Workshop | Mensch | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ38.1 (DEV) Kriterien je Ticket gegen Code und Tests belegen, Prüfliste für 🧑 (AC-01 bis AC-04).
- WZ38.2 (🧑) Abnahme am PC (AC-05).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
