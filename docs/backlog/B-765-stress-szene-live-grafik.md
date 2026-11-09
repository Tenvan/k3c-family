# B-765 · Stress-Szene Live-Grafik: Dichte-Test mit Szenario-Datei, Rampe und Presets (1 und 2 Spieler)

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** RX1
- **Projekt:** LST
- **Erstellt:** 2026-11-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ob die Xbox bei vielen Gegnern und Sprites einbricht, lässt sich nur in einer echten Partie beobachten (`?perf=1`, PM1) und nicht reproduzierbar abstufen. B-761/B-762 liefern Renderer-, Textur- und Leistungswerte, aber keine Szene, die gezielt Last erzeugt.

**Wiederverwendbar (geprüft):**
- `src/tools/devTiles.ts`: neue Werkzeug-Seite `stress.html`, Kachel im Abschnitt `perf` (neben `monitor`), Texte `tile.stress.title/.desc`; Seite ruft `installPageChrome()` auf (Regel in CLAUDE.md).
- `src/online/clientPerfReport.ts` (phaser-frei): `Histogram` (p50/p95/p99), `detectDrop`, `FRAME_DROP_MS`, `sendPerfReport` (`POST /api/report` → `reports/*.json`).
- `src/online/clientPerf.ts`: `collectPerf`, `startDiag` (Diagnose-Zeile alle 5 s).
- `src/core/renderInfo.ts` (B-761): `readRenderInfo` (Renderer, GPU, Texturzahl, `textureMiB`), `FrameAverage`; B-762 (`renderLog`) für Start-, Verlaufs- und Kontextzeilen.
- Papierschnitt-Zuordnung `data/paper.json`, Gegnerwerte `data/enemies.json`, `src/scenes/render/worldRenderer.ts` zum Zeichnen.

**Nicht direkt wiederverwendbar:** `src/scenes/perfProbe.ts` liest nur im Raum (`status === 'room'`, Latenz, Zustands-Lücke) und hängt an der `GameScene`; die Stress-Szene braucht eine eigene, serverlose Quelle für dieselben Werte.

## Ziel

Eine startbare Szene zeichnet reproduzierbar N Figuren (Gegner, Truppen, Sprite-Mix) für 1 oder 2 Spieler, steigert die Last stufenweise und schreibt je Stufe die Leistung ins Client-Log. Damit findet man am TV die Dichte, ab der die Xbox einbricht.

## Beteiligte und Zielgruppen

🧑 startet die Szene per Kachel oder Link auf Xbox, Pi und PC; Claude liest `logs_query` und `report_read`. Nutzer von B-762, B-763, B-764 (RX1) und PF1.

## Anforderungen

**Seite und Start**
- `stress.html` (Werkzeug-Kachel auf `dev.html`, Abschnitt Performance), eigene Phaser-Szene `StressScene`, **ohne Server**; sie zeichnet mit dem echten Renderer und den echten Papierschnitt-Sheets.
- URL-Parameter: `?scenario=<name>` (Standard `typisch`), `?players=1|2` (überschreibt die Datei), `?ramp=0|1`, `?seed=<n>`; kombinierbar mit `?renderer=`, `?aa=`, `?perf=1`. Ohne Tastatur: Kacheln für beide Presets und 1/2 Spieler, Start per Gamepad (A).

**Szenario-Datei** `data/stress.json` (Einzige Quelle, Werte gehören nach `data/`; Prüfung im Test):
```json
{
  "typisch": {
    "name": "Typisch",
    "players": 1,
    "camera": { "x": 0, "zoom": 1, "pan": "none|sweep" },
    "enemies": [ { "type": "greed", "count": 12 }, { "type": "wolf", "count": 6 } ],
    "troops":  [ { "type": "archer", "count": 8 } ],
    "sprites": { "mix": "typical", "distinctSheets": 12 },
    "ramp": { "everyMs": 10000, "steps": 6, "factor": 1.5 }
  }
}
```
- Felder: `name`; `players` 1|2; `camera` (Start-X in Units, Zoom, Pan `none` oder `sweep`; bei 2 Spielern je Viewport eine Kamera); `enemies[]` und `troops[]` mit `type` (Schlüssel aus `enemies.json`/`troops.json`) und `count`; `sprites.mix` (`typical` = Verteilung wie in einer Nacht, `worstCase` = gleichmäßig über alle Sheets) und `distinctSheets` (Zahl verschiedener Papierschnitt-Assets, steuert die Texturlast); `ramp`.
- Unbekannter Typ, `players` ≠ 1|2, `count` < 0 oder Datei ohne Preset → Fehlermeldung beim Laden, Szene startet nicht.

