# Messung: Ruckler Rust-Wiedergabe gegen Go-Spiel (RU39, Gate G2)

Stand: Werkzeug und Probelauf (RU39.1, 2026-11-18). Die Gate-Werte erhebt 🧑 in RU39.2, die Bewertung und Entscheidung
G2 folgen in RU39.3.

## Aufbau

- **Szene:** `nacht` aus `native/measure.ps1`: Seed `k3c`, 2 Spieler, 15 600 Takte Vorlauf (520 s, erste Nacht), beide
  stehen an der Burg, die Welle kommt dort an. Gemessen werden 300 s danach.
- **Go:** `k3c-game.exe --seed k3c --players 2 --tick 15600 --input 0@0=-,1@0=- --measure 300 --warmup 5`, das Spiel
  rechnet die Sim selbst.
- **Rust:** `k3c-game.exe replay messung-nacht.jsonl.gz --window --measure 9000` spielt eine Aufzeichnung derselben
  Szene ab (60 Takte Aufwärmen, dann 9 000 Takte = 300 s). Die Aufzeichnung schreibt
  `go test ./engine/internal/trace/ -run TestMessungNacht -write` nach `.work/trace/messung-nacht.jsonl.gz` (Raum wie
  `native/k3c-game/local/session.go` mit Seed `k3c`, ohne Dev-Mode; 9 061 Zustände, 2,1 MB gepackt, 187 MB JSON).
  Inhalt: Nacht mit 9 Gegnern am Anfang, nach rund 200 s keine Gegner mehr, am Ende Morgendämmerung; 8 Truppen.
- **Was die Wiedergabe zeichnet:** die Zeichner des Spiels in der Anmeldereihenfolge von `game/parts.go` (Welt, Bauten,
  Figuren, Effekte, HUD), zwei Zellen übereinander, Kameras über die Regel der Welt, Zwischenbilder (`blend`).
  Nicht dabei: Werkzeuge, Menü, Ton, Lichtkarte (B-1056). Abgeleitete Sim-Werte liefert `NeutralSimView` (keine
  Wirtschaft, keine Zahlziele, alle Plätze sichtbar) bis RU15/RU18; die Hub-Stufe steht fest auf 1 (nicht in der
  Aufzeichnung).
- **Unterschiede, die beim Lesen zählen:** Rust hält die ganze Aufzeichnung im Speicher (Arbeitsspeicher entsprechend
  höher), rechnet keine Sim (Takt = Kopie eines Zustands) und hat keinen GC. Go rechnet die Sim im selben Prozess. Das
  Rust-Fenster ist fest 1280x720 (B-1069), darum messen beide mit `-Size 1280x720`; die Wiedergabe misst immer mit VSync.
  Ein Rust-Takt kopiert den nächsten Zustand der Aufzeichnung (Kosten in `takt_ms`, nicht in der Rechenzeit je Bild).
  Der Bericht der Wiedergabe nennt in `fenster` immer die echte Größe 1280x720.

## Messanleitung

Auf dem Entwicklungs-PC (Repo, ruhiger Rechner: keine Builds, keine anderen Sessions):

1. `task native:game:build` (Go-Paket `dist/k3c-game/` mit `measure.ps1`, `messen.cmd`) und `task rust:game:dist`
   (`dist/k3c-rust/`).
2. Aufzeichnung schreiben: `go test ./engine/internal/trace/ -run TestMessungNacht -write` und nach
   `dist/k3c-rust/trace/messung-nacht.jsonl.gz` kopieren.
3. Je Programm zwei Läufe abwechselnd (Go, Rust, Go, Rust):
   - `powershell -File dist\k3c-game\measure.ps1 -Engine game -Szenen nacht -Seconds 300 -Vsync -Frames -Size 1280x720`
   - `powershell -File dist\k3c-game\measure.ps1 -Engine replay -Seconds 300 -Vsync -Frames -Size 1280x720`

