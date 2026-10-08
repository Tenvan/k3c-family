# B-067 · k3c-dev startet, überwacht und stoppt die Entwicklungs-Dienste, auch für Agenten

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** M3
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (M3 Revision 1 mit B-067, gopsutil v4.26.8 und x/sys)

## Ausgangslage

Den Vite-Dev-Server (`npm run dev`, Port 5173) und den Heimnetz-Server (`npm start`, Port 8080) startet man von Hand
im Terminal; ab SP03 kommt der Go-Server `k3c-server` dazu. Agenten starten sie mit freien Shell-Befehlen, verlieren
die Ausgabe und lassen Prozesse übrig. `k3c-dev` (B-046) hat einen Konsolenpuffer und Log-Tools, führt aber keine Dienste.

## Ziel

`k3c-dev` startet, überwacht und stoppt die Entwicklungs-Dienste, auch für Agenten. Nutzen: ein Ort für Start, Stopp,
Zustand und Ausgabe; Agenten brauchen dafür keine Shell, und kein Dienst bleibt verwaist zurück.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten am Entwickler-PC (Windows); die Dienste-Seite (B-068) als Oberfläche.

## Anforderungen

- **Konfiguration** `tools/k3c-dev/services.json`, eine Liste ohne Modus. Je Dienst: `name`, `command` (Programm und
  Argumente als Liste, kein Shell-String), `cwd` (relativ zur Repo-Wurzel), `port`, `health` (`http` = `GET
  http://127.0.0.1:<port>/` antwortet mit 2xx/3xx, `tcp` = Port nimmt Verbindungen an), optional `log` (Name einer
  Log-Quelle `logs/<log>.jsonl`), `autoRestart` (ja/nein), `env` (zusätzliche Variablen). Anfangs:
  `Vite` (`npm run dev -- --strictPort`, 5173, `http`, Auto-Restart; ohne `--strictPort` wiche Vite bei belegtem
  Port still auf 5174 aus) und `Heimnetz` (`npm start`, 8080, `http`); `k3c-server` trägt
  SP03 nach (B-066). Unbekannte Felder, doppelte Namen oder Ports und fehlende Pflichtfelder → Fehler mit Pfad und Feld.
- **Zustände:** `gestoppt`, `startet`, `läuft`, `übernommen`, `stoppt`, `fehlgeschlagen`. Start: Prozess ohne
  Konsolenfenster, dann Health-Prüfung alle 1 s bis höchstens 60 s → `läuft`, sonst `fehlgeschlagen` mit Grund. Im
  Lauf Prüfung alle 10 s; drei Fehlschläge in Folge oder Prozessende → `fehlgeschlagen`, mit `autoRestart` neu starten
  (höchstens 3 Neustarts je 10 min, Zähler sichtbar). Stopp beendet den ganzen Prozessbaum (Paket `internal/proc`, dasselbe wie für `check_run` aus B-046; unter
  Windows über ein Job Object, damit auch verwaiste Enkel enden).
- **Ein Steuerpunkt:** Alle Befehle (Oberfläche und MCP) laufen durch einen Controller, der je Dienst serialisiert; ein
  zweiter Start eines laufenden Dienstes ist ein Fehler, kein zweiter Prozess. Zustandswechsel gehen als Rückruf hinaus
  (für die Oberfläche) und ins eigene Log (B-046). `Alle starten` startet parallel, `Alle stoppen` in umgekehrter
  Reihenfolge der Konfiguration.
- **Übernahme:** Beim Start von `k3c-dev` gilt ein Dienst, dessen Health-Prüfung schon besteht, als `übernommen`; die
  PID kommt über den Port (Verbindungstabelle des Systems). Übernommene Dienste haben keine Konsolenausgabe und keinen
  Auto-Restart; besteht ihre Prüfung nicht mehr, werden sie `gestoppt` (Grund `übernommener Prozess nicht mehr
  erreichbar`). Stopp beendet den Prozessbaum dieser PID nur mit ausdrücklicher Bestätigung (`force`).
- **Port belegt** beim Start durch einen fremden Prozess → `fehlgeschlagen` mit `Port 8080 bereits belegt (PID 8812)`.
- **Metriken** mit `github.com/shirou/gopsutil/v4` alle 2 s für laufende und übernommene Dienste: PID, CPU in %,
  Speicher (RSS), Laufzeit.
- **Konsole:** stdout und stderr jedes selbst gestarteten Dienstes fließen in den Konsolenpuffer unter seinem Namen;
  `logs_sources` und `console_tail` (B-046) kennen die Dienste damit automatisch, die Logs-Seite (B-064) auch.