**Rampe**
- Alle `ramp.everyMs` (Standard 10 000 ms) wird die Last um `factor` erhöht (`count` je Eintrag, aufgerundet), höchstens `steps` Stufen; danach Halten und Ende nach einer Schlussstufe (10 s).
- Positionen und Typ-Mischung nur aus `engine/rng`-gleichem Seed-Generator (`?seed`), kein `Math.random()`; gleicher Seed → gleiche Szene auf allen Geräten.
- Figuren bewegen sich auf festen Bahnen (Laufen/Idle abwechselnd), damit Animationen und Sheet-Wechsel mitgemessen werden.

**Messung und Log** (je Stufe eine Zeile, Emoji 💓 Verlauf, 🚀 Start, ✅ Ende; Mengen im Rate-Limit)
- Felder je Stufen-Zeile: `device`, `scenario`, `step`, `players`, `entities` (Zahl gezeichneter Figuren), `sheets` (verschiedene Texturen), `fps` (Ø und min), `frameP95Ms`, `textureMiB`, `renderer` (WebGL2/WebGL/Canvas).
- Startzeile mit Renderer, GPU, `maxTex`, Größe, `devicePixelRatio` (aus `readRenderInfo`/B-762, keine zweite Messung); Endzeile mit der größten Stufe, die ≥ 30 FPS hielt („Dichte-Grenze“), und der ersten Stufe mit Kontextverlust, falls eintritt.
- Mit `?perf=1` zusätzlich Bericht über `sendPerfReport` (`Histogram`, Einbrüche); Stufen stehen als Reihe im Bericht.
- Technik-Anzeige (B-761) läuft in der Szene mit, zusätzlich Zeile „Stufe n/N, Figuren x“.

**Objektive Auswertung (Claude bewertet allein aus dem Log, ohne Blick auf den TV)**
- Jeder Lauf hat eine `runId` (Gerät, Szenario, Seed, Startzeit) in allen Zeilen, damit Läufe getrennt und verglichen werden können; die Startzeile nennt zusätzlich Client-Version, User-Agent, Szenario-Datei-Hash und alle URL-Parameter.
- Stufen-Zeile zusätzlich: `durationMs`, Frames gezählt, FPS min/p5/Ø, Frame-Zeit Ø/p50/p95/p99/max, Zahl Frames über 50 ms (Einbrüche) und über 33 ms, längste Lücke, `textureMiB` und Texturzahl, JS-Heap (wo der Browser es liefert), gezeichnete Objekte und Draw-Calls (wo Phaser sie liefert), je Spieler-Viewport Figuren im Bild.
- Die Aufwärm-Zeit (erste 2 s nach jedem Stufenwechsel) wird getrennt markiert und nicht in die Perzentile der Stufe gerechnet, damit Ladespitzen die Messung nicht verfälschen.
- Ereignisse mit Zeit seit Start: Kontextverlust und -wiederherstellung, Canvas-Fallback, Textur-Ladefehler, Tab im Hintergrund (Pause), Abbruch, jeweils mit Stufe.
- Endzeile `✅ Ergebnis` als eine maschinenlesbare Zeile: Dichte-Grenze (Stufe und Figuren), Gründe für das Ende (Ziel erreicht, Kontextverlust, Abbruch), Zusammenfassung je Stufe als Tabelle; mit `?perf=1` dieselben Werte als Bericht in `reports/`.
- Ohne Server-Zeitstempel allein reicht die Zeit seit Start nicht: jede Zeile trägt die Client-Zeit; Claude liest über `logs_query` nach `runId`.
- Mengenbudget: höchstens eine Zeile je Stufe plus Ereigniszeilen, keine Zeile je Frame (Rate-Limit von `/api/clientlog`).

