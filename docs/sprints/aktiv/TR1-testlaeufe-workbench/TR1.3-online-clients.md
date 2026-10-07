# TR1.3 · `sim_test` online mit 1–4 Clients: Bot-Feed und Browser

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** tr1/3-online-clients
- **Abhängig von:** TR1.2, TR2.1 (Bot-Eingabe im Client, B-349) für den Nachweis im Browser
- **Tickets:** B-348
- **Kriterien:** AC-04, AC-05

## Ziel

`sim_test` mit `mode: online`, `clients: 1`–`4` startet die Clients als installierten Browser headless oder hängt sich per Raumcode an laufende Clients. Über den Bot-Feed der Workbench steuern Bots die Monarchen der lokalen Spieler. FPS und Latenz der Clients stehen im Status.

## Kontext

- Bot-Feed: WebSocket der Workbench (eigene Route am MCP-HTTP-Server, nur Loopback) je Lauf; Nachricht laut B-349 › Notizen (`{"slot":0,"moveX":…}` wie `sim.PlayerCommand`). Die Entscheidung trifft der Bot aus TR1.2. Den Zustand liest der Bot über ein eigenes Beobachter-Gerät im selben Raum oder über den Zustand, den der Client mitschickt; welcher Weg gilt, entscheidet die Session nach dem Code und begründet es im Ergebnis.
- Client öffnen: `game.html?room=<Code>&players=<n>&botfeed=<Adresse>` am Vite-Port des Worktrees (`svc_status`). Browser: Edge oder Chrome aus der Installation mit `--headless=new` (Pfad über eine feste Liste bekannter Orte), Prozess über `internal/proc`; keine neue Abhängigkeit. `attach: <Raumcode>` nutzt laufende Clients (z. B. Xbox) ohne Start.
- Client-Kennzahlen: Diagnose-Zeilen, die der Client über `/api/clientlog` schickt (FPS, Latenz, Puffer), aus dem Log `k3c-client` seit Laufbeginn.
- Firewall: Proben nur auf Loopback (Memory „Firewall-Abfrage vermeiden“).

## Erlaubte Dateien

- `tools/k3c-dev/internal/` (Bot-Feed, Browser-Start, Runner, Tests), `tools/k3c-dev/internal/mcpsrv/server.go` (Route des Feeds)
- `docs/sprints/aktiv/TR1-testlaeufe-workbench/` (Status), `docs/backlog/` (Status, neue Tickets)

## Nicht-Ziele

Client-Code (TR2), Messung auf Xbox oder TV (Mensch), neue Client-Kennzahlen (CLI-Ticket, falls nötig).

## Schritte

1. Branch, `Status: in Arbeit`. B-349 und den Stand von TR2.1 prüfen.
2. Bot-Feed-Route und Runner `clients` 1–4 (Start oder Anhängen), Kennzahlen aus dem Client-Log.
3. Tests: Fake-Client (Go-WebSocket) bekommt Kommandos je Slot; kein Browser → Ablehnung mit Grund; Feed nur Loopback.
4. Nachweis im Browser nur, wenn TR2.1 fertig ist und 🧑 die Prüfung freigegeben hat; sonst `verschoben` mit Grund.
5. `task check:dev`, Ergebnis, `Status: fertig`.

## Fertig, wenn

- [x] AC-04: Test mit Fake-Client: Kommandos kommen je Slot an, Status zeigt FPS und Latenz aus dem Client-Log; Nachweis im Browser nach TR2.1.
- [x] AC-05: Test: `clients` > 0 ohne Browser oder ohne Vite-Dienst lehnt mit einer Zeile Grund ab.

## Prüfen

```bash
task check:dev
```

Manuelle Prüfung im Browser nur mit Freigabe durch 🧑 für diesen Lauf.

## Ergebnis

- **AC-04** umgesetzt, geprüft mit `TestSimTestClientsFakeClient` (Fake-Client in Go: 2 Clients × 2 Spieler bekommen Kommandos je Slot 0 und 1; Status zeigt Feed, FPS, Latenz, Puffer aus dem Client-Log) und `botfeed`-Tests (Nachricht je Slot, nur Loopback, Origin vom Vite-Port). **Nachweis im Browser verschoben:** TR2.1 (B-349) ist noch nicht umgesetzt, und der Client schreibt heute keine FPS-/Latenz-Zeile ins Client-Log → neues Ticket B-351 (CLI).
- **AC-05** umgesetzt, geprüft mit `TestSimTestClientsAblehnung`: ohne Browser, ohne Vite, `players` 4, `rooms` mit Clients, `attach` falsch → je eine Zeile Grund.
- **Zustand für den Bot:** über ein Beobachter-Gerät (`botdev`) je Client-Raum, nicht über den Client. Grund: Das Protokoll v5 kennt keinen Beitritt ohne Platz (`room.ValidSlots`), und der Zustand über den Feed bräuchte eine Erweiterung von B-349 (Client-Code, nicht diese Domäne). Folge: Der Beobachter belegt einen Platz und spielt als Bot mit, daher mit Clients `players` 1–3; Slot k des Clients ist sein k-ter Monarch nach Index. Zuschauer ohne Platz und Feed im LAN (Xbox-`attach`) → B-352.
- Neu: `internal/botfeed` (Route `/bot/<lauf>/<n>`), `internal/browser` (Edge/Chrome aus fester Liste, `--headless=new`, eigenes Profil unter `.work/`), `mcpsrv/simtest_clients.go`, `simtest_clientlog.go`; `attach: <Raumcode>` (nur `clients: 1`). Review (code-reviewer): zwei schwere Funde (doppeltes `Wait`, Beobachter nicht gestoppt) und zwei kleinere behoben.
- `task check:dev` grün (2026-10-07, im Worktree per Shell: der MCP-Header `X-K3C-Root` kam nicht an, `workbench_status` zeigte die Repo-Wurzel, B-341). Der Worktree basierte auf `main`; Branch neu von `origin/develop` angelegt.
