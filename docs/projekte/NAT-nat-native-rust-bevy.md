# NAT · Native Umstellung (Go + Ebitengine, lokal, ohne Server)

- **Status:** aktiv
- **Rang:** 1
- **Ziel-Tickets:** B-775

## Ziel

K3C läuft als native Windows-Anwendung in **Go + Ebitengine** auf dem Mini-PC (Entscheidung 008), als **lokal laufender Monolith**: ein Prozess für 1–2 Spieler mit je einem Gamepad, **ohne Server, ohne Netz, ohne Browser**. Die Go-Simulation läuft im selben Programm; Rust entfällt komplett. Zielbild der Grafik sind Rig-Modelle statt PNG-Sheets (zuerst Monarch und Reittier, B-778; weitere Figuren nach B-780), `gen:paper` schrumpft dabei Figur für Figur. Erledigt, wenn das Spiel auf dem heutigen Stand (Regeln, Grafik, HUD, Menüs, Spielstände) nativ läuft. Gesamt-Spec: B-775.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| NV1 | Bevy-Prototyp: Papier-Sheets, zwei Gamepads, Messung | aktiv |
| NV2 | Rig-Prototyp: Monarch aus Teilen in Ebitengine | aktiv |

## Nicht-Ziele

- Browser-Client, Xbox/Edge und Online-Spiel über den Go-Server (bleiben bis zur Umstellung, wie sie sind).
- Neue Regeln oder Balancing-Änderungen während der Umstellung: nur umstellen, Ideen als Ticket.
- Rust und Bevy (Entscheidung 008): Projekt RST ruht und wird mit B-779 geschlossen.

## Notizen

### Gesammelte Daten (2026-11-10, develop 98dfed8e, read-only gezählt)

- **Code ohne Tests:** Client `src/` 19,9k Zeilen TS (scenes 9,5k, Werkzeug-Seiten 3,3k, core 2,4k, online 1,3k, stress 1,1k). Go `engine/` 28,5k: sim 10,4k, balance 7,2k, room 3,2k, net 3,1k, level 1,7k, store 0,7k; Go-Tests in `engine/` 33k. `tools/paper` 2,9k TS, Workbench 30k Go + 22k TS/TSX, `rust/` 2,6k.
- **Trennung:** `engine/sim` importiert nur `data`, `core`, `level`, `rng`; `engine/room` kein Netz; nur `engine/net` kennt WebSocket. Der Client hat keine Simulation (`src/scenes/noSim.test.ts`).
- **Rust heute:** `rust/k3c-core` (rng, Münzen, Level per FFI) wird nur in Tests aufgerufen (`engine/sim/coins_spike_test.go`, `engine/level/rust_test.go`); das Spiel rechnet mit Go. RS1: Sim ≈ 2 % der Raum-Tick-Zeit, 97,7 % im JSON-Roundtrip – der fällt lokal ganz weg.
- **Grafik:** 569 PNG-Sheets in `public/grafik`, 105 MB auf der Platte, **757 MiB RGBA** wenn alle geladen; größtes Sheet 2776×288. `manifest.json` (frameW/H, animationen{frames, fps, datei}) ist engine-neutral. Xbox/Edge verlor WebGL bei 214–347 MiB Texturen.
- **Übertragbar ohne Änderung:** `data/*.json` (288 KB), Papier-Specs, `tools/paper`, Sheets + Manifeste, `testdata/golden/` (Schiedsrichter für die Portierung), Planung und Assets der Workbench.
- **Tempo:** Repo seit 2026-09-29; letzte 30 Tage 257 Commits, +62,5k Zeilen in `engine/` und `src/`.

### Schätzung (Einschätzung, nicht gemessen)

Mit Go + Ebitengine entfällt die Portierung von Sim, Level und Spielständen. Rig-Prototyp Monarch (NV2) einige Tage · Ebitengine-Client Welt/Kamera/2 Pads 1–2 Wochen · HUD/Menüs/Lobby 1–2 Wochen · Balancing-Werkzeuge und Workbench bleiben in Go → **ca. 2–4 Wochen**. Risiken: Grafikspeicher mit allen Sheets (1330 MiB im Prototyp, deshalb Texturen je Level und Monarch als Rig), Abnahmen am Mini-PC, Umbau-Versuchung während der Umstellung.

### Reihenfolge (Entscheidung 008, 2026-11-11)

1. NV1 Prototypen (Bevy, Ebitengine) → Entscheidung 🧑: **Go + Ebitengine**, Rust entfällt. Offen: Messung am Mini-PC (NV1.3).
2. NV2 Rig-Prototyp: Monarch aus Teilen (B-778) → Entscheidung 🧑 über den Weg für den Monarchen.
3. Rust und Bevy entfernen (B-779), nach der Messung am Mini-PC.
4. Client: Welt, Kamera/Split-Screen, Eingabe mit der Go-Simulation im selben Prozess; dann HUD, Menüs, Spielstände.
5. Abnahme-Ticket des Projekts (Sim, Balance, Gen).
