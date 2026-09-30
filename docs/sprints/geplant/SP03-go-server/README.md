# SP03 · SRV · Go-Server Basis

- **Status:** geplant
- **Domäne:** SRV
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-020, B-027, B-028
- **Start-Commit:** –

## Ziel

`cmd/k3c-server` ersetzt `server/*.mjs` für Dateien, Spielstände und Berichte. Am Ende sichtbar: Das Spiel läuft von der Windows-EXE und aus dem Docker-Image wie heute mit `npm run serve`.

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben.

- SP03.1 `cmd/k3c-server` + `engine/store`: liefert `dist/` aus, `/api/save`, `/api/report`, `/api/health` wie heute, Konfiguration per Umgebungsvariablen, Tests mit `httptest` (ersetzen den Smoke-Test in `ci.yml`).
- SP03.2 `/api/status` mit Token (B-027), `log/slog`, rotierende Sicherungen (B-028), `Dockerfile` multi-arch, `compose.yaml` mit Volumes; `release.yml` baut Windows-EXE und Linux-arm64.
- SP03.3 🔍 Review.

## Nicht im Sprint

Räume und WebSocket (SP07). Der Online-Modus läuft bis SP08 weiter über den Node-Dev-Server.

## Abnahme

–
