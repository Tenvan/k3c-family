# tools/k3c-dev/frontend/src/mcp/

## Responsibility

Reiter MCP: Beobachtbarkeit des eingebauten MCP-Servers (Zustand, Tool-Kacheln, Live-Verlauf, Aufruf-Log, Nutzungsstatistik je Sitzung, Gesamtzeit und Zeitraum) samt Instructions-Dialog. Enthält außerdem den kleinen Markdown-Parser des Frontends.

## Design

- Container: `McpPage.tsx` mit Unteransichten `uebersicht` (Widgets) / `statistik` (Radix `SegmentedControl`, gemerkt über `prefs`); in der Übersicht trennt ein `Splitter` (`mcp.split.band`) Tool-Kacheln und Live-Monitore.
- Data-Hook: `useMcpData.ts` lädt `mcpOverview`, `mcpCalls`, `mcpUsage` parallel; kein Polling, sondern gebündeltes Reload (300 ms) nach `mcp:start`/`mcp:call`; `mcp:state` patcht lokal; ein `latest`-Ref verwirft veraltete Antworten.
- Pure Logik: `overview.ts` (`share`, `sortByCalls`, `toolLine`, `weightedAvg`, `uptimeText`, `durationText`), `stats.ts` (`kpis`, `SCOPES`, `ruleLine`, Sortierung der Tool-Tabelle), `series.ts` (`METRICS` `calls`/`duration`, `RANGES` `15m`/`1h`/`24h`/`7d`, `RANGE_SPEC`, `windowOf`, `pointsOf`, `header`, `avgOf`, `ticks`), `rangeScope.ts` (`rangeScope`: `UsageScope` für einen Zeitraum aus der Minuten-Zeitreihe; Fehler je Tool und Perzentile sind dort NaN und erscheinen als „–“), `lanes.ts` (`buildGraph` Bahnen gleichzeitiger Aufrufe, `CallFilter`), `markdown.ts` (`parseMarkdown`/`parseInline` -> Baum statt HTML-String).
- Präsentation: `ServerCards`, `ToolTiles` (kompakte Kachel je Tool mit Anteilsleiste und Kennzahlenzeile, aktivste zuerst), `LiveMonitors` (Top-Tools und Zeitreihe, Messgröße und Zeitraum gemerkt über `prefs`) + `LineChart` (SVG), `CallLog`, `StatsView` (Sitzung / All time / Zeitraum; Tabelle | `Splitter` `mcp.split.stats` | `StatsSide`; im Zeitraum nur `StatsTable`), `InstructionsDialog`.

## Flow

1. `McpPage` ruft `useMcpData()`; bis `overview` da ist, zeigt sie einen Ladehinweis bzw. eine `NoticeCard` bei Fehler.
2. Übersicht: `ServerCards` (Neustart via `backend.mcpRestart`), `ToolTiles` | `LiveMonitors` (Minuten-Zeitreihe aus `usage.minutes`, Fenster wandert per `useNow(30_000)`), `CallLog`.
3. Neuer Tool-Aufruf: Go emittiert `mcp:call` -> Hook lädt gebündelt neu -> Komponenten rendern neu.
4. Statistik: `StatsView` wählt Scope (`session`/`allTime`) oder Zeitraum (`rangeScope` aus `usage.minutes`) und berechnet KPIs und Tabelle daraus.
5. `InstructionsDialog` lädt `backend.mcpInstructions()` und rendert es mit `ui/MarkdownView`.

## Integration

- Konsument: `App.tsx` (Tab `mcp`); `markdown.ts` nutzt zusätzlich `ui/MarkdownView.tsx` (auch von `planning/`).
- Abhängigkeiten: `api` (`backend`, `McpCall`, `McpOverview`, `McpUsage`, `ToolStats` …), `lib/` (`format`, `prefs`, `useNow`, `errors`), `tasks/TaskBars` (`Splitter`, `useSplit`), `ui/parts`, `@radix-ui/themes`.
- Go-Seite: `McpOverview`, `McpRestart`, `McpInstructions`, `McpCalls`, `McpUsage`; Events `mcp:state`, `mcp:start`, `mcp:call`.
