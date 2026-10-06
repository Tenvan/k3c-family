# B-078 · `task dev` leitet `/ws` an den Go-Server weiter

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP09
- **Erstellt:** 2026-10-01
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („SP09 freigegeben“), Revision 2

## Ausgangslage

Der Vite-Dev-Server (`vite.config.ts`) hängt den Node-Raumserver (`src/online/wsServer.ts`, Protokoll v1) an `/ws`. Ab SP08
spricht der Browser Protokoll v2 mit dem Go-Server; im Dev-Server käme er aber beim v1-Server an und bekäme `version`.

## Ziel

Der Browser im Dev-Server (`task dev`, Hot Reload) erreicht den Go-Server unter `/ws` und `/api/`. Nutzen: Entwickeln am Client
ohne jedes Mal zu bauen (`task serve:go`).

## Beteiligte und Zielgruppen

Entwickler und Agenten (CLI-Sessions); 🧑 entscheidet.

## Anforderungen

- Vite leitet `/ws` (WebSocket) an den Go-Server (Port 8080) weiter; der v1-Server im Dev-Server entfällt.
- `task dev` meldet, wenn der Go-Server nicht läuft, statt still zu scheitern (optional ein Dienst in k3c-dev).

## Nicht-Ziele

Löschen des Node-Servers (SP09.2 erledigt beides in einer Session).

## Regeln und Einschränkungen

Domäne INF (`vite.config.ts`, `Taskfile.yml`); Befehle nur über `task`.

## Beispiele

`task dev` und `task serve:go` laufen → `http://<LAN-IP>:5173/game.html` verbindet sich mit dem Go-Server.

## Ausnahme- und Fehlerfälle

Go-Server läuft nicht → Hinweis im Terminal, der Browser zeigt „Server nicht erreichbar“.

## Akzeptanzkriterien

- **AC-01** Ein WebSocket-Test gegen den Dev-Server erreicht den Go-Server (`hello` → `welcome`).

## Offene Fragen

keine (SP09.2: der k3c-dev-Dienst „Heimnetz“ startet den Go-Server per `task start`, `/api` und `/ws` laufen über den Vite-Proxy).

## Notizen

Entstanden beim Bereitmachen von SP08. Bis dahin: Client mit `task serve:go` ausprobieren.
