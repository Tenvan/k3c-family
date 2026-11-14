# M7 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** k3c-dev VII: Seiten Tasks, Planung und Git
- **Projekt:** –

## Ziel

Tasks, Planung (mit Plan und Fragenkatalog) und Git sind in k3c-dev bedienbar, im Mock und im Fenster. Am Ende sichtbar: drei neue Reiter in `task k3c-dev`, im Browser-Pane mit `npx vite` im Frontend.

## Ergebnis

2026-10-03: AC-01 bis AC-04 geprüft in M7.1 (Tests und Mocks), AC-05 in M7.3 (`task check:dev` und `task check` grün), AC-06 durch 🧑 in M7.2 (echtes Fenster, bestanden). Review ohne schweren Sicherheitsbefund; behoben: `TasksPage` auf unter 60 Zeilen, `TestFindRoot` mit Temp-Ordner im Repo. Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Seite Tasks mit Katalog, Start, Stopp und Live-Ausgabe (B-171/AC-01). |
| AC-02 | geprüft | Seite Planung mit Sprints, Backlog, Plan und Fragenkatalog, die sich bei Dateiänderungen s … |
| AC-03 | geprüft | Seite Git mit Staging und Commit (B-171/AC-03). |
| AC-04 | geprüft | Standard-Theme Dark/Light und Mock für alle Seiten (B-171/AC-04, B-171/AC-05). |
| AC-05 | offen | `task check:dev` grün (B-171/AC-06). |
| AC-06 | geprüft | 🧑 hat die Seiten im echten Fenster abgenommen (B-171/AC-07). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-171 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 offen, AC-06 geprüft | k3c-dev zeigt Tasks, Planung und Git wie die Workbench der ErpApi |
