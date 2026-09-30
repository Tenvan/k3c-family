# SP03 · SRV · Go-Server Basis

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-020, B-027, B-028
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`server/server.mjs` (Node, ohne Abhängigkeiten) liefert `dist/` aus und speichert Spielstände (`server/saves.mjs`) und Berichte. Der Smoke-Test steckt als Bash in `ci.yml` (B-020).

## Ziel

`cmd/k3c-server` ersetzt `server/*.mjs` für Dateien, Spielstände und Berichte. Am Ende sichtbar: Das Spiel läuft von der Windows-EXE und aus dem Docker-Image wie heute mit `npm run serve`.

## Beteiligte und Zielgruppen

🧑 betreibt den Server am PC; Spieler an Xbox und Handy; Umsetzung durch Entwickler oder Agent.

## Anforderungen

B-020, B-027 und B-028 › Anforderungen. Sprint-eigen: gleiches Verhalten wie heute für die Auslieferung, `/api/save`, `/api/report` und `/api/health`; Konfiguration per Umgebungsvariablen; Windows-EXE und Docker-Image (amd64 + arm64).

## Nicht-Ziele

Räume und WebSocket (SP07). Der Online-Modus läuft bis SP08 weiter über den Node-Dev-Server.

## Regeln und Einschränkungen

Go-Standardbibliothek (`net/http`, `log/slog`), Entscheidung 001; Schichtgrenzen aus `docs/arbeitsweise.md`; `release.yml` baut die Artefakte.

## Beispiele

`k3c-server.exe` starten → das Spiel läuft am TV wie mit `npm run serve`.

## Ausnahme- und Fehlerfälle

Unbekannte Route oder kaputte Spielstand-Datei → Fehlercode und Log, der Server läuft weiter.

## Akzeptanzkriterien

- **AC-01** Windows-EXE und Docker-Image liefern das Spiel aus wie `npm run serve`.
- **AC-02** `/api/save`, `/api/report` und `/api/health` verhalten sich wie heute, geprüft mit `httptest` (B-020/AC-01, B-020/AC-02, B-020/AC-03).
- **AC-03** `/api/status` antwortet nur mit Token (B-027/AC-01, B-027/AC-02).
- **AC-04** Spielstände werden rotierend gesichert und per API wiederhergestellt (B-028/AC-01, B-028/AC-02).

## Offene Fragen

Anzahl N der Sicherungen (B-028, 🧑).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- SP03.1 `cmd/k3c-server` + `engine/store`: liefert `dist/` aus, `/api/save`, `/api/report`, `/api/health` wie heute, Konfiguration per Umgebungsvariablen, Tests mit `httptest` (ersetzen den Smoke-Test in `ci.yml`) (AC-01, AC-02).
- SP03.2 `/api/status` mit Token (B-027), `log/slog`, rotierende Sicherungen (B-028), `Dockerfile` multi-arch, `compose.yaml` mit Volumes; `release.yml` baut Windows-EXE und Linux-arm64 (AC-01, AC-03, AC-04).
- SP03.3 🔍 Review (alle).

## Abnahme

–
