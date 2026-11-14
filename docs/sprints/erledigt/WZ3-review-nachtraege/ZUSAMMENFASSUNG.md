# WZ3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Review-Nachträge zu WZ2: Filter-Zählung, Splitter-Test, Kleinigkeiten
- **Projekt:** WZG

## Ziel

Die offenen Review-Punkte zu WZ2 sind umgesetzt. Vorher ging nur: Sprints ausgefilterter Projekte zählten in der Projekt-Ansicht mit, der Splitter war nicht getestet, und Server/Tools hatten Lücken. **Am Ende sichtbar:** In der Projekt-Ansicht fallen Sprints ausgefilterter Projekte aus der Zählung „Sprints x von y“. `go test` und `golangci-lint` laufen grün, der Splitter-Test prüft Ziehen und Dopp …

## Ergebnis

- 2026-10-11, Review WZ3.2: AC-01 bis AC-06 geprüft (Nachweise in `WZ3.1-umsetzung.md` › Ergebnis und `WZ3.2-review.md` › Ergebnis). - Umgesetzt: M1 (Zählung), N1 (Hinweise Worktrees und Refs), N4 (Fahrplan-Platz), N5 (Job-Ausgabe), N6 (Kanal statt Sleep), AC-08 (Splitter-Logik testbar). - `task check:dev` nur teilweise belegt, von 🧑 akzeptiert; Splitter-Doppelklick ohne DOM-Test (B-406). - Neue Tickets: B-404 (Fahrplan-Projekt `–`), B-406 (Splitter-DOM-Test). B-405 verworfen. - Abnahme durch 🧑: 2026-10-11 abgenommen.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Sprints ausgefilterter Projekte fallen aus der Zählung „Sprints x von y“ (B-403/M1). |
| AC-02 | geprüft | `ticketnr.go` meldet im Fehlerfall fehlende Worktrees und Refs in der Antwort, statt still … |
| AC-03 | geprüft | Der Fahrplan springt bei leerem Feld `Projekt` nicht mehr ans Tabellenende, mit Test (B-40 … |
| AC-04 | geprüft | `check_job.go` hält nur die Fehlerzeilen je Job (B-403/N5). |
| AC-05 | geprüft | `check_background_test.go` wartet über einen Kanal statt über Zeit (B-403/N6). |
| AC-06 | geprüft | `Splitter.test.tsx` prüft Ziehen und Doppelklick; der Test „fehlt in der Projekt-Ansicht“ … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-403 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 geprüft | Review-Nachträge zu Sprint WZ2: Filter-Zählung, Schnellfilter, Splitter-Test und … |
