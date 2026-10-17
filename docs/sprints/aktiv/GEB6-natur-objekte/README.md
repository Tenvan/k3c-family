# GEB6 · DEV, CLI · Natur und Objekte: Bäume, Fels, Erz, Adern, Truhe, Portal, Rekrutierungslager

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-435, B-438
- **Start-Commit:** 554d478c
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-16, Ralf, Chat: Umsetzung auf Annahmen, offene Fragen nach Abschluss (Rev. 1)

## Ausgangslage

Bäume, Fels, Erz, Busch, Adern, Truhe, Portal, Ausgang und Rekrutierungslager stammen aus gemischten 16-px-Packs und stehen im Spiel meist in Maßstab 2 (`src/scenes/worldSprites.ts`, `objectView.ts`); die Register-Wahl steht in `data/assets.json`. GEB1 hat Tanne, Pappel und Birke (je 1 × 2,25 Kacheln) als Beispiel mit `tools/gebaeude` erzeugt (`public/grafik/gebaeude/`, Regeln `docs/standards/gebaeude-grafik.md`); GEB3 bis GEB5 haben Stufen (§ 6) und Maßstab 1 im Spiel eingeführt, GEB8 die Vorlage (§ 5).

| Slot | Datei unter `public/grafik/` | Pixel | im Spiel |
|---|---|---|---|
| `node:tree` | `gotthicvania-swamp/umgebung/trees.png` (ein Bild) | 288 × 208 | ×2 |
| `node:rock` | `various-stones-and-oregem-veins-16x16/ressourcen/Stones_ores_gems_without_grass.png`, Frame 1 | 16 × 16 | ×2 |
| `node:copperOre` | dasselbe Blatt, Frame 7 | 16 × 16 | ×2 |
| `node:bush` | `sunnyland-tall-forest-environment/props/Plant.png` | 42 × 27 | ×2 |
| `vein:stone`, `vein:copper`, `vein:iron`, `vein:crystal` | dasselbe Erz-Blatt, Zeile 1, 2, 3 und 9 (je 5 Spalten) | 80 × 16 | im Register, im Spiel **ungenutzt** (siehe unten) |
| `pickup:chest` | `gold-treasure-icons-16x16/icons/8.png` (offene Truhe mit Gold) | 16 × 16 | ×2 |
| `portal` | `portals-32-x-48/portale/portalsSpriteSheet.png`, 4 Frames animiert | 32 × 48 | ×1 |
| `exit` | `k3c-paletten/props/fort-door.png` | 96 × 80 | ×2 |
| `camp:recruit` | `tent-8/props/objs.png`, Ausschnitt `tent`; dazu `camp:recruit:fire` aus `16x16-animated-campfire/props/campfire-16x16.png` (4 Frames, nur in `worldSprites.ts`) | 63 × 79 und 16 × 16 | ×2 |

- **Adern im Spiel (belegt, `worldRenderer.ts` `createNode`):** Adern sind `ResourceNode` mit den Arten `stoneVein`, `copperVein`, `ironVein`, `crystalVein` (`data/economy.json` › `veins`; je Biom ein Material: Höhle Stein, Mine Kupfer, Eisenstollen Eisen, Kristallhöhle Kristall, `data/biomes/*.json`). `worldRenderer.createNode` fragt `node:<Art>`; für `stoneVein` usw. gibt es keinen Eintrag, die Ader erscheint daher als grüner Platzhalter-Kreis, die Slots `vein:*` fragt niemand ab.
- **Bäume im Spiel:** `node:tree` zeigt für jeden Baum dasselbe Bild. Bäume kommen in Wald-Chunks (4–7 je Chunk) und vereinzelt in der Höhle (Kaverne, 0–1) vor; Fels in allen Biomen, Erzbrocken (`copperOre`) in Mine, Eisenstollen und Kristallhöhle, Busch (Deko) nur im Wald (`data/biomes/*.json › resourcesPerChunk`).
- **Animationen:** Portal und Lagerfeuer sind heute 4-Frame-Animationen (`prepareWorld`, `objectView.ts`).
- **Mitnutzer (bei der Planung belegt, 2026-10-16):** Das Erz-Blatt dient auch den Slots `material:*`, `icons/8.png` auch dem Slot `icon:8`; `k3c-paletten` enthält nur `fort-door.png`; `portals-32-x-48`, `tent-8` und `16x16-animated-campfire` werden nur von den Slots dieses Sprints genutzt. Gelöscht wird nur, was bei der Umsetzung (Suche) keinen Nutzer mehr hat.
- **GEB5 parallel:** GEB5.4 ordnet die Plantage zu (Plantage-Bäume zeigen `pappel-32.png`) und ändert `worldRenderer.ts`, `buildingSprites.ts` und `data/assets.json`; GEB6.5 ändert `node:tree` an denselben Stellen.

