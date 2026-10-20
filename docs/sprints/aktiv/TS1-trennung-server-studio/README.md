# TS1 · DEV, SRV, INF · Spielserver und Asset-Studio trennen, Server beobachtet Dateien

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV, SRV, INF
- **Reife:** bereit
- **Tickets:** B-507
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-20, Ralf im Chat („anlegen und umsetzen“, Revision 1)

## Ausgangslage

`B-507 › Ausgangslage`: Der Spielserver bindet das Studio-Backend ein und liefert die Studio-Seite aus; Pages baut den vollen Client. Der Chat-Code von WZ19.1 liegt in `engine/studio` (Branch `sprint/wz19`, hier als Basis). Der Sprint setzt auf `sprint/wz19` auf und muss vor WZ19.2 erledigt sein.

## Ziel

Server und Studio sind getrennt, der Server beobachtet die Dateien des Studios, Pages baut nur `site/`.
**Am Ende sichtbar:** `go list`-Test zeigt keine Importe von `tools/` in `engine/` und `cmd/k3c-server`; `/studio/` antwortet 404; eine im Studio gespeicherte Welt erscheint im laufenden Server; Studio-Vorschau läuft ohne Server; von 🧑 am PC abgenommen.

## Beteiligte und Zielgruppen

`B-507 › Beteiligte und Zielgruppen`: 🧑 nimmt am PC ab; Agenten je Session; ein getrennter Reviewer.

## Anforderungen

`B-507 › Anforderungen` (Trennung, Datei-Watch, GitHub Pages, Dokumentation). Reihenfolge: TS1.1 Umzug, TS1.2 Engine im Studio, TS1.3 Server-Seite streichen, TS1.4 Watch, TS1.5 Pages und Doku, TS1.6 Review, TS1.7 Abnahme.

## Nicht-Ziele

`B-507 › Nicht-Ziele`. Zusätzlich: WZ19.2 bis WZ19.5 (Chat-Oberfläche, Prompt, Abnahme) laufen erst danach; WZ19.2 wird in TS1.5 nur im Text angepasst.

## Regeln und Einschränkungen

`B-507 › Regeln und Einschränkungen`. Jede Session ändert nur ihre Domäne und lässt `task check` (bzw. die genannten Prüfungen) grün. Der Chat-Code (WZ19.1) bleibt funktional unverändert und zieht nur um.

## Beispiele

`B-507 › Beispiele`.

## Ausnahme- und Fehlerfälle

`B-507 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** Studio-Backend in `tools/k3c-studio/`, Import-Regel als Test (`B-507/AC-01`) – TS1.1
- **AC-02** Studio-Vorschau und Spielstart ohne Spielserver (`B-507/AC-02`) – TS1.2
- **AC-03** Keine Studio-Seite im Server (`B-507/AC-03`) – TS1.3
- **AC-04** Datei-Watch mit Neuladen oder verzögertem Neustart und Hinweis (`B-507/AC-04`) – TS1.4
- **AC-05** Pages nur `site/` (`B-507/AC-05`) – TS1.5
- **AC-06** Entscheidung, Doku, WZ19.2-Text (`B-507/AC-06`) – TS1.5
- **AC-07** Abnahme durch 🧑 am PC (`B-507/AC-07`) – TS1.7

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| TS1.1 | `TS1.1-studio-backend-umziehen.md` | Umsetzung | autonom | fertig |
| TS1.2 | `TS1.2-studio-engine-testspielstaende.md` | Umsetzung | autonom | fertig |
| TS1.3 | `TS1.3-studio-seite-streichen.md` | Umsetzung | autonom | offen |
| TS1.4 | `TS1.4-datei-watch-server.md` | Umsetzung | autonom | offen |
| TS1.5 | `TS1.5-pages-und-doku.md` | Umsetzung | autonom | offen |
| TS1.6 | `TS1.6-review.md` | Review | autonom | offen |
| TS1.7 | `TS1.7-abnahme.md` | Review | Mensch | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
