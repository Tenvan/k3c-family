# Abnahme B-1141 (Teil Second Worker): Paket im fremden Ordner (RU64.7)

2026-11-19, STIRA-PC (Entwicklungsrechner, kein PC ohne Werkzeuge), Second Worker, Commit 4956b08a. Paket aus `task buildall` (RU64.2), 2131 Dateien laut `rustpaket pruefen`. B-1141 bleibt offen (Doppelklick, Ton, Mini-PC, Entscheidungen bei Ralf).

## Befehle und Befunde

| Schritt | Ergebnis |
|---|---|
| `dist/k3c-game/` nach `.work/paket-test/` kopiert (`cp -r`) | 2163 Dateien (inklusive der leeren Ordner `saves/` und `reports/`, die vor dem Start entfernt wurden) |
| Dateiliste gegen B-1141 | vorhanden: `k3c-game.exe` (19,3 MB), `k3c-devwin.exe`, `data/`, `public/` (Grafik, Ton, Schrift, Rig-Teile), `LICENSE`, `LICENSE-ASSETS`, `LIZENZEN.txt`, `README.txt` |
| Start aus anderem Ordner: PowerShell `Start-Process .work\paket-test\k3c-game.exe` mit Arbeitsverzeichnis `.work\fremd` | Fenster nach **246 ms**, Titel „K3C \| Vulkan \| 60 Bilder/s“, RAM 361 MiB nach 5 s; Beenden per `CloseMainWindow`: sauber, **Exit-Code 0**, Log „K3C natives Spiel beendet“ |
| Spielstand-Ordner | Pfad im Log `…\paket-test\saves` (neben der EXE, nicht im Arbeitsverzeichnis); der Ordner wurde nach einem reinen Menü-Start **nicht angelegt** (`saves`, `reports` fehlen nach dem Lauf; erst beim Speichern erwartet, ungeprüft) |
| Konsolenausgabe | Log mit INFO/WARN (Ausgabe in Dateien umgeleitet, ob ein Konsolenfenster erscheint, nicht geprüft; Frage bei 🧑) |

Warnungen im Log (nicht als Fehler gewertet):

- `Tongerät nicht verfügbar, Spiel läuft stumm: Could not find any output device` (dieser Rechner im Lauf ohne Ausgabegerät; Ton am Gerät bleibt bei Ralf).
- `Ton-Atlas nicht ladbar, Effekte als Einzeldateien: public\audio\atlas.ogg: Ausschnitte kann k3c-platform noch nicht laden (Haltepunkt RU17.5)`; Effekte laden einzeln (23 Dateien).

## LIZENZEN.txt (nur Fakten)

- Kopfzeile: „Rust-Crates im Spiel (132)“; im Text 14-mal „Das Crate enthält keine Lizenzdatei“, davon neun Einträge mit MPL-2.0 (Zeilen 15790 bis 15846) und fünf mit MIT/Apache (Zeilen 3303, 5112, 5120, 12497, 15553).
- Kopf der Liste (Engine, Schrift, Grafik, Audio): nur wgpu, winit, pollster, gilrs, Go Regular (Schrift, BSD-3-Clause), fontdue, png, flate2, rodio und der CC0-Hinweis; **keine Go-Engine-Einträge** (Ebitengine u. a.). „Go Regular“ ist die Schrift, kein Go-Code.
- Versionszeile: `v0.24.1-233-g6e3fd403-dirty` (Paket wurde mit unsauberem Arbeitsbaum gebaut; für die Weitergabe sauber neu bauen).

## Für den Hauptrechner

- Start aus fremdem Ordner und sauberes Beenden: belegt. Abweichung zum Soll „`saves\` entsteht neben der EXE“: Ordner entsteht erst später oder gar nicht; Spielstand-Pfad im Log stimmt (neben der EXE). Prüfen, wann der Ordner angelegt wird (Ticket-Kandidat, ungeprüft).
- Version „dirty“ im Paket: vor der ersten Weitergabe aus sauberem Stand bauen.
- Warnung Ton-Atlas (RU17.5): Effekte laufen als Einzeldateien; offen, ob Absicht.

## Prüfliste für Ralf (AC-01 am zweiten PC, AC-02)

1. Paket auf einen PC ohne Entwicklungswerkzeuge kopieren; Doppelklick auf `k3c-game.exe` (Grafik, Ton, Menü, Neues Spiel), danach aus einer Konsole in einem fremden Ordner.
2. Entscheiden: Hinweis statt Volltext für die 14 Crates ohne Lizenzdatei (u. a. `symphonia-*` MPL-2.0), Go-Kopf der Liste (im Kopf der Liste keine Go-Einträge, siehe oben), Konsolenfenster (`windows_subsystem`).
3. Mini-PC: Ton am Gerät, Frame-Zeit (`abnahme-b-1130.md` Teil 2).
