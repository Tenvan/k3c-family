# BAL1.3 · Replay-Wiedergabe in k3c-dev

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** bal1/3-replay-k3c-dev
- **Abhängig von:** BAL1.2
- **Tickets:** B-159
- **Kriterien:** AC-06, AC-07

## Ziel

k3c-dev spielt eine Replay-Datei ab (neben `sim_run`) und nennt Endzustand-Hash und Burgfall-Tick.

## Kontext

- k3c-dev ist ein eigenes Go-Modul (`tools/k3c-dev/go.mod`, importiert `k3c`), Domäne SRV laut `docs/arbeitsweise.md`. Diese Session ändert dort nur das Nötige für AC-06 (B-159 › Anforderungen verlangt „Wiedergabe-Werkzeug in k3c-dev neben `sim_run`“); das gilt mit der Freigabe der Spec als erlaubte Ausnahme. Steht das bei der Freigabe nicht so, Session `blockiert`.
- `sim_run`: MCP-Werkzeug in `tools/k3c-dev/internal/mcpsrv/tools_engine.go` (`simRun` ruft `enginetools.Run`), Logik in `tools/k3c-dev/internal/enginetools/run.go`. Neues Werkzeug (z. B. `replay_run`) mit Eingabe Dateipfad, Ausgabe Endzustand-Hash, Burgfall-Tick, Warnung bei abweichendem Datenstand; ruft die Wiedergabe aus BAL1.2 (Paket, nicht nachgebaut).
- Pfade: nur Dateien unterhalb des Repos bzw. eines erlaubten Ordners lesen (Pfad prüfen, kein beliebiger Zugriff), Muster vorhandener Werkzeuge mit Dateipfaden übernehmen.
- Tests in k3c-dev: `task check:dev` (Frontend, go test, golangci-lint); MCP-Tests z. B. `tools/k3c-dev/internal/mcpsrv/tools_server_test.go`.

## Erlaubte Dateien

- `tools/k3c-dev/internal/mcpsrv/` (neues Werkzeug mit Test), `tools/k3c-dev/internal/enginetools/` (falls eine dünne Hülle nötig ist)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Oberfläche für Replays in der k3c-dev-Ansicht, Aufnahme in k3c-dev, Änderungen an `engine/sim` oder am Format (BAL1.2).

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Werkzeug `replay_run` (Name im Ergebnis) mit Pfadprüfung; ruft die Wiedergabe aus BAL1.2.
3. Test: Beispiel-Datei aus `testdata/replay/` → Hash und Burgfall-Tick wie bei der Aufnahme; Pfad außerhalb des Repos → Fehler.
4. `task check:go` und `task check:dev`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-06: k3c-dev spielt eine Replay-Datei ab und nennt Endzustand-Hash und Burgfall-Tick (Test).
- [ ] AC-07: `task check:go` grün; zusätzlich `task check:dev` grün.

## Prüfen

```bash
task check:go
task check:dev
```

## Ergebnis

–
