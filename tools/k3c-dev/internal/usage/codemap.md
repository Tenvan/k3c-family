# tools/k3c-dev/internal/usage/

## Responsibility
Auswertung der MCP-Aufrufe von k3c-dev (B-062): Laufzeiten je Tool, Perzentile, Ausreißer, häufigste Argumente und Fehler, getrennt für Sitzung und Gesamtzeit, plus Minuten-Zeitreihe der letzten sieben Tage. Persistiert in eine JSON-Datei. Kennt den Server nicht; die Middleware in `mcpsrv` meldet Events.

## Design
- Aggregat: `Tracker` (`usage.go`), threadsafe; `Record(Event)`, `Snapshot() Usage`, `New(now)` nur im Speicher, `Open(path, now, onErr)` mit Datei.
- Zwei Bereiche `scope` (`scope.go`: Sitzung/Gesamt) mit `toolAgg`, Listen `SlowCall` (`insertSlowest`) und Ausreißer-Baseline je Tool/Argument.
- Histogramm (`hist.go`): `durAgg` mit Buckets (`histIndex`, `histValue`), `p(q)`-Perzentile, `merge`; `normArgs`, `clip`, `bump` normalisieren und begrenzen Argument-Zähler.
- Zeitreihe: `minute`/`Bucket`, `minuteFor` (Einfügen in beliebiger Reihenfolge, aufsteigend, ohne Doppel), `prune` (7 Tage).
- Snapshot-Typen (`snapshot.go`): `Usage`, `Scope`, `ToolUsage`, `Count`; Listen leer statt nil.
- Persistenz (`persist.go`): `fileData`, verzögertes `scheduleSave`, `Flush`, `write`; kaputte Datei wird per `moveAside` mit Zeitstempel gesichert, `repair` bereinigt Teilwerte; unlesbare Datei: dieser Lauf speichert nicht.

## Flow
1. Start: `Open` lädt `fileData` (`decode`) in die Gesamtzeit.
2. Je MCP-Aufruf: Middleware → `Record` → Baseline-Vergleich (Ausreißer) → `scope.add` beider Bereiche → `addMinute` → `scheduleSave`.
3. Oberfläche: `Snapshot` → `Usage`.
4. Beenden: `Flush`.

## Integration
- Konsumenten: `internal/mcpsrv/observe.go` und `server.go` (Middleware, `Record`), `tools/k3c-dev/app.go` (Anlegen, `Flush`), `app_mcp.go` (Snapshot für die MCP-Seite).
- Abhängigkeiten: nur Standardbibliothek.
