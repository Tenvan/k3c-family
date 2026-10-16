# GEB7 · DEV, PLAT, CLI · Hintergründe je Biom

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV, PLAT, CLI
- **Reife:** bereit
- **Tickets:** B-435
- **Start-Commit:** b9a22a82
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-16, Chat (Ralf: Umsetzung auf Annahmen, offene Fragen nach Abschluss)

## Ausgangslage

Jedes Biom zeigt hinter dem Hub eine **Hintergrund-Schicht**-Reihe (Parallax, Glossar). Heute stammt sie aus fremden 16-px-Packs im Maßstab ×2/×3; zwei Biome haben keine Grafik. Stand (Code und Daten, 2026-10-16):

| Biom (Tiefe) | Slot im Register | Schichten im Spiel (`worldSprites.ts` › `BACKGROUNDS`) | Pack, Datei, Pixel | Zoom |
|---|---|---|---|---|
| `forest` (0) | `bg:forest` (`wahl`: nur Schicht 1) | 4: `bg:forest-back` 0,15 · `-lights` 0,25 · `-middle` 0,35 · `-front` 0,5 | `forest-background/ebenen/parallax-forest-*.png`, je 272 × 160 | ×3 |
| `cave` (1) | `bg:cave` | 1: `bg:cave` 0,2 | `blue-cave-background/ebenen/startcavebg.png`, 400 × 225 (kein Kachelraster) | ×2 |
| `mine` (2) | `bg:mine` (`wahl`: nur `back`) | 3: `bg:mine-back` 0,1 · `-far` 0,25 · `-middle` 0,45 | `warped-super-grotto-escape-pack/ebenen/back|far|middle.png`, 304/352/368 × 240 | ×2 |
| `ironhold` (3) | `bg:ironhold`, `wahl` leer | keine → Silhouetten aus der Biom-Palette | – (Lücke) | – |
| `crystal` (4) | `bg:crystal`, `wahl` leer | keine → Silhouetten aus der Biom-Palette | – (Lücke) | – |

- **Zeichnen:** `objectView.backgroundLayers` legt je Schicht einen `TileSprite` (Ursprung unten links, Unterkante `GROUND_Y` = 900, Skalierung = Zoom, Scroll-Faktor je Schicht) an; fehlt eine Textur oder ist das Biom nicht in `BACKGROUNDS`, zeichnet `stageView` zwei Silhouetten (`RIDGES`) in `palette.far`/`near`. Darüber steht der Himmel als Kamera-Hintergrund `palette.sky` (`GameScene`), die Schichten sind oben transparent.
- **Fluss:** `stageView.addRiver` spiegelt dieselben `TileSprite`-Schichten kopfüber im Wasser unter der Bodenlinie (`frame.height`, `tilePositionY`); die Unterkante der Textur muss an der Bodenlinie liegen.
- **Register** (`data/assets.json`, Kategorie `umgebung`): je Biom ein Slot `bg:<biom>`; das Spiel nutzt es nicht, sondern die fest verdrahteten Schlüssel in `worldSprites.ts` (`WORLD_TEXTURES`). `src/tools/zuordnung.test.ts` verlangt je Biom eine Zeile `bg:<id>` in `docs/assets/zuordnung-welt.md`; `worldSprites.test.ts` prüft nur `forest`, `cave`, `mine`.
- **Biom-Paletten** (`data/biomes/*.json` › `palette`, nicht DB32): `ironhold` Himmel `#1f0f0a`, `crystal` Himmel `#0d1626`.
- **Nutzer der Packs außerhalb des Spiels:** `site/index.html` (GitHub-Pages-Seite, Domäne PLAT) zeigt `forest-background` (Parallax-Streifen, 4 Schichten + Kachel `.d0`), `blue-cave-background` (`.d1`) und `warped-caves-pixel-art-pack/ebenen/background.png` (`.d2`) über `app/grafik/…`; `src/tools/credits.test.ts` nennt `warped-caves-pixel-art-pack` als erwarteten CC-BY-Eintrag. `warped-caves-pixel-art-pack` ist im Register **nicht** zugeordnet (nur als Alternative in `zuordnung-welt.md`).
- **Packs und was bleibt:** `forest-background`, `blue-cave-background`, `warped-super-grotto-escape-pack` dienen im Spiel nur als Hintergrund (`git grep` über `src/`, `data/`, `tests/`, `tools/`: keine weiteren Treffer). `sunnyland-tall-forest-environment` liefert außerdem `props/Plant.png` für `node:bush` (GEB6 ersetzt es); seine `ebenen/*` sind nur als Alternative dokumentiert.
- **Schon vorhanden aus GEB3:** `hintergrundmauer` (1 × 4 Kacheln, kachelbar, Stufen 1–5) und die Regeln § 1–§ 9 in `docs/standards/gebaeude-grafik.md`; kachelbar heißt dort „erste und letzte Pixelspalte gleich“ (`TestHintergrundmauerKachelbar`). `tools/gebaeude` zeichnet nur Rechtecke aus DB32 und rendert immer 16/32/64/128 px je Kachel.

