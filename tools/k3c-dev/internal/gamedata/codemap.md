# tools/k3c-dev/internal/gamedata/

## Responsibility
Read-only-Zugriff auf Xbox-Berichte (`reports/*.json`) und Spielstände (`saves/*.json`) für Agenten (B-063): liefert kompakte Textzusammenfassungen statt Roh-JSON.

## Design
- `reports.go`: `ReportsList` (neueste zuerst, eine Zeile je Bericht) und `ReportRead`; interne Typen `report`, `pad`, `perf`, `attempt`; Hilfsmethoden `when`, `device`, `biggestStage`, `fps`, `fullscreen`; `buttons` übersetzt Indizes über `buttonNames`.
- Pfadschutz: `inside` + Regex `fileName` erlauben nur einfache `*.json`-Namen im Ordner (kein Traversal).
- `saves.go`: `SavesList` (inkl. Sicherungen), `SavesDir(root, env)` mit Override `EnvSavesDir` (`K3C_SAVES_DIR`, relativ ab Repo-Wurzel), `cycleSeconds` liest den Tageszyklus aus den Daten für die Zeitangabe.
- Funktionen sind zustandslos und nehmen `root` explizit (Worktree-fähig).

## Flow
1. `ReportsList(root)`: `reports/` lesen → `named` sortieren (`sortKey`) → `listLine` je Datei.
2. `ReportRead(root, name)`: `inside` validiert Namen → `readReport` → Zeilen für Gerät, FPS, Vollbild, Tasten, größte Stufe.
3. `SavesList(root, dir)`: `cycleSeconds` → `saveLine` je Datei.

## Integration
- Konsument: `internal/mcpsrv/tools_gamedata.go` (MCP `reports_list`, `report_read`, `saves_list`).
- Abhängigkeiten: nur Standardbibliothek; Daten aus `reports/`, `saves/`, `data/`.
