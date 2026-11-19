# Messung: Ruckler Rust-Wiedergabe gegen Go-Spiel (RU39, Gate G2)

Stand: Werkzeug, Probelauf (RU39.1) und vorläufige Messung mit eigenem Messcode (RU39.3, 2026-11-18). Die Gate-Werte
(ruhiger PC, Mini-PC, Sichtprüfung) sind auf das Ende verschoben (B-1074, Ralf 2026-11-18: Messungen zum Schluss).

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

1. `task native:game:build` (Go-Paket `dist/k3c-game-go/` mit `measure.ps1`, `messen.cmd`) und `task rust:game:build`
   (Rust-Paket `dist/k3c-game/`).
2. Aufzeichnung schreiben: `go test ./engine/internal/trace/ -run TestMessungNacht -write` und nach
   `dist/k3c-game/trace/messung-nacht.jsonl.gz` kopieren (der Ordner `trace\` ist kein Teil des Pakets, die Prüfung
   `rust:game:dist:check` stört er nicht).
3. Je Programm zwei Läufe abwechselnd (Go, Rust, Go, Rust):
   - `powershell -File dist\k3c-game-go\measure.ps1 -Engine game -Szenen nacht -Seconds 300 -Vsync -Frames -Size 1280x720`
   - `powershell -File dist\k3c-game-go\measure.ps1 -Engine replay -Seconds 300 -Vsync -Frames -Size 1280x720`

Auf dem Mini-PC: Beide Ordner `k3c-game-go` und `k3c-game` nebeneinander kopieren (samt `k3c-game\trace\`), dann im
übergeordneten Ordner dieselben zwei Befehle mit `k3c-game-go\measure.ps1`. Fehlt das Gerät, steht in der Tabelle
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

## Gate-Läufe (verschoben nach B-1074, 🧑)

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

## Vorläufige Messung (eigener Messcode)

**Vorläufig, unruhiger Rechner** (WS1285, integrierte Grafik, parallel laufende Agenten-Sessions und Builds). Kein
Gate-Wert, keine Entscheidung G2. Werkzeug: `rust/crates/k3c-game/tests/messung_cpu.rs` (`--ignored`, Aufruf im Kopf der
Datei). Es spielt die Aufzeichnung `messung-nacht` (Szene `nacht`, 2 Spieler, 9 060 Takte, mit
`go test ./engine/internal/trace/ -run TestMessungNacht -write` geschrieben, fester Seed `k3c`) rund 9 000 Bilder (8 998 gezählt) lang ab
(60 Bilder Aufwärmen), mit den echten Zeichnern des Spiels (Welt, Bauten, Figuren, Effekte, HUD, zwei Zellen) und der
gerenderten Grafik (21 + 8 MiB RGBA, 24 Rigs) gegen `NullCanvas`, Release-Build. Gemessen ist nur der **CPU-Teil**: Befehlsliste
erzeugen (je Bild im Mittel 510 Zeichenbefehle, davon 363 Sprites) plus `step` (Zustand kopieren, Blend, Kameras, Zellen, Listener). Nicht enthalten: Textur-Upload,
GPU, VSync, Treiber. Die Werte sind eine Untergrenze der Rechenzeit je Bild, kein Bildzeit-Ersatz.

Bild = `step` + `draw` in ms, drei Läufe nacheinander:

| Lauf | Mittel | p50 | p99 | p99,9 | längstes | über 1,5 × p50 | über 8,3 ms | über 16,7 ms | davon `draw` p50 / p99 / max |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 0,074 | 0,062 | 0,273 | 0,945 | 3,07 | 788 | 0 | 0 | 0,029 / 0,117 / 3,02 |
| 2 | 0,102 | 0,081 | 0,259 | 2,644 | 9,15 | 1 723 | 1 | 0 | 0,038 / 0,105 / 5,84 |
| 3 | 0,078 | 0,063 | 0,224 | 0,827 | 3,03 | 1 216 | 0 | 0 | 0,029 / 0,106 / 2,72 |

- **Budget:** Das längste CPU-Bild liegt bei 9,2 ms (Lauf 2, ein einziges Bild über der halben Bildzeit von 8,3 ms), kein Bild
  über 16,7 ms. Der Median (0,06 bis 0,08 ms) belegt nur 0,4 bis 0,5 % der Bildzeit.
- **Verhältnis längstes zu Median** (G2-Schwelle 1) liegt wegen des Mikrosekunden-Medians bei 48 bis 113 und taugt
  auf der CPU-Basis nicht als Maß; maßgeblich ist die absolute Zeit im Bildbudget. Schwelle 1 wird erst mit den Bildzeiten
  des Fensters entschieden (B-1074).
- **Verlauf:** Die Drittel der Läufe zeigen keinen Anstieg über die Nacht (p50 0,06 bis 0,09 ms in allen), kein Wachstum der
  Bildzeit; die Ausreißer (p99,9 bis 2,6 ms) streuen zwischen den Läufen und folgen vermutlich dem unruhigen Rechner, nicht
  der Szene (ungeprüft).
- **Allokationen je Bild:** nicht messbar. Ein zählender `GlobalAlloc` braucht `unsafe`, der Workspace setzt
  `unsafe_code = "forbid"`. Ticket B-1075.

Zum Vergleich der Probelauf von oben (60 s im Fenster, ebenfalls unruhig): Rust Rechenzeit je Bild 0,15 / 0,37 ms
(Median/p99), `lang_programm` 0, längstes Bild 72,9 ms; Go 0,44 / 3,39 ms, `lang_programm` 11, längstes Bild 570 ms.
Die Rust-Zeichner brauchen also auch mit GPU-Übergabe nur einen Bruchteil des Bildbudgets; die langen Bilder
entstehen vermutlich außerhalb der Rust-Zeichner (Treiber, Präsentation oder Rechner, ungeprüft); beim Go-Lauf
zählen 11 Bilder als vom Programm verursacht (vermutet: GC und Sim im selben Prozess).

### Vorläufige Einschätzung G2

**Vorläufig bestanden, nur für den CPU-Teil der Zeichner; Gate-Messung am Ende (B-1074).** Schwelle 1, Schwelle 2 und „sichtbar besser als Go“ sind nicht gemessen und damit offen. Kein klares Leistungsrisiko im CPU-Teil der Zeichner: kein
Bild über dem Bildbudget, kein Anstieg über die Nacht. Offen und nicht belegt: Allokationen je Bild (B-1075), Bildzeiten
am ruhigen PC und Mini-PC, die Sichtprüfung „sichtbar besser als Go“ und der Vorbehalt, dass die Wiedergabe keine Sim
rechnet (in Go 12 bis 21 µs je Takt, 008). Die Entscheidung G2 bleibt bei 🧑.

## Allokationen je Bild (RU42, B-1075)

`cargo test --release -p k3c-game --test alloc_je_bild -- --nocapture` (Zähler `k3c-alloc-probe`, je Thread; Wiedergabe `nacht-300` gegen `NullCanvas`, nach 120 Aufwärmbildern, 2026-11-18, zwei Läufe gleich): 179 Bilder, Allokationen je Bild Mittel 478, Maximum 805 (`step` 788, `draw` 29), alle 179 Bilder allokieren, Summe 85 588 mit 9,3 MB angeforderten Bytes. Die Obergrenze 805 steht fest im Test. Treffer: `step` (Welt kopieren, Zustand je Aufzeichnungs-Takt bauen, B-1077) und `draw` (Zeichner, `Rc`-Klone, B-1078); beide Tickets im Sprint RU43. Ob `step` nur im Replay oder auch im echten Spiel so viel allokiert, klärt B-1077 (vermutet: nur Replay).
