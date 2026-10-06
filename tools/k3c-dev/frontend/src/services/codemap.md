# tools/k3c-dev/frontend/src/services/

## Responsibility

Reiter Dienste: Übersicht und Steuerung der vom Werkzeug verwalteten Dienste (Vite-Dev-Server, Spielserver …) mit Status, Metriken, Start/Stop/Neustart und eingebettetem Logs-Bereich.

## Design

- Pure Logik: `tables.ts` (`badgeFor`, `buttonsFor`, `Command`, `applyStatus` verwirft Updates mit kleinerer `seq`, `orderLine`, `levelShares` über `LEVELS`, `metricsOf` für CPU/Speicher/Uptime).
- Container: `ServicesPage.tsx` – Kartenliste, `BulkBar` mit „Neu laden“ (`servicesReload`), „Alle starten/stoppen“ und `orderLine`, gemerkte Quellenwahl mit `logs/SourceBar` und `logs/SourcePanel`.
- Präsentation: `ServiceCard.tsx` (Aktionen, `AlertDialog` für erzwungenes Stoppen, `RoleTags`, `HealthUrl` mit klickbarer http-Adresse über `backend.openUrl`, `Notes` mit Neustart-Zähler, Änderungsdatei und Fehlern, `Metrics`), `LogBox.tsx` (Level-Verteilung der letzten 60 min).
- Event-getriebener Zustand: `service:state` aktualisiert einzelne Karten, ältere `seq` werden ignoriert.

## Flow

1. `ServicesPage` lädt `backend.services()` (`ServicesView`; `error` meldet eine nicht geladene `services.json`) und `useSources()`.
2. Ereignisse vor der Liste werden gepuffert und nachgetragen; `backend.on('service:state')` -> `applyStatus` ersetzt den Eintrag.
3. Karte: Button -> `serviceStart` / `serviceStop(name, force)` / `serviceRestart`.
4. `LogBox` ruft `serviceLogLevels(name)` und zeigt die Level-Anteile.
5. Gewählte Quelle -> `logFor` -> `SourcePanel` (Konsole/Log/Fehler).

## Integration

- Konsument: `App.tsx` (Tab `dienste`).
- Abhängigkeiten: `api` (`ServiceStatus`, `ServicesView`, `LevelCounts`), `logs/` (Quellenleiste, Panel, `RoleTags`, `pickSource`), `lib/`, `ui/parts`, `@radix-ui/themes`.
- Go-Seite: `Services`, `ServiceStart`, `ServiceStop`, `ServiceRestart`, `ServicesStartAll`, `ServicesStopAll`, `ServiceLogLevels`, `ServicesReload`; Event `service:state`.
