# tools/k3c-workbench/internal/botfeed/

## Responsibility

Bot-Feed der Workbench (B-348, TR1.3): ein WebSocket je Client eines Testlaufs unter `/bot/<lauf>/<client>`, über den die Workbench die Kommandos ihrer Bots an die lokalen Spieler des Clients schickt. Nur Loopback-Verbindungen werden angenommen; was der Client sendet, wird verworfen.

## Design

- `Cmd`/`CmdOf`: Nachricht eines Slots, Felder wie `sim.PlayerCommand`.
- `Feed`: Verbindungen eines Clients (nach Neuladen kurz zwei); `Send` stellt je Slot eine Nachricht an alle Verbindungen zu (Schreib-Timeout, nicht annehmende Verbindung wird geschlossen), `Connected` (jetzt/je), `Sent` (Zähler), `closeAll` außerhalb des Locks.
- `Hub`: Feeds aller laufenden Läufe nach Name `<lauf>/<client>`; `Open`, `Close`, `ServeHTTP` (Loopback-Prüfung über `Loopback`, nur für offene Feeds, Origins localhost/127.0.0.1 erlaubt, weil die Client-Seite auf dem Vite-Port liegt).

## Flow

1. Lauf ruft `Hub.Open(name)` und gibt die Feed-Adresse (`Prefix` + name) an den Client (`botfeed=`-Parameter).
2. Client verbindet -> `ServeHTTP` -> `Feed.add`.
3. Im Takt des Bots `Feed.Send(cmds)`; Ende des Laufs `Hub.Close`.

## Integration

- Konsumenten: `internal/mcpsrv/server.go` (`feeds`, Route `botfeed.Prefix` am Mux), `simtest_clients.go` (`clientSeat`, `feedTotals`).
- Abhängigkeiten: `engine/sim`, `github.com/coder/websocket`.