## Ziel

Jedes der fünf Biome (Wald, Höhle, Mine, Eisenstollen, Kristallhöhle) hat einen eigenen Hintergrund aus drei Hintergrund-Schichten, erzeugt mit `tools/gebaeude` in der DB32-Palette der Gebäude, im Spiel in Maßstab 1; die ersetzte Pack-Grafik ist gelöscht. Am Ende sichtbar: Vorschau je Biom (drei Schichten gestapelt) und Spiel, von 🧑 abgenommen.

🧑 hat die Planung am 2026-10-16 angefordert (Projekt GEB › Notizen nannte den Sprint „optional“); er folgt im Projekt nach GEB6.

## Beteiligte und Zielgruppen

🧑 entscheidet Motive, Maße und die Fragen unten und nimmt Vorschau und Spiel ab; Agenten bauen Daten-JSONs, Generator-Ergänzung, Zuordnung und Aufräumen; Spieler erkennen das Biom auf einen Blick, Besucher der Pages-Seite sehen dieselben Hintergründe wie im Spiel.

## Anforderungen

B-435 › Anforderungen (Dichte-Regel, Determinismus, Daten + Skript im Repo) und `docs/standards/gebaeude-grafik.md` (Raster § 1, Palette § 2, Materialien § 3, Vorlage § 5, Dichte § 7); Aufräum-Regel aus Projekt GEB › Notizen. Sprint-eigen (Maße und Motive sind Annahmen, siehe Offene Fragen):

- **Schichten:** je Biom **drei** Hintergrund-Schichten (fern, mitte, nah), je eine Daten-JSON `hg-<biom>-<schicht>` (z. B. `hg-wald-fern`), Klasse `hintergrund` (kachelbar wie `kachel`, Revision 2), **24 × 15 Kacheln** (768 × 480 px Master, Höhe wie die heutigen Schichten), in der Breite nahtlos kachelbar, oben transparent (Himmel bleibt `palette.sky`), unterste Pixelzeile deckend (Bodenlinie und Fluss-Spiegelung). Das sind 15 JSONs.
- **Motive** (Vorschlag, 🧑 prüft in der Vorschau): Wald: ferne Hügel und Stämme im Dunst, mittlere Laubbäume (Farbwelt der Bäume aus GEB1/GEB6), nahe Stämme; Höhle: Felswand, Stalaktiten, ferne Hallen; Mine: Stützbalken und Gerüst vor Fels (Holz § 3), Schienenbett; Eisenstollen: dunkler Fels mit Eisenträgern und Lavaschein (Glut § 3); Kristallhöhle: Kristallgruppen und Kristalllicht (Kristall § 3).
- **Farbe und Lesbarkeit:** nur DB32; fernere Schichten dunkler und entsättigter als nähere, alle dunkler als die Gebäude und Figuren davor (Zahl der Stufen legt die Regeldatei fest); Farbwahl passt zur Himmelsfarbe des Bioms.
- **Generator** (`tools/gebaeude`, nur soweit nötig): (a) Wiederholung eines Bausteins entlang x (`wiederholung`: Schritt und Anzahl), damit Bäume, Stalaktiten und Kristalle nicht als hunderte Einzel-Rechtecke stehen; (b) für Klasse `hintergrund` nur die Dichten 16 und 32 (64/128 wären mehrere MB je Schicht ohne Spielwert); (c) ein Kachelbarkeits-Test (`TestKachelKlassen`, ersetzt `TestHintergrundmauerKachelbar` nicht) gilt für alle Klassen `kachel` und `hintergrund`, für `hintergrund` dazu „oberste Zeile transparent, unterste Zeile deckend“. Alles ohne neue Abhängigkeit.
- **Vorschau:** `assetvorschau.html?szene=gebaeude` zeigt je Biom die drei Schichten gestapelt auf der Himmelsfarbe des Bioms, 32 px ×1 und 16 px ×2.
- **Spiel (CLI):** `BACKGROUNDS` und `WORLD_TEXTURES` in `worldSprites.ts` verweisen auf die neuen PNGs, Zoom 1 (statt ×2/×3), Scroll-Faktoren je Biom wie heute (hinten langsam), ein Eintrag für **alle fünf** Biome (Silhouetten bleiben nur als Rückfall für unbekannte Biome). Register: je Schicht ein Slot `bg:<biom>:<n>` (n = 1 hinten … 3 vorn) statt `bg:<biom>`; `zuordnung-welt.md` und `zuordnung.test.ts` ziehen mit.
- **Pages-Seite (PLAT):** `site/index.html` nutzt die neuen Schichten statt der alten Packs (Frage 5).
- **Aufräumen:** ersetzte Pack-Grafik samt Verweisen löschen (Kriterium AC-06), nur was nachweislich nichts anderes nutzt.

