# NG1.1 · Modul native/k3c-game und Vertrag view

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** ng1/1-modul-vertrag
- **Abhängig von:** NV3
- **Tickets:** B-792
- **Kriterien:** AC-01, AC-02

## Ziel

Das Go-Modul `native/k3c-game` baut, und das Paket `view` legt den Vertrag fest, an den NG2–NG8 ihre Pakete hängen; jedes Fachpaket existiert als Platzhalter.

## Kontext

Spec: B-792 (Anforderungen, Offene Fragen mit den Entscheidungen 🧑 vor der Freigabe). Vorbilder: `native/k3c-ebiten/go.mod` (Ebitengine v2.10.6, `replace k3c/native/k3c-rig => ../k3c-rig`), `native/k3c-ebiten/main.go`, `native/k3c-rig/RIG.md`, `native/k3c-rig/paint`. Simulation: `k3c/engine/sim` (Typ `World`, `PlayerCommand`), `k3c/engine/room` (`Peer`, `Manager.Snapshot`), Wurzel-Modul `k3c` (`go.mod` an der Wurzel, kein `go.work`). Fachpakete: `world`, `actors`, `props`, `input`, `hud`, `fx`, `menu`, `audio`. Ebenen-Reihenfolge: fern, mitte, nah, Boden, Bauten, Figuren, Fluss, Effekte, HUD (je Zelle), Overlay (je Fenster). Dieser Vertrag ist nach NG1 eingefroren: so vollständig schneiden, dass NG2–NG8 ihn nicht ändern müssen (Eingabe-Typen, `Screen`, `Session`, Ereignisse des Takts). Ein Platzhalter-`Drawer` je Fachpaket zeichnet farbige Rechtecke. Kein `math/rand`. Superpowers TDD.

## Erlaubte Dateien

- `native/k3c-game/main.go`
- `native/k3c-game/go.mod`
- `native/k3c-game/go.sum`
- `native/k3c-game/view/**`
- `native/k3c-game/game/**`
- `native/k3c-game/world/**`
- `native/k3c-game/actors/**`
- `native/k3c-game/props/**`
- `native/k3c-game/input/**`
- `native/k3c-game/hud/**`
- `native/k3c-game/fx/**`
- `native/k3c-game/menu/**`
- `native/k3c-game/audio/**`
- `native/Taskfile.yml`
- Planung NG1, B-792 (über MCP)

## Nicht-Ziele

Simulation anbinden (NG1.2), Grafik laden (NG1.3), Fenster mit Zellen (NG1.4); Änderungen unter `engine/` oder an den Prototypen; Module außer den von 🧑 freigegebenen.

## Schritte

1. Modul anlegen: `go.mod` mit Ebitengine-Version wie `native/k3c-ebiten`, `replace k3c => ../..`, `replace k3c/native/k3c-rig => ../k3c-rig`; freigegebene Zusatz-Module für Text und Ton eintragen.
2. `view`: Frame, Cell, Camera, Layer, Drawer, Eingabe-Typen (Befehle je Spieler, Menü-Aktionen), Screen, Session; Paket-Doku als Vertrag.
3. Platzhalter je Fachpaket mit Konstruktor, den `game` aufruft.
4. Test der Importregel (Fachpakete importieren nur `view`, `art`, `k3c/engine/sim`, `k3c/data`; nie einander), z. B. per `go list -deps` oder `go/packages`.
5. Tasks `native:game:build` und `native:game` in `native/Taskfile.yml` (ohne cgo, `sources`/`generates`).

## Fertig, wenn

- [ ] AC-01: `task native:game:build` baut `native/k3c-game/bin/k3c-game.exe` mit `CGO_ENABLED=0`; `go.mod` nennt Ebitengine v2.10.6 und beide `replace`.
- [ ] AC-02: `go test ./view/ ./game/` grün, der Importregel-Test schlägt bei einem künstlichen Querimport fehl (im Test belegt).
- [ ] Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
