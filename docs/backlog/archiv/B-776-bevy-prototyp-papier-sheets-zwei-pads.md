# B-776 · Ein Bevy-Prototyp zeigt ein Level mit echten Papier-Sheets, zwei Gamepads steuern je einen Monarchen, FPS und Grafikspeicher sind gemessen

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** NV1
- **Projekt:** NAT
- **Erstellt:** 2026-11-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-10 🧑 Chat-Auftrag (Bevy-Prototyp, gepinnte Version, nur Darstellung und Eingabe)

## Ausgangslage

Bevor K3C nach Rust + Bevy umzieht (B-775), fehlt der Beleg, dass Bevy die Papier-Sheets (569 PNG, bis 757 MiB RGBA) flüssig zeichnet und zwei Gamepads sauber trennt. Die Sheets liegen nach `task gen:paper` unter `public/grafik/<gruppe>/<asset>/` mit `manifest.json`; die Zuordnung Rolle → Asset steht in `data/paper.json`.

## Ziel

Ein kleiner Bevy-Prototyp, der ein Level mit echten Papier-Sheets zeigt, zwei Monarchen über zwei Gamepads steuert und FPS sowie Grafikspeicher als Bericht schreibt – Grundlage für die Entscheidung in B-775.

## Beteiligte und Zielgruppen

🧑 Ralf (misst auf dem Mini-PC, entscheidet), Claude (baut, misst auf dem Entwicklungs-PC als Referenz).

## Anforderungen

- Eigenes Cargo-Projekt `native/k3c-proto/`, Bevy exakt gepinnt (`=0.20.1`), `Cargo.lock` eingecheckt, Bau mit `--locked`.
- Liest `data/paper.json` und die Manifeste unter `public/grafik/`; zeichnet Hintergrund, Gebäude, Truppen, Gegner und zwei berittene Monarchen, animiert nach `fps` aus dem Manifest.
- Zwei Gamepads: Pad 1 → Monarch 1, Pad 2 → Monarch 2, nur horizontal; Split-Screen mit einer Kamera je Spieler. Tastatur als Ersatz (A/D und ←/→).
- Messmodus: nach Aufwärmen eine feste Zeit messen und einen JSON-Bericht schreiben (FPS Mittel und 1-%-Low, Frame-Zeit p50/p95/p99, berechneter Texturspeicher, GPU-Speicher des Prozesses aus dem Windows-Zähler, Adapter und Backend).
- Zwei Lastfälle: `level` (nur was das Level braucht) und `alle` (alle Sheets geladen, schlimmster Fall).

## Nicht-Ziele

Keine Simulationslogik (keine Münzen, Kämpfe, Gebäude-Logik), kein Netz, kein HUD/Menü, keine Portierung von Go-Code (B-775).

## Regeln und Einschränkungen

Nicht im Cargo-Workspace `rust/` (dessen Release-Profil ist für die FFI-DLL gebaut: `panic = abort`, `lto = fat`). Befehle nur über `task native:*`. Logging mit Emojis. Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Zufall nur deterministisch (fester Seed für die Platzierung).

## Beispiele

`task native:measure` → Fenster läuft je Lastfall ca. 25 s, danach liegt `reports/native-proto-<rechner>-<zeit>.json` mit beiden Lastfällen vor.

## Ausnahme- und Fehlerfälle

Kein `public/grafik` (kein `gen:paper`) → Abbruch mit Meldung `task gen:paper` ausführen. Nur ein Gamepad → Spieler 2 über Tastatur. Windows-Zähler fehlt → Feld GPU-Speicher leer, Bericht trotzdem.

## Akzeptanzkriterien

- **AC-01** `task native:proto:build` baut mit Bevy `=0.20.1` und `--locked` ohne Fehler.
- **AC-02** Das Fenster zeigt ein Wald-Level aus Papier-Sheets (Hintergrund, Gebäude, Truppen, Gegner, zwei berittene Monarchen), animiert nach Manifest-fps.
- **AC-03** Zwei Gamepads bewegen je einen Monarchen unabhängig; jede Bildschirmhälfte folgt ihrem Spieler; Tastatur-Ersatz geht.
- **AC-04** `task native:measure` schreibt einen Bericht mit FPS, Frame-Zeiten, Texturspeicher, GPU-Speicher, Adapter und Backend für `level` und `alle`.
- **AC-05** Referenzwerte vom Entwicklungs-PC stehen unter Notizen.
- **AC-06** Messwerte vom Mini-PC stehen unter Notizen (Abnahme 🧑 am Mini-PC).

## Offene Fragen

keine

## Notizen

### Referenz Entwicklungs-PC (AC-05, 2026-11-10, NV1.1)

Rechner STIRA-PC: Desktop MSI MS-7C56, Ryzen 7 5800X, 32 GB, **Radeon RX 6700 XT** (Treiber 25.10.30.02), Backend **Vulkan**, Fenster 1920×1080, ohne VSync, Spieler fahren automatisch (`--auto`), je 20 s nach 5 s Aufwärmen (`task native:measure`). **Kein Mini-PC** – der Mini-PC-Wert (AC-06) fehlt noch.

| Lastfall | Sheets | Texturen RGBA | FPS Mittel | FPS 1-%-Low | Frame p50 / p95 / p99 / max | GPU dediziert | GPU geteilt | RAM | Laden |
|---|---|---|---|---|---|---|---|---|---|
| level | 38 | 32 MiB | 343 | 205 | 2,8 / 3,9 / 4,3 / 9,9 ms | 224 MiB | 117 MiB | 318 MiB | 0,08 s |
| alle | 569 | 757 MiB | 323 | 185 | 3,2 / 4,0 / 4,5 / 13,8 ms | 996 MiB | 783 MiB | 1040 MiB | 0,49 s |

Einordnung: Auch mit **allen** Sheets im Speicher bleibt die Bildrate fast gleich; der Grafikspeicher (≈ 1 GiB) ist das, was die Xbox nicht hatte (Kontextverlust bei 214–347 MiB). Ein Level braucht nur ≈ 32 MiB Texturen. Grundlast von Bevy/Vulkan ohne Texturen ≈ 190 MiB GPU.

### Mini-PC messen (AC-06, 🧑)

`task native:proto:paket` → Ordner `.work/native-proto/` (EXE, `measure.ps1`, `messen.cmd`, `public/grafik`, ≈ 194 MB) auf den Mini-PC kopieren, `messen.cmd` doppelklicken. Bericht: `reports\native-proto-<rechner>-<zeit>.json` neben der EXE. Interaktiv: `k3c-proto.exe --root .` (Pad 1 / A,D und Pad 2 / ←,→).

### Offen

- AC-03 mit zwei **echten** Gamepads ist nicht geprüft (am Entwicklungs-PC keine zwei Pads angeschlossen); Pads werden nach Verbindungs-Reihenfolge Spieler 1 und 2 zugeordnet. Tastatur-Ersatz und Auto-Fahrt laufen.
- Bevy 0.20 API-Abweichung gegenüber älterem Wissen: Laden mit Einstellungen heißt `load_builder().with_settings(..).load(..)`.