## Nicht-Ziele

Animationen (Lichtstrahlen, Lava-Flackern, Staub, Wolken); Himmel und Tag-Nacht-Tönung (Himmel bleibt `palette.sky`); Vordergrund-Dekor und Objekte (Bäume, Fels, Erz, Adern, Pflanzen: GEB6); Hintergrundmauer (GEB3); Änderungen an Scroll-Logik, Fluss-Spiegelung oder Kamera (nur Daten und Zuordnung); Änderungen an `data/biomes/*.json` (Paletten, Level-Regeln), `engine/`, Protokoll oder Spielständen; neue Biome; Skalierungsfaktor am TV (GR7.4, B-251); Figuren, Münzen, Icons, Items, HUD. Die Biom-Paletten an DB32 anzugleichen wäre ein eigenes REG-Ticket, falls 🧑 es will.

## Regeln und Einschränkungen

- **Domänen nacheinander:** **DEV** (GEB7.1–GEB7.3: `tools/gebaeude/` für die Generator-Ergänzung, `public/grafik/gebaeude/`, `src/tools/assetvorschau*.ts`, Regeldatei) → **PLAT** (GEB7.4: `site/`, nur wenn Frage 5 mit „umstellen“ beantwortet wird, sonst verworfen) → **CLI** (GEB7.5: `src/scenes/`, `data/assets.json`, Löschen alter Grafik samt Fixtures und Mocks). Die PLAT-Session liegt **vor** dem Löschen, weil `site/index.html` die alten Packs noch lädt. `engine/` und `data/biomes/` bleiben unberührt.
- **Verhältnis zu GEB5 und GEB6:** GEB5 (aktiv) und GEB6 laufen in anderen Checkouts. GEB7 fasst deren Slots und Dateien nicht an (Gebäude, `node:*`, `vein:*`, Objekte), teilt aber `docs/standards/gebaeude-grafik.md`, `public/grafik/gebaeude/index.json`, `data/assets.json` und `src/scenes/worldSprites.ts` (GEB6.5 ändert dort `OBJECTS`/`WORLD_TEXTURES`, GEB7.5 `BACKGROUNDS` und die `bg:*`-Texturen). **Merge-Reihenfolge GEB5 → GEB6 → GEB7**. Revision 2 (autonomer Lauf): GEB7 wird auf Basis von `develop` mit GEB5 aktiviert (`Start-Commit` dort); GEB6 läuft noch (Branch `sprint/geb6`). Die DEV-Sessions GEB7.1–GEB7.4 laufen schon, **GEB7.5 (CLI) beginnt erst, wenn GEB6 auf `develop` gemergt ist und keine andere CLI-Session `in Arbeit` ist**; vorher holt `sprint/geb7` `origin/develop` per `git merge`. Der PR von GEB7 wird erst nach dem Merge von GEB6 geöffnet. Beim Merge beider Sprints ist GEB6 `erledigt` (nie zwei aktive Sprints im Projekt, `tests/planning.test.ts`).
- **Materialien** nur aus `gebaeude-grafik.md` § 3 (Holz, Stein, Eisen, Glut, Kristall, Laub, Nadeln, Putz u. a.); Fels = Zeile Stein, wie es GEB6 Frage 4 festlegt. Fehlt etwas (z. B. Erde, Lava-Fels), zuerst dort mit Zustimmung von 🧑 ergänzen.
- **Determinismus:** gleiche JSON → byte-gleiche PNGs; vorhandene PNGs bleiben unverändert (md5 vor und nach `task gebaeude`). Keine Würfel im Client, keine Zufallsvarianten; Variation entsteht nur in den Daten (Wiederholung mit festen Versätzen).
- **Aufräum-Regel (🧑, 2026-10-16):** Alte Grafik, die der neue Hintergrund ersetzt, wird in **diesem** Sprint gelöscht: PNG, Einträge in `public/grafik/index.json`, Register-Wahl in `data/assets.json`, `docs/assets/zuordnung-*.md`, Test-Fixtures und Mocks, CREDITS-Zeilen und `grafikPacks.ts` (wird ein Pack leer, fällt er ganz weg samt LICENSE). Gelöscht wird nur, was **vorher per Suche belegt** nichts anderes nutzt; Kandidaten und Belege stehen in GEB7.5. Historische Belege (`docs/funde/`, erledigte Sprints, Archiv, andere Tickets) bleiben.
- Keine neue Abhängigkeit. Datei ≤ 400, Funktion ≤ 60 Zeilen (gilt auch für die Generator-Ergänzung).