**Presets**
- **Typisch** (`typisch`): 1 Spieler, Nachtwelle wie im Spiel: Start 12 Gegner (Greed, Wolf), 8 Truppen, `mix: typical`, `distinctSheets: 12`, 6 Stufen × 1,5.
- **Worst Case** (`worstCase`): 2 Spieler (Split-Screen), Start 40 Gegner über alle Gegnertypen, 20 Truppen, `mix: worstCase`, `distinctSheets` = alle Papierschnitt-Assets der Gegner und Truppen, 8 Stufen × 1,5, Kamera `sweep` über die Stufe. Die Zahlen sind Startwerte und werden nach dem ersten Xbox-Lauf (RX1.4) angepasst.
- Beide Presets mit je `players: 1` und `players: 2` startbar (URL-Parameter).

## Nicht-Ziele

Kein Server, keine Simulation, keine Netz-/Snapshot-Messung (das wäre ein Dev-Spawn-Befehl im Raum, Variante B, eigenes SRV-Ticket); keine Leistungsoptimierung; keine neue Grafik; keine Werte für Balancing.

## Regeln und Einschränkungen

Kein Spiel-Zustand im Client-Spiel: die Szene ist ein Werkzeug und liegt nicht im Spielfluss. `installPageChrome()`, Vollbild nur über `toggleFullscreen()`, B nicht belegt, View+Menu = Home. Phaser-freie Logik (Szenario-Prüfung, Rampe, Stufen-Zusammenfassung) mit Vitest-Tests. Datei ≤ 400, Funktion ≤ 60 Zeilen. Keine neue Abhängigkeit.

## Beispiele

`stress.html?scenario=worstCase&players=2&perf=1` auf der Xbox → 8 Stufen à 10 s, Log „💓 Stufe 5: 180 Figuren, 41 Sheets, FPS Ø 22, p95 58 ms, 380 MiB“, Endzeile „✅ Dichte-Grenze: Stufe 3 (96 Figuren)“.

## Ausnahme- und Fehlerfälle

Unbekanntes Preset oder Typ → Fehlertext im Bild und Log (🚫). Kontextverlust während der Rampe → Stufe und Zustand loggen (B-763 greift), Lauf endet mit Hinweis. Tab im Hintergrund → Rampe pausiert wie die Messung in PM1. `?renderer=webgl` ohne WebGL → Phaser bricht ab (gewollt).

## Akzeptanzkriterien

- **AC-01** Test: `data/stress.json` enthält `typisch` und `worstCase`; alle Typen existieren in `enemies.json`/`troops.json` und haben einen Eintrag in `data/paper.json` und `index.paper.json`; `players` ist 1 oder 2.
- **AC-02** Test: Ungültige Szenarien (unbekannter Typ, `players: 3`, negative Anzahl) werden mit Fehler abgelehnt.
- **AC-03** Test: Die Rampe erhöht die Last alle 10 s um den Faktor, höchstens `steps` Stufen; gleicher Seed → gleiche Verteilung.
- **AC-04** Test: Jede Stufe erzeugt genau eine Log-Zeile mit `device`, FPS, Frame-Zeit p95, `textureMiB`, `entities`, `sheets`.
- **AC-05** Test: Die Endzeile nennt die Dichte-Grenze (größte Stufe ≥ 30 FPS) korrekt, trägt `runId` und die Zusammenfassung je Stufe; die Aufwärm-Zeit zählt nicht in die Perzentile (Test).
- **AC-06** `stress.html` ist als Kachel in `devTiles.ts` eingetragen und nutzt `installPageChrome()` (`tests/projectRules.test.ts` grün).
- **AC-07** Abnahme (🧑): Beide Presets laufen mit 1 und 2 Spielern am PC und auf der Xbox, Log im Pi lesbar, Dichte-Grenze notiert.

