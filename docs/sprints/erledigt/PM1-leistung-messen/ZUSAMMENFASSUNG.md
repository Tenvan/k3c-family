# PM1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Leistung messen: Diagnose-Zeile und Performance-Modus
- **Projekt:** LST

## Ziel

Leistungswerte der Clients landen ohne Ablesen beim Server: regelmäßig im Client-Log für `sim_test` und auf Wunsch als Bericht mit Einbrüchen. Am Ende sichtbar: Der Status von `sim_test` zeigt Client-FPS und Latenz, und nach einem Spiel im Performance-Modus liegt ein Bericht in `reports/` (`report_read`).

## Ergebnis

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009). PM1.1–PM1.3 fertig (#472), PM1.4 (Messlauf) verworfen, B-334 und B-351 verworfen. Status `erledigt`, weil das Werkzeug für Sprints keinen Status `verworfen` kennt.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Die Diagnose-Meldung entsteht im Takt mit `ctx` `{fps, latencyMs, bufferMs}` (`B-351/AC-01 … |
| AC-02 | – | `sim_test` mit einem Client zeigt FPS und Latenz im Status (`B-351/AC-02`, Nachweis im Bro … |
| AC-03 | geprüft | Ohne Performance-Modus wird nichts gemessen oder gesendet (`B-334/AC-01`, Test). |
| AC-04 | geprüft | Im Performance-Modus entsteht ein Bericht in `reports/` mit Frame-Zeit (p50, p95, p99), Zu … |
| AC-05 | geprüft | Ein Einbruch steht mit Begleitwerten im Bericht (`B-334/AC-03`, Test). |
| AC-06 | – | 🧑 hat einen Messlauf am PC gemacht und den Bericht gesehen (`B-334/AC-04`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-351 | verworfen | AC-01 –, AC-02 – | Der Client schreibt FPS, Latenz und Puffer regelmäßig als Diagnose-Zeile ins Cli … |
| B-334 | verworfen | AC-03 geprüft, AC-04 geprüft, AC-05 geprüft, AC-06 – | Der Client misst Leistung in einem Performance-Modus automatisch und überträgt d … |