## Beispiele

- `task gebaeude` mit `hg-wald-mitte.json` (24 × 15) → `hg-wald-mitte-32.png` (768 × 480 px) und `-16.png` (384 × 240), keine 64/128; zweimal gerendert → byte-gleich.
- Tiefe 3 (Eisenstollen) im Spiel → drei Schichten `bg:ironhold:1` … `:3` mit dunklem Fels und Lavaschein statt der Silhouetten; hinten scrollt langsamer als vorn.
- Spieler 2 läuft im Split-Screen in die Mine → seine Kamera zeigt dieselben Schichten, jede Kamera verschiebt sie mit dem eigenen Scroll-Faktor.
- Fluss unter dem Hub → die drei Schichten spiegeln sich kopfüber im Wasser, ohne Spalt an der Bodenlinie.
- Vorschau → je Biom ein Streifen mit den drei Schichten auf der Himmelsfarbe, darunter derselbe Streifen in 16 px ×2.

## Ausnahme- und Fehlerfälle

- Schicht nicht geladen (Textur fehlt) → heutiger Rückfall: zwei Silhouetten aus der Biom-Palette; `backgroundLayers` liefert `null`, sobald eine Schicht des Bioms fehlt.
- Unbekanntes Biom → `biomeBackground` gibt `null`, Silhouetten.
- Erste und letzte Pixelspalte einer Schicht verschieden (sichtbare Naht) oder unterste Zeile mit Lücken (Spalt über dem Boden, Fluss-Spiegelung zeigt Streifen) → Go-Test schlägt fehl.
- Baustein außerhalb der 24 × 15 Kacheln, Rechteck nicht im 1/32-Raster, `wiederholung` mit Schritt 0 oder Überlauf → `task gebaeude` bricht mit klarer Meldung ab.
- Pack, den noch etwas nutzt (z. B. `sunnyland-tall-forest-environment` mit `Plant.png`, falls GEB6 es nicht ersetzt hat) → bleibt, nur ungenutzte Dateien (`ebenen/*`) fallen weg; nie ein Pack löschen, den ein Treffer in `src/`, `data/`, `tests/`, `tools/` oder `site/` nennt.
- Antwort auf Frage 5 „Pages-Seite unverändert lassen“ → GEB7.4 entfällt (`verworfen`), `forest-background`, `blue-cave-background` und `warped-caves-pixel-art-pack` bleiben mit Vermerk im Ergebnis, nur `warped-super-grotto-escape-pack` und die ungenutzten `ebenen/*` von `sunnyland-tall-forest-environment` fallen weg.
- **Wald-Gate entfallen (Revision 2):** Die Spec verlangte, dass 🧑 den Wald in der Vorschau prüft, bevor GEB7.2/GEB7.3 beginnen. Im autonomen Lauf entfällt das; 🧑 prüft nach Abschluss alle fünf Biome (AC-08). Ändert 🧑 dann Motiv, Maße oder Stil des Waldes, werden die übrigen Biome im Stil nachgezogen (Farbtausch oder Neuzeichnen der JSON, kein Code).
- Hintergrund zu hell und lenkt von Gebäuden ab → 🧑 lehnt in der Vorschau ab; die Schicht wird dunkler gefärbt (Farbtausch in der JSON), kein Code.