- **Log-Level-Zähler:** Für Dienste mit `log` zählt der Log-Leser (B-046) die Einträge je Level der letzten 60 min
  (für die Karte in B-068 und `svc_status`).
- **Beenden:** Beim Beenden von `k3c-dev` werden selbst gestartete Dienste in umgekehrter Reihenfolge gestoppt,
  übernommene bleiben.
- **MCP-Tools** im Katalog aus B-046: `svc_status` (alle Dienste, eine Zeile je Dienst: Zustand, Port, PID, CPU,
  Speicher, Laufzeit, Neustarts, letzter Fehler, Log-Level der letzten 60 min; `readOnlyHint`), `svc_start {service}`,
  `svc_stop {service, force?}`, `svc_restart {service}` (`destructiveHint: false` für Start, `true` für Stopp und
  Neustart). Die Antwort nennt den neuen Zustand, bei Fehlern den Grund und die letzten 10 Konsolenzeilen.
  Instructions (B-046) nennen die Tools und dass Agenten Dienste nie per Shell starten.

## Nicht-Ziele

Modus-Umschalter dev/production; Build-Schritte vor dem Start; Dienste auf anderen Rechnern oder im Docker;
Dienste über die Oberfläche anlegen oder ändern; Einstellung „Dienste beim Beenden weiterlaufen lassen“.

## Regeln und Einschränkungen

Neue Abhängigkeiten nur `gopsutil/v4` und `golang.org/x/sys` als direkte (heute schon indirekte) für das Job Object (Freigabe 🧑). Kein Shell-Aufruf: `command` wird direkt gestartet (unter Windows
`npm.cmd`). Test-Nähte für Prozessstart, Health-Prüfung und Uhr, damit die Zustandsmaschine ohne echte Prozesse getestet
wird. Komplexitäts-Budget aus `docs/arbeitsweise.md`.

## Beispiele

- `svc_start {"service":"Vite"}` → `Vite · läuft · Port 5173 · PID 41232 · 3,1 s bis gesund`.
- Vite stürzt ab → `fehlgeschlagen`, Auto-Restart, `Neustarts: 1`; Karte und `svc_status` zeigen es.
- `k3c-dev` startet, während `npm start` schon im Terminal läuft → `Heimnetz · übernommen · PID 22140`.

## Ausnahme- und Fehlerfälle

- Unbekannter Dienst → Ablehnung mit allen gültigen Namen.
- `svc_stop` auf einen übernommenen Dienst ohne `force` → Ablehnung mit Hinweis auf `force`.
- Health-Prüfung nach 60 s nicht bestanden → Prozessbaum beendet, `fehlgeschlagen`, letzte Konsolenzeilen in der Antwort.
- Konfiguration fehlt oder ist kaputt → keine Dienste, Fehler mit Pfad und Ursache (Oberfläche und `svc_status`);
  die übrigen Tools laufen weiter.
- Mehr als 3 Neustarts in 10 min → kein weiterer Auto-Restart, `fehlgeschlagen` bleibt stehen.

## Akzeptanzkriterien

- **AC-01** Die Konfiguration wird geladen und geprüft (Tests für gültig, unbekanntes Feld, doppelter Name/Port,
  fehlendes Pflichtfeld).
- **AC-02** Die Zustandsmaschine startet, prüft, meldet Fehler und startet mit Grenze neu (Tests mit gestelltem Start,
  gestellter Health-Prüfung und Uhr).
- **AC-03** Übernahme per Port und `Port belegt` werden erkannt; Stopp eines übernommenen Dienstes braucht `force` (Tests).
- **AC-04** Metriken kommen aus `gopsutil` (Test gegen einen gestarteten Kindprozess).
- **AC-05** `svc_status`, `svc_start`, `svc_stop`, `svc_restart` stehen im Katalog, lehnen unbekannte Dienste ab und
  antworten verdichtet (Roundtrip-Tests).
- **AC-06** Dienst-Ausgabe steht im Konsolenpuffer und in `logs_sources`; Log-Level-Zähler der letzten 60 min (Tests).
- **AC-07** Von Hand: `svc_start Vite` startet den Dev-Server, `svc_stop Vite` beendet ihn samt Kindprozessen;
  `npm run check:dev` ist grün.

## Offene Fragen

keine (Liste ohne Modus, Metriken mit gopsutil, Backend vor der Oberfläche: 🧑 2026-09-30 im Chat).

## Notizen

Wunsch von 🧑 (2026-09-30, Chat) während M1.1. Die Dienste-Seite ist B-068.
