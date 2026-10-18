# CI1.1 · Release-Image ohne Dev-Mode

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** offline
- **Branch:** ci1/1-release-image-dev-mode
- **Abhängig von:** –
- **Tickets:** B-273
- **Kriterien:** AC-01

## Ziel

Das Release-Image und `compose.yaml` starten den Server mit Dev-Mode aus (`K3C_DEV=0`), einschaltbar per `K3C_DEV=1`; der Docker-Job der CI prüft `dev=false` in der Startzeile.

## Kontext

- `cmd/k3c-server/main.go` Zeile 78: `dev: os.Getenv("K3C_DEV") != "0"`; nur der Wert `0` schaltet den Dev-Mode aus. Die Startzeile `🚀 Server startet` loggt `dev` (Text: `dev=false`; in Containern ohne Farbe, `NO_COLOR=1`).
- `Dockerfile` setzt die Laufzeit-Umgebung in einer `ENV`-Zeile (`NO_COLOR`, `K3C_DIST`, `K3C_SAVES_DIR`, `K3C_REPORTS_DIR`, `K3C_HTTP_PORT`); `compose.yaml` reicht `K3C_STATUS_TOKEN` durch. Das Pi-Image baut `.github/workflows/release.yml` aus demselben `Dockerfile`.
- Entscheidung 🧑 2026-10-18 (B-273 › Offene Fragen): Dev-Mode im Image standardmäßig aus, am Pi per `K3C_DEV=1` einschaltbar; `task start`/`task dev` am Entwicklerrechner bleiben Dev (Standard von `main.go` unverändert).
- Grenzfall Domäne: `Dockerfile` und `compose.yaml` gehören eigentlich SRV; die Änderung ist zwei Zeilen und bleibt in dieser INF-Session (Hinweis im Ergebnis). Kein Eingriff in `cmd/` oder `engine/`.
- Der Docker-Smoke-Test der CI (Job `docker`, `.github/workflows/ci.yml`) startet den Container ohne Variablen; der Test darf nur ergänzt werden, nichts darin darf gelockert werden.

## Erlaubte Dateien

- `Dockerfile`, `compose.yaml`, `.github/workflows/ci.yml` (nur der Job `docker`)
- Kommentare in `compose.yaml` und `Dockerfile`; Sprint-Dateien von CI1 (Status, Ergebnis), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Änderungen an `cmd/k3c-server/main.go` oder `engine/`; Debug-Overlay im Client (B-098), Dev-Tasten über den Server (B-080), Debug-Panel (B-107); Abdeckung (CI1.2).

## Schritte

1. `Dockerfile`: `K3C_DEV=0` in die `ENV`-Zeile der Laufzeit-Stufe aufnehmen, Kommentar nennt `K3C_DEV=1` zum Einschalten.
2. `compose.yaml`: `K3C_DEV: ${K3C_DEV:-0}` unter `environment`, Kopfkommentar um `K3C_DEV=1 docker compose up -d` ergänzen.
3. CI-Job `docker`: Nach dem Start des amd64-Containers prüfen, dass `docker logs k3c` die Startzeile mit `dev=false` enthält (`grep -q 'dev=false'`); in einem weiteren Lauf mit `-e K3C_DEV=1` muss `dev=true` stehen.
4. `docker compose config` lokal prüfen (falls Docker vorhanden), sonst nur Syntax per Sichtprüfung; `task check` grün.

## Fertig, wenn

- [ ] AC-01 (B-273/AC-01): `Dockerfile` und `compose.yaml` setzen `K3C_DEV` auf `0`; der Docker-Job der CI prüft `dev=false` beim Start ohne Variablen.
- [ ] AC-01 (B-273/AC-02): Mit `K3C_DEV=1` loggt der Container `dev=true` (Prüfung im Docker-Job der CI).
- [ ] AC-01 (B-273/AC-03): `cmd/k3c-server/main.go` ist unverändert, `task start` bleibt Dev (`git diff --stat -- cmd engine` leer).
- [ ] `task check` grün; der Docker-Job der CI nach dem Push grün (Nachweis im Ergebnis des Reviews).

## Prüfen

```bash
task check
git diff --stat -- cmd engine
```

Der Docker-Smoke-Test läuft nur in der CI; lokal nur, wenn Docker verfügbar ist (`docker compose config`).

## Ergebnis

- **AC-01 geprüft (B-273/AC-01, AC-02):** `Dockerfile` setzt `K3C_DEV=0` in der `ENV`-Zeile, `compose.yaml` reicht `K3C_DEV: ${K3C_DEV:-0}` durch; der Docker-Job der CI prüft `dev=false` (Start ohne Variablen) und `dev=true` (neuer Schritt mit `-e K3C_DEV=1`). Die Prüfung läuft nur in der CI (kein Docker lokal); der Nachweis der CI folgt im Review CI1.3.
- **AC-01 geprüft (B-273/AC-03):** `git diff --stat -- cmd engine` leer, `main.go` unverändert, `task start` bleibt Dev.
- `task check` grün (17 s). Grenzfall SRV: `Dockerfile` und `compose.yaml` in dieser INF-Session angefasst, zwei Zeilen.
- Getrenntes Review (Subagent, 2026-10-18): keine Befunde.
- Neue Tickets: keine.
