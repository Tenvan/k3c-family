# NV1.1 · Bevy-Prototyp bauen und auf dem Entwicklungs-PC messen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** opus
- **Umgebung:** live
- **Branch:** sprint/nv1
- **Abhängig von:** –
- **Tickets:** B-776
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05

## Ziel

`native/k3c-proto/` (Rust, Bevy `=0.20.1`) zeigt ein Wald-Level aus Papier-Sheets mit zwei per Gamepad gesteuerten Monarchen im Split-Screen; `task native:measure` schreibt FPS und Grafikspeicher nach `reports/`.

## Kontext

Daten und Begründung: Projekt NAT › Notizen, B-775, B-776. Sheets erzeugt `task gen:paper` nach `public/grafik/<gruppe>/<asset>/` (`manifest.json`: `frameW`, `frameH`, `animationen{name: {frames, fps, datei}}`); Gruppen-Index `public/grafik/<gruppe>/index.paper.json`; Rollen → Assets in `data/paper.json`. Rust-Toolchain liegt unter `~/.cargo/bin` (nicht im PATH der Shell). Der Cargo-Workspace `rust/` ist für die FFI-DLL und bleibt unberührt.

## Erlaubte Dateien

- `native/**`
- `Taskfile.yml` (nur die Einbindung `native:`)
- `.gitignore` (nur `native/**/target/`)
- Planung NAT, B-775, B-776, NV1 (über MCP)

## Nicht-Ziele

Simulation, Netz, HUD, Menüs; Änderungen an `src/`, `engine/`, `rust/`.

## Schritte

1. Cargo-Projekt mit gepinntem Bevy, Taskfile `native/Taskfile.yml`.
2. Manifeste laden, Sheets als Texture-Atlas, Animation nach fps.
3. Level aufbauen (Hintergrund, Gebäude, Truppen, Gegner, zwei Monarchen mit Reittier).
4. Zwei Gamepads + Tastatur, Split-Screen.
5. Messmodus und Messskript, Referenzmessung, Werte in B-776 › Notizen.

## Fertig, wenn

- [x] AC-01: `task native:proto:build` grün (Bevy `=0.20.1`, `--locked`, keine Warnungen).
- [x] AC-02: Screenshot des Levels geprüft (Wald, 3 Parallax-Schichten je Spieler, Gebäude, Natur, Truppen, Gegner, zwei berittene Monarchen).
- [ ] AC-03: Zwei echte Pads nicht geprüft (keine zwei Pads am Entwicklungs-PC); Tastatur-Ersatz und Auto-Fahrt laufen. Prüfung durch 🧑 am Mini-PC.
- [x] AC-04: Bericht mit `level` und `alle` unter `reports/`.
- [x] AC-05: Referenzwerte in B-776 › Notizen.

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

`native/k3c-proto` (Rust, Bevy 0.20.1, ≈ 600 Zeilen) zeigt das Wald-Level im Split-Screen; `task native:measure` misst beide Lastfälle, `task native:proto:paket` baut das Paket für den Mini-PC. Entwicklungs-PC (RX 6700 XT, Vulkan): `level` 343 FPS / 224 MiB GPU, `alle` 323 FPS / 996 MiB GPU. Offen: AC-03 mit echten Pads und AC-06 (Mini-PC) bei 🧑.
