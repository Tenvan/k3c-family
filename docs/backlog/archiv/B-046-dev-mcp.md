# B-046 · Entwickler-Werkzeug k3c-dev gibt Agenten über MCP verdichteten Zugriff auf Prüfungen und Logs

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** M1
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-09-30 🧑 Chat (Revision 2, mit Abhängigkeit und Ausnahmen aus M1)

## Ausgangslage

Agenten prüfen über Shell-Befehle mit langer Rohausgabe (`npm test`, `go test`) und haben keine Logs, in denen sie
nachsehen könnten. Ab SP01 gibt es ein Go-Modul mit `golangci-lint`. Der MCP-Kern ist der erste von vier Sprints (M1–M4)
für das Entwickler-Werkzeug `k3c-dev`: ein Programm mit Oberfläche (Wails), das einen MCP-Server für Agenten hostet und
dessen Aufrufe, Logs und Prüfläufe zeigt. Revision 2 ersetzt den reinen stdio-Server aus Revision 1.

## Ziel

`k3c-dev` bietet Agenten einen MCP-Server über HTTP, der Prüfungen verdichtet ausführt, Logs lesbar macht und jeden
Aufruf zählt und protokolliert. Nutzen: weniger Rohausgabe im Kontext der Agenten, keine freien Shell-Befehle für
Standard-Prüfungen, und die Grundlage für die Oberfläche (B-064, B-065).

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten (Claude Code) am Entwickler-PC (Windows); 🧑 stimmt Abhängigkeiten und Ausnahmen zu.

## Anforderungen

- **Modul:** `tools/k3c-dev/` ist ein eigenes Go-Modul `k3c/tools/k3c-dev` (Go 1.27) mit eigener `.golangci.yml`
  (dieselben Grenzen und Linter wie im Hauptmodul). `main.go` im Modul-Ordner, Pakete unter `internal/`:
  `mcpsrv` (Server, Katalog, Tools, Zähler), `console` (Konsolenpuffer), `logs` (Log-Leser, Verdichtung), `applog`
  (eigenes Log). Kein Import aus dem Hauptmodul; Repo-Wurzel = zwei Ebenen über dem Modul bzw. Arbeitsverzeichnis mit
  `go.mod` `module k3c`, sonst Abbruch mit Meldung.
- **Transport:** offizielles SDK `github.com/modelcontextprotocol/go-sdk`, Streamable HTTP an `127.0.0.1:5180`, Pfad
  `/mcp`; `K3C_DEV_PORT` überschreibt den Port (ungültig → Abbruch mit Meldung). Nur `127.0.0.1`, nie `0.0.0.0`.
  Der Server läuft, solange `k3c-dev` läuft. Neustart des Servers (für die Oberfläche) ohne Programmende möglich.
- **Katalog:** eine einzige Stelle registriert alle Tools mit Name, Beschreibung, Annotations und Handler.
  Wahrheitsgemäße Annotations: `readOnlyHint` für alle Tools außer `check_run` (`destructiveHint: false`).
  Unbekannter Parameter → die Fehlermeldung nennt zusätzlich die gültigen Parameternamen des Tools (Middleware).
  Instructions beim Verbinden aus `internal/mcpsrv/instructions.md` (eingebettet): welches Tool wofür, statt Shell.
- **`check_run {target, pattern?}`:** fester Katalog im Code: `npm:check` → `npm run check`, `npm:test` → `npm test`,
  `npm:typecheck`, `npm:lint`, `npm:build`, `go:test` → `go test ./...`, `go:lint` → `golangci-lint run`,
  `dev:test` → `go test ./...` in `tools/k3c-dev`. Zeitlimit 10 min (`*:test` 5 min); beim Zeitlimit wird der ganze
  Prozessbaum beendet (Windows `taskkill /T /F`, sonst Prozessgruppe). Testmuster nur für `npm:test` (`npm test -- <m>`)
  und `go:test`/`dev:test` (`-run <m>`), geprüft gegen `^[A-Za-z0-9_./:-]{1,100}$`, nie bereinigt. Je Ziel höchstens
  ein Lauf gleichzeitig. Antwort: `<ziel> · exit <n> · <dauer>` und nur die Fehlerzeilen (Muster je Werkzeug: Vitest,
  `tsc`, Oxlint, `go test`, `golangci-lint`, Vite; ANSI entfernt; höchstens 60, dann `… N weitere`); rot ohne erkannte
  Fehlerzeile → die letzten 20 Zeilen. Die volle Ausgabe fließt live in den Konsolenpuffer `check:<ziel>`, der beim
  Start eines neuen Laufs geleert wird. Das Ende jedes Laufs steht im eigenen Log.
