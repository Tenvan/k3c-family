# TR1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Testläufe über `sim_test` in der Workbench
- **Projekt:** –

## Ziel

Jeder Testlauf (Balancing, Performance von Server und Client, Stabilität) startet und läuft über ein MCP-Tool `sim_test` der Workbench, in den Modi offline/online und headless/mit 1–4 Clients. Am Ende sichtbar: `sim_test start` → ID, `sim_test status` → höchstens 10 Zeilen, Bericht unter `reports/simtest-*`.

## Ergebnis

2026-10-07: Kriterien siehe Ergebnisse TR1.1–TR1.3; AC-04 Browser-Nachweis verschoben (B-349/TR2.1, B-351). Keine schweren Befunde offen (zwei in TR1.3 vor dem Merge behoben). Neue Tickets: B-351, B-352. Version: v0.15.0 vorgeschlagen (neues Werkzeug `sim_test`), nicht gesetzt (Entscheidung 🧑, 2026-10-07).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Lauf-Register und Aktionen `start`, `status`, `stop`, `list` (B-348/AC-01). |
| AC-02 | – | Offline headless: Balancing über viele Seeds mit Bewertung der Ziele (B-348/AC-02). |
| AC-03 | – | Online headless: Bot-Geräte gegen den Spielserver, Performance und Stabilität im Status (B … |
| AC-04 | – | Online mit 1–4 Clients: Clients starten oder anhängen, Bots steuern ihre Monarchen, Client … |
| AC-05 | – | Ungültige Parameter und Kombinationen abgelehnt (B-348/AC-05). |
| AC-06 | – | Regel „jeder autonome Testlauf über `sim_test`“ steht in `CLAUDE.md` und `docs/arbeitsweis … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-348 | erledigt | AC-01 –, AC-02 –, AC-03 –, AC-04 –, AC-05 –, AC-06 – | Jeder Testlauf startet und läuft über das MCP-Tool `sim_test` |
