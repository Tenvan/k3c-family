# tools/k3c-workbench/frontend/src/mcp/

## Responsibility

Reiter MCP: Beobachtbarkeit des eingebauten MCP-Servers (Zustand, Tool-Kacheln, Live-Verlauf, Auswertung der `check_run`-Läufe, Aufruf-Log, Nutzungsstatistik je Sitzung, Gesamtzeit und Zeitraum) samt Instructions-Dialog.

## Design

- Container: `McpPage.tsx` mit Unteransichten `uebersicht` (Widgets) / `statistik` (`SegmentedControl`, gemerkt über `prefs`); in der Übersicht trennt ein `Splitter` (`mcp.split.band`) Tool-Kacheln und Live-Monitore, ein waagerechter `SplitStack` (`mcp.split.log`, B-875) Widgets samt `CheckRunPanel` und Aufruf-Log.
- Data-Hook: `useMcpData.ts` lädt `mcpOverview`, `mcpCalls`, `mcpUsage` beim Öffnen; kein Abfrage-Intervall. Das Aufruf-Log folgt `mcp:start`/`mcp:call` selbst (`callBuffer.ts`: `applyCall` ersetzt nach `id`, höchstens `MAX_CALLS = 250` wie der Go-Ringpuffer plus Spielraum); Übersicht und Statistik lädt `liveReload` (`lib/liveReload.ts`) höchstens alle 5 s und nicht bei verborgenem Fenster (B-895); `mcp:state` patcht lokal.
- Pure Logik: `overview.ts` (`share`, `sortByCalls`, `toolLine`, `latestErrorTool`, `weightedAvg`, `uptimeText`, `durationText`), `stats.ts` (`kpis`, `SCOPES`, `rateText`, `dur`, Sortierung der Tool-Tabelle), `series.ts` (`METRICS` `calls`/`duration`, `RANGES` `15m`/`1h`/`24h`/`7d`, `RANGE_SPEC`, `windowOf`, Punkte und Ranglisten), `liveStats.ts` (`toolTotals`, `barValue`, `barLabel`, `barTip`, Werteachse; B-874), `rangeScope.ts` (`UsageScope` für einen Zeitraum aus der Minuten-Zeitreihe; Fehler je Tool und Perzentile sind dort NaN und erscheinen als „–“), `lanes.ts` (`buildGraph` Spuren gleichzeitiger Aufrufe, `CallFilter`, `PALETTE_SIZE`), `checkRuns.ts` (`runOf`, `runsIn`, `parseDuration`: `check_run`-Aufrufe als Läufe je Ziel mit Ergebnis `ok`/`failed`/`pending`, B-876).
- Präsentation: `ServerCards` (Kennzahlen `Kpi`, Neustart, Systemprompt, Aktualisieren), `ToolTiles` (kompakte Kachel je Tool, aktivste zuerst, nur das Tool mit dem jüngsten Fehler rot), `LiveMonitors` (Verlauf und Tool-Balken, Messgröße und Zeitraum gemerkt) + `LineChart` (SVG gestreckt, Achsen und Legende als HTML), `CheckRunPanel` (Kennzahlen je Ziel, `CheckRunChart`, Klick auf einen Lauf zeigt Log-Eintrag und `ConsoleView` des Ziels), `CallLog` (Live/Alle, Start- und Endzeile mit Graph-Spuren), `StatsView` (Sitzung / All time / Zeitraum; `StatsTable` | `Splitter` `mcp.split.stats` | `StatsSide`), `InstructionsDialog`.

## Flow

1. `McpPage` ruft `useMcpData()`; bis `overview` da ist, zeigt sie einen Ladehinweis bzw. eine `NoticeCard` bei Fehler.
2. Übersicht: `ServerCards` (Neustart via `backend.mcpRestart`), `ToolTiles` | `LiveMonitors` (Minuten-Zeitreihe aus `usage.minutes`, Fenster wandert per `useNow`), `CheckRunPanel`, `CallLog`.
3. Neuer Tool-Aufruf: Go emittiert `mcp:start`/`mcp:call` -> `applyCall` aktualisiert das Log sofort -> `liveReload.soon()` holt Übersicht und Statistik gedrosselt nach.
4. Statistik: `StatsView` wählt Scope (`session`/`allTime`) oder Zeitraum (`rangeScope` aus `usage.minutes`) und berechnet KPIs und Tabelle daraus.
5. `InstructionsDialog` lädt `backend.mcpInstructions()` und rendert es mit `MarkdownView` (`@webkit/molecules`).

## Integration

- Konsument: `App.tsx` (Tab `mcp`).
- Abhängigkeiten: `api` (`backend`, `McpCall`, `McpOverview`, `McpUsage`, `ToolStats` …), `lib/` (`format`, `prefs`, `useNow`, `liveReload`), `logs/ConsoleView`, `MarkdownView` (`@webkit/molecules`), `@webkit/lib/errors`, `@webkit/lib/split` (`useSplit`), `@webkit/atoms` (`Splitter`, `SegmentedControl` …), `@webkit/molecules`, `@webkit/organisms` (`Dialog`).
- Go-Seite: `McpOverview`, `McpRestart`, `McpInstructions`, `McpCalls`, `McpUsage`; Events `mcp:state`, `mcp:start`, `mcp:call`. Stile: `../styles/mcp.css`, `../styles/mcpCharts.css`.