- **Konsolenpuffer:** je Quelle ein Ringpuffer (2000 Zeilen) mit Zeile, Strom (`stdout`/`stderr`/`log`) und
  fortlaufender Nummer; Leser blockieren den Kindprozess nie (Benachrichtigung ohne Warten, Zeilen über 64 KB bleiben
  ganz, Text wird gültiges UTF-8). Quellen: `k3c-dev` (Spiegel des eigenen Logs) und `check:<ziel>`.
- **Eigenes Log:** `log/slog` mit JSON-Handler nach `logs/k3c-dev.jsonl` (Repo-Wurzel), eine Zeile je Eintrag:
  `time` (RFC 3339 mit Zone), `level` (`DEBUG`, `INFO`, `WARN`, `ERROR`), `msg`, optional `ns` (Bereich, z. B. `mcp`,
  `check`), weitere Felder = Daten. Geschrieben werden: Start/Stopp, Server-Neustart, jeder fehlgeschlagene Tool-Aufruf,
  jedes Lauf-Ende. Ein nicht beschreibbarer Ordner bricht nichts ab (dann nur Konsolenpuffer, Meldung auf stderr).
  Dasselbe Format schreibt ab SP03 der Go-Server nach `logs/k3c-server.jsonl` (B-066); jede `logs/*.jsonl` ist eine Log-Quelle.
- **Log-Leser:** liest von hinten in Blöcken von 1 MB, höchstens 8 MB je Anfrage (`budget erreicht` im Ergebnis);
  Filter: Mindest-Level, `ns`, regulärer Ausdruck auf `msg`, `since` (Dauer wie `30m` oder Zeitpunkt), Limit.
  Zeilen ohne gültiges JSON werden übersprungen und gezählt. Vorwärts lesen ab Byte-Cursor (`logs_since`), mit
  `gekürzt`, wenn die Datei kleiner wurde oder Budget/Limit ältere Einträge abschnitt.
- **Verdichtung:** gleichartige Meldungen bekommen einen Fingerabdruck, indem in dieser Reihenfolge UUIDs → `<id>`,
  Pfade → `<path>`, Texte in Anführungszeichen → `<s>` und Zahlen → `<n>` ersetzt werden (höchstens 200 Zeichen).
  Gruppen je `ns` + Fingerabdruck mit Anzahl, höchstem Level, erstem und letztem Zeitpunkt und einem Beispiel,
  häufigste zuerst.
- **Log-Tools:** `logs_sources` (alle Quellen: Log-Datei mit Größe und letztem Eintrag, Lauf mit Zustand),
  `logs_query {source, minLevel?, ns?, pattern?, since?, limit?}` (Standard 100, höchstens 500, neueste zuerst,
  eine Zeile je Eintrag `HH:MM:SS LEVEL ns msg`), `logs_errors {source, minLevel?, since?}` (Standard `WARN`, `24h`,
  verdichtet), `logs_since {source, cursor?, limit?}` (neue Einträge und neuer Cursor), `console_tail {source, lines?}`
  (Standard 50, höchstens 500).
- **`workbench_status`:** Adresse, Laufzeit, Aufrufe, Fehler, Clients, letzter Lauf je `check_run`-Ziel (Exit, Dauer,
  Zeitpunkt), Log-Quellen. Kurz, eine Zeile je Punkt.
- **Zähler und Aufruf-Log** (Daten für die MCP-Seite, B-065): Startzeit; Aufrufe und Fehler gesamt; je Tool Aufrufe,
  Fehler, Ø Dauer, letzter Aufruf; Clients aktuell und höchstens; parallel laufende Aufrufe aktuell und höchstens.
  Aufruf-Log als Ring der letzten 200 Aufrufe: `id`, `ref` (7 Hex-Zeichen), `startedAt`, `ts` (Ende), `startSeq` und
  `endSeq` (eine gemeinsame Folge über alle Start- und End-Ereignisse), `running`, `tool`, `args` (JSON, 120 Zeichen),
  `durationMs`, `ok`, `error`, `summary` (erste Antwortzeile, 120 Zeichen). Laufende Aufrufe sind sichtbar. Rückrufe
  bei Start und Ende eines Aufrufs (für B-062 und B-065). Panik in einem Handler → Fehler
  `Tool-Aufruf durch Panik abgebrochen`, der Server läuft weiter. Alle Zähler sind nebenläufig sicher.
- **Start in M1:** `go run .` in `tools/k3c-dev` startet den Server ohne Fenster und endet mit Strg+C (ab B-064 im Fenster).

## Nicht-Ziele

Oberfläche (B-064, B-065); Statistik über Sitzungen, Perzentile, Ausreißer, Zeitreihe (B-062); Berichte und
Spielstände (B-063); Dienste starten oder stoppen; Commits; Räume und Simulation (B-047); stdio-Transport;
Betrieb im Docker oder auf dem Pi.

