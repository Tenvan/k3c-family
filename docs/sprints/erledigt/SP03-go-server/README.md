# SP03 · SRV · Go-Server Basis

- **Status:** erledigt
- **Domäne:** SRV
- **Prio:** hoch
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-020, B-027, B-028
- **Start-Commit:** c8a221a
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat (Pauschalauftrag „beide komplett autonom fertig stellen“; N = 5, /api/health neu, ohne Token Diagnose aus)

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

Go-Standardbibliothek (`net/http`, `log/slog`; B-001), Entscheidung 001; Schichtgrenzen aus `docs/arbeitsweise.md`; `release.yml` baut die Artefakte.
Keine neuen Abhängigkeiten. Ausnahmen außerhalb der Domäne (INF): `.github/workflows/ci.yml` (Smoke-Test raus, Docker-Job rein), `.github/workflows/release.yml`, `requirements.md`, `package.json` (Skript `serve:go`). Der Node-Server bleibt für den Online-Modus bis SP08 und wird in SP09 gelöscht.

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

keine. Geklärt von 🧑 (2026-09-30, Chat): N = 5 Sicherungen je Spielstand (B-028); `/api/health` wird neu angelegt
(`GET` → 200 `{"ok":true}`); ohne `K3C_STATUS_TOKEN` ist die Diagnose aus (404, B-027).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP03.1 | `SP03.1-server-basis.md` | Umsetzung | autonom | fertig |
| SP03.2 | `SP03.2-status-sicherung-docker.md` | Umsetzung | autonom | fertig |
| SP03.3 | `SP03.3-review.md` | Review | autonom | fertig |

## Abnahme

- 2026-09-30, leichtes Review über `c8a221a..main` durch zwei Reviewer (Sonnet): Sicherheit/Daten und
  Start/Docker/CI. Kriterien: AC-01 (SP03.1 EXE von Hand, SP03.2 Docker per CI, grün laut 🧑), AC-02 (SP03.1),
  AC-03 und AC-04 (SP03.2).
- Behoben (schwer): SIGTERM beendet sauber (vorher SIGKILL nach `docker stop`); gleichzeitiges Speichern auf denselben
  Slot unter einer Sperre. Behoben (vermutet): CI-Prüfung des Benutzers. Neue Tickets: keine.
