# Messung: erster Rust-Client (RU2.4, AC-04)

Stand 2026-11-16. Client: `k3c-game demo --driver rust/demo/reiten.json` (Release, winit 0.30 + wgpu 30, Fifo/VSync,
Takt 30 Hz mit alpha-Mischung, 1280×720 Fenster, 60-Hz-Bildschirm). Szene: Wald-Welt mit drei Parallaxe-Ebenen, Gebäuden,
81 Figuren und Monarch; Zusatzlast `--sprites N` (kleine bewegte Sprites, ein Draw-Call je Textur-Ebene).
Aufruf je Lauf: `WGPU_BACKEND=<vulkan|dx12> k3c-game demo --driver rust/demo/reiten.json --sprites N --measure 60 --out <datei>.json`.

Kennzahlen je Bild: dt = Abstand zwischen zwei Bildern, CPU = Zeit von Bildanfang bis unmittelbar vor `present`.
**Spät** = dt > 25 ms; „ohne Start“ zählt die späten Bilder ab Sekunde 4 (Fenster öffnen und 259 Texturen hochladen
kosten das erste Bild rund 500–850 ms, gehört nicht zum Laufen).

## Ergebnis: zwölf Läufe à 60 s

| Backend | Sprites | Lauf | Bilder | dt Median ms | dt p99 ms | dt max ms | CPU Median ms | CPU p99 ms | späte gesamt | späte ohne Start | Sekunden mit späten Bildern |
|---|---|---|---|---|---|---|---|---|---|---|---|
| vulkan | 0 | 1 | 3515 | 16,68 | 17,8 | 844 | 1,01 | 2,2 | 7 | 3 | 3/56 |
| vulkan | 0 | 2 | 3494 | 16,66 | 33,7 | 851 | 0,39 | 3,1 | 85 | 82 | 33/56 |
| vulkan | 5000 | 1 | 3469 | 16,65 | 57,7 | 624 | 0,76 | 14,0 | 169 | 164 | 44/56 |
| vulkan | 5000 | 2 | 3504 | 16,66 | 30,4 | 556 | 0,71 | 7,2 | 59 | 57 | 26/56 |
| vulkan | 20000 | 1 | 3518 | 16,65 | 39,0 | 501 | 2,13 | 18,4 | 80 | 73 | 35/56 |
| vulkan | 20000 | 2 | 3499 | 16,66 | 36,3 | 498 | 2,54 | 23,3 | 90 | 88 | 36/56 |
| dx12 | 0 | 1 | 3450 | 16,66 | 37,8 | 639 | 16,26 | 34,2 | 108 | 103 | 34/56 |
| dx12 | 0 | 2 | 3456 | 16,68 | 24,2 | 1447 | 16,29 | 21,2 | 32 | 27 | 20/56 |
| dx12 | 5000 | 1 | 3469 | 16,65 | 37,8 | 708 | 16,24 | 34,3 | 129 | 122 | 46/56 |
| dx12 | 5000 | 2 | 3529 | 16,68 | 22,8 | 702 | 16,34 | 21,5 | 27 | 22 | 18/56 |
| dx12 | 20000 | 1 | 3460 | 16,64 | 34,4 | 810 | 16,22 | 33,2 | 142 | 137 | 41/56 |
| dx12 | 20000 | 2 | 3529 | 16,68 | 23,5 | 703 | 16,31 | 21,3 | 28 | 25 | 19/56 |

Kontrolllauf danach (vulkan, 20 000 Sprites, 30 s): Median 16,68 ms, p99 18,4 ms, 3 späte Bilder, **alle in den ersten
zwei Sekunden, danach 0**; CPU Median 6,3 ms. Ein Lauf davor (RU2.1, Probe mit 20 000 Sprites, 15 s) ergab ebenfalls nur
1–3 späte Bilder am Anfang.

## Bewertung

1. **Belegt:** Der Median hält in allen zwölf Läufen 16,64–16,68 ms (60 Bilder/s), Vulkan und DX12 gleich. Die CPU-Zeit je
   Bild liegt auf Vulkan bei 0,4–2,5 ms; auf DX12 bei rund 16,3 ms, weil `present` dort auf VSync wartet (wie in RU0.2:
   der DX12-CPU-Wert ist Warten, nicht Arbeit).
2. **Belegt: Die späten Bilder hängen nicht an der Last.** Lauf „vulkan 0 Sprites“ hat einmal 3, einmal 82 späte Bilder;
   „20 000 Sprites“ liegt im selben Bereich wie „0“. Wäre der Client die Ursache, müssten späte Bilder mit der Spritezahl
   steigen. Sie schwanken zwischen gleichen Einstellungen stärker als zwischen verschiedenen.
3. **Vermutet (ungeprüft): Die Läufe waren gestört.** Während der Messung liefen auf dem Rechner weitere Claude-Sessions
   (Cargo-Builds, Messläufe); die CPU-Last schwankte nach Stichproben zwischen 2 % und 93 %. Der ruhigere Kontrolllauf (0 späte
   Bilder ab Sekunde 3) stützt das, beweist es aber nicht. Die Tabelle zeigt deshalb **keine belastbare Aussage zu
   „ruckelfrei“**, nur: Der Client erreicht die 60 Bilder/s im Median und der Median bleibt unter Störung stabil.
4. **Startspitze:** Das erste Bild dauert 0,5–1,4 s (Fenster, Pipeline, 259 Texturen im Block). Für das echte Spiel
   später vor dem ersten sichtbaren Bild laden (RU4/RU35), hier unerheblich.

## Vergleich mit RU0.2 (Lasttest ohne Spielcode, Go/Ebitengine gegen Rust)

| Sprites | Go SYNC Bilder/s | Go CPU Median ms (p99) | Rust RU0.2 Bilder/s | Rust RU0.2 CPU ms | Client RU2.4 Bilder/s | Client RU2.4 CPU Median ms (Vulkan) |
|---|---|---|---|---|---|---|
| 5 000 | 58–59 | 3–3,6 (23–25) | 60 | 0,2–0,4 | 60 | 0,7–0,8 |
| 20 000 | 56–59 | 9–11 (37–46) | 60 | 0,5 | 60 | 2,1–2,5 |

Der Client ist mit Szene (Parallaxe, 81 animierte Figuren, Schrift, 30-Hz-Takt) etwa 2–5× teurer je Bild als der nackte
Lasttest, bleibt aber 4–5× unter Go SYNC bei gleicher Spritezahl und hält 60 Bilder/s. Späte Bilder lassen sich wegen der
Störung (Punkt 3) mit RU0.2 nicht sauber vergleichen; dafür ist ein Lauf auf ruhigem Rechner nötig.

## Vorbehalte

- Ein PC (Intel Arc Pro integriert), Windows 11, nicht ruhend; kein Notebook, kein Mini-PC am TV.
- Zwei Läufe je Zelle, 60 s; die Streuung zwischen den Läufen ist größer als der Unterschied zwischen den Zellen.
- Die Zusatzlast-Sprites sind klein (0,3 × Frame) und bündeln in einen Draw-Call; die echte Szene im Spiel
  mischt mehr Texturen.

## Nächste Schritte

- **Ralf (AC-05):** `dist/k3c-rust-demo/start.cmd` auf PC und Notebook, „ruckelfrei ja/nein“ (Ergebnis in RU2.4).
- Messung wiederholen, wenn der Rechner ruhig ist (keine parallelen Builds); dazu `--measure 60` wie oben.
