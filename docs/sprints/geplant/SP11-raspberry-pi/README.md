# SP11 · SRV · Raspberry Pi

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-028, B-035, B-042
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Server läuft nur auf dem Windows-PC; Docker-Image und Sicherungen gibt es ab SP03.

## Ziel

Der Server läuft dauerhaft auf dem Raspberry Pi. Am Ende sichtbar: 2er- und 3er-Spiel parallel auf dem Pi, Lastmessung gegen das Ziel aus B-042.

## Beteiligte und Zielgruppen

🧑 richtet den Pi ein und betreibt ihn; die Familie spielt.

## Anforderungen

B-028 (Pi-Teil), B-035 und B-042 › Anforderungen.

## Nicht-Ziele

Server-Suche per QR/mDNS (B-040).

## Regeln und Einschränkungen

Die Einrichtung am Pi macht 🧑 (Session mit `Agent: Mensch`).

## Beispiele

2er- und 3er-Spiel parallel auf dem Pi → Tick-Dauer p99 im Ziel aus B-042.

## Ausnahme- und Fehlerfälle

Ziel verfehlt → Messwerte und Befund als Ticket, keine stille Absenkung des Ziels.

## Akzeptanzkriterien

- **AC-01** Nach einem Neustart des Pi ist der Server erreichbar (B-035/AC-01).
- **AC-02** Ein Update per `docker compose pull` bringt die neue Version (B-035/AC-02).
- **AC-03** Die Spielstände liegen im Volume und werden gesichert (B-028/AC-03).
- **AC-04** Die Lastmessung mit 2er- und 3er-Spiel parallel ist gegen das Ziel aus B-042 bewertet (B-042/AC-01).

## Offene Fragen

Pi-Modell und Leistungsziel (B-042, 🧑) – blockiert die Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP11.1 🧑 Pi einrichten, Docker-Compose, Autostart, Sicherungen der Spielstände (AC-01, AC-02, AC-03).
- SP11.2 Lastmessung auf dem Pi, Ergebnis gegen B-042 (AC-04).
- SP11.3 🔍 Review (alle).

## Abnahme

–
