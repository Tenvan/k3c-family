# RU2.3 · Demo-Szene mit einfachem Driver

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/ru2
- **Abhängig von:** RU2.1, RU2.2
- **Tickets:** B-1016
- **Kriterien:** AC-03

## Ziel

Erster lauffähiger Client: `k3c-game demo --driver <datei>` zeichnet eine Papierschnitt-Welt, durch die die Kamera reitet; Figuren laufen nach einem einfachen Skript (Driver), nicht nach der Sim.

## Kontext

Baut auf `k3c-platform` (RU2.1) und `art` (RU2.2). Wie das Go-Spiel Ebenen, Kamera und Figuren zeichnet, zeigen `native/k3c-game/world/backdrop.go`, `camera.go`, `layers.go` und `native/k3c-game/actors/anim.go`; davon nur Bildaufbau und Parallaxe übernehmen, kein vollständiger Port. Welt-Koordinaten in Units (1 Unit = 32 px). Zwischenbilder: Lage vom Takt davor merken, in `draw` mit alpha mischen (wie `native/k3c-game/game/blend.go`).

Driver-Format (JSON, Beispiel `rust/demo/reiten.json`): `{"laenge": 400, "biom": "<id>", "ebenen": [{"asset": "…", "parallaxe": 0.3}], "boden": "…", "gebaeude": [{"x": 20, "asset": "…"}], "figuren": [{"asset": "…", "anzahl": 40, "tempo": 2.5, "von": 0, "bis": 400}], "kamera": {"tempo": 7.0, "figur": "…"}}`. Figuren laufen zwischen `von` und `bis` hin und her (Spiegelung beim Wenden), die Kamera folgt einer gerittenen Figur mit Monarch-Tempo aus `data/monarch.json`.

## Erlaubte Dateien

- `rust/crates/k3c-game/src/demo/**`
- `rust/crates/k3c-game/src/main.rs`
- `rust/demo/**`

## Nicht-Ziele

Sim, Eingabe außer Esc (Ende) und F (Vollbild), HUD, Ton, zwei Bildhälften.

## Schritte

1. Driver-Typen (serde) und `World`-Zustand je Takt (Kamera-x, Figuren-x, Richtung, Animationsphase).
2. `step`: Bewegung je Takt (30 Hz), Animationsphase aus Takt-Zähler.
3. `draw`: Ebenen mit Parallaxe, Boden gekachelt, Gebäude, Figuren; nur Sichtbares (Culling); Positionen mit alpha gemischt.
4. CLI: `demo --driver`, `--sprites N` (Zusatzlast: N kleine bewegte Sprites aus einem Figuren-Asset), `--measure S`, `--headless --steps N`, `--fullscreen`.
5. `rust/demo/reiten.json` mit Assets einer echten Welt aus `data/worlds` (Biom, Gebäude, Bürger, Truppen).
6. Tests: Driver lesen, Wende an den Enden, headless 600 Schritte.

## Fertig, wenn

- [ ] AC-03: `cargo run --release -p k3c-game -- demo --driver rust/demo/reiten.json` zeigt die Welt, die Kamera reitet durch, Figuren laufen animiert.
- [ ] AC-03: `--headless --steps 600` endet mit Code 0.
- [ ] AC-03: `cargo test -p k3c-game` grün.

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
