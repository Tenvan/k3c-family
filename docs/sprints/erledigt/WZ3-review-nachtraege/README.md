# WZ3 · DEV · Review-Nachträge zu WZ2: Filter-Zählung, Splitter-Test, Kleinigkeiten

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-403
- **Start-Commit:** 008803c0
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-11, Nutzer im Chat (Revision 1)

## Ausgangslage

Sprint WZ2 (Planungs-Werkzeuge) ist abgeschlossen. Die unabhängige Review WZ2.8 hat die sofort behobenen Befunde erledigt und die Nachträge offen gelassen. Details stehen in B-403.

## Ziel

Die offenen Review-Punkte zu WZ2 sind umgesetzt. Vorher ging nur: Sprints ausgefilterter Projekte zählten in der Projekt-Ansicht mit, der Splitter war nicht getestet, und Server/Tools hatten Lücken.

**Am Ende sichtbar:** In der Projekt-Ansicht fallen Sprints ausgefilterter Projekte aus der Zählung „Sprints x von y“. `go test` und `golangci-lint` laufen grün, der Splitter-Test prüft Ziehen und Doppelklick.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Verweise auf die Ticket-Specs (`B-009 › Anforderungen`) statt Kopien, dazu nur sprint-eigene Anforderungen.

## Nicht-Ziele

- Senkrechter Splitter zwischen den Spalten (Entscheidung: Höhe reicht, Spec wird auf „Höhe“ korrigiert).
- „Hardware-Bahn“-Doku in `B-186` und `PF1.4` (§ 11.6) bleibt außen vor; eigenes Ticket, falls nötig.

## Regeln und Einschränkungen

- Domäne DEV (k3c-dev und Planung). Keine Änderung am Spielcode.
- M1-Entscheidung: Sprints ausgefilterter Projekte verschwinden und zählen nicht mit (🧑 am 2026-10-11).
- Splitter: Höhe des Detailbereichs reicht (🧑 am 2026-10-11).
- Komplexitätsbudget: Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** Sprints ausgefilterter Projekte fallen aus der Zählung „Sprints x von y“ (B-403/M1).
- **AC-02** `ticketnr.go` meldet im Fehlerfall fehlende Worktrees und Refs in der Antwort, statt still zu fehlen (B-403/N1).
- **AC-03** Der Fahrplan springt bei leerem Feld `Projekt` nicht mehr ans Tabellenende, mit Test (B-403/N4).
- **AC-04** `check_job.go` hält nur die Fehlerzeilen je Job (B-403/N5).
- **AC-05** `check_background_test.go` wartet über einen Kanal statt über Zeit (B-403/N6).
- **AC-06** `Splitter.test.tsx` prüft Ziehen und Doppelklick; der Test „fehlt in der Projekt-Ansicht“ prüft nicht mehr tautologisch (B-403/AC-08).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ3.1 | `WZ3.1-umsetzung.md` | Umsetzung | autonom | fertig |
| WZ3.2 | `WZ3.2-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-10-11, Review WZ3.2: AC-01 bis AC-06 geprüft (Nachweise in `WZ3.1-umsetzung.md` › Ergebnis und `WZ3.2-review.md` › Ergebnis).
- Umgesetzt: M1 (Zählung), N1 (Hinweise Worktrees und Refs), N4 (Fahrplan-Platz), N5 (Job-Ausgabe), N6 (Kanal statt Sleep), AC-08 (Splitter-Logik testbar).
- `task check:dev` nur teilweise belegt, von 🧑 akzeptiert; Splitter-Doppelklick ohne DOM-Test (B-406).
- Neue Tickets: B-404 (Fahrplan-Projekt `–`), B-406 (Splitter-DOM-Test). B-405 verworfen.
- Abnahme durch 🧑: 2026-10-11 abgenommen.
