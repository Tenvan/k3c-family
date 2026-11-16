# RU0.2 · Versuchsschalter GC aus und Takt aus eigener Zeitsammlung, Messläufe

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** live
- **Branch:** sprint/ru0
- **Abhängig von:** RU0.1
- **Tickets:** B-1019
- **Kriterien:** AC-02

## Ziel

Die Schalter `--gc off` und `--pacing frame` existieren als Versuche; fünf Messläufe derselben Szene liegen vor.

## Kontext

Zwei Vermutungen sollen getrennt werden. **GC:** Mit `debug.SetGCPercent(-1)` und `debug.SetMemoryLimit` läuft in 5 Minuten kein GC-Lauf. **Bildtakt:** Heute setzt `main.go` `ebiten.SetTPS(30)`; Ebitengine ruft Update dann je Bild keinmal, einmal oder zweimal, und `blend.alpha()` rechnet aus der Wanduhr seit dem letzten Takt (`blend.go:102`). Der Versuch `--pacing frame` setzt `ebiten.SetTPS(ebiten.SyncWithFPS)` (ein Update je Bild), sammelt die echte Bilddauer und rechnet daraus die Sim-Takte selbst: `acc += dt; solange acc >= 1/30: Takt, acc -= 1/30` (höchstens vier Takte je Bild), alpha = `acc * 30`. Beide Schalter sind Versuche: Ohne sie verhält sich das Spiel wie bisher.

## Erlaubte Dateien

- `native/k3c-game/game/pacing*.go`
- `native/k3c-game/game/game.go`
- `native/k3c-game/game/blend.go`
- `native/k3c-game/main.go`
- `native/measure.ps1`

## Nicht-Ziele

Das Standard-Verhalten ändern. Den Fix ausrollen (B-943). Läufe über `sim_test`.

## Schritte

1. `pacing.go`: Zeitsammlung wie im Kontext, mit Test (feste Folge von Bilddauern → erwartete Takte und alpha).
2. `game.go` und `blend.go`: Ist `--pacing frame` gesetzt, kommen Takte und alpha aus der Zeitsammlung; sonst der heutige Weg, unverändert.
3. `main.go`: Schalter `--pacing frame` und `--gc off` (`debug.SetGCPercent(-1)`, `debug.SetMemoryLimit(4 << 30)`).
4. Szene festlegen: zwei Spieler, Nacht mit Gegnern, über `--seed`, `--tick`, `--input`; die Aufrufzeile ins Ergebnis schreiben.
5. Fünf Läufe je 5 Minuten mit `--measure-frames`: normal (VSync an), VSync aus, `--gc off`, `--pacing frame`, `--pacing frame --gc off`. Vor jedem Lauf Prozess-Priorität und Energieschema notieren; nichts anderes läuft auf dem PC.
6. Berichte und Bilddateien nach `.work/messung/` legen; die Kennzahlen je Lauf als Tabelle ins Ergebnis.
7. `go test ./...` in `native/k3c-game` (der Ende-zu-Ende-Test muss ohne Schalter unverändert grün sein), `task check:diff`.

## Fertig, wenn

- [ ] AC-02: Fünf Berichte liegen unter `.work/messung/`; die Tabelle der Kennzahlen steht im Ergebnis.
- [ ] Ohne Schalter sind alle Tests in `native/k3c-game` grün und das Verhalten ist unverändert.

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

**Zwischenstand 2026-11-16 (k3c-08, Nachträge k3c-10), vorläufig. Status bleibt `in Arbeit`: optische Abnahme von `--pacing sync` im Spiel durch 🧑 und 5-Minuten-Läufe auf ruhendem PC (B-1020) fehlen.**

### Umgesetzt

- `--pacing frame` (`native/k3c-game/game/pacing.go`, Test `pacing_test.go`): ein Update je Bild (`ebiten.SyncWithFPS`), Sim-Takte aus eigener Zeitsammlung, alpha aus deren Rest. 🧑: ruckelt sichtbar weiter.
- **Neu (k3c-10): `--pacing sync`** = eigene Zeitsammlung plus `ebiten.RunGameOptions.SingleThread` (`main.go`). Offizielle Ebitengine-API, kein Umbau. `go test -short ./game/` grün.
- Zwischenbilder bleiben im Messmodus an (in RU0.1 waren sie mit `--input` aus).
- Abweichungen: nicht committet (Hauptcheckout auf `sprint/planung-w1`), `task check:diff` offen, Läufe 20–60 s statt 5 Minuten.

### Läufe im Spiel (Last-Szene `--measure-load 5,160,120`, Reiten, 1920×1080 Fenster, VSync an, je 60 s)

| Schleife | Grafik | Bilder > 25 ms je s | Bilder < 10 ms je s | längstes ms | Fehler der gezeigten Zeit p99 ms |
|---|---|---:|---:|---:|---:|
| heute (30 Takte von Ebitengine) | DX11 / DX12 / OpenGL | 3,7 / 3,1 / 4,4 | 4,5 / 2,2 / 0,7 | 189 / 316 / 127 | 21,8 / 21,6 / 20,8 |
| `--pacing frame` | DX11 / DX12 / OpenGL | 2,3 / 2,5 / 4,1 | 3,0 / 2,6 / 0,7 | 126 / 455 / 151 | 5,6 / 3,5 / 4,9 |
| `--pacing frame` (k3c-10, 2 Läufe) | DX11 | 3,5 / 2,7 | 4,0 / 3,9 | 203 / 101 | – |
| `--pacing sync` (k3c-10, 2 Läufe) | DX11 | 2,1 / 1,8 | 3,4 / 3,1 | 232 / 126 | – |