## Ziel

Alle Strukturen und Objekte der Welt (Bäume, Busch, Fels, Erzbrocken, Adern, Truhe, Portal, Ausgang, Rekrutierungslager) haben einheitliche Grafik aus `tools/gebaeude` in Maßstab 1, mit Looks je Biom, wo ein Objekt in mehreren Biomen steht; die ersetzte Pack-Grafik ist gelöscht. Am Ende sichtbar: Vorschau je Biom und Spiel, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

🧑 entscheidet Maße, Materialien (Offene Frage 4) und Annahmen, gibt die Spec frei und nimmt Vorschau und Spiel ab; Agenten bauen Daten-JSONs, Zuordnung und Aufräumen; Spieler erkennen Ressourcen, Truhe, Portal, Ausgang und Lager auf einen Blick und sehen jedes Biom im passenden Look.

## Anforderungen

B-435 › Anforderungen und `docs/standards/gebaeude-grafik.md` (Raster § 1, Palette § 2, Vorlage § 5, Stufen § 6, Dichte § 7); B-438 (Bäume im Spiel); Aufräum-Regel aus Projekt GEB › Notizen (🧑, 2026-10-16). Sprint-eigen (Maße und Ansätze sind Annahmen, siehe Offene Fragen):
- **Biom-Look** = Farbtausch derselben Quelle über `stufen` (§ 6), keine zweite Zeichnung. Für Natur-Objekte bedeutet **Stufe den Look**: Stufe 1 Wald, Stufe 2 Höhle, Stufe 3 Mine; `aenderung` nennt ihn. Die Regeldatei hält das in § 6 fest. Ein Objekt zeigt den Look des Bioms, begrenzt auf den höchsten Look, den es hat (Eisenstollen und Kristallhöhle → Mine; Bäume haben nur Wald und Höhle, also Mine → Höhle).
- **Bäume:** Tanne, Pappel und Birke bleiben unverändert als Form 1 (Wald-Look, Dateinamen stabil, GEB5.4 nutzt `pappel-32.png`). Je Art zwei weitere Formen als eigene JSONs `tanne-2`, `tanne-3`, `pappel-2`, `pappel-3`, `birke-2`, `birke-3` (je 1 × 2,25 Kacheln, Klasse `baum`, unterscheidbarer Umriss), jede mit `stufen` für Wald und Höhle (Kronenfarben per Farbtausch). **Busch** (`busch`): 1 × 1, nur Wald-Look.
- **Fels** (`felsen`): 1 × 1, `stufen` Wald, Höhle, Mine. **Erzbrocken** (`erzbrocken`): 1 × 1, Stein mit Kupfer, ein Look (Mine).
- **Adern:** eine JSON `ader`, 2 × 2 Kacheln, `stufen` 1–4 = Stein, Kupfer, Eisen, Kristall als Farbtausch plus Zusätze (Kristallspitzen), nach § 6.
- **Truhe** (`truhe`) 1 × 1; **Portal** (`portal`) und **Ausgang** (`ausgang`) je 2 × 3; **Rekrutierungslager** (`rekrutierungslager`) 3 × 2 mit Zelt und Feuerstelle in einem Bild. Alle statisch; Portal-Flackern und Lagerfeuer-Flammen entfallen (Nicht-Ziel Animationen, Offene Frage 5).
- **Regeldatei:** Maße in § 1, Materialien in § 3 (nach Zustimmung von 🧑, Offene Frage 4), Bausteine in § 4, Biom-Look in § 6 nachtragen.
- **Spiel (CLI):** Slots `node:tree`, `node:rock`, `node:copperOre`, `node:bush`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit` in `data/assets.json` und `src/scenes/worldSprites.ts` auf die neuen PNGs in Maßstab 1.
  - Baum: Art und Form aus der **Position** (reine Funktion, kein Zufall), Look aus dem Biom; Plantage-Bäume bleiben die Pappel aus GEB5 (B-438).
  - Fels, Erzbrocken, Busch: Look aus dem Biom (`world.biome.id`).
  - Adern: Der Renderer ordnet die Ader-Arten (`stoneVein` …) den Slots `vein:<Material>` zu.
  - Markierungsfahne und Fortschrittsbalken über Knoten sowie die Beschriftung am Ausgang folgen der neuen Bildhöhe.
- **Aufräumen:** ersetzte Grafik samt Verweisen löschen, sofern nichts anderes sie nutzt: PNG unter `public/grafik/`, Einträge in `public/grafik/index.json`, Register-Wahl in `data/assets.json`, Zuordnungs-Doku (`docs/assets/zuordnung-*.md`), Test-Fixtures und Mocks, CREDITS-Zeilen; ein leerer Pack fällt ganz weg (CREDITS, `src/tools/grafikPacks.ts`, LICENSE).

## Nicht-Ziele

Animationen (Portal-Flackern, Lagerfeuer-Flammen; Folge in GRA, Offene Frage 5) und Figuren (GR7, GR8); Hintergründe `bg:*` (GEB7); Münzen, Icons, Items, HUD (`icons/8.png` bleibt für `icon:8`); Plantage und Setzling (GEB5); Hub-Gebäude und ihre Slots (`storage`, `farm`, `barracks`, `stairs*`, `plantation`); Änderungen an `engine/`, Protokoll oder Spielständen; neue Biome, Level-Regeln oder Platzierung der Objekte; zusätzliche Ader-Stufen jenseits der vier Slots; Biom-Looks für Eisenstollen und Kristallhöhle (nutzen vorerst Mine).

## Regeln und Einschränkungen

- Domänen nacheinander: **DEV** (GEB6.1–GEB6.4: `public/grafik/gebaeude/`, `tools/gebaeude/` nur bei Bedarf, `src/tools/assetvorschau*.ts`, Regeldatei) → **CLI** (GEB6.5: `src/scenes/`, `data/assets.json`, Löschen alter Grafik samt Fixtures und Mocks) → **Review** (GEB6.6, Domäne CLI). `engine/` bleibt unberührt.
- **Aktivierung und Domänen-Sperre (GEB5 läuft):** Je Projekt ist höchstens ein Sprint aktiv; GEB6 wird erst nach der Freigabe von 🧑 aktiviert (`geplant/` → `aktiv/`), und zwar erst, wenn GEB5 abgeschlossen ist (nur Hardware offen zählt als abgeschlossen). Als Sicherheitsnetz, falls 🧑 eine Ausnahme erlaubt: GEB5.3 (DEV) sperrt DEV, **GEB6.1 bis GEB6.4 starten erst, wenn GEB5.3 `fertig` ist**; GEB5.4 (CLI) sperrt CLI, **GEB6.5 startet erst, wenn GEB5 auf `develop` steht** (Autonomer Ablauf, Schritt 1). Ist die Domäne gesperrt, nichts tun und melden.
- **Merge-Reihenfolge: GEB5 zuerst.** GEB6.5 beginnt erst, wenn GEB5 auf `develop` steht und der Sprint-Branch `origin/develop` eingemergt hat: GEB5.4 ändert `worldRenderer.ts`, `buildingSprites.ts`, `data/assets.json` und die Plantage-Pappel, GEB6.5 dieselben Stellen. Vorher entstehende Konflikte in `public/grafik/gebaeude/index.json` oder Regeldatei § 1 löst ein erneuter `task gebaeude` nach dem Merge (PNGs nie von Hand) und das Zusammenführen der Tabellen beider Seiten.
- Determinismus: gleiche JSON → byte-gleiche PNGs; vorhandene PNGs bleiben unverändert (md5 vor und nach `task gebaeude`). Die Dateien `tanne-<d>.png`, `pappel-<d>.png`, `birke-<d>.png` behalten Namen und Inhalt (GEB5.4 und Vorschau nutzen sie). Keine Würfel im Client: Variantenwahl aus der Position (kein `Math.random()`).
- Materialien nur aus `gebaeude-grafik.md` § 3; fehlt eine (Erde, Fels, Zelttuch, Gold), zuerst dort mit Zustimmung von 🧑 ergänzen (Offene Frage 4); bis zur Freigabe gilt der Vorschlag.
- **Aufräum-Regel (🧑, 2026-10-16):** GEB6.5 löscht ersetzte Grafik im selben Sprint, aber nur, was keine andere Stelle mehr nutzt (Suche mit `git grep` vorher und nachher belegen). Bleiben müssen voraussichtlich das Erz-Blatt (`material:*`) und `icons/8.png` (`icon:8`).
- GEB6 fasst die Dateien und Register-Slots von GEB5 nicht an (`storage`, `farm`, `barracks`, `stairs*`, `plantation`, ihre PNGs).
- Keine neue Abhängigkeit. Datei ≤ 400, Funktion ≤ 60 Zeilen. Planung nur über k3c-dev.

## Beispiele

- `task gebaeude` mit `tanne-2.json` (Stufen Wald, Höhle) → `tanne-2-1-32.png` und `tanne-2-2-32.png` (je 32 × 72 px) plus 16/64/128; `tanne-32.png` bleibt byte-gleich.
- Ein Baum im Wald bei x = 412 → `node:tree` wählt aus der Position Art und Form (etwa „Birke, Form 3“); bei jedem Client, Neustart und Spieler dieselbe, Nachbarbäume wechseln.
- Ein Baum in der Höhle → nur Formen mit Höhle-Look (2 und 3), im Look Höhle.
- Mine, `copperVein` → `ader-2-32.png` (64 × 64 px); Kristallhöhle, `crystalVein` → `ader-4-32.png`.
- Fels in der Höhle → `felsen-2-32.png`; im Eisenstollen (kein eigener Look) → `felsen-3-32.png` (Mine).
- Rekrutierungslager → `rekrutierungslager-32.png` (96 × 64 px) mit Zelt und Feuerstelle, ohne Flackern.
- Portal und Ausgang → je `portal-32.png` und `ausgang-32.png` (64 × 96 px).

## Ausnahme- und Fehlerfälle

- Biom ohne eigenen Look (Eisenstollen, Kristallhöhle, später neue) → Look Mine, begrenzt auf den höchsten Look des Objekts (Bäume → Höhle, Busch → Wald); Biom-ID fehlt im Snapshot → Look Wald.
- Unbekannte Ader-Art oder Objekt-ID → Platzhalter-Form wie heute; Grafik nicht geladen → Platzhalter-Form.
- Plantage-Baum (GEB5, Position auf einem Platz) → bleibt die Pappel aus GEB5, die Positionswahl greift nicht.
- Positionswahl mit negativer oder gebrochener Position → gleiche Wahl für gleiche Position, ohne Fehler (gerundet, Tests mit Grenzwerten).
- Baum in einem Biom, für das die gewählte Form keinen Look hat (Höhle, Form 1) → Auswahl nur aus Formen mit Look für dieses Biom.
- Markierter oder in Abbau befindlicher Knoten → Fahne und Balken sitzen über der neuen, niedrigeren Grafik; die Stauchung beim Abbau (`setScale`) bleibt.
- Spielstände und Server bleiben unberührt (Grafik wirkt nur im Client).

## Akzeptanzkriterien

- **AC-01** Bäume (je Art drei Formen), Busch, Fels, Erzbrocken, vier Adern, Truhe, Portal, Ausgang und Rekrutierungslager liegen als Daten-JSON und PNG vor und folgen den Regeln aus GEB1.
- **AC-02** Die Slots `node:tree`, `node:rock`, `node:copperOre`, `node:bush`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit` sind den neuen PNGs zugeordnet (Register `data/assets.json` und `worldSprites.ts`, Maßstab 1).
- **AC-03** 🧑 hat die Objekte in der Vorschau je Biom und im Spiel abgenommen.
- **AC-04** Jede neue JSON hat eine Vorlage (§ 5) und die Maße aus der Regeldatei § 1 (Bäume 1 × 2,25 in den Formen 2 und 3 mit Looks Wald und Höhle; Busch, Fels, Erzbrocken, Truhe je 1 × 1, Fels mit Looks Wald, Höhle, Mine; Adern 2 × 2 in vier Stufen; Portal und Ausgang je 2 × 3; Lager 3 × 2); `task gebaeude` erzeugt byte-gleich wiederholbare PNGs in 16/32/64/128; ältere PNGs bleiben unverändert; Regeldatei (§ 1, § 3, § 4, § 6) und Vorschau (Biom-Looks, Adern-Stufenreihe) sind nachgezogen.
- **AC-05** Im Spiel (B-438/AC-01): `node:tree` wählt Art und Form deterministisch aus der Position und den Look aus dem Biom, Plantage-Bäume bleiben die Pappel; Fels, Erzbrocken und Busch folgen dem Biom; Adern (`stoneVein` … `crystalVein`) zeigen `vein:*`; Fahne, Balken und Ausgang-Text folgen der Bildhöhe; reine Auswahl-Funktionen mit Vitest.
- **AC-06** Ersetzte Grafik (Baum-, Busch-, Portal-, Ausgang-, Zelt- und Lagerfeuer-Bilder samt ihrer Register-Wahl, Zuordnungs-Doku, Tests, Mocks, CREDITS) ist mit allen Verweisen gelöscht (Suche ohne Treffer außerhalb historischer Doku), sofern nichts anderes sie nutzt; ein leerer Pack fällt ganz weg; `task check` und `task check:go` grün.

