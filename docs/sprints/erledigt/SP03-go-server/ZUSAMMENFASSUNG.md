# SP03 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Go-Server Basis
- **Projekt:** –

## Ziel

`cmd/k3c-server` ersetzt `server/*.mjs` für Dateien, Spielstände und Berichte. Am Ende sichtbar: Das Spiel läuft von der Windows-EXE und aus dem Docker-Image wie heute mit `npm run serve`.

## Ergebnis

- 2026-09-30, leichtes Review über `c8a221a..main` durch zwei Reviewer (Sonnet): Sicherheit/Daten und Start/Docker/CI. Kriterien: AC-01 (SP03.1 EXE von Hand, SP03.2 Docker per CI, grün laut 🧑), AC-02 (SP03.1), AC-03 und AC-04 (SP03.2). - Behoben (schwer): SIGTERM beendet sauber (vorher SIGKILL nach `docker stop`); gleichzeitiges Speichern auf denselben Slot unter einer Sperre. Behoben (vermutet): CI-Prüfung des Benutzers. Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Windows-EXE und Docker-Image liefern das Spiel aus wie `npm run serve`. |
| AC-02 | geprüft | `/api/save`, `/api/report` und `/api/health` verhalten sich wie heute, geprüft mit `httpte … |
| AC-03 | geprüft | `/api/status` antwortet nur mit Token (B-027/AC-01, B-027/AC-02). |
| AC-04 | geprüft | Spielstände werden rotierend gesichert und per API wiederhergestellt (B-028/AC-01, B-028/A … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-020 | erledigt | AC-02 geprüft | Server-Tests laufen lokal wie in der CI |
| B-027 | erledigt | AC-03 geprüft | Diagnose-Schnittstelle ist abgesichert |
| B-028 | erledigt | AC-04 geprüft | Spielstände werden rotierend gesichert |
