# B-161 · Jedes Spielobjekt hat eine Zuordnung zu Asset und Lizenz oder eine dokumentierte Lücke

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Status:** eingeplant
- **Sprint:** GR1
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bestandsaufnahme (Stand 2026-10-02):

- **Umgebungs-Packs:** zwölf Packs unter `public/grafik/` (elf CC0 1.0, Warped Caves CC BY 3.0), 119 PNG, ebenso viele Einträge in `public/grafik/index.json` (Gruppen: tileset-einzeln 37, ebenen 24, props 19, icons 11, props-einzeln 10, umgebung 9, vorschau 4, ressourcen 2, muenzen 2, portale 1; größtes Pack gothicvania-town mit 53). Lizenzen in `public/grafik/CREDITS.md`. Die Packs sind nur auf `grafiken.html` ansehbar; kein Spiel-Code lädt `grafik/` (nur `src/tools/grafiken.ts`).
- **Figuren:** 41 Ordner mit 110 PNG unter `public/sprites/` (Lizenzen in `public/sprites/CREDITS.md`). `data/sprites.json` hat Einträge für alle 6 Truppen (`data/troops.json`), alle 10 Gegner (`data/enemies.json`), 4 Spieler und 13 Reittiere. Diese Spielobjekte haben Grafik; Bosse gibt es in den Daten noch nicht.
- **Gebäude:** `data/buildings.json` kennt 10 Arten (castle, wall, tower, gate, workshop, storage, farm, barracks, stairsUp, stairsDown); alle werden heute als Rechtecke gezeichnet (`src/scenes/worldRenderer.ts`, Bauplatz-Zeichnung um Zeile 229–256), keine ist einem Asset zugeordnet.
- **Ohne Treffer im Bestand** (laut B-010 und `docs/funde/b010-grafik-funde.html`): Mine-Hintergrund (`data/biomes/mine.json`), Rekrutierungslager (`data/economy.json` › `recruitCamp`), Werkstatt, Farm, Kaserne, Treppen (stairsUp, stairsDown).
- **Weder zugeordnet noch geprüft:** Hub-Stufen 1–5 und Mauer-/Turm-Materialstufen (B-112), die fünf Materialien Holz, Stein, Kupfer, Eisen, Kristall, Adern, Plantage, Truhen (`economy.json` › `chestGold`), Portale, Münzen, UI- und Skill-Icons, Hintergründe je Biom und Tiefe (Wald, Höhle, Mine), Bosse.

Eine Zuordnungstabelle Spielobjekt → Asset → Lizenz fehlt.

## Ziel

Eine Tabelle ordnet jedes Spielobjekt einem Asset (Pack, Datei, Frame), seinem Stil und seiner Lizenz zu oder markiert eine Lücke. Nutzen: GR2 (Suche) und GR3 (Einbau) arbeiten gegen eine gemeinsame Liste; Stilbrüche fallen vorher auf.

## Beteiligte und Zielgruppen

🧑 entscheidet den Grafik-Stil (Q13 in `docs/fragenkatalog.md`) und gibt die Zuordnung frei; der Agent erstellt die Tabelle; Spieler am TV profitieren später vom einheitlichen Aussehen.

## Anforderungen

- Datei `docs/assets/zuordnung.md` (neu): je Spielobjekt eine Zeile mit Spalten Objekt, Herkunft in `data/` oder Regel, Pack, Datei/Frame, Stil, Lizenz, Status (zugeordnet oder Lücke). Lücken sind fett markiert („keine Grafik“, „Stil passt nicht“).
- Umfang: Gebäude je Hub-Stufe, Mauer/Turm je Materialstufe, Ressourcen, Truhen, Adern, Plantage, Portale, Gegner je Trait und Boss, Truppen je Beruf, Reittiere, UI- und Skill-Icons, Hintergründe je Biom und Tiefe.
- Stil-Spalte mit Grundraster (16 oder 32 px), Palette und Skalierung; Nachbearbeitung nur Skalieren und Palette (Q13).
- Test (Vorbild `src/tools/grafikPacks.test.ts`): jede Objekt-ID aus `data/buildings.json`, `data/enemies.json` und `data/troops.json` hat einen Eintrag; jede Lücke verweist auf ein Ticket; jedes zugeordnete Asset hat einen Eintrag in `public/grafik/CREDITS.md` oder `public/sprites/CREDITS.md`.

## Nicht-Ziele

Suche nach neuen Assets (B-162), Einbau in den Renderer (B-010), Atlas (B-163), neue Grafiken für Mechaniken, die es noch nicht gibt (Bosse, Reittier-Mechanik B-152): nur als „Lücke, später“ eintragen.

## Regeln und Einschränkungen

Nur CC0 oder CC-BY, Credits sofort in `public/*/CREDITS.md`; `src/scenes` zeichnet nur Snapshots; Seiten-Regeln aus `CLAUDE.md`; Datei ≤ 400 Zeilen (Tabelle bei Bedarf nach Gruppen teilen).

## Beispiele

`wall` Stufe 1 (Holz) → Pack gothicvania-town, Datei und Stil eingetragen, CC0, zugeordnet. `workshop` → **keine Grafik**, Ticket B-162.

## Ausnahme- und Fehlerfälle

Asset ohne Credit-Eintrag → Test rot. Objekt in `data/` ohne Zeile → Test rot. Stil eines Packs passt nicht → Status „Lücke, Stil passt nicht“, nicht stillschweigend übernommen.

## Akzeptanzkriterien

- **AC-01** `docs/assets/zuordnung.md` enthält zu jeder Objekt-ID aus `data/buildings.json`, `data/enemies.json` und `data/troops.json` eine Zeile mit Status zugeordnet oder Lücke (Test).
- **AC-02** Die Tabelle deckt zusätzlich Hub-Stufen 1–5, Mauer-/Turm-Materialstufen, die fünf Materialien, Adern, Plantage, Truhen, Portale, UI-/Skill-Icons und Hintergründe je Biom (Wald, Höhle, Mine) ab.
- **AC-03** Jedes zugeordnete Asset hat einen Eintrag in `public/grafik/CREDITS.md` oder `public/sprites/CREDITS.md` (Test).
- **AC-04** Jede Lücke steht fett markiert in der Tabelle und verweist auf ein Ticket (Test).
- **AC-05** Jede Zeile nennt Stil (Raster, Palette, Skalierung) und Lizenz; der Stilbeschluss von 🧑 (Q13) steht im Kopf der Datei.

## Offene Fragen

- Welcher Grundstil gilt (Raster 16 oder 32 px, Palette, Skalierung)? Entscheidet 🧑, `docs/fragenkatalog.md` Q13.

## Notizen

Stilrisiko laut Plan: gemischte Asset-Packs; Gegenmittel ist die Stil-Spalte. Quelle: `docs/plan-weiterentwicklung.md` Schiene G, GR1.
