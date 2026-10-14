# SV1.3 · Spielstand „voll ausgebaut“ und Dev-API

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** sv1/3-spielstand-voll-ausgebaut
- **Abhängig von:** SV1.1
- **Tickets:** B-315
- **Kriterien:** AC-01, AC-05, AC-06

## Ziel

Der Server erzeugt im Dev-Mode auf Anfrage aus einem Seed einen Spielstand, in dem alle Bauplätze aller Inselstufen gebaut und voll ausgebaut sind, und legt ihn unter `saves/` ab.

## Kontext

- Seit SV1.1 hat eine neue Insel alle Stufen aus den Biomen. „Alle Level“ = alle Inselstufen eines Raums, nicht alle Biome (Beschluss 🧑 2026-10-06).
- Spielstand-Format: `engine/sim/save.go` (`IslandSave`, `SiteSave` mit `State`, `Level`, `Upgrade`, `HubLevel`). Beim Laden übernimmt `FromIslandSave` `State == "built"` und `Level` bis `len(buildings[kind].Levels)` (Zeile ~279). Hub-Stufen: `engine/sim/hub_level.go` (`siteHubLevel`, `nextLevel`). Mauer- und Turm-Materialstufen hängen am Feld `Upgrade` (ungeprüft, vorher lesen).
- Fallstrick: `buildings` und die Stufenzahlen sind in `engine/sim/` nicht exportiert. Der Raum darf `engine/sim/` nicht ändern. Weg: Insel mit `sim.CreateIsland` anlegen, als Save serialisieren, im Save die Plätze setzen und mit `sim.FromIslandSave` laden; die höchste Stufe aus `data/buildings.json` bzw. `data/hub.json` (`data.Files`) lesen. Reicht das nicht (z. B. Material- oder Bogenstand fehlt), abbrechen und ein SIM-Ticket anlegen.
- Speichern über `engine/store` (`Saves.Store`); `saves/` ist der normale Ort, der Name ist mit Namen ladbar. Nicht mit Präfix `test-` (`room.TestPrefix`), weil der Server die beim Start löscht. Den Namen (Vorschlag `voll-<seed>`) legt diese Session fest und schreibt ihn ins Ergebnis.
- Dev-API: `engine/net/dev.go` (`/api/dev`, POST nur im Dev-Mode, sonst 403), Dev-Mode `Manager.Dev` (`K3C_DEV`). Die neue Aktion gehört dorthin oder als eigener Pfad daneben; der Level-Betrachter (SV1.4) ruft sie auf.
- Deterministisch: nur `engine/rng`, gleicher Seed → gleicher Spielstand (Zeitstempel wie `isl.ID` aus dem Vergleich nehmen).
- Datei ≤ 400 Zeilen, Funktion ≤ 60; neue Datei z. B. `engine/room/fullsave.go`.

## Erlaubte Dateien

- `engine/room/`, `engine/net/`, `engine/store/`
- Planungs-Dateien (`docs/sprints/`, `docs/backlog/`)

## Nicht-Ziele

Knopf im Level-Betrachter (SV1.4), Darstellung der Ausbaustufen im Renderer (B-208), Protokoll-Änderungen, Balancing, alle Biome je Stufe.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Lesen: `engine/sim/save.go`, `engine/sim/hub_level.go`, `data/buildings.json`, `data/hub.json`; klären, welche Felder „voll ausgebaut“ ausmachen.
3. Test zuerst: erzeugter Stand hat jeden Bauplatz aller Stufen `built`, jede Ausbaustufe (Hub-Stufen, Mauer- und Turmstufen) kommt mindestens einmal vor; gleicher Seed → gleiche Bytes (ohne Zeitstempel).
4. Erzeugung in `engine/room/` umsetzen, über `sim.FromIslandSave` gegenprüfen, dass der Stand lädt.
5. Dev-API in `engine/net/` ergänzen (Seed rein, Spielstandname raus); Test: ohne Dev-Mode 403, mit Dev-Mode liegt der Stand im Store. Log 🐛 bzw. 💾.
6. `task check:go` grün.

## Fertig, wenn

- [x] AC-01: Go-Test belegt jeden Bauplatz aller Stufen gebaut und jede Ausbaustufe mindestens einmal (B-315/AC-01).
- [x] AC-01: Go-Test belegt gleicher Seed → gleicher Spielstand (B-315/AC-02).
- [x] AC-05: Go-Test belegt 403 ohne Dev-Mode.
- [x] AC-06: `task check:go` grün.

## Prüfen

```bash
task check:go
```

## Ergebnis

- **Name:** `voll-<seed>` (`room.FullSavePrefix`); Seed 1–40 Zeichen `[A-Za-z0-9_-]`, weil er Teil des Dateinamens wird. Ein gleichnamiger Stand wird überschrieben, wobei der Store wie üblich eine Sicherung anlegt.
- **„Voll ausgebaut“** (`engine/room/fullsave.go`): neue Insel mit allen Stufen (`islandDepths`), jeder Bauplatz `built` mit den HP seiner Stufe. Mauern und Türme laufen reihum über alle Inselstufen durch die Stufen 1–5. Die Hub-Stufe ist auf der ersten Stufe 5 und fällt je Tiefe um eins (5…1), damit jede Hub-Stufe einmal steht und der Start (Tiefe 0) Hub 5 zeigt. Die Stufen kommen aus `data/buildings.json` › `levels` und `data/hub.json` › `levels`, `engine/sim/` bleibt unverändert.
- **AC-01** umgesetzt, geprüft: `engine/room/fullsave_test.go` › `TestVollAusgebauterSpielstand` (fünf Stufen, jeder Platz `built` mit HP > 0, Hub-, Mauer- und Turmstufen 1–5 je mindestens einmal; lädt über `sim.FromIslandSave` mit allen Plätzen `built` und Hub 5 auf der ersten Stufe) und `TestVollAusgebautDeterministisch` (gleicher Seed → gleiche Bytes ohne `savedAt`).
- **AC-05** umgesetzt, geprüft: `POST /api/dev/fullsave {"seed"}` → `{"save": "voll-…"}` (`engine/net/dev.go`). `engine/net/fullsave_test.go` belegt: mit Dev-Mode liegt der Stand im Store, ungültiger Seed → 400, ohne Dev-Mode → 403 und nichts gespeichert; `TestVollAusgebautNurImDevMode` belegt dasselbe für `Manager.FullSave`. Logs 💾, 🐛, 🚫, 💥.
- **AC-06** geprüft: `task check:go` grün (k3c-dev `check_run`: `go:test` und `go:lint` nach Auslagern von `loadsBuilt`, gocyclo).
- Nicht im Stand: Bögen im Turm, Rüstung, Werkstatt-Angebote. B-315 verlangt sie nicht. Kommen sie dazu, braucht es ein SIM-Ticket, weil die Werte in `engine/sim/` nicht exportiert sind.
