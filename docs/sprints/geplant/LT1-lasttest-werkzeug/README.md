# LT1 · SRV · Lasttest-Werkzeug

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-175
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Lastmessung am Pi (B-042) ist Handarbeit; die erste Messung am 2026-10-03 liegt knapp am Ziel (Nacht 10,2 bis 10,3 ms bei Ziel < 10 ms). SP11 schließt ohne diese Messung ab (AC-04 verschoben, siehe SP11 Revision 2). `/api/status` kennt keine CPU-Last.

## Ziel

Ein Befehl `task load` misst Tick-Dauer und CPU gegen das Ziel und bewertet es. Am Ende sichtbar: Ein Messlauf am Pi mit Tabelle und Bewertung („erreicht“, „knapp“, „verfehlt“).

## Beteiligte und Zielgruppen

🧑 startet den Messlauf am Pi; Agenten setzen das Werkzeug um.

## Anforderungen

B-175 › Anforderungen.

## Nicht-Ziele

Bandbreite (B-140, F4), Lasttest mit echten Geräten, Balancing (B-099).

## Regeln und Einschränkungen

Domäne SRV (`cmd/k3c-load`, `engine/net/status.go`, `Taskfile.yml` für den Task `load`, Tests). Der Task gehört zu INF-Dateien; Freigabe dieser Spec erlaubt die eine Zeile im `Taskfile.yml`.

## Beispiele

`task load -- -url http://pi-gaming:8080 -token … -rooms 2 -players 3 -duration night` → Bericht mit p99 je Raum und Phase.

## Ausnahme- und Fehlerfälle

Server nicht erreichbar → Exit-Code 2; Ziel verfehlt → Messwerte und Befund als Ticket (keine stille Absenkung des Ziels).

## Akzeptanzkriterien

- **AC-01** Das Werkzeug erzeugt gegen einen In-Prozess-Server einen Bericht mit Tick-Reihe je Raum (B-175/AC-01).
- **AC-02** Gleicher Seed ergibt dieselbe Eingabefolge (B-175/AC-02).
- **AC-03** Bewertung und Exit-Code folgen den drei Stufen (B-175/AC-03).
- **AC-04** `/api/status` liefert `cpu`, wo die Quelle existiert (B-175/AC-04).
- **AC-05** Keine Test-Räume bleiben offen, kein Token in der Ausgabe (B-175/AC-05).
- **AC-06** Der Messlauf am Pi ist bewertet; B-042 ist archiviert (B-175/AC-06).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| LT1.1 | `LT1.1-bots-client.md` | Umsetzung | autonom | offen |
| LT1.2 | `LT1.2-status-bericht.md` | Umsetzung | autonom | offen |
| LT1.3 | `LT1.3-messlauf-pi.md` | Workshop | Mensch | offen |
| LT1.4 | `LT1.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
