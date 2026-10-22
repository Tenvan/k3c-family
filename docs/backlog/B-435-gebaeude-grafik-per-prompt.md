# B-435 · Gebäude und Strukturen entstehen im Asset-Manager aus Beschreibung und Prompt

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** GEB1
- **Projekt:** GEB
- **Erstellt:** 2026-10-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-16, Chat (Ralf), mit Sprint GEB1 Revision 1

## Ausgangslage

Gebäude und Strukturen bestehen heute aus gemischten Grafiksammlungen (`public/grafik/`, über 40 Packs mit eigenen Paletten und Pixeldichten). Proportionen und Erweiterbarkeit sind nicht gegeben. Ein Entwurf der Werkstatt (Handelsstand, Vorderansicht, 192×128 px = 6×4 Kacheln à 32 px, DB32, mit Gestell rechts für abholbereite Hämmer und Bögen) wurde als Skript erzeugt und von 🧑 am 2026-10-15 als Stilvorlage angenommen (Chat, Ralf).

## Ziel

Gebäude und Strukturen werden im Asset-Manager (k3c-workbench) geplant: Aus Beschreibung und Prompt entsteht die Grafik dort und wird präsentiert, statt eine fertige Grafik auszuwählen. Alle Gebäude folgen denselben Grundregeln, die auf dem Werkstatt-Entwurf beruhen und mit jedem Gebäude wachsen.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Vor dem Erzeugen gelten Grundregeln (Raster, Maße je Gebäude in Kacheln, Palette DB32, Licht von links oben, Holz-/Stoff-Materialsprache, Bausteine wie Pfosten, Tuch, Tresen, Gestell), abgeleitet aus der Werkstatt.
- Ein Gebäude ist Daten plus Bausteine: Ausbaustufen und Level-Looks entstehen über Paletten und Zusätze, nicht durch Neuzeichnen.
- **Dichte-Regel:** Die Quelle speichert Formen in Kachel-Einheiten, die Pixeldichte ist ein Render-Parameter. Master-Dichte 32 px je Kachel (`UNIT_PX`); feinere Dichten (×2, ×4) entstehen verlustfrei, die gröbere (×½) nur mit vereinfachter Detail-Ebene; nur ganzzahlige Verhältnisse, kein Anti-Aliasing, feste DB32-Farben.
- Ausgabe ist PNG ohne Glättung; Daten (JSON) und Build-Skript bleiben als Quelle im Repo.
- Der Asset-Manager nimmt Beschreibung und optional ein Referenzbild und zeigt das Ergebnis zur Abnahme.

## Nicht-Ziele

Figuren und Gegner (GR7, GR8); Animationen; die allgemeine Palette-Pipeline für fremde Packs (B-331, GR8).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

- Werkstatt (Handelsstand, Vorderansicht, 6×4 Kacheln, Gestell rechts für abholbereite Hämmer und Bögen): Stilvorlage, angenommen 2026-10-15.
- Bäume: drei schmale Arten (Tanne, Pappel, Birke), je 1 Kachel breit (32×72 px), stehen überlappend in Reihen als kleiner Wald wie in K2C. Fürs Erste angenommen 2026-10-15; genaue Varianten (mehr Formen je Art, Höhen, Level-Paletten) feilt 🧑 später aus.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

- Entschieden 2026-10-15 (Chat, Ralf): Skript-Generator im Repo. Beschreibung → Daten (JSON) → Build-Skript → PNG.
- Entschieden 2026-10-15 (Chat, Ralf): Ablage `public/grafik/gebaeude/` (PNG, Daten-JSON daneben), Build-Skript in `scripts/`.
- Entschieden 2026-10-15 (Chat, Ralf): Dichte-Regel (siehe Anforderungen) und Projekt GEB auf Rang 3 vor GRA und SND.
- Offen: Skalierung am TV (Faktor, Geltung für Gebäude, Pixelgröße relativ zu den Figuren), siehe GEB1 › Offene Fragen.

## Notizen

### Umfang laut Slot-Register (`data/assets.json`, Stand 2026-10-15)

Quelle: `assets_list` Kategorien `gebaeude`, `objekte`, `umgebung`. **Lücke** = heute ohne Grafik, **gemischt** = Grafik aus einem fremden Pack.

**Gebäude (14 Typen, `data/buildings.json`)**

| Slot | Gebäude | Stand heute | Stufen |
|---|---|---|---|
| `castle` | Burg/Thron (Hub-Kern) | gemischt | über Hub-Stufen 1–5 |
| `workshop` | Werkstatt | eigen (GEB2, alte Grafik gelöscht) | **Stilvorlage fertig**, Stufen offen |
| `storage` | Lager | eigen (GEB5, alte Grafik gelöscht) | – |
| `farm` | Farm | eigen (GEB5, alte Grafik gelöscht) | – |
| `barracks` | Kaserne | eigen (GEB5, alte Grafik gelöscht) | – |
| `tavern` | Taverne | **Lücke** | – |
| `healer` | Heilplatz | **Lücke** | – |
| `smithy` | Schmiede | **Lücke** | – |
| `armory` | Rüstkammer | **Lücke** | – |
| `gate` | Tor | gemischt | Stufen über Mauer-Linien |
| `stairsUp` / `stairsDown` | Treppe hoch / runter | eigen (GEB5, alte Grafik gelöscht) | – |
| `wall` | Mauer | gemischt | Stufen 1–5; `wall:1` **Lücke** |
| `tower` | Turm | gemischt | Stufen 1–5; `tower:1` und `tower:5` (Zaubertum) **Lücke** |

**Hub-Stufen:** `hub:1` bis `hub:5` (Burg-Ausbau, aus drei fremden Packs gemischt).

**Strukturen und Objekte**

| Slot | Struktur | Stand heute |
|---|---|---|
| `node:tree` | Bäume | gemischt; **Beispiel Tanne/Pappel/Birke fertig** |
| `plantation` | Farm-Plantage (nachwachsende Bäume) | eigen (GEB5): Setzling, gewachsen Pappel |
| `node:rock`, `node:copperOre`, `node:bush` | Fels, Erzbrocken, Busch | gemischt |
| `vein:stone`, `vein:copper`, `vein:iron`, `vein:crystal` | Adern je Mine-Stufe | gemischt |
| `portal`, `exit` | Portal, Ausgang | gemischt |
| `pickup:chest` | Truhe | gemischt |
| `camp:recruit` | Rekrutierungslager (Zelt) | gemischt |

**Umgebung (nur falls 🧑 will):** Hintergründe `bg:forest`, `bg:cave`, `bg:mine` gemischt; `bg:ironhold` und `bg:crystal` **Lücke**.

Nicht Teil: Münzen, Icons, Items, HUD (eigene Slots, keine Gebäude).

### Zählung

14 Gebäudetypen, davon 4 Lücken (Taverne, Heilplatz, Schmiede, Rüstkammer); 5 Hub-Stufen; je 5 Mauer- und Turm-Stufen (3 Lücken: `wall:1`, `tower:1`, `tower:5`); 13 Slots für Strukturen und Objekte (7 Zeilen der Tabelle), davon 1 Lücke (Plantage).
