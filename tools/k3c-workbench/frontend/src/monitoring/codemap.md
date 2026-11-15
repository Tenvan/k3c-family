# tools/k3c-workbench/frontend/src/monitoring/

## Responsibility

Reiter **Monitoring** im Bereich Development: Speicher, Kindprozesse, Goroutinen, Handles, Puffer, GC-Läufe und die
Vorgänge mit der größten Allokation der Workbench über die Zeit, dazu Trend je Stunde und abgelegte Heap-Profile.

## Design

- `MonitoringPage.tsx`: bleibt eingehängt, fragt aber nur ab und rechnet nur, solange `active`; Zeitraum `15m`, `1h`,
  `6h`, `24h` (gemerkt über `prefs`). Diagramme: Speicher der Workbench, Kindprozesse (RSS), Goroutinen, Handles und
  Kinder, Puffer nach Bytes und Anzahl (`ResGauge`), GC-Läufe je Punkt (aus `deltas`); darunter `Summary` und `OpsTable`.
- `SeriesChart.tsx`: Liniendiagramm mit Legende (Wert am Cursor, sonst der letzte), Achsenmaximum, Hinweis unter der
  Maus; SVG gestreckt mit `non-scaling-stroke`, Text als HTML.
- `Summary.tsx`: Kennzahlen der letzten Probe mit Trend (Warnschwelle wie `growthWarn` in
  `tools/k3c-workbench/internal/resmon/report.go`), größte Kindprozesse, abgelegte Profile.
- `OpsTable.tsx`: Vorgänge nach Allokation mit Balken relativ zum größten, Aufrufe, Ø und Höchstwerte.
- `chart.ts` (rein): Fläche `W = 1000` × `H = 100`, `niceMax`, `peakOf`, `linePath`, `areaPath`, Zeitmarken, Treffer
  unter der Maus, `deltas`.
- `useResources.ts`: kein Verlauf im Browser – jede Antwort ersetzt die vorige (Go verdichtet auf höchstens 240 Punkte);
  Abfrage alle `REFRESH_MS = 10 s`, nur bei `active` und sichtbarem Fenster.

## Flow

1. `App` hängt `MonitoringPage active` dauerhaft ein -> `useResources(windowMin, active)` -> `backend.resources(windowMin)`
   -> `ResourceView`.
2. `ResourceView` -> Linien je Kennzahl -> `SeriesChart` (über `chart.ts`), `Summary`, `OpsTable`.
3. Seite verdeckt oder Fenster im Hintergrund -> kein Timer, keine Abfrage.

## Integration

- Konsument: `src/App.tsx` (Reiter `monitoring`).
- Backend: Binding `Resources` in `tools/k3c-workbench/app_resources.go` (Paket `internal/resmon`); dieselben Daten
  liefert das MCP-Tool `workbench_resources`.
- Abhängigkeiten: `src/api` (Typen aus `api/typesResources.ts`), `lib/format`, `lib/prefs`, `@webkit/atoms|molecules`.
  Stile: `../styles/monitoring.css`.
