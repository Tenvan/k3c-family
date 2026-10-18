# tools/k3c-dev/internal/browser/

## Responsibility

Startet den installierten Browser (Edge oder Chrome) headless für Testläufe mit Clients (B-348, TR1.3). Gesucht wird nur an einer festen Liste bekannter Orte, es gibt keine zusätzliche Abhängigkeit.

## Design

- `candidates(goos, getenv)`: bekannte Orte je Plattform, `${…}` über `getenv` aufgelöst.
- `Find`/`find`: erster vorhandener Ort, sonst `ErrNotFound`; `exists`/`getenv` als Nähte für Tests.
- `Args(exe, profile, url)`: headless, eigenes Profil (keine Nutzersitzung), Ton aus, ohne Erststart-Dialog.
- `Start`: öffnet `url` über `proc.Command`; `Kill` und danach genau ein `Wait` des Aufrufers beenden den Prozessbaum.

## Flow

1. `Find` liefert den Pfad (`Server.findBrowser`).
2. `Start(ctx, exe, profile, url)` mit eigenem Profilordner je Client.
3. Der Aufrufer beendet den Browser und löscht das Profil.

## Integration

- Konsument: `internal/mcpsrv/simtest_clients.go` (`launchBrowser`, `Server.findBrowser`).
- Abhängigkeiten: `internal/proc`.
