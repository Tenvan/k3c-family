# tools/k3c-dev/frontend/src/mcp/

## Responsibility

Reiter MCP: Beobachtbarkeit des eingebauten MCP-Servers (Zustand, Live-Verlauf, Aufruf-Log, Nutzungsstatistik) samt Instructions-Dialog. Enthält außerdem den kleinen Markdown-Parser des Frontends.

## Design

- Container: `McpPage.tsx` mit Unteransichten `uebersicht` / `statistik` (Radix `SegmentedControl`, gemerkt über `prefs`).
- Data-Hook: `useMcpData.ts` lädt `mcpOverview`, `mcpCalls`, `mcpUsage` parallel; kein Polling, sondern gebündeltes Reload (300 ms) nach `mcp:start`/`mcp:call`; `mcp:state` patcht lokal; ein `latest`-Ref verwirft veraltete Antworten.
- Pure Logik: `overview.ts` (`share`, `weightedAvg`, `uptimeText`, `durationText`), `stats.ts` (`kpis`, `SCOPES`, `rateText`), `series.ts` (`METRICS` `calls`/`duration`, `RANGES` `15m`/`1h`/`24h`/`7d`, `RANGE_SPEC` mit Spanne und Punkt-Abstand, `windowOf`, `pointsOf`, `header`, `avgOf`, `ticks`), `lanes.ts` (`buildGraph` Bahnen gleichzeitiger Aufrufe, `CallFilter`), `markdown.ts` (`parseMarkdown`/`parseInline` -> Baum statt HTML-String).
- Präsentation: `ServerCards`, `LiveMonitors` (flaches Live-Band über volle Breite, Messgröße und Zeitraum gemerkt über `prefs`) + `LineChart` (SVG), `CallLog`, `StatsView` mit `StatsTable`/`StatsSide`, `InstructionsDialog`.

## Flow

1. `McpPage` ruft `useMcpData()`; bis `overview` da ist, zeigt sie einen Ladehinweis bzw. eine `NoticeCard` bei Fehler.
2. Übersicht: `ServerCards` (Neustart via `backend.mcpRestart`), `LiveMonitors` (Minuten-Zeitreihe aus `usage.minutes`, Fenster wandert per `useNow(30_000)`), `CallLog`.
3. Neuer Tool-Aufruf: Go emittiert `mcp:call` -> Hook lädt gebündelt neu -> Komponenten rendern neu.
4. Statistik: `StatsView` wählt Scope (`session`/`allTime`) und berechnet KPIs und Tabelle aus `McpUsage`.
5. `InstructionsDialog` lädt `backend.mcpInstructions()` und rendert es mit `ui/MarkdownView`.

## Integration

- Konsument: `App.tsx` (Tab `mcp`); `markdown.ts` nutzt zusätzlich `ui/MarkdownView.tsx`.
- Abhängigkeiten: `api` (`backend`, `McpCall`, `McpOverview`, `McpUsage` …), `lib/` (`format`, `prefs`, `useNow`, `errors`), `ui/parts`, `@radix-ui/themes`.
- Go-Seite: `McpOverview`, `McpRestart`, `McpInstructions`, `McpCalls`, `McpUsage`; Events `mcp:state`, `mcp:start`, `mcp:call`.