## Offene Fragen

keine (am 2026-11-09 vorgeschlagen und entschieden, siehe unten; 🧑 kann widersprechen)

### Entscheidungen

- **E-1 Eigene Seite `stress.html`, kein Modus in `game.html`.** Gründe: Die Szene ist serverlos, `game.html` startet Lobby, Verbindung und Raum und verfälscht damit Start- und Ladezeiten. Eine eigene Seite hält die Messung isoliert (nur `StressScene` und Renderer), lässt sich per Kachel und Link starten und ist als Werkzeug-Seite durch `projectRules.test.ts` abgedeckt. Gemeinsam genutzt werden nur Renderer, Papierschnitt-Lader und Messbausteine.
- **E-2 Preset-Zahlen darf Claude nach dem ersten Xbox-Lauf (RX1.4) ohne neue Freigabe anpassen.** Rahmen: nur Zahlen in `data/stress.json` (Anzahl, Stufen, Faktor, `distinctSheets`), keine neuen Felder, keine neuen Presets, keine Änderung der Preset-Namen. Jede Anpassung steht mit alter und neuer Zahl und dem Log-Wert (`runId`) als Begründung im Commit und in den Notizen. Neue Presets oder Felder brauchen eine Freigabe durch 🧑.
- **E-3 Gegner und Truppen haben ein Asset (geprüft 2026-11-09).** `data/paper.json` ordnet alle 17 Gegnertypen aus `enemies.json` (`greed` bis `crystalGuardian`) und alle 6 Truppentypen aus `troops.json` (`vagrant` bis `eliteWarrior`) einem Asset zu; die Ordner liegen unter `public/grafik/gegner/` und `public/grafik/truppen/` und stehen in den Gruppen-Indizes. **Lücke:** Bosse stehen nicht in dieser Zuordnung (eigene Gruppe `public/grafik/bosse/`, `data/bosses.json`); die Preset-Gegnerlisten enthalten deshalb zunächst keine Bosse, ein Boss-Preset wäre Folgearbeit. Da PNG und Manifeste nicht eingecheckt sind (B-737), prüft der Test in AC-01 die Zuordnung gegen `data/paper.json` und `index.paper.json`, nicht gegen vorhandene PNG; die Stress-Szene braucht vor dem Start `task gen:paper` (`task dev`/`build`/`test` rufen es selbst auf).

## Notizen

Eingeplant als RX1.3, vor dem Xbox-Lauf RX1.4 (der misst dann gleich mit). Variante B (Dev-Spawn im Raum, SRV) bleibt ein Folge-Ticket.

### Einplanung und Voraussetzungen

- **Sprint:** RX1, Session RX1.3 (Domäne CLI), Reihenfolge RX1.1 → RX1.2 → **RX1.3** → RX1.4 (Xbox-Lauf) → RX1.5 → RX1.6 → RX1.7.
- **Abhängigkeiten:** Setzt RX1.1 voraus (Render-Log, `renderInfo.ts`, `clientLog`); RX1.4 (Xbox-Lauf, 🧑) setzt RX1.3 voraus, damit der Lauf mit der Stress-Szene misst.
- **Voraussetzung vor jedem Start (Entwicklung, Test und Lauf auf Xbox/Pi): `task gen:paper`.** PNG und Manifeste sind nicht eingecheckt (B-737); ohne sie fehlen der Szene die Sheets. `task dev`, `task build` und `task test` rufen es selbst auf, `task check:diff` nicht (B-742). Der Pi-/Xbox-Lauf braucht die neu erzeugten PNG ausgeliefert.
- **Kriterien-Zuordnung im Sprint:** `B-765/AC-01` bis `AC-06` = RX1 `AC-11` bis `AC-16`, `B-765/AC-07` = RX1 `AC-17`.
