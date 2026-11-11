# NAT · Native Umstellung (Rust + Bevy, lokal, ohne Server)

- **Status:** aktiv
- **Rang:** 12
- **Ziel-Tickets:** B-775

## Ziel

K3C läuft als native Windows-Anwendung in **Rust + Bevy** auf dem Mini-PC (Vulkan-GPU), lokal für 1–2 Spieler mit je einem Gamepad, **ohne Server und ohne Browser**. Erledigt, wenn das Spiel auf dem heutigen Stand (Regeln, Grafik, HUD, Menüs, Spielstände) nativ läuft und Balancing-Läufe ohne Go-Simulation gehen. Gesamt-Spec: B-775. **Vorläufig** (angelegt 2026-11-10): Weg und Umfang entscheidet 🧑 nach dem Prototyp NV1.

## Sprints

In Abarbeitungs-Reihenfolge; der erste nicht erledigte Sprint ist der nächste. Jeder Sprint nennt das Projekt im Feld `Projekt`.

| Sprint | Thema | Status |
|---|---|---|
| NV1 | Bevy-Prototyp: Papier-Sheets, zwei Gamepads, Messung | aktiv |

## Nicht-Ziele

- Browser-Client, Xbox/Edge und Online-Spiel über den Go-Server (bleiben bis zur Umstellung, wie sie sind).
- Neue Regeln oder Balancing-Änderungen während der Portierung: nur portieren, Ideen als Ticket.
- Rust-Server (RST, Entscheidung 007 Entwurf): entfällt bei lokalem Spiel; ob RST ruht, entscheidet 🧑.

## Notizen

### Gesammelte Daten (2026-11-10, develop 98dfed8e, read-only gezählt)

- **Code ohne Tests:** Client `src/` 19,9k Zeilen TS (scenes 9,5k, Werkzeug-Seiten 3,3k, core 2,4k, online 1,3k, stress 1,1k). Go `engine/` 28,5k: sim 10,4k, balance 7,2k, room 3,2k, net 3,1k, level 1,7k, store 0,7k; Go-Tests in `engine/` 33k. `tools/paper` 2,9k TS, Workbench 30k Go + 22k TS/TSX, `rust/` 2,6k.
- **Trennung:** `engine/sim` importiert nur `data`, `core`, `level`, `rng`; `engine/room` kein Netz; nur `engine/net` kennt WebSocket. Der Client hat keine Simulation (`src/scenes/noSim.test.ts`).
- **Rust heute:** `rust/k3c-core` (rng, Münzen, Level per FFI) wird nur in Tests aufgerufen (`engine/sim/coins_spike_test.go`, `engine/level/rust_test.go`); das Spiel rechnet mit Go. RS1: Sim ≈ 2 % der Raum-Tick-Zeit, 97,7 % im JSON-Roundtrip – der fällt lokal ganz weg.
- **Grafik:** 569 PNG-Sheets in `public/grafik`, 105 MB auf der Platte, **757 MiB RGBA** wenn alle geladen; größtes Sheet 2776×288. `manifest.json` (frameW/H, animationen{frames, fps, datei}) ist engine-neutral. Xbox/Edge verlor WebGL bei 214–347 MiB Texturen.
- **Übertragbar ohne Änderung:** `data/*.json` (288 KB), Papier-Specs, `tools/paper`, Sheets + Manifeste, `testdata/golden/` (Schiedsrichter für die Portierung), Planung und Assets der Workbench.
- **Tempo:** Repo seit 2026-09-29; letzte 30 Tage 257 Commits, +62,5k Zeilen in `engine/` und `src/`.

### Schätzung (Einschätzung, nicht gemessen)

Sim + Level + Spielstände portieren gegen Goldens 1–2 Wochen · Bevy-Client Welt/Kamera/2 Pads 1–2 Wochen · HUD/Menüs/Lobby 1–2 Wochen · Balancing-Werkzeuge und Workbench-Anbindung ca. 1 Woche → **ca. 4–6 Wochen**. Risiken: Bevy-API ändert sich je Version (Version pinnen, Doku über context7), Abnahmen am Mini-PC, Umbau-Versuchung während der Portierung.

### Reihenfolge (Vorschlag)

1. NV1 Prototyp (Darstellung + Eingabe, Messung) → Entscheidung 🧑.
2. Goldens aus Go breit erzeugen, dann Sim/Level/Store portieren, jede Stufe gegen die Goldens.
3. Client: Welt, Kamera/Split-Screen, Eingabe; dann HUD, Menüs, Spielstände.
4. Balancing-Läufe und Workbench (`sim_run`, `level_generate`) auf den Rust-Kern.
5. Abnahme-Ticket des Projekts (Sim, Balance, Gen).
