# WZ48 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Tests und CI: check:go ohne Sim, Workbench-Go-Tests in der CI, stabile und schnelle Tests
- **Projekt:** WZG

## Ziel

Prüfläufe respektieren die Sim-Sperre, die CI prüft die Workbench-Go-Tests, die Workbench-Tests sind stabil und schnell, das Repo enthält keine gebaute EXE.

## Ergebnis

2026-11-14, Review WZ48.4 (Claude, autonom, Klasse A): AC-01 bis AC-05 laut Ergebnissen WZ48.1–WZ48.3 geprüft; AC-02 auf `develop` steht aus, AC-04 `mcpsrv` knapp (9,8–10,3 s). Behoben im Review: `check:go` lief im Runner der Workbench nicht (`tr` fehlt), jetzt Paketliste über `checkdiff -gopkgs`; `task check` und `task check:go` grün. Neue Tickets (anzulegen, Nummern im Hauptcheckout): CI-Nachweis WZ48 auf `develop`; Konsolenpuffer von `check_run` je Checkout trennen. Abnahme-Ticket (anzulegen): „Abnahme-Läufe WZ48“, AC-01 Sim und AC-02 Balance verschoben (Sperre 2026-11-11), AC-03 Gen `check …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | `task check:go` führt keine Sim-, Balance- oder Lasttests aus; Nachweis im Ergebnis (B-832 … |
| AC-02 | – | Die CI fährt die Workbench-Go-Tests mit `-short` (B-828). |
| AC-03 | – | `TestDateiAenderungStartetNeu` stabil oder Ursache belegt (B-829). |
| AC-04 | – | `mcpsrv` und `planning` mit `-short` im Rahmen von B-830. |
| AC-05 | – | Keine gebaute EXE mehr im Repo, `.gitignore` verhindert neue (B-835). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-832 | erledigt | AC-01 – | task check:go führt trotz Sperre die Balance- und Lasttests aus |
| B-828 | erledigt | AC-02 – | Der CI-Job k3c-workbench fährt die Go-Tests der Workbench |
| B-829 | erledigt | AC-03 – | TestDateiAenderungStartetNeu bleibt unter Last stabil |
| B-830 | erledigt | AC-04 – | mcpsrv und planning bleiben mit -short unter 3 s je Test und 10 s je Paket |
| B-835 | erledigt | AC-05 – | cmd/k3c-server/k3c-server.exe liegt als 14-MB-Binary im Repo |
