# tools/k3c-dev/frontend/src/mcp/

## Responsibility

Reiter MCP: Beobachtbarkeit des eingebauten MCP-Servers (Zustand, Tool-Zähler, Live-Monitore, Aufruf-Log, Nutzungsstatistik) samt Instructions-Dialog. Enthält außerdem den kleinen Markdown-Parser des Frontends.

## Design

- Container: `McpPage.tsx` mit Unteransichten `uebersicht` / `statistik` (Radix `SegmentedControl`, gemerkt über `prefs`).
- Data-Hook: `useMcpData.ts` lädt `mcpOverview`, `mcpCalls`, `mcpUsage` parallel; kein Polling, sondern gebündeltes Reload (300 ms) nach `mcp:start`/`mcp:call`; `mcp:state` patcht lokal; ein `latest`-Ref verwirft veraltete Antworten.
- Pure Logik: `overview.ts` (`sortTools`, `share`, `weightedAvg`, `uptimeText`), `stats.ts` (`kpis`, `SCOPES`, `rateText`), `series.ts` (`METRICS`, `RANGES`, `RANGE_SPEC`, Zeitfenster/Buckets), `lanes.ts` (`buildGraph` Bahnen gleichzeitiger Aufrufe, `CallFilter`), `markdown.ts` (`parseMarkdown`/`parseInline` -> Baum statt HTML-String).
- Präsentation: `ServerCards`, `ToolTiles`, `LiveMonitors` + `LineChart` (SVG), `CallLog`, `StatsView` mit `StatsTable`/`StatsSide`, `InstructionsDialog`.

## Flow

1. `McpPage` ruft `useMcpData()`; bis `overview` da ist, zeigt sie einen Ladehinweis bzw. eine `NoticeCard` bei Fehler.
2. Übersicht: `ServerCards` (Neustart via `backend.mcpRestart`), `ToolTiles`, `LiveMonitors` (Minuten-Buckets, `useNow`), `CallLog`.
3. Neuer Tool-Aufruf: Go emittiert `mcp:call` -> Hook lädt gebündelt neu -> Komponenten rendern neu.
4. Statistik: `StatsView` wählt Scope (`session`/`allTime`) und berechnet KPIs und Tabelle aus `McpUsage`.
5. `InstructionsDialog` lädt `backend.mcpInstructions()` und rendert es mit `ui/MarkdownView`.

## Integration

- Konsument: `App.tsx` (Tab `mcp`); `markdown.ts` nutzt zusätzlich `ui/MarkdownView.tsx`.
- Abhängigkeiten: `api` (`backend`, `McpCall`, `McpOverview`, `McpUsage` …), `lib/` (`format`, `prefs`, `useNow`, `errors`), `ui/parts`, `@radix-ui/themes`.
- Go-Seite: `McpOverview`, `McpRestart`, `McpInstructions`, `McpCalls`, `McpUsage`; Events `mcp:state`, `mcp:start`, `mcp:call`.
