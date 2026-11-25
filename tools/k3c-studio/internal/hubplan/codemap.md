# tools/k3c-studio/internal/hubplan/

## Responsibility

Gebäude-Planer des Studios (B-923, Port aus dem früheren Asset-Studio, B-462): zeigt `data/hub.json` als Matrix Hub-Stufe × Bauplatz, prüft einen Kandidaten über die Engine, vergleicht die Regeltabelle mit dem Planer-Stand und speichert einen geprüften Kandidaten format-stabil.

## Design

- `Editor{Root, Engine}` (`hubplan.go`): `Plan` (lädt `data/hub.json`), `Check` (`hubplan_edit.go`, Kandidat ohne Schreiben), `Rules` (`hubplan_rules.go`), `Save` (`hubplan_save.go`). `Engine func(ctx, input) (json.RawMessage, error)` schickt `{"op":"plan","raw":…}` an `k3c-game sim hub`; `Exe(root)` rechnet über die Spiel-EXE (`internal/gameexe`, Timeout `engineTimeout`); ohne Engine (`nil`) liefert `ask` einen Fehler statt eines Absturzes.
- Sicht: `HubPlanView` (Pfad, Inhalt als Kandidat der Live-Vorschau, `HubMatrix` mit Spalten aus `levels` und Zeilen `HubRow` je Bauplatz mit `HubCell` je Hub-Stufe, Befunde `HubFindingView`, `Stand`, erlaubte Arten `Kinds`, `Error`). Matrix und Befunde rechnet die Engine, die Workbench hält keine Prüfregel.
- Regeltabelle (`hubplan_rules.go`): `Rules(hubJSON)` vergleicht den Block „Hub-Stufe → Gebäude“ in `docs/rules/materialien-gebaeude.md` mit dem aus dem Planer-Stand erzeugten Block (`table.Gegenprobe` aus `tools/ruletable/table`, dieselbe Funktion wie der Task `task gen:rules`); `HubRulesView` nennt Ist, Soll, abweichende Zeilen (`diffLines`) und den Task zum Neuschreiben; leerer `hubJSON` = Datei des Checkouts.
- Speichern (`hubplan_save.go`): nur ein Kandidat ohne Befund. `HubStand` hält die SHA-256 von `data/hub.json` und Regeltabelle beim Laden; hat sich eine Datei seitdem geändert, wird nichts geschrieben (`Stale`). Danach `patchJSON`, atomar über `assets.WriteAtomic`, dann `writeRules` (`table.Ersetzen`, nur der Block zwischen den Marken); scheitert die Regeltabelle, bleibt `RulesError`, `hub.json` ist gespeichert. Inhaltlich gleicher Kandidat = `Unchanged`, `Notice` weist auf die Regel-Änderung hin. `hubSaveMu` reiht Speichervorgänge des Prozesses.
- Format-stabiles Schreiben (`patch.go`): `scanJSON` liest den Text mit den Lagen aller Teilwerte, `patchJSON(old, cand)` ersetzt nur geänderte Werte; Schlüsselreihenfolge, Einrückung, Zahlenformat und Zeilenenden bleiben, neue Listen-Einträge folgen dem Layout der Liste (`fresh`, `layout`, `inline`); das Ergebnis muss dasselbe bedeuten wie der Kandidat (`sameJSON`), sonst Fehler.

## Flow

1. Oberfläche → `App.HubPlan`/`HubCheck`/`HubRules`/`HubSave` (`app_hub.go`) bzw. MCP `hub_get`/`hub_check`/`hub_save` (`mcpsrv/tools_hub.go`) → `Editor`.
2. `Plan`/`Check`: Datei bzw. Kandidat lesen → `ask` (`k3c-game sim hub`) → `engineReport` → `HubPlanView` mit Matrix, Befunden und `Stand`.
3. `Save`: Prüfung → Befunde leer → Stand der Dateien vergleichen → `patchJSON` → `WriteAtomic` → `writeRules` → neuer `Stand`.

## Integration

- Konsumenten: `tools/k3c-studio/app_hub.go` (Bindings, `hubEngine = hubplan.Direct`, in `uiMethods` von `uiapi.go`), `internal/mcpsrv/tools_hub.go`.
- Abhängigkeiten: Spiel-EXE (`internal/gameexe`), `tools/assetkit` (`WriteAtomic`), `tools/ruletable/table`; Daten `data/hub.json`, `docs/rules/materialien-gebaeude.md`.
