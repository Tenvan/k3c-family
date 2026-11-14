# B-771 · Die Log-Tools des MCP-Servers lesen auch entfernte Server (Pi)

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** verworfen
- **Sprint:** LG1
- **Projekt:** LST
- **Erstellt:** 2026-11-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude (Nutzer: „LST ganz abarbeiten“, 2026-11-10)

## Ausgangslage

Die Log-Tools des MCP-Servers (`logs_errors`, `logs_query`, `logs_since`, `logs_context`, `logs_stats`, `logs_services`) lesen nur die lokalen Logs der Workbench. Der Pi (`pi-gaming:8080`) ist nur per Hand über `curl` auf `/api/dev/*` erreichbar. Die Lese-Endpunkte kommen mit B-769, die vollständige Log-Auswahl mit B-770.

## Ziel

Dieselben Log-Tools lesen wahlweise einen entfernten Server (Pi oder anderen), sodass Claude und der Entwickler Fehler auf dem Pi ohne SSH und ohne `curl` auswerten.

## Beteiligte und Zielgruppen

Entwickler (🧑) und Claude als Nutzer der MCP-Tools.

## Anforderungen

- Die `logs_*`-Tools nehmen ein optionales Argument `server` (Adresse `host:port` oder ein Kurzname wie `pi`); ohne bleibt das Verhalten lokal wie bisher.
- Bekannte Server (Kurzname → Adresse) stehen in einer Konfiguration der Workbench, Standard `pi` = `pi-gaming:8080`.
- Hinter `server` holt die Workbench die Zeilen über `/api/dev/log` und `/api/dev/logs` (B-770), alle Log-Dateien (Server und Client), und wendet dieselben Filter, Limits und Verdichtung an wie lokal.
- `server_status` und `workbench_status` zeigen den Remote-Server (Erreichbarkeit, Version, Log vorhanden) über `/api/dev/status`.
- Berichte des Servers lesbar über `reports_list`/`report_read` mit `server`.
- Antwort nennt immer die Quelle (lokal oder Adresse).

## Nicht-Ziele

Kein Schreiben auf dem Remote-Server, keine Dienststeuerung (`svc_*`) aus der Ferne, kein Dauer-Streaming.

## Regeln und Einschränkungen

Nur GET gegen die Dev-Endpunkte, nur Adressen aus der Konfiguration oder Heimnetz-Adressen (kein beliebiges Ziel, keine Weiterleitung folgen). Zeitlimit je Anfrage. Tool-Beschreibungen enden mit „Nutze es bei: … Statt: …“ (MCP-Regel). Verdichtete Antworten mit `limit` und Cursor.

## Beispiele

- `logs_errors {server: "pi"}` → Fehlerzeilen des Pi-Servers und der Clients, Quelle `pi-gaming:8080`.
- `logs_query {server: "pi", pattern: "WebGL"}` → Treffer im Client-Log.
- `logs_errors {}` → wie bisher lokal.

## Ausnahme- und Fehlerfälle

- Server nicht erreichbar → Fehlermeldung mit Adresse und Grund, kein Hängen (Zeitlimit).
- Server meldet „Log aus“ oder 403 → die Meldung wird durchgereicht, samt Hinweis (`K3C_LOG_DIR`, Dev-Mode).
- Unbekannter Kurzname oder nicht erlaubte Adresse → abgelehnt, Antwort nennt die gültigen Werte.

## Akzeptanzkriterien

- **AC-01** Test: `logs_*` mit `server` holen von einem Test-Server (httptest) die Zeilen und wenden Filter und Limit an; ohne `server` unverändert lokal.
- **AC-02** Test: Nicht erreichbar, 403, 404 „Log aus“, unbekannter Name und fremde Adresse ergeben jeweils eine klare Meldung.
- **AC-03** Test: `server_status` mit `server` zeigt Version, Laufzeit und Log-Status.
- **AC-04** Abnahme (🧑): `logs_errors {server: "pi"}` zeigt echte Zeilen vom Pi.

## Offene Fragen

- Wo liegt die Liste bekannter Server? Default: Konfigurationsdatei der Workbench, Eintrag `pi` voreingestellt.
- Sollen lokale und entfernte Zeilen zusammen sortierbar sein? Default: nein, je Quelle getrennt.

## Notizen

Abhängig von B-770 (Endpunkt mit allen Logs). Quelle der Anforderung: Nutzer am 2026-11-10.

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009; Projekt LST gesammelt verworfen). Umsetzung aus LG1.1–LG1.5 liegt mit #541 auf `develop`, die Abnahme LG1.6 entfällt.
