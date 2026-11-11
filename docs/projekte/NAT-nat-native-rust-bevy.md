# NAT · Native Umstellung (Rust + Bevy, lokal, ohne Server)

- **Status:** aktiv
- **Rang:** 12
- **Ziel-Tickets:** B-775

## Ziel

K3C läuft als native Windows-Anwendung in **Go + Ebitengine** auf dem Mini-PC, lokal für 1–2 Spieler mit je einem Gamepad, **ohne Server und ohne Browser**; die Go-Simulation (`engine/sim`) läuft unverändert im selben Programm. Erledigt, wenn das Spiel auf dem heutigen Stand (Regeln, Grafik, HUD, Menüs, Spielstände) nativ läuft. Gesamt-Spec: B-775. **Weg:** 🧑 hat sich am 2026-11-11 nach den Prototypen NV1 für Go + Ebitengine ausgesprochen („ich denke, wir bleiben bei Go“); die Messung am Mini-PC steht noch aus. Der Projekttitel nennt noch den ersten Ansatz Rust + Bevy.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| NV1 | Bevy-Prototyp: Papier-Sheets, zwei Gamepads, Messung | aktiv |

## Nicht-Ziele

- Browser-Client, Xbox/Edge und Online-Spiel über den Go-Server (bleiben bis zur Umstellung, wie sie sind).
- Neue Regeln oder Balancing-Änderungen während der Umstellung: nur umstellen, Ideen als Ticket.
- Rust: keine Portierung der Simulation, kein Bevy-Client. Ob RST (Rust-Engine, Entscheidungen 006/007) damit ruht, entscheidet 🧑.
- Entity-System (ECS) im Client: Die Simulation hält den Zustand, der Client zeichnet ihn; Szenen (Menü, Lobby, Spiel) als kleine eigene Lösung oder Bibliothek.

## Notizen

### Gesammelte Daten (2026-11-10, develop 98dfed8e, read-only gezählt)

- **Code ohne Tests:** Client `src/` 19,9k Zeilen TS (scenes 9,5k, Werkzeug-Seiten 3,3k, core 2,4k, online 1,3k, stress 1,1k). Go `engine/` 28,5k: sim 10,4k, balance 7,2k, room 3,2k, net 3,1k, level 1,7k, store 0,7k; Go-Tests in `engine/` 33k. `tools/paper` 2,9k TS, Workbench 30k Go + 22k TS/TSX, `rust/` 2,6k.
- **Trennung:** `engine/sim` importiert nur `data`, `core`, `level`, `rng`; `engine/room` kein Netz; nur `engine/net` kennt WebSocket. Der Client hat keine Simulation (`src/scenes/noSim.test.ts`).
- **Rust heute:** `rust/k3c-core` (rng, Münzen, Level per FFI) wird nur in Tests aufgerufen (`engine/sim/coins_spike_test.go`, `engine/level/rust_test.go`); das Spiel rechnet mit Go. RS1: Sim ≈ 2 % der Raum-Tick-Zeit, 97,7 % im JSON-Roundtrip – der fällt lokal ganz weg.
- **Grafik:** 569 PNG-Sheets in `public/grafik`, 105 MB auf der Platte, **757 MiB RGBA** wenn alle geladen; größtes Sheet 2776×288. `manifest.json` (frameW/H, animationen{frames, fps, datei}) ist engine-neutral. Xbox/Edge verlor WebGL bei 214–347 MiB Texturen. Der Monarch allein: 210 Sheets, 387 MiB (51 %), weil jede Kombination Gangart × Reittier × Spielerfarbe einzeln gerendert ist (B-778).
- **Übertragbar ohne Änderung:** `data/*.json` (288 KB), Papier-Specs, `tools/paper`, Sheets + Manifeste, `testdata/golden/` (Schiedsrichter für die Portierung), Planung und Assets der Workbench.
- **Tempo:** Repo seit 2026-09-29; letzte 30 Tage 257 Commits, +62,5k Zeilen in `engine/` und `src/`.

### Prototypen NV1 (2026-11-11, Entwicklungs-PC RX 6700 XT, kein Mini-PC)

| | Bevy 0.20.1 (Vulkan) | Ebitengine 2.10.6 (DirectX 11) |
|---|---|---|
| FPS ein Level / alle Sheets | 399 / 364 | 5238 / 5146 |
| GPU-Speicher ein Level / alle | 224 / 996 MiB | 104 / 1330 MiB |
| EXE, Bauzeit | 93 MB, Minuten | 14 MB, 7,7 s |
| Go-Simulation | muss portiert werden | läuft unverändert im selben Programm |

Einzelheiten: B-776, B-777 › Notizen. Beide reichen für das Spiel weit aus; der Bevy-Prototyp ist nicht optimiert. Mini-PC-Werte und echte Gamepads fehlen noch (🧑).

Effekte des heutigen Clients (Parallaxe, Fluss mit Spiegelung, Lichtkarte, Partikel, Tweens) sind in beiden Engines umsetzbar; Partikel und Tweens bringt Phaser fertig mit, nativ sind sie eigener Code oder Zusatzpaket.

### Schätzung (Einschätzung, nicht gemessen)

- **Rust + Bevy:** Sim + Level + Spielstände portieren gegen Goldens 1–2 Wochen · Client Welt/Kamera/2 Pads 1–2 Wochen · HUD/Menüs/Lobby 1–2 Wochen · Balancing-Werkzeuge und Workbench-Anbindung ca. 1 Woche → **ca. 4–6 Wochen**.
- **Go + Ebitengine:** Sim bleibt, Netz weg → **ca. 2–4 Wochen**.
- Dazu in beiden Fällen B-778 (Monarch als Gliederpuppe) ca. 1 Woche.
- Risiken: Bevy-API ändert sich je Version (Version pinnen, Doku über context7), Abnahmen am Mini-PC, Umbau-Versuchung während der Portierung.

### Reihenfolge (Vorschlag)

1. NV1 Prototypen (Darstellung + Eingabe, Messung) → Messung am Mini-PC → Entscheidung 🧑 (Entscheidung 008).
2. Nur bei Rust: Goldens aus Go breit erzeugen, dann Sim/Level/Store portieren, jede Stufe gegen die Goldens.
3. Client: Welt, Kamera/Split-Screen, Eingabe.
4. B-778: Monarch, Reittier, Rüstung, Waffe als Gliederpuppe (vor HUD und Menüs).
5. HUD, Menüs, Spielstände.
6. Balancing-Läufe und Workbench (`sim_run`, `level_generate`) auf den neuen Kern.
7. Abnahme-Ticket des Projekts (Sim, Balance, Gen).
