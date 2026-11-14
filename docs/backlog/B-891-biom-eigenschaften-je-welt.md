# B-891 · Biome haben einstellbare Eigenschaften (Walddichte, Steindichte, Baumarten-Mischung), aus dem Seed gewürfelt oder im Welten-Editor fest gesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** BI1
- **Projekt:** NAT
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** Ralf, 2026-11-14 (Defaults der offenen Fragen; Erweiterung auf alle Biome)
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Die Biome (`data/biomes/*.json`) legen die Natur nur über `resourcesPerChunk` fest (z. B. `"tree": [0, 1]`, `"rock": [2, 4]` je Chunk-Typ). Es gibt eine Baumart `tree`, keine Dichte- oder Mischungsregler. Zwei Welten mit anderem Seed sehen deshalb im selben Biom fast gleich aus. Der Welten-Editor (B-879) zeigt Chunks, setzt aber keine Natur-Eigenschaften.

## Ziel

Jedes Biom bietet Eigenschaften (Walddichte, Steindichte, Baumarten-Mischung usw.), deren Wert je Welt aus dem Seed gewürfelt oder im Welten-Editor fest gesetzt wird. So reicht die Spanne vom lichten Monokultur-Wald bis zum dichten Mischwald.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

### Eigenschaften je Biom (Erweiterung freigegeben von Ralf, 2026-11-14: gilt für alle Biome)

Neutraler Wert (`default`, Faktor 1): bei min 0 der Wert 1, bei min 1 die Mitte. Wirkung je Eigenschaft genau eine: Anzahl von Objekten (`scales`), Gewicht von Chunk-Arten (`chunks`) oder nur Darstellung (`display`).

| Biom | Eigenschaft | Bereich | Wirkung |
|---|---|---|---|
| Wald (forest) | Walddichte `forestDensity` | 1–5 | Bäume |
| | Steindichte `rockDensity` | 0–3 | Felsen |
| | Buschdichte `bushDensity` | 0–3 | Büsche |
| | Lichtungen `clearings` | 0–3 | Chunk-Gewicht clearing |
| | Bachläufe `streams` | 0–2 | Chunk-Gewicht stream |
| | Baumarten `treeSpecies` | Gewicht 0–5 je Art (tanne, pappel, birke) | Art je Baum |
| Höhle (cave) | Felsdichte `rockDensity` | 1–5 | Felsen |
| | Deckenhöhe `ceilingHeight` | 1–3 | Darstellung |
| | Pilze und Höhlenbäume `mushrooms` | 0–3 | Höhlenbäume und Büsche |
| | Tropfsteine `stalactites` | 0–3 | Darstellung |
| | Abgründe `chasms` | 0–2 | Chunk-Gewicht chasm |
| | Steinadern `stoneVeins` | 1–3 | Adern |
| Mine (mine) | Kupferader-Dichte `copperDensity` | 1–5 | Kupfererz und Kupfer-Adern |
| | Schienen-Anteil `railShare` | 0–3 | Chunk-Gewicht rails |
| | Stützbalken und Verfall `decay` | 0–3 | Darstellung |
| | Geröll `rubble` | 0–3 | Felsen |
| | Schächte `shafts` | 0–2 | Chunk-Gewicht shaft |
| Eisenstollen (ironhold) | Eisenader-Dichte `ironDensity` | 1–5 | Eisen-Adern |
| | Lava-Anteil `lavaShare` | 0–3 | Chunk-Gewicht lava |
| | Schmieden `forges` | 0–2 | Chunk-Gewicht forge |
| | Hitze und Glut `glow` | 0–3 | Darstellung |
| | Galerie-Breite `galleryWidth` | 1–3 | Chunk-Gewicht gallery |
| Kristallhöhle (crystal) | Kristalldichte `crystalDensity` | 1–5 | Kristall-Adern |
| | Kristallfarben `crystalColors` | Gewicht 0–5 je Farbe (blau, violett, gruen) | Farbe je Ader |
| | Geoden `geodes` | 0–3 | Chunk-Gewicht geode |
| | Leuchtstärke `brightness` | 1–3 | Darstellung |
| | Grotten-Anteil `grottoShare` | 0–3 | Chunk-Gewicht grotto |