## Regeln und Einschränkungen

Komplexitäts-Budget und Review aus `docs/arbeitsweise.md`. Standardbibliothek zuerst; neue Abhängigkeiten nur die hier
genannten (Freigabe 🧑). `check_run` bleibt fail-closed: neue Ziele nur per Code-Änderung. `.mcp.json` bleibt lokal
(`.gitignore`), der README zeigt den Eintrag
`{ "mcpServers": { "k3c-dev": { "type": "http", "url": "http://127.0.0.1:5180/mcp" } } }`.
`logs/` steht in `.gitignore`. Unter Windows heißt `npm` `npm.cmd`.

## Beispiele

- `check_run {"target":"npm:test"}` grün → `npm:test · exit 0 · 12,4 s`.
- `check_run {"target":"npm:test","pattern":"planning"}` rot → Kopfzeile und nur die fehlschlagenden Tests mit Datei und Meldung.
- `logs_errors {"source":"k3c-dev"}` → `3× ERROR mcp check_run: Ziel <s> unbekannt · 13:02–13:40`.
- `logs_query {"source":"k3c-dev","limit":2}` → zwei Zeilen, dazu `2 Einträge · 4,1 KB gelesen`.

## Ausnahme- und Fehlerfälle

- Ziel nicht im Katalog → Ablehnung mit allen gültigen Zielen, kein Prozess startet.
- Testmuster mit verbotenen Zeichen oder bei einem Ziel ohne Muster → Ablehnung, kein Prozess startet.
- Ziel läuft schon → Ablehnung `läuft bereits`, der laufende Lauf bleibt unberührt.
- Zeitlimit → Prozessbaum beendet, Antwort mit den bisherigen Fehlerzeilen und `Zeitlimit`.
- Unbekannte Log-Quelle → Ablehnung mit allen gültigen Quellen; fehlende Datei → `noch keine Einträge`.
- Ungültiger regulärer Ausdruck → Ablehnung mit der Meldung von `regexp`.
- Port belegt → Meldung mit Port und Hinweis auf `K3C_DEV_PORT`, Programm endet (ab B-064: Oberfläche zeigt den Fehler).

## Akzeptanzkriterien

- **AC-01** `tools/k3c-dev` ist ein eigenes Modul; `go run .` startet den MCP-Server an `127.0.0.1:5180/mcp`; ein
  Roundtrip-Test (In-Memory-Transport) listet alle Tools mit Beschreibung und Annotations und zeigt, dass ein
  unbekannter Parameter die gültigen Namen nennt.
- **AC-02** `check_run` führt nur Katalog-Ziele aus; unbekanntes Ziel, verbotene Zeichen, Muster am falschen Ziel und ein
  zweiter gleichzeitiger Lauf werden abgelehnt, ohne einen Prozess zu starten (Tests).
- **AC-03** `check_run` liefert Kopfzeile und höchstens die Fehlerzeilen (Tests mit echten Beispielausgaben je
  Werkzeug), ein grüner Lauf ist eine Zeile, das Zeitlimit beendet den Kindprozess, die Ausgabe steht im Konsolenpuffer.
- **AC-04** Zähler und Aufruf-Log zählen je Tool, Fehler, Clients und parallele Aufrufe richtig, der Ring hält 200
  Aufrufe mit Start-/End-Folge, eine Panik wird zum Fehler (Tests).
- **AC-05** `k3c-dev` schreibt `logs/k3c-dev.jsonl`; `logs_sources`, `logs_query`, `logs_errors`, `logs_since` und
  `console_tail` liefern verdichteten Text mit Budget und Cursor; die Verdichtung maskiert in der festgelegten
  Reihenfolge (Tests mit Beispieldateien in `testdata/`).
- **AC-06** Instructions kommen beim Verbinden; README beschreibt Start und `.mcp.json`; `.mcp.json` und `logs/`
  stehen in `.gitignore`; `npm run check:dev` und der CI-Job für `tools/k3c-dev` sind grün, die Grenzen sind eingehalten.

## Offene Fragen

keine (Transport, Ablage und Schnitt entschieden von 🧑 am 2026-09-30 im Chat; Abhängigkeit und Ausnahmen stehen in
der Sprint-README M1 zur Freigabe).

## Notizen

Revision 2 (2026-09-30, 🧑 im Chat): Werkzeug mit Oberfläche statt stdio-Server; Logs-Seite zeigt Läufe und
JSON-Logs; eigenes Modul `tools/k3c-dev/`; vier Sprints M1–M4. Folge-Tickets: B-062, B-063, B-064, B-065, B-066, B-047.
