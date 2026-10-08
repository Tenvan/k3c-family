# B-273 · Das Release-Image startet den Server ohne Dev-Mode

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** CI1
- **Projekt:** REL
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Server schaltet den Dev-Mode an, solange `K3C_DEV` nicht `0` ist (`cmd/k3c-server/main.go`, `dev: os.Getenv("K3C_DEV") != "0"`). Weder `Dockerfile` noch `compose.yaml` setzen `K3C_DEV=0`, das Pi-Image aus `release.yml` nimmt also Dev-Aktionen an (Gold, Stufe, Zeitraffer, Pause über `engine/room/dev.go`). Gefunden im Probelauf der Release-Checkliste (RL1.2), Punkt „Dev-Reste aus“.

## Ziel

Ein Server aus dem Release-Image lehnt Dev-Aktionen ab; Spielende am Pi können nicht versehentlich Gold, Stufe oder Zeit verändern. Der Punkt „Dev-Reste aus“ der Release-Checkliste wird grün.

## Beteiligte und Zielgruppen

Familie am Pi (spielt), 🧑 (betreibt den Pi und entscheidet, ob der Dev-Mode dort an sein darf).

## Anforderungen

- Das Docker-Image (bzw. `compose.yaml`) startet den Server mit Dev-Mode aus; einschalten bleibt per Umgebungsvariable möglich (`K3C_DEV=1 docker compose up -d`).
- `task dev` / `task start` am Entwicklerrechner behalten den Dev-Mode an.

## Nicht-Ziele

Debug-Overlay im Client (B-098), Dev-Tasten über den Server (B-080), Debug-Panel (B-107).

## Regeln und Einschränkungen

Aufgaben über `task`; Logging mit Emoji (Startzeile nennt `dev`); Docker-Smoke-Test der CI muss grün bleiben.

## Beispiele

`docker compose up -d` am Pi → Startzeile `dev=false`, eine Dev-Aktion wird mit „🚫 Dev-Aktion abgelehnt“ geloggt. `K3C_DEV=1 docker compose up -d` → Dev-Aktionen wirken.

## Ausnahme- und Fehlerfälle

Ungültiger Wert für `K3C_DEV` → Verhalten wie heute in `main.go` (nur `0` schaltet aus), im Image aber Standard `0`.

## Akzeptanzkriterien

- **AC-01** Ein Container aus `docker compose up -d` ohne weitere Variablen loggt beim Start `dev=false` und lehnt eine Dev-Aktion mit „🚫 Dev-Aktion abgelehnt“ ab.
- **AC-02** Mit `K3C_DEV=1` ist der Dev-Mode im Container an.
- **AC-03** `task start` am Entwicklerrechner startet weiter mit Dev-Mode an.

## Offene Fragen

Soll der Dev-Mode am Pi während der Entwicklungsphase bewusst an bleiben (wie das Overlay nach B-093 Revision 2)? Entscheidet 🧑.

## Notizen

Quelle: RL1.2, Probelauf 2026-10-04.