## Offene Fragen

Keine blockiert den Start der Planung; jede Annahme gilt mit der Freigabe als bestätigt, eine Antwort von 🧑 ersetzt sie vorher.

1. Annahme: Maße Fels, Erzbrocken, Busch, Truhe je 1 × 1; Adern 2 × 2; Portal und Ausgang je 2 × 3; Rekrutierungslager 3 × 2 Kacheln. Das ist **kleiner** als heute bei Baum (heute 18 × 13 Kacheln in einem Bild, neu 1 × 2,25) und Busch (heute etwa 2,6 × 1,7) und **größer** beim Portal (heute 1 × 1,5).
2. Annahme: Je Art zwei zusätzliche Baum-Formen (gesamt 3 je Art); Tanne, Pappel und Birke bleiben als Form 1 mit Wald-Look unverändert (Dateinamen stabil für GEB5.4). Biom-Looks als Farbtausch über `stufen`, Stufe = Look (1 Wald, 2 Höhle, 3 Mine); die Bäume der Formen 2 und 3 bekommen Wald und Höhle (Bäume stehen nur im Wald und in der Höhle), die Höhle zeigt deshalb nur Formen 2 und 3. Alternative: `stufen` an den drei vorhandenen Bäumen, dann wechselt `pappel-32.png` zu `pappel-1-32.png` und GEB5.4 und GEB6.5 müssen den Pfad nachziehen.
3. Annahme: Die vier Adern sind **eine** JSON mit vier Stufen (Stein, Kupfer, Eisen, Kristall) nach § 6.
4. **Bestätigt (🧑, Option 1, 2026-10-16, Chat):** Erde, Fels und Zelttuch haben keine eigene Zeile in `gebaeude-grafik.md` § 3 (Gold für die Truhe auch nicht). Es gilt der Vorschlag: Fels = Zeile Stein (`dim-gray`, Licht `topaz`, Schatten `smokey-ash`, Fuge `valhalla`); Zelttuch = „Stoff hell“ (`pancho`, Falte `white`, Schatten `twine`); Erde (Wurzeln, Hügelrand) = „Pfosten, Balken“ (`oiled-cedar`, Licht `rope`); Gold = Zeile Glut (`tahiti-gold`, Licht `golden-fizz`, Schatten `brown`). GEB6.1 hat die Zeilen in § 3 eingetragen; Fels, Zelttuch und Erde sind nur Verweise auf vorhandene Zeilen, neu ist allein Gold.
5. Annahme: Portal und Lagerfeuer werden **statische** Bilder; die Animationen (heute je 4 Frames) entfallen, weil Animationen Nicht-Ziel sind, und das Pack `16x16-animated-campfire` fällt weg. Alternative: Animation in einem eigenen Sprint des Projekts GRA nachholen.
6. Annahme: Eisenstollen (`ironhold`) und Kristallhöhle (`crystal`) nutzen den Look Mine, bis 🧑 eigene Looks will; ihre Adern zeigen schon Eisen und Kristall.
7. Annahme: Die Truhe bleibt eine offene Truhe mit sichtbarem Gold (wie heute `icons/8.png`, als Beute erkennbar); eine geschlossene gibt es nicht (ungeprüft vermutet: die Truhe verschwindet beim Einsammeln, `Pickup`).
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB6.1 | `GEB6.1-baeume-busch.md` | Umsetzung | autonom | fertig |
| GEB6.2 | `GEB6.2-fels-erzbrocken.md` | Umsetzung | autonom | fertig |
| GEB6.3 | `GEB6.3-adern.md` | Umsetzung | autonom | fertig |
| GEB6.4 | `GEB6.4-truhe-portal-lager.md` | Umsetzung | autonom | fertig |
| GEB6.5 | `GEB6.5-zuordnung-aufraeumen.md` | Umsetzung | autonom | fertig |
| GEB6.6 | `GEB6.6-review-abnahme.md` | Review | autonom | blockiert |

