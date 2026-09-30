# 001 · Spiel-Engine auf einem Go-Server, Browser als Client

Stand: 2026-09-30 · Status: **beschlossen** · Backlog: B-001, B-002, B-003, B-005, B-036

## Kontext

- Kern des Spiels ist Couch- **und** Online-Koop gemischt, auch mit mehr als zwei Spielern
  (z. B. zwei auf der Couch an der Xbox, zwei per Handy).
- Mehrere Spiele sollen gleichzeitig laufen (Beispiel: ein 2er-Spiel auf der Xbox, ein 3er-Spiel per Handy im Haus).
- Betrieb **nur im Heimnetz**: zuerst auf dem Windows-PC, später als Docker-Container auf einem Raspberry Pi.
- Heute rechnet der Browser die Simulation (`src/world/sim/`, 1794 Zeilen + 620 Zeilen Tests). Der Online-Modus
  rechnet dieselbe TypeScript-Simulation im Node-Server (`src/online/`). Das war der Machbarkeitsnachweis.
- Deployment soll ohne installierte Laufzeit gehen (einzelne EXE bzw. Docker-Image).
- Der Entwickler hat viel Go-Erfahrung. Eine Diagnose-TUI für den laufenden Betrieb ist gewünscht.

## Optionen

| Option | Bewertung |
|---|---|
| **Node/TypeScript-Server** (Simulation bleibt TS) | keine Portierung; EXE über Node SEA / `bun --compile` möglich. Aber: keine Go-Erfahrung nutzbar, TUI schwächer, Pi-Speicherbedarf höher |
| **Python-Server** | EXE per PyInstaller groß (30–80 MB), langsamer Start, häufig Virenscanner-Fehlalarme; GIL begrenzt parallele Räume; keine Stärke gegenüber Go |
| **Go-Server** | eine statische Binärdatei (Windows-EXE, `linux/arm64` für den Pi), winziges Docker-Image, eine Goroutine pro Raum, Bubble Tea für die TUI, vorhandene Erfahrung |
| Go/**Wails** als Server | Wails baut Desktop-Fenster (WebView), keinen Server ohne Oberfläche. Für Docker ungeeignet, als optionaler Desktop-Starter aber möglich |

## Entscheidung

1. **Go-Server ist die einzige Spiel-Engine** (autoritativ). Er rechnet alle Räume, der Browser schickt nur
   Eingaben und zeichnet Snapshots, lokal wie online. Es gibt danach **keine** Simulation mehr im Browser.
2. **Mehrere Räume** und **mehrere lokale Spieler pro Gerät** sind von Anfang an im Kern (Raum-Verwaltung,
   Protokoll). Die Bedienung dafür (Lobby, Raum wählen) darf später kommen.
3. Aufbau im Repo (ein Go-Modul im Root):

   ```text
   go.mod                    Modul k3c
   data/                     Balancing-JSON (heute src/data), per go:embed im Server, per Import im Client
   engine/sim/               Simulation (Port von src/world/sim), deterministisch, getestet
   engine/level/             Level-Generator (Port von src/world/levelGenerator.ts)
   engine/room/              Räume, Takt, Spieler pro Gerät
   engine/net/               HTTP, WebSocket, Protokoll
   engine/store/             Spielstände, Berichte
   cmd/k3c-server/           ohne Oberfläche: Windows-EXE, Linux, Docker (amd64 + arm64)
   cmd/k3c-tui/              Bubble Tea, spricht nur mit /api/status (lokal oder über das Netz)
   cmd/k3c-desktop/          optional, später: Wails-Fenster um denselben Kern
   ```

4. **Schrittweise Umstellung** statt großem Wurf, abgesichert durch **Golden-Tests**: Die TS-Simulation erzeugt für
   feste Seeds und Eingabe-Folgen Snapshots (JSON), die Go-Simulation muss sie exakt nachbilden.
5. **Feature-Stopp** für `src/world/` ab sofort: nur Fehlerbehebungen, keine neuen Mechaniken, bis die TS-Simulation
   gelöscht ist. Neue Mechaniken entstehen danach direkt in Go.

## Folgen

- Spielen braucht immer den Server, auch reines Couch-Spiel. Im Heimnetz ist das gegeben (EXE oder Pi).
  GitHub Pages zeigt danach nur noch Testseiten (B-032).
- Der Browser braucht Interpolation zwischen Snapshots. Für ein ruhiges Spiel wie K2C reicht das, höchstens die eigene
  Laufbewegung wird lokal vorhergesagt (B-039).
- Zwei Werkzeugketten: `npm run check` für den Client, `go test` + `golangci-lint` für den Server.
  Das Komplexitäts-Budget gilt für beide.
- Portierungs-Fallen, die Golden-Tests abdecken müssen:
  `hashSeed` rechnet über UTF-16-Codeeinheiten (`charCodeAt`), in Go über `utf16.Encode` nachbilden;
  `Math.imul`/`>>> 0` → `uint32`-Arithmetik; Reihenfolge von Fließkomma-Operationen beibehalten;
  Iteration über Objekte (`Object.keys`, Einfügereihenfolge) → in Go feste Reihenfolge statt `map`.
- `server/*.mjs`, `src/online/room.ts`, `src/online/wsServer.ts`, `vite.server.config.ts` und `src/world/` entfallen
  am Ende. Der Smoke-Test aus `ci.yml` wird durch Go-Tests (`httptest`) ersetzt (B-020 entfällt).

## Nachfolgende Entscheidungen

- [002 Protokoll v2](002-protokoll-v2.md) (Geräte, lokale Spieler, Räume, Snapshots, Wiederverbinden) — Sprint SP02
- Raspberry-Pi-Modell und Leistungsziel (B-042)
