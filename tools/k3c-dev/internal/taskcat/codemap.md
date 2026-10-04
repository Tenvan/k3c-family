# tools/k3c-dev/internal/taskcat/

## Responsibility
Task-Katalog der Taskfile-Hierarchie: lädt alle Tasks über `task --list-all --json` und gruppiert/filtert sie nach Namensräumen für die Oberfläche.

## Design
- Kein eigener YAML-Parser: `go-task` löst Includes, Aliase und `internal: true` selbst auf; `Load` ruft `listCommand` (`task --list-all --json --no-status`) in `root` auf.
- Typen: `Task`, `Namespace`, `listOutput` (JSON-Schema der Ausgabe, unbekannte Felder ignoriert); Konstante `Workspace` für Tasks ohne Doppelpunkt (Root-Aggregate).
- `Parse(data, root)` ist rein und testbar; `split` trennt Namensraum/Blatt, `relative` macht Taskfile-Pfade relativ zu `root`; `describe` übersetzt Exec-Fehler.
- `group.go`: `Group(tasks)` (Workspace zuerst, Rest sortiert, nie nil) und `Filter(tasks, query)` (Name/Beschreibung, case-insensitive; Namensräume ohne Treffer entfallen). Die Logik liegt im Backend, damit auch ein MCP-Tool sie nutzen kann.

## Flow
1. `Load(ctx, root)`: `task --list-all --json` mit Frist aus `ctx`.
2. `Parse` → `[]Task` (Namensraum, Pfad relativ).
3. `Filter(tasks, query)` → `Group` → `[]Namespace` an die Oberfläche.

## Integration
- Konsument: `tools/k3c-dev/app_tasks.go` (Wails-Bindings Task-Ansicht).
- Abhängigkeit: externes Programm `task` (go-task); sonst Standardbibliothek.
