# RU2 · CLI · Spike: schlanker Rust-Client gegen die Ebitengine-Messung (Gate G1)

- **Status:** geplant
- **Projekt:** RSU
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1016
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-16

## Ausgangslage

G0 ist entschieden (🧑 2026-11-16, RU0.2): Ebitengine zeigt auf demselben PC auch ohne Spielcode späte Bilder (2–5 je s), mit offizieller API (`SyncWithFPS` + `SingleThread`) weniger, aber sichtbar leichte Ruckler bleiben; ab etwa 20 000 Sprites je Bild fällt Ebitengine ab, Rust + wgpu hält 60 Bilder/s bis 160 000. 🧑: „Wir steigen definitiv auf Rust um.“ Der Stack ist damit gemessen (winit 0.30 + wgpu, Vulkan/DX12); der frühere Spike gegen Bevy-Zahlen entfällt.

## Ziel

So schnell wie möglich ein **lauffähiger Rust-Client nur mit Grafik**, gesteuert von einem einfachen Driver statt der Sim. Am Ende sichtbar: `rust/target/release/k3c-game.exe demo --driver rust/demo/reiten.json` zeigt eine Papierschnitt-Welt (Hintergrund-Ebenen, Boden, Gebäude, laufende Figuren), durch die die Kamera reitet; 🧑 prüft am PC und auf dem Notebook, ob es ruckelfrei ist.

## Beteiligte und Zielgruppen

Agenten setzen um, die Review-Session prüft. 🧑 gibt die Spec frei und entscheidet an den Gates (B-1014).

## Anforderungen

Welle R1 der Rust-Umstellung (erste Rust-Welle nach G0). Stack fest: `winit` 0.30, `wgpu` (Vulkan bevorzugt, DX12 per `WGPU_BACKEND`), `pollster`, `png`, `serde`/`serde_json`. Weitere Crates nur mit Freigabe 🧑.

Vorlage für Schleife, Sprite-Bündelung und Messung: die Vergleichsprogramme aus RU0.2 (`.work/vergleich/`, Quelle beschrieben in RU0.2 › Ergebnis). Der Port-Leitfaden `docs/standards/rust-port.md` (B-1018) gilt, sobald RU1.1 ihn angelegt hat. Modell-Empfehlung: `sonnet`.

## Nicht-Ziele

Sim, Regeln, Spielstände, Menüs, HUD, Ton, Gamepads, Rig-Figuren (RU5), zwei Bildhälften (RU4). Code-Port der Go-Pakete `view/`, `world/`, `actors/` (RU7, RU13): der Driver-Client lädt nur dieselben Grafik-Dateien. Änderungen am Go-Spiel.

## Regeln und Einschränkungen

Der Go-Code bleibt unverändert lauffähig (Rückfall). Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen. Kein `unsafe` außer im Byte-Upload der Instanzen (oder `bytemuck`, Freigabe 🧑). Die Sim-Sperre gilt; der Driver ist keine Sim (keine Regeln, nur Bewegung nach Skript).

- **Bereich:** `rust/crates/k3c-platform/**`, `rust/crates/k3c-game/src/art/**`, `rust/crates/k3c-game/src/demo/**`, `rust/crates/k3c-game/src/main.rs`, `rust/demo/**`, `docs/messung/rust-client-1.md`
- **Abhängig von:** RU0, RU1
- **Braucht 🧑:** Abnahme (optisch, PC und Notebook)
- **Sperre:** –

## Beispiele

- `k3c-game demo --driver rust/demo/reiten.json` → Fenster 1280×720, Kamera reitet mit Monarch-Tempo durch eine Welt, Figuren laufen, Titel zeigt Backend und späte Bilder.
- `k3c-game demo --driver rust/demo/reiten.json --sprites 20000 --measure 60` → Messbericht als JSON.
- `k3c-game demo --driver rust/demo/reiten.json --headless --steps 600` → kein Fenster, Code 0.

## Ausnahme- und Fehlerfälle

Fehlt `public/grafik` (nicht gerendert), meldet der Client den Befehl `task gen:paper` und endet mit Code 1. Unbekanntes Asset im Driver → Meldung mit Asset-Namen, kein Absturz. Fehlt eine Voraussetzung aus einem anderen Sprint, entsteht ein Ticket statt einer Änderung außerhalb des Bereichs.

## Akzeptanzkriterien

- **AC-01** Crate `k3c-platform`: Fenster (winit), wgpu-Kontext, Schleife mit eigener Zeitsammlung (Sim-Takt 30 Hz, alpha an `draw`, ein Redraw je Bild, VSync Fifo), `trait App` und `trait Canvas` mit `sprite(tex, src, dst, tint)` (gebündelt als Instanzen, ein Draw-Call je Textur), `rect`, Pixel-Schrift `text`; `NullCanvas` und `run_headless`. Test: im Leerlauf keine Speicher-Anforderung je Bild.
- **AC-02** `k3c-game/src/art`: liest `public/grafik/<gruppe>/index.paper.json`, je Asset `manifest.json` und PNG-Sheets, und `data/paper.json`; liefert je Asset Bilder und Frame-Rechtecke. Tests gegen eine kleine Fixture.
- **AC-03** Driver: JSON-Datei mit Welt-Länge, Hintergrund-Ebenen (Parallaxe), Boden, Gebäuden (x, Asset), Figuren-Gruppen (Asset, Anzahl, Tempo, Bereich) und Kamera-Fahrt (Tempo, Wende an den Enden); Zwischenbilder über alpha. Unterbefehl `demo` mit `--driver`, `--sprites N` (Zusatzlast), `--measure S`, `--headless --steps N`.
- **AC-04** Messbericht `docs/messung/rust-client-1.md`: je Backend (Vulkan, DX12) und Last (Driver allein, +5 000, +20 000) Bilder/s, Median, p99, längstes Bild, späte Bilder je s; Vergleich mit RU0.2.
- **AC-05** 🧑 bestätigt am PC und auf einem schwächeren Notebook: das Durchreiten ist ruckelfrei (manuelle Abnahme, Ergebnis in RU2.4).

## Offene Fragen

Keine; Stack, Reihenfolge und Abnahme hat 🧑 am 2026-11-16 entschieden.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RU2.1 | `RU2.1-plattform-kern.md` | Umsetzung | autonom | offen |
| RU2.2 | `RU2.2-grafik-laden.md` | Umsetzung | autonom | offen |
| RU2.3 | `RU2.3-demo-szene-und-driver.md` | Umsetzung | autonom | offen |
| RU2.4 | `RU2.4-messung-und-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
