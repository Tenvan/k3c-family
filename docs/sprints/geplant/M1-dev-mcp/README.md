# M1 · SRV · Entwickler-MCP-Server

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-046
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Agenten prüfen über Shell-Befehle mit langer Rohausgabe und lesen Xbox-Berichte und Spielstände als rohe Dateien.
ErpApi hat dafür einen eingebauten MCP-Server (`../ErpApi/tools/go/dev-workbench/internal/mcpsrv/`). Ab SP01 gibt es
ein Go-Modul mit `golangci-lint` in der CI.

## Ziel

Agenten haben einen MCP-Server `k3c-dev`, der Prüfungen verdichtet ausführt und Berichte und Spielstände lesbar macht.
Am Ende sichtbar: Claude Code zeigt `k3c-dev` als verbunden; `check_run npm:check` antwortet mit Exit-Code, Dauer und nur
den Fehlerzeilen.

## Beteiligte und Zielgruppen

Entwickler und Coding-Agenten am Entwickler-PC; 🧑 stimmt der neuen Abhängigkeit und dem Transport zu.

## Anforderungen

B-046 › Anforderungen.

## Nicht-Ziele

Räume, Live-Zustand und Simulation (B-047, SP07.3). Dienste starten oder stoppen, Commits, Oberfläche.
Betrieb im Docker oder auf dem Pi.

## Regeln und Einschränkungen

B-046 › Regeln und Einschränkungen. Einschiebbar nach SP01 (braucht Go-Modul und `golangci-lint`), unabhängig von der Engine.

## Beispiele

B-046 › Beispiele.

## Ausnahme- und Fehlerfälle

B-046 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Gerüst: `cmd/k3c-mcp` startet über stdio, der Roundtrip-Test listet alle Tools (B-046/AC-01).
- **AC-02** `check_run` ist fail-closed und verdichtet (B-046/AC-02, B-046/AC-03).
- **AC-03** Berichte und Spielstände sind als verdichteter Text lesbar, Pfade sind begrenzt (B-046/AC-04).
- **AC-04** Instructions, README-Eintrag und `.gitignore` sind da (B-046/AC-05).
- **AC-05** `go test ./...` und `golangci-lint` sind grün, das Budget ist eingehalten (B-046/AC-06).

## Offene Fragen

B-046 › Offene Fragen (Abhängigkeit, Transport, Go auf dem Entwickler-PC; 🧑) – blockieren die Freigabe.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- M1.1 Gerüst `cmd/k3c-mcp` (SDK, stdio, `tools.go` als Katalog, Parameter-Hinweis, Roundtrip-Test) und `check_run` (AC-01, AC-02, AC-05).
- M1.2 `reports_list`, `report_read`, `saves_list`, `instructions.md`, README-Eintrag, `.gitignore` (AC-03, AC-04, AC-05).
- M1.3 🔍 Review (alle).

## Abnahme

–
