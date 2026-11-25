# RU69 · DEV · Workbench-Backend in Rust: Dienste, Tasks, Logs, Git, Planung, MCP

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-25

## Ausgangslage

Entwurf. Abhängig von RU66, RU68 und RU40 (Workbench ruft nur noch die Spiel-EXE). Workbench-Backend: rund 19.000 Zeilen Go, 12.000 Zeilen Tests; Frontend bleibt React.

## Ziel

Das Backend der Workbench läuft in Rust hinter dem unveränderten Frontend, Teil für Teil: Dienste und Prozesse, Tasks und Prüfläufe, Logs, Git, Planung (gegen die Golden aus RU68), MCP-Server `k3c-workbench` (5180). Der Go-Stand bleibt bis zur Abnahme durch Ralf lauffähig.

## Beteiligte und Zielgruppen

Agenten setzen um; 🧑 nimmt die Workbench in Rust ab (Fenster, MCP, Planungsseite).

## Anforderungen

Backend von `tools/k3c-workbench` in Rust (Tauri nach Entscheidung 014), Frontend und `tools/webkit` unverändert. Teile in dieser Reihenfolge, jeder Teil für sich lauffähig: 1. Gerüst: Fenster, Commands, MCP-Server `k3c-workbench` auf Port 5180 mit `workbench_status`. 2. Dienste und Tasks (`svc_*`, `task_*`, `check_run`, `console_tail`). 3. Logs (`logs_*`). 4. Git und GitHub-Status (`gh_status`, Git-Seite). 5. Planung (`plan_*`), gegen die Golden aus RU68. 6. übrige Tools und Seiten (Monitoring, Playtest, Ports). Die Go-Workbench bleibt bis zur Abnahme lauffähig.

## Nicht-Ziele

Studio (RU70). Entfernen von Go (RU71). Neue Funktionen der Workbench.

## Regeln und Einschränkungen

Verhalten wie Go; die Tool-Namen, Parameter und Antworten des MCP bleiben gleich. Planung nur gegen die Golden aus RU68 grün. Datei ≤ 400, Funktion ≤ 60 Zeilen. Abnahme (Fenster, MCP, Seiten) nur durch 🧑 am PC.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, nur IDs und Pfade:

- **Bereich:** `rust/crates/k3c-workbench/**`, `rust/Cargo.toml`, `rust/Cargo.lock`, `tools/k3c-workbench/**`, `Taskfile.yml`
- **Abhängig von:** RU66, RU68, RU40
- **Braucht 🧑:** Abnahme
- **Sperre:** deps, taskfile, exe

## Beispiele

`plan_next` in der Rust-Workbench liefert für denselben Dateibaum dieselbe Antwort wie die Go-Workbench; `claude` verbindet sich auf Port 5180 und sieht dieselben Tools.

## Ausnahme- und Fehlerfälle

Weicht Rust von Go ab, gilt Go. Fehlt eine Funktion im MCP, schlägt Claude eine Tool-Erweiterung vor statt Dateien direkt zu ändern.

## Akzeptanzkriterien

- **AC-01** Die Rust-Workbench startet als Fenster und im Dev-Betrieb und bedient den MCP auf 5180 mit allen bisherigen Tools.
- **AC-02** Die Golden der Planung (RU68) sind grün.
- **AC-03** `task check:all` ist grün; 🧑 hat die Workbench am PC abgenommen.

## Offene Fragen

Abhängig von Entscheidung 014 (🧑).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|

Stichpunkte (Reife Entwurf): 1. Gerüst (AC-01). 2. Dienste, Tasks (AC-01). 3. Logs (AC-01). 4. Git, GitHub (AC-01). 5. Planung (AC-02). 6. Übriges (AC-01). 7. Review und Abnahme 🧑 (AC-03).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
