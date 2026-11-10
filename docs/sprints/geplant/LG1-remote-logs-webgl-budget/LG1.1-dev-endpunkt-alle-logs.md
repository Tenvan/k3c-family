# LG1.1 · Dev-Endpunkt liefert alle Logs

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** lg1/1-dev-endpunkt-alle-logs
- **Abhängig von:** –
- **Tickets:** B-770
- **Kriterien:** AC-01

## Ziel

`/api/dev/logs` listet alle Log-Dateien, `/api/dev/log?file=` liest Server- und Client-Log mit Filter und Cursor, `/api/dev/status` nennt den Log-Ordner (B-770).

## Kontext

`engine/net/dev_read.go` liest bisher nur `<LogDir>/k3c-server.jsonl` (`LogDir` aus `K3C_LOG_DIR`, `cmd/k3c-server/logging.go`); ohne Ordner `404 „Log aus“` (Zeile 67 und 80). Das Client-Log `k3c-client.jsonl` (Browser-Meldungen aus `/api/clientlog`) ist nicht erreichbar. `engine/net/status_log.go` hat den Cursor-Leser `readLog` für `/api/status/log` (mit Token), der wiederverwendet wird. Protokoll: `docs/protocol.md` › Dev-Lese-Endpunkte (ca. Zeile 933). Spec: B-770.

## Erlaubte Dateien

- `engine/net/dev_read.go`, `engine/net/dev_read_test.go`, `engine/net/status.go` (nur Status-Feld), `engine/net/status_log.go` (nur gemeinsamer Leser)
- `docs/protocol.md`, `README.md` (Endpunkte nachtragen)

## Nicht-Ziele

Kein Schreiben, kein Streaming, kein Zugriff außerhalb von `LogDir`, keine Rotation ändern, keine Pi-Konfiguration (liegt bei 🧑).

## Schritte

1. Feste Namensliste (`server`, `client`, rotierte Dateien) und `GET /api/dev/logs` mit Name, Größe, Änderungszeit.
2. `GET /api/dev/log` um `file` und `since` (Cursor, `truncated`) erweitern, `pattern`/`limit`/`mib` behalten.
3. `/api/dev/status` nennt Log-Ordner und Dateien; „Log aus“ nennt `K3C_LOG_DIR`.
4. Tests für Liste, beide Logs, Filter, Cursor, Pfad-Ablehnung (400), 403, 405, 404.
5. Protokoll und README nachtragen. Reviewer-Agent vor dem Commit (SRV, Pfad-Code).

## Fertig, wenn

- [ ] AC-01: `engine/net/dev_read_test.go` deckt Liste, Server- und Client-Log, Filter, Cursor und Pfad-Ablehnung ab; `B-770/AC-01` bis `AC-03` grün.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