- GEB6.1 (DEV) Bäume: je 2 weitere Formen für Tanne, Pappel, Birke mit Looks Wald und Höhle, Busch 1 × 1; Material-Zeilen in § 3 und Biom-Look in § 6 (AC-01, AC-04).
- GEB6.2 (DEV) Fels (Looks Wald, Höhle, Mine) und Erzbrocken, je 1 × 1 (AC-01, AC-04).
- GEB6.3 (DEV) Adern: eine JSON `ader` mit vier Stufen (Stein, Kupfer, Eisen, Kristall), 2 × 2 (AC-01, AC-04).
- GEB6.4 (DEV) Truhe 1 × 1, Portal und Ausgang je 2 × 3, Rekrutierungslager 3 × 2 (AC-01, AC-04).
- GEB6.5 (CLI) Zuordnung der Slots `node:*`, `vein:*`, `pickup:chest`, `portal`, `exit`, `camp:recruit` im Spiel in Maßstab 1, Auswahl aus Position und Biom, alte Grafik löschen, Bäume nach B-438 (AC-02, AC-05, AC-06).
- GEB6.6 (CLI) Review und Abnahme durch 🧑 (AC-03, alle).

## Abnahme

2026-10-17 (Review, Agent): AC-01, AC-02, AC-04, AC-05, AC-06 geprüft laut Ergebnis GEB6.1–GEB6.6 (`task check` und `task check:go` grün, getrennter Reviewer ohne schwere Befunde, keine Befunde zu beheben). **AC-03 offen für 🧑** (Abnahmeliste in GEB6.6, Ticket B-451; GEB6.6 `blockiert`, Sprint bleibt `aktiv`); Offene Frage 4 ist von 🧑 mit Option 1 bestätigt (2026-10-16).
Sichtfragen für 🧑: Mine-Fels eher dunkelgrau als braun, Feuerstelle klein, Ausgang ohne Platz für die Beschriftung, Kristall-Ader ohne durchgehende Fuge. Neue Tickets: B-451 (Abnahme).
Version: v0.19.0 vorgeschlagen (Minor, Wirkung im Spiel: neue Welt-Grafik); gesetzt wird sie erst nach Bestätigung durch 🧑.
