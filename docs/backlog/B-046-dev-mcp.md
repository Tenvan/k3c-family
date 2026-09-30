# B-046 · Entwickler-MCP-Server gibt Agenten verdichteten Zugriff auf Prüfungen, Berichte und Spielstände

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** M1
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Agenten prüfen über Shell-Befehle mit langer Rohausgabe (`npm test`, `go test`) und lesen Xbox-Berichte (`reports/*.json`) und Spielstände (`saves/`) als rohe Dateien. In ErpApi löst ein eingebauter MCP-Server dasselbe Problem (`../ErpApi/tools/go/dev-workbench/internal/mcpsrv/`): verdichtete Antworten, `check_run` mit Whitelist, Instructions beim Verbinden.

## Ziel

Entwickler-MCP-Server gibt Agenten verdichteten Zugriff auf Prüfungen, Berichte und Spielstände. Nutzen: Weniger Rohausgabe im Kontext der Agenten, keine freien Shell-Befehle für Standard-Prüfungen, Xbox-Berichte ohne JSON-Wühlen.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten (Claude Code) am Entwickler-PC; 🧑 stimmt der neuen Abhängigkeit zu.

## Anforderungen

- Eigenes Programm `cmd/k3c-mcp` in Go mit dem offiziellen SDK `github.com/modelcontextprotocol/go-sdk`, Transport stdio: Claude Code startet es selbst über `.mcp.json`.
- `check_run {target, pattern?}`: fester, von Hand gepflegter Katalog (`npm:check`, `npm:test`, `npm:typecheck`, `npm:lint`, `npm:build`, `go:test`, `go:lint`); Antwort mit Exit-Code, Dauer und nur den Fehlerzeilen; ein optionales Testmuster wird gegen verbotene Zeichen geprüft und abgelehnt, nie bereinigt.
- `reports_list` und `report_read {name}`: Berichte aus `reports/*.json` verdichtet (Datum, Gerät, Controller, Tasten, FPS je Sprite-Stufe).
- `saves_list`: Spielstände aus `saves/` mit Stufe, Tag, Spielern und Datum.
- MCP-`instructions` aus `cmd/k3c-mcp/instructions.md`: welches Tool wofür, statt Shell.
- Wahrheitsgemäße Tool-Annotations: `readOnlyHint` für alle Tools außer `check_run`.
- Unbekannter Parameter → die Fehlermeldung nennt die gültigen Namen (Muster `param_hint.go`).
- Eine Datei je Thema (`tools_check.go`, `tools_reports.go`, `tools_saves.go`) mit Test daneben.

## Nicht-Ziele

HTTP-Transport; Aufrufstatistik und Call-Log (in ErpApi für die Oberfläche); Dienste starten oder stoppen; Commits; Räume und Live-Zustand (B-047); Betrieb im Docker oder auf dem Pi.

## Regeln und Einschränkungen

Go-Server ist die einzige Engine (Entscheidung 001), Standardbibliothek zuerst; Schichtgrenzen und Komplexitäts-Budget aus `docs/arbeitsweise.md`. Muster ErpApi `mcpsrv` (`tools.go` als einziger Katalog, `tools_check.go` fail-closed, `instructions.md`, `roundtrip_test.go`). Bewusste Abweichung beim Transport: dort lebt der Server in einer GUI ohne stdio, hier ist er ein eigenes Kommandozeilen-Programm; so bleiben `cmd/k3c-server` und das Docker-Image frei von Entwickler-Code. Neue Ziele für `check_run` nur per Code-Änderung mit Review. Pfade nur innerhalb von `reports/` und `saves/`. `.mcp.json` bleibt lokal und steht in `.gitignore`, der README zeigt den Eintrag.

## Beispiele

- `check_run {"target":"npm:test"}` bei grünem Lauf → eine Zeile wie `npm:test · exit 0 · 12,4 s`.
- `check_run {"target":"npm:test","pattern":"planning"}` mit Fehler → Exit-Code, Dauer und nur die fehlschlagenden Tests mit Datei und Meldung.
- `reports_list` → neueste Berichte zuerst, eine Zeile je Bericht.

## Ausnahme- und Fehlerfälle

- Ziel nicht im Katalog → Ablehnung mit allen gültigen Zielen, kein Prozess startet.
- Testmuster mit `;`, `&`, `|`, `$`, Backtick oder Anführungszeichen → Ablehnung, kein Prozess startet.
- `report_read` mit `..` oder einem Namen außerhalb von `reports/` → Ablehnung.
- Kaputte JSON-Datei in `reports/` → Zeile „nicht lesbar“, die anderen Berichte bleiben sichtbar.
- Lauf überschreitet das Zeitlimit → der Prozessbaum wird beendet, die Antwort enthält die bisherigen Fehlerzeilen.

## Akzeptanzkriterien

- **AC-01** `cmd/k3c-mcp` startet über stdio; ein Roundtrip-Test (In-Memory-Transport des SDK) listet alle Tools mit Beschreibung und Annotations.
- **AC-02** `check_run` führt nur Ziele aus dem Katalog aus; unbekanntes Ziel und verbotene Zeichen im Muster werden abgelehnt, ohne einen Prozess zu starten (Tests).
- **AC-03** `check_run` liefert Exit-Code, Dauer und höchstens die Fehlerzeilen; ein grüner Lauf ist eine Zeile (Test mit Beispielausgabe).
- **AC-04** `reports_list`, `report_read` und `saves_list` liefern verdichteten Text; Pfade außerhalb von `reports/` und `saves/` werden abgelehnt (Tests mit Beispieldateien in `testdata/`).
- **AC-05** Beim Verbinden liefert der Server `instructions.md` aus; der README beschreibt den `.mcp.json`-Eintrag; `.mcp.json` steht in `.gitignore`.
- **AC-06** `go test ./...` und `golangci-lint` sind grün, das Komplexitäts-Budget ist eingehalten.

## Offene Fragen

Zustimmung zur Abhängigkeit `github.com/modelcontextprotocol/go-sdk` (🧑; aktuelle Version beim Umsetzen prüfen, ErpApi nutzt v1.7.0). Transport stdio statt HTTP bestätigen (🧑). Ist Go auf dem Entwickler-PC installiert? Sonst startet `.mcp.json` die gebaute EXE.

## Notizen

Vorbild: `../ErpApi/tools/go/dev-workbench/internal/mcpsrv/` (`codemap.md` dort). Fallstricke unter Windows: `npm` heißt `npm.cmd`; beim Zeitlimit den ganzen Prozessbaum beenden (ErpApi: Job-Object in `internal/proc`). Folge-Ticket: B-047.