Auf dem Mini-PC: Beide Ordner `k3c-game` und `k3c-rust` nebeneinander kopieren (samt `k3c-rust\trace\`), dann im
übergeordneten Ordner dieselben zwei Befehle mit `k3c-game\measure.ps1`. Fehlt das Gerät, steht in der Tabelle
„angenommen, Validierung offen“.

Ergebnis je Lauf: `reports\native-<engine>-<rechner>-<zeit>.json` (Feld `bilder` mit den Kennzahlen, GPU- und
Arbeitsspeicher) und die Zeilen-Datei `reports\native-<zeit>-<engine>-<szene>.frames.jsonl`. Eine Zeilen-Datei allein
auswerten: `measure.ps1 -StatsOnly <datei>`.

## Kennzahlen

Eine Funktion für beide Programme (`Get-FrameStats` in `measure.ps1`), aus der Zeilen-Datei je Bild
(`{"t","dt_ms","draw_ms","takte","alpha","gc"}`; Rust schreibt `gc` = 0):

| Kennzahl | Definition |
|---|---|
| Median, p99, p99,9 | Bilddauer `dt_ms`, Perzentil nach nächstem Rang: sortiert[round((n−1)·p)] wie `percentile` in `measure.go` |
| längstes Bild | größtes `dt_ms`; `max_zu_median` = längstes / Median |
| `lang` | Bilder mit `dt_ms` über dem 1,5-Fachen des Medians |
| `lang_programm` | davon mit `draw_ms` über dem halben Median oder GC im Bild (vom Programm verursacht; Faustregel der Sprint-README: Rechnet das Programm ein halbes Bild lang, kann es den Takt verpassen, sonst gilt die Verzögerung als extern) |
| Rechenzeit je Bild | Median und p99 von `draw_ms` (CPU-Zeit von Bildanfang bis vor `present`) |
| Grafikspeicher | Höchstwert dediziert und geteilt (WMI `GPUProcessMemory` je Prozess) |
| Arbeitsspeicher | Höchstwert `WorkingSet64` |

Probe von Hand: zehn Zeilen mit `dt_ms` 16 (6×), 17 (2×), 30, 40 und `draw_ms` 1 bis 10 ergaben Median 16, p99 40,
`lang` 2, `lang_programm` 1, wie nachgerechnet. Beim Go-Lauf stimmt die Auswertung mit Gos eigenem Feld (`bilder_go`)
überein.

## Schwellen (Vorschlag, Entscheidung 🧑)

Aus der Sprint-README: längstes Bild höchstens das Doppelte des Medians; kein Bild über dem 1,5-Fachen des Medians, das
das Programm verursacht (`lang_programm` = 0). Ist Rust nicht sichtbar besser als das Go-Spiel nach B-943, empfiehlt
RU39.3 den Abbruch.

## Gate-Läufe (RU39.2, 🧑)

| Rechner | Programm | Lauf | Median | p99 | p99,9 | längstes | lang | lang_programm | Rechenzeit Median/p99 | GPU ded./geteilt | RAM |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Entwicklungs-PC | Go | 1 | – | – | – | – | – | – | – | – | – |
| Entwicklungs-PC | Rust | 1 | – | – | – | – | – | – | – | – | – |
| Entwicklungs-PC | Go | 2 | – | – | – | – | – | – | – | – | – |
| Entwicklungs-PC | Rust | 2 | – | – | – | – | – | – | – | – | – |
| Mini-PC | Go | 1 | – | – | – | – | – | – | – | – | – |
| Mini-PC | Rust | 1 | – | – | – | – | – | – | – | – | – |
| Mini-PC | Go | 2 | – | – | – | – | – | – | – | – | – |
| Mini-PC | Rust | 2 | – | – | – | – | – | – | – | – | – |

## Probelauf (RU39.1, nicht für G2)

60 s je Programm, 1280x720, VSync, auf dem Rechner der Session (WS1285, integrierte Grafik) mit parallel laufenden
Agenten-Sessions; die Werte belegen nur, dass die Kette läuft.

| Programm | Bilder | Median | p99 | p99,9 | längstes | lang | lang_programm | Rechenzeit Median/p99 | GPU ded./geteilt | RAM |
|---|---|---|---|---|---|---|---|---|---|---|
| Go | 3 454 | 16,64 ms | 48,55 ms | 86,17 ms | 570,45 ms | 155 | 11 | 0,44 / 3,39 ms | 0 / 201 MiB | 588 MiB |
| Rust | 3 601 | 16,67 ms | 26,25 ms | 46,73 ms | 72,91 ms | 42 | 0 | 0,15 / 0,37 ms | 0 / 357 MiB | 872 MiB |
