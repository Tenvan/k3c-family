# Go-Referenz: Messwerte, Farbwerte, Pause (RU58.3)

Stand 2026-11-19. Das Go-Spiel aus dem Tag `go-referenz-final` (Commit `dbb87c33`), gemessen mit `tools/measure/measure.ps1 -Engine game
-Runs 3 -Frames` (20 s nach 5 s Aufwärmen, 2 Spieler, Seed `k3c`). Rechner: Micro-Star International Co., Ltd. MS-7C56, AMD Ryzen 7 5800X 8-Core Processor, 32 GB, AMD Radeon RX 6700 XT (Treiber 32.0.21030.2001).
Rohdaten und Zahlen je Lauf: `go-referenz.json` (Median der drei Läufe, schlechtester Frame über die Läufe). Referenz, nicht live.

## Frame-Zeit, Start, Speicher

| Szene | VSync | FPS Mittel | Frame p50 ms | Frame p99 ms | schlechtester Frame ms | Start ms | RAM MiB | GPU MiB |
|---|---|---|---|---|---|---|---|---|
| tag | aus | 3169 | 0.298 | 0.578 | 5.011 | 1298 | 371 | 162 |
| nacht | aus | 2197 | 0.432 | 0.828 | 7.4 | 2102 | 331 | 162 |
| tag | an | 13 | 78.004 | 79.999 | 80.877 | 1303 | 290 | 166 |
| nacht | an | 13 | 77.925 | 79.952 | 80.981 | 2125 | 303 | 162 |

**Vorsicht beim Lesen (ungeprüft):** Das Fenster wurde mit 1028x749 statt 1920x1080 geöffnet (das System begrenzt die
Größe). Ohne VSync laufen rund 2 000 bis 3 000 Bilder je Sekunde (die Messung zählt Bilder, nicht Anzeige). Mit VSync zeigten
das Spiel und die Messung nur 13 FPS (80 ms je Bild, keine langen Bilder); das ist keine echte 60-Hz-Messung und ein
Hinweis auf einen Anzeige- oder Treiberbefund dieses Rechners (VSync-Takt, Fensterverdeckung). Für G2 braucht es eine ruhige
Wiederholung auf dem Mini-PC (B-1074). Frame-Zeit zwei Ansichten (B-1130) ist hier der Wert der Szene `tag` (zwei Spieler).

## Farbwerte im Fenster (B-1129, Schwelle ±2 je Kanal)

Fensteraufnahme der Go-EXE (PrintWindow, Clientbereich 1028x720, Szene `tag`, Takt 300). Stellen als Anteile der Breite und Höhe;
`testdata/rust/go-referenz/farben.json`.

| Stelle | x / y | RGB |
|---|---|---|
| himmel_oben | 0.25 / 0.02 | [142, 202, 230] |
| hud_flaeche_beutel | 0.06 / 0.03 | [64, 91, 104] |
| boden_streifen | 0.50 / 0.34 | [88, 129, 87] |
| wasser | 0.50 / 0.45 | [72, 131, 179] |
| hauptmenue_himmel | 0.50 / 0.05 | [11, 16, 38] |
| hauptmenue_papier | 0.50 / 0.55 | [224, 210, 176] |
| pause_himmel | 0.20 / 0.05 | [11, 16, 38] |
| pause_boden | 0.50 / 0.85 | [59, 58, 107] |

## Pause im Fenster

`testdata/rust/go-referenz/pause.png` (51 KiB, 1028x720): Das Go-Pausenmenü (`--menu pause`) ist ein eigener Bildschirm mit
Nachthimmel und Menüleiste, **kein** Overlay über Welt und HUD. B-1127 (Rust: Pausenmenü sichtbar über Welt und HUD) ist
damit kein Abgleich mit dem Go-Bild, sondern ein Rust-Entwurf.

## Befehle

```bash
# Messung (Tag-Worktree als Root, EXE aus dem Worktree), wie in docs/standards/go-referenz.md gebaut
powershell -File tools/measure/measure.ps1 -Engine game -Root <worktree> -Exe <worktree>/native/k3c-game/bin/k3c-game.exe -Runs 3 -Frames [-Vsync]
```

`measure.ps1 -Engine game` gibt ohne Go-EXE diese Referenz (`go-referenz.json`) aus und endet mit Code 0.
