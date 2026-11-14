# LG1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Remote-Logs vom Pi und Xbox mit WebGL 1 und 130 MiB
- **Projekt:** LST

## Ziel

Alle Logs eines entfernten Servers (Pi) sind per Endpunkt und MCP-Tool lesbar, und die Xbox läuft mit WebGL 1 unter 130 MiB Texturspeicher. Am Ende sichtbar: `logs_errors {server: "pi"}` zeigt Server- und Client-Zeilen vom Pi, das Client-Log eines Xbox-Laufs nennt WebGL 1 und den Verbrauch unter 130 MiB, die Textur-Testseite passt ohne Scrollen auf den TV.

## Ergebnis

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). LG1.1–LG1.5 fertig (#541), LG1.6 (Abnahme Pi/Xbox) verworfen, B-770 bis B-774 verworfen. Status `erledigt`, weil das Werkzeug für Sprints keinen Status `verworfen` kennt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Die Endpunkte listen und lesen alle Log-Dateien, filtern, nutzen Cursor und lehnen Pfade a … |
| AC-02 | geprüft | `logs_*` und `server_status` lesen mit `server` einen entfernten Server, Fehler werden kla … |
| AC-03 | erfüllt | Wahl von WebGL 1 und Budgetprüfung (`B-772/AC-01`, `AC-02`, Test). |
| AC-04 | geprüft | Textur-Testseite ohne Seitenscroll bei drei Auflösungen (`B-773/AC-01`, `AC-02`, Test). |
| AC-05 | – | Abnahme (🧑): Log-Ordner am Pi gesetzt, `pi-gaming:8080/api/dev/log?file=client` liefert Ze … |
| AC-06 | – | Abnahme (🧑): `logs_errors {server: "pi"}` zeigt echte Pi-Zeilen (`B-771/AC-04`). |
| AC-07 | – | Abnahme (🧑): Xbox-Lauf ohne Kontextverlust, Log zeigt WebGL 1 und Texturen unter 130 MiB ( … |
| AC-08 | – | Abnahme (🧑): Textur-Testseite passt auf der Xbox auf einen Screen (`B-773/AC-03`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-770 | verworfen | AC-01 geprüft, AC-05 – | Alle Logs eines Servers sind über den Dev-Endpunkt abrufbar |
| B-771 | verworfen | AC-02 geprüft, AC-06 – | Die Log-Tools des MCP-Servers lesen auch entfernte Server (Pi) |
| B-772 | verworfen | AC-03 erfüllt, AC-07 – | Auf der Xbox läuft WebGL 1 mit einem Texturspeicher von höchstens 130 MiB |
| B-773 | verworfen | AC-04 geprüft, AC-08 – | Die Textur-Testseite passt responsiv auf einen Screen |