## Akzeptanzkriterien

- **AC-01** Die 15 Hintergrund-Schichten (`hg-<biom>-fern|mitte|nah` für `wald`, `hoehle`, `mine`, `eisenstollen`, `kristall`, je 24 × 15 Kacheln) liegen als Daten-JSON mit Vorlage (§ 5) und als PNG in 16 und 32 px je Kachel vor; `task gebaeude` erzeugt sie byte-gleich wiederholbar, ältere PNGs bleiben unverändert; Maße und Klasse stehen in der Regeldatei § 1.
- **AC-02** Jede Schicht ist in der Breite nahtlos kachelbar (erste = letzte Pixelspalte), oben transparent und in der untersten Pixelzeile deckend; ein Go-Test prüft das für alle 15 Schichten, der Generator lehnt Verstöße gegen Raster, Maße und `wiederholung` mit Meldung ab.
- **AC-03** Die Asset-Vorschau zeigt je Biom die drei Schichten gestapelt auf der Himmelsfarbe des Bioms (32 px ×1 und 16 px ×2).
- **AC-04** Alle Schichten nutzen nur DB32-Farben aus § 3 der Regeldatei; ist ein Material neu, steht es dort mit Zustimmung von 🧑; fernere Schichten sind dunkler als nähere (Regeldatei nennt die Mindeststufen, ein Test prüft die mittlere Helligkeit je Biom).
- **AC-05** Im Spiel zeigen alle fünf Biome ihre drei neuen Schichten in Maßstab 1 mit Scroll-Faktoren hinten < vorn; `biomeBackground` liefert für `forest`, `cave`, `mine`, `ironhold`, `crystal` je drei Schichten, für unbekannte Biome `null` (Vitest, `worldSprites.test.ts` über alle fünf); die Slots `bg:<biom>:<n>` sind im Register zugeordnet, `zuordnung-welt.md` und `zuordnung.test.ts` (`bg:${id}`) stimmen überein.
- **AC-06** Ersetzte Grafik ist mit allen Verweisen gelöscht (Suche je Kandidat vorher und nachher, keine Treffer außerhalb historischer Doku): der Pack `warped-super-grotto-escape-pack` ganz, `forest-background`, `blue-cave-background` und `warped-caves-pixel-art-pack` ganz (sobald `site/` sie nicht mehr lädt, AC-07), die ungenutzten `ebenen/*` von `sunnyland-tall-forest-environment`. Mitziehen: `public/grafik/index.json`, `public/grafik/CREDITS.md` (Tabellenzeilen **und** Fließtext-Hinweise zu Warped Caves/Super Grotto), `src/tools/grafikPacks.ts` (inkl. Beschreibung von `sunnyland-tall-forest-environment`: „Hintergrund Wald“, „Höhe der Hintergründe“), Lizenztext `lic.warped` in `src/tools/texts.de.ts` und `texts.en.ts` samt Lizenzseite, `docs/assets/zuordnung.md` (Pack-Tabelle, Ergebnis-Zeile) und `zuordnung-welt.md`, `credits.test.ts` (ohne `warped-caves-pixel-art-pack`), `worldSprites.test.ts`, `zuordnung.test.ts`; `task check`, `task check:go` und `task check:dev` grün. Bei „unverändert“ auf Frage 5 bleiben `forest-background`, `blue-cave-background` und `warped-caves-pixel-art-pack` mit Vermerk `verschoben` im Ergebnis; die übrigen Löschungen gelten weiter.
- **AC-07** `site/index.html` zeigt statt der alten Packs die neuen Hintergrund-Schichten (Wald, Höhle, Mine); `task pages` baut ohne fehlende Dateien. Entfällt bei Antwort „unverändert“ auf Frage 5.
- **AC-08** 🧑 hat die Hintergründe in der Vorschau und im Spiel abgenommen.