Gewichts-Eigenschaften (Baumarten, Kristallfarben) funktionieren gleich: je Variante ein Gewicht 0–5, Wahrscheinlichkeit = Gewicht / Summe; Monokultur = eine Variante > 0.

### Datenmodell
- `data/biomes/<biom>.json`: `traits` je Eigenschaft `{name, min, max, default, scales | chunks | display}`, `weights` je Gruppe `{name, kind, options: {<variante>: {min, max}}}`. Die Liste steht in den Daten, nicht im Code.
- Welt (`data/worlds/<welt>.json`) je Insel: `nature` (Vorgabe der Insel für alle Stufen, deren Biom die Eigenschaft hat) und `natureByDepth` (je Tiefe, gewinnt); Form je Eintrag `{traits: {…}, weights: {<gruppe>: {…}}}`. Ohne Eintrag würfelt der Level-Generator den Wert aus dem Seed (`engine/rng`, eigener Strom `<biom>:<seed>:nature`) in `min..max`.
- Der Level-Generator (`engine/level`) skaliert Bereiche in `resourcesPerChunk`, die Zahl der Adern und die Chunk-Gewichte mit Wert/Default und wählt die Variante je Objekt nach den Gewichten (Strom je Ort, `<biom>:<seed>:variant:<gruppe>:<x>`). Geplante Chunks würfeln ohne Faktoren (Chunk-Seed bleibt gleich).
- Der Spielstand speichert die geltenden Werte je Stufe (`nature`), das Laden baut damit dieselbe Natur.

### Welten-Editor
- Zeigt je Insel/Stufe die geltenden Werte, erlaubt Festsetzen und Zurücksetzen auf „aus Seed“; die Chunk-Vorschau zeigt das Ergebnis.

### Grafik
- Baumarten nutzen die vorhandenen Papierschnitt-Bäume (tanne, pappel, birke). Kristallfarben brauchen Assets (PAP-Folge-Ticket). Reine Darstellungs-Eigenschaften (Deckenhöhe, Tropfsteine, Stützbalken und Verfall, Hitze und Glut, Leuchtstärke) bekommen je ein Folge-Ticket für die Darstellung.

## Nicht-Ziele

Balancing der Erträge (gesperrt bis auf Widerruf); neue Biome.

## Regeln und Einschränkungen

Determinismus nur über `engine/rng`; keine Map-Iteration ohne Sortierung. Balancing der Erträge bleibt gesperrt (Beschluss 2026-11-11): Die Dichte ändert nur die Anzahl der Objekte, nicht den Ertrag je Objekt. Golden-Änderungen über `task go:golden` mit Begründung im Commit.

- **Bereich:** `data/biomes/**`, `engine/level/**`, `engine/sim/island*.go`, `engine/sim/save.go`, `engine/sim/types.go`, `engine/sim/world.go`, `engine/sim/campaign.go`, `cmd/k3c-sim/world.go`, `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/frontend/src/world/**`, `native/k3c-game/props/nature.go`, `native/k3c-game/props/plan.go`, `testdata/golden/**`, `docs/rules/welten.md`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Gleicher Seed und gleiche Welt ergeben dieselben Werte und dieselbe Natur (Golden-Test).
- **AC-02** Zwei Seeds ergeben im selben Biom messbar verschiedene Walddichte oder Mischung (Test).
- **AC-03** Ein fest gesetzter Wert in der Welt gilt unabhängig vom Seed (Test).
- **AC-04** Monokultur (eine Art Gewicht > 0) erzeugt nur diese Art; Gewichte 0 überall lehnt die Datenprüfung ab.
- **AC-05** Der Welten-Editor zeigt und setzt die Werte; Sichtprüfung durch 🧑.

## Offene Fragen

Entschieden mit den Defaults (Freigabe Ralf, 2026-11-14):

- Baumarten: die vorhandenen Papierschnitt-Bäume tanne, pappel, birke.
- Wert je Stufe, mit Vorgabe je Insel (`nature` und `natureByDepth`).
- Die Dichte ändert nur die Anzahl der Objekte; Golden-Werte ändern sich (`task go:golden`).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