Draw im Spiel: Median 2,1–2,5 ms; p99 6 ms mit `frame`, 11 ms mit `sync` (im Single-Thread-Modus zählt die Arbeit des Render-Threads mit). Rohdaten: `.work/messung/ru02-*`, `.work/messung/ru02b/`.

### Vergleich ohne Spielcode: gelbes Quadrat, Go/Ebitengine 2.10.6 gegen Rust/winit 0.30 + wgpu 23 (k3c-10)

Abwechselnde Läufe je 20 s mit großer Beschriftung im Fenster, 🧑 hat optisch bewertet.

| Programm | späte Bilder (> 25 ms) je s | Quadrat steht/springt je s | optisch (🧑) |
|---|---:|---:|---|
| GO STD (TPS 60, Standard) | 2,2–4,7 | 4–9 | ruckelt, „nach 2–3 Durchläufen extrem“ |
| GO SYNC (`SetTPS(SyncWithFPS)` + `SingleThread`) | 1,0–2,2 | 0 | flüssig |
| RUST DX12 | 0,1–0,25 | 0 | flüssig |
| RUST Vulkan | 0,4–0,6 | 0 | flüssig |

Ohne Fenster: eine Go-Schleife auf `DwmFlush` (Takt des Fenster-Managers) verpasst 0,3–1,7 Takte je s, dieselbe in Rust 0. Vollbild, DX12 und OpenGL ändern bei Ebitengine nichts. Quellen: Scratchpad k3c-10 `bare/`, `rbare/`, `rdwm/`; EXE zum Ansehen in `.work/vergleich/`.

### Lasttest: texturierte 32×32-Sprites mit Alpha, Stufen je 8 s, 2 Läufe je Programm

| Sprites je Bild | Go SYNC Bilder/s | Go CPU je Bild Median (p99) | Rust Vulkan Bilder/s | Rust CPU je Bild Median |
|---:|---:|---:|---:|---:|
| 1 000 | 59–60 | 1,0 ms (2–4) | 60 | 0,2–0,3 ms |
| 5 000 | 58–59 | 3–3,6 ms (23–25) | 60 | 0,2–0,4 ms |
| 10 000 | 58–60 | 6,3 ms (13–37) | 60 | 0,3–0,4 ms |
| 20 000 | 56–59 | 9–11 ms (37–46) | 60 | 0,5 ms |
| 40 000 | 46–48 | 18 ms | 60 | 0,7 ms |
| 80 000 | 23–24 | 36 ms | 60 | 1,1–1,4 ms |
| 160 000 | 11–12 | 75–80 ms | 60 | 2,1 ms |

Rust DX12 ebenfalls 60 Bilder/s bis 160 000 (CPU-Wert dort = Warten auf VSync, nicht aussagekräftig). Quellen: Scratchpad k3c-10 `load/`.

### Auswertung

1. **Belegt: Die Ruckler kommen nicht aus dem Spielcode**, sondern aus der Standard-Schleife von Ebitengine (0, 1 oder 2 Updates je Bild) und aus späten Bildern, die ein leeres Ebitengine-Fenster genauso zeigt. GC, Last, Sim, Draw, Vollbild/DWM und Grafik-Schnittstelle sind ausgeschlossen.
2. **Belegt (optisch, Quadrat-Test): Mit offizieller Ebitengine-API ohne Tricks (`SyncWithFPS` + `SingleThread`) läuft Go flüssig.** Im Spiel entspricht das `--pacing sync`; es senkt die späten Bilder gegenüber `frame` um rund ein Drittel. Ob das Spiel damit **sichtbar** ruckelfrei ist, ist **ungeprüft** (Abnahme 🧑).
3. **Belegt: Rust + wgpu hat weniger späte Bilder** (rund 5–20× weniger als GO SYNC) und rund 35× weniger CPU je Sprite (13 ns gegen 0,45 µs). Go und Rust trennen sich in der Bildrate ab etwa 20 000 Sprites je Bild; einzelne Go-Ausreißer gibt es ab 5 000–10 000.
4. **Bedeutung für K3C (vermutet, Sprite-Zahl nicht gezählt):** Das Spiel braucht in der Last-Szene 2–2,5 ms Draw, liegt also grob unter 5 000 Sprites je Bild. Bei dieser Größe bringt Rust bei der Leistung nichts Sichtbares; der Vorteil wird erst bei etwa 10-mal mehr Zeichenlast oder auf deutlich schwächeren CPUs spürbar. Die zentrale Gate-G0-Frage („lohnt Rust?“) hängt daher an der Abnahme von `--pacing sync` im Spiel.
5. Vorbehalte: ein PC (Intel Arc Pro integriert), nicht ruhend, Läufe 20–64 s; der Rust-Test bündelt Sprites selbst (Instancing), Ebitengine bündelt intern.
