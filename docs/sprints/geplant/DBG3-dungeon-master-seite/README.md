# DBG3 · PLAT · Dungeon-Master-Seite /dm

- **Status:** geplant
- **Domäne:** PLAT
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-232
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Cheat-Dialog (B-231, PR #119) bietet im Spiel Gold, Material, Zeitraffer und Pause. Dev-Aktionen laufen nur über den WebSocket eines Spiel-Geräts mit Monarch; HTTP kennt nur `/api/status*`, `/api/save*`, `/api/level`, `/api/report`. Eine Seite ohne Monarch kann heute keine Dev-Aktion auslösen. Details: B-232.

## Ziel

Der Spielleiter steuert und beobachtet laufende Räume vom Handy oder Tablet, ohne das Spielbild zu stören. Am Ende sichtbar: `http://<server>/dm` am Handy, Raum wählen, „Zeit 4×“ tippen, der Raum am TV läuft schneller.

## Beteiligte und Zielgruppen

🧑 als Spielleiter am Spieleabend und Tester; 🧑 entscheidet die Offenen Fragen und nimmt am Handy ab.

## Anforderungen

`B-232 › Anforderungen`, erste Fassung ohne Passwortschutz.

## Nicht-Ziele

Spielen auf der Seite; Passwortschutz (eigenes Ticket, sobald die Seite steht); Grad wechseln (B-107) und Raum-Neustart (B-080), solange 🧑 sie nicht in die erste Fassung holt.

## Regeln und Einschränkungen

- Dev-Aktionen nur im Dev-Mode des Servers.
- Domäne PLAT (Seite `dm.html`); der Server-Zugang (HTTP-API in `engine/net` oder Beobachter im WebSocket-Protokoll) liegt in SRV bzw. ist eine Protokoll-Änderung und bekommt eine eigene Session, die nur Protokoll und beide Enden anpasst (`docs/arbeitsweise.md` › Domänen). Passt das nicht in einen Sprint, wird der Server-Teil ein eigener SRV-Sprint vor DBG3.
- Aufruf über `/dm`, nicht über die Landingpage-Kacheln; Seiten-Regeln aus `CLAUDE.md` gelten, soweit die Seite in der Shell läuft.
- Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit.

## Beispiele

`B-232 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-232 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** `B-232/AC-01` – `/dm` liefert die Seite, auf 375 px Breite ohne waagerechtes Scrollen bedienbar.
- **AC-02** `B-232/AC-02` – Diagnosedaten eines gewählten Raums, live aktualisiert.
- **AC-03** `B-232/AC-03` – Gold, Material, Zeitraffer und Pause wirken über die Seite (Test).
- **AC-04** `B-232/AC-04` – 🧑 hat die Seite am Handy abgenommen.

## Offene Fragen

- **Zugang (blockierend):** HTTP-API `/api/dev/...` (SRV, Empfehlung: kein Protokoll-Bump, einfach vom Handy) oder WebSocket als Beobachter (Protokoll-Änderung)? Entscheidet, ob ein SRV-Sprint vorgeschaltet wird. (🧑)
- **Umfang (blockierend):** Nur heutige Dev-Aktionen oder zusätzlich Grad, Welle auslösen, Tageszeit, Gegner? (🧑)
- Passwortschutz: Zeitpunkt und Art (🧑, nicht blockierend).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| DBG3.1 | `DBG3.1-seite-dm.md` | Umsetzung | autonom | offen |
| DBG3.3 | `DBG3.3-review.md` | Review | autonom | offen |
| DBG3.2 | `DBG3.2-abnahme-handy.md` | Workshop | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