## Offene Fragen

Keine blockiert die Planung; jede Annahme gilt mit der Freigabe als bestätigt, eine Antwort von 🧑 ersetzt sie. Frage 5 ist die einzige mit Auswirkung auf den Zuschnitt. Freigabe (Revision 2): „Ralf, Chat 2026-10-16: Umsetzung auf Annahmen, offene Fragen nach Abschluss“; **Annahme zu Frage 5: umstellen** (Empfehlung). Je Frage steht, was sich bei anderer Antwort ändert.

1. Annahme: **drei Schichten je Biom** (fern, mitte, nah), also 15 Schichten; die vierte Wald-Schicht „Lichtstrahlen“ (`bg:forest-lights`) und die Höhle mit einer Schicht werden auf drei vereinheitlicht. Scroll-Faktoren: Wald 0,15 / 0,35 / 0,5, Höhle 0,1 / 0,2 / 0,4, Mine 0,1 / 0,25 / 0,45, Eisenstollen und Kristallhöhle wie Mine. *Bei anderer Antwort:* andere Schichtzahl → Daten-JSONs, `BACKGROUNDS` und Register-Slots in GEB7.1–GEB7.3 und GEB7.5 anpassen; andere Faktoren → nur `BACKGROUNDS` (GEB7.5).
2. Annahme: Schichten **24 × 15 Kacheln** (768 × 480 px), **Zoom 1** im Spiel (Pixelraster wie Gebäude, § 8 gemeinsames Raster), nur Dichten **16 und 32**. Der Faktor am TV bleibt offen (GR7.4, B-251) und gilt für alle gemeinsam. *Bei anderer Antwort:* andere Maße → Neuzeichnen der 15 JSONs (GEB7.1–GEB7.3), Zoom in GEB7.5.
3. Annahme: Himmel bleibt `palette.sky` aus `data/biomes/*.json` (nicht DB32, REG-Werte); die Schichten sind oben transparent. *Bei anderer Antwort (Himmel in DB32):* eigenes REG-Ticket für `data/biomes/*.json`, kein Eingriff in diesen Sprint.
4. Annahme: **Slots je Schicht** `bg:<biom>:<n>` (n = 1 hinten) ersetzen die fünf `bg:<biom>`-Slots (wie `hub:<n>`); die Reihenfolge der Schichten steht in `BACKGROUNDS` (Code), das Register hält je Slot die gewählte Datei. *Bei anderer Antwort:* nur Register, `zuordnung-welt.md` und `zuordnung.test.ts` in GEB7.5.
5. **Frage an 🧑:** `site/index.html` (Pages-Seite) lädt `forest-background`, `blue-cave-background` und `warped-caves-pixel-art-pack/ebenen/background.png`. Empfehlung: **umstellen** (PLAT-Session GEB7.4 vor dem Löschen), dann fallen alle drei Packs weg und die Pages-Seite zeigt dieselben Hintergründe wie das Spiel. Alternative: **unverändert** lassen, dann bleiben die drei Packs (samt CC-BY-Eintrag Warped Caves) und GEB7.4 entfällt. Falls der Sprint PG1 (Präsentation Bilder, ändert ebenfalls `site/`) zuerst läuft, trifft GEB7.4 auf dessen Stand und ersetzt nur die `url()`-Zeilen. *Bei Antwort „unverändert“:* GEB7.4 wird `verworfen`, GEB7.5 lässt `forest-background`, `blue-cave-background` und `warped-caves-pixel-art-pack` stehen (AC-06 für sie `verschoben`, AC-07 entfällt).
6. Annahme: Motive wie in Anforderungen (Wald: Dunst-Hügel, Laubbäume, nahe Stämme; Höhle: Felswand mit Stalaktiten; Mine: Stützbalken; Eisenstollen: Fels mit Lavaschein; Kristallhöhle: Kristallgruppen). 🧑 gibt in der Vorschau pro Biom frei oder ändert die Motive; die Farbwelt der Bäume zieht aus GEB6 (Kronen-Paletten je Biom) nach. *Bei anderer Antwort:* Motive stehen nur in den Daten-JSONs (GEB7.1–GEB7.3), kein Code.
7. Annahme: Der Generator bekommt `wiederholung` (Schritt, Anzahl) und für Klasse `hintergrund` die Dichten 16/32; kein Perlin-Rauschen, keine Zufallsgrafik, nur Rechtecke aus DB32 wie bisher. Reicht das für ansehnliche Hintergründe nicht, entscheidet 🧑 in der Vorschau (nach Abschluss, Revision 2) über weitere Bausteine (z. B. Dreiecke für Berge) oder über ein Ticket. *Bei anderer Antwort:* zusätzliche Bausteinart im Generator (neue Session DEV) und Neuzeichnen betroffener JSONs.
8. Annahme: `sunnyland-tall-forest-environment/ebenen/*` ist nur als „Alternative“ dokumentiert und fällt weg, `props/Plant.png` bleibt bis GEB6 es ersetzt; hat GEB6 den Pack schon ganz entfernt, entfällt dieser Punkt. *Bei anderer Antwort:* `ebenen/*` bleibt, AC-06 ohne diesen Punkt.
9. Annahme: Ein Eintrag im Glossar (**Hintergrund-Schicht**) steht bereits; „Ebene“ bleibt für die Insel-Variante reserviert, `ebenen/` in Pack-Pfaden ist kein Glossar-Begriff.
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB7.1 | `GEB7.1-generator-wald.md` | Umsetzung | autonom | fertig |
| GEB7.2 | `GEB7.2-hoehle-mine.md` | Umsetzung | autonom | fertig |
| GEB7.3 | `GEB7.3-eisenstollen-kristall.md` | Umsetzung | autonom | fertig |
| GEB7.4 | `GEB7.4-pages-umstellen.md` | Umsetzung | autonom | fertig |
| GEB7.5 | `GEB7.5-zuordnung-aufraeumen.md` | Umsetzung | autonom | fertig |
| GEB7.6 | `GEB7.6-review-abnahme.md` | Review | autonom | blockiert |

## Abnahme

2026-10-16 (Review GEB7.6, getrennter Reviewer, keine schweren Befunde): AC-01 bis AC-04 geprüft (GEB7.1–GEB7.3), AC-05 und AC-06 geprüft (GEB7.5), AC-07 geprüft (GEB7.4); `task check`, `task check:go`, `dev:test`, `task build` grün.
AC-08 offen für 🧑 (B-450): Vorschau, Spiel und Pages-Seite sieht nur 🧑 ab; Wald-Gate entfiel (Revision 2), Annahmen 1–9 gelten bis zur Antwort.
Behobene Befunde: Lint `errcheck` (GEB7.1), Höhle `mitte` mit unterbrochener Decke (auf Wunsch von 🧑). Neue Tickets: B-450.
Hinweis: GEB7 liegt in `geplant/`, bis GEB6 `erledigt` ist (ein aktiver Sprint je Projekt); dann nach `aktiv/` und mit Abnahme nach `erledigt/`.
Version: Minor vorgeschlagen (Wirkung im Spiel und Werkzeug; Nummer nach dem GEB6-Vorschlag), gesetzt wird sie erst nach Bestätigung durch 🧑.
