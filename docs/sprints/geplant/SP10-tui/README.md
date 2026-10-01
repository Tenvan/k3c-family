# SP10 · SRV · Diagnose-TUI

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-002
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 2
- **Freigabe:** –

## Ausgangslage

Ab SP07 zeigt `/api/status` Räume und Tick-Dauer; Sprint D1 ergänzt Speicher, Geräte, Log und die Aktionen (B-088). Eine Oberfläche dafür gibt es nicht.

## Ziel

`cmd/k3c-tui` (Bubble Tea) zeigt den laufenden Server. Am Ende sichtbar: Räume, Geräte, Tick-Dauer und Log live, auch gegen den Docker-Container.

## Beteiligte und Zielgruppen

🧑 als Betreiber am PC und später am Pi.

## Anforderungen

B-002 › Anforderungen.

## Nicht-Ziele

Wails-Starter (B-041).

## Regeln und Einschränkungen

Nur die Diagnose-Endpunkte `/api/status…` mit Token (B-027, B-088); Bubble Tea (Zustimmung mit der Freigabe, B-002); Voraussetzung: Sprint D1 erledigt.

## Beispiele

Server im Docker mit 2 Räumen → `docker exec -it k3c k3c-tui` zeigt beide live.

## Ausnahme- und Fehlerfälle

Server weg oder Token falsch → Meldung und erneuter Versuch.

## Akzeptanzkriterien

- **AC-01** Die Anzeige läuft live, lokal und im Docker (B-002/AC-01, B-002/AC-02).
- **AC-02** Die Aktionen wirken (B-002/AC-03).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP10.1 Anzeige live (nur über `/api/status` mit Token) (AC-01).
- SP10.2 Aktionen: Raum ansehen, Gerät trennen, Spielstand sichern, Log folgen (AC-02).
- SP10.3 🔍 Review (alle).

## Abnahme

–
