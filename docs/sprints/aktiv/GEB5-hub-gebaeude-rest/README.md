# GEB5 · DEV, CLI · Lager, Farm mit Plantage, Kaserne, Treppen

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-435
- **Start-Commit:** ca628e17
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-16, Chat (Ralf)

## Ausgangslage

Lager, Farm, Kaserne und Treppen zeigen Grafik aus fremden 16-px-Packs, im Spiel in Maßstab 2 (`textureScale`, `src/scenes/buildingSprites.ts`); Register-Wahl in `data/assets.json`:

| Slot | Datei unter `public/grafik/` | Pixel | im Spiel (×2, Units) |
|---|---|---|---|
| `storage` | `gothicvania-town/props-einzeln/crate-stack.png` (Kistenstapel) | 73 × 68 | ≈ 4,6 × 4,3 |
| `farm` | `gothicvania-town/props-einzeln/house-a.png` | 168 × 183 | ≈ 10,5 × 11,4 |
| `barracks` | `gothicvania-town/props-einzeln/house-c.png` | 221 × 183 | ≈ 13,8 × 11,4 |
| `stairsUp` | `gothicvania-town/tileset-einzeln/stairs-left.png`, `stairs.png` ×3, `stairs-right.png` (je 16 × 32), zusammengesetzt in `siteView.stairsUp` aus `STAIRS_UP_TILES` | 5 Kacheln | vermutet ≈ 5 × 6 |
| `stairsDown` | `phantasy-dungeon-entrance/ebenen/dungeon-door.png`, Ausschnitt `entrance` 80 × 112 (`siteView.ts`) | 368 × 208 | ≈ 5 × 7 |
| `plantation` | – (Lücke, `wahl` leer) | – | – |

- **Plantage im Code:** `engine/sim/plantation.go` legt je Platz einen normalen Baum (`ResourceNode`, `kind: tree`, `Marked: true`) an der Position `plotX` an (6 Plätze gleichmäßig im Streifen ±`stripUnits` = ±3 um die Farm, `data/buildings.json` › `farm.plantation`). Der Client zeichnet ihn wie jeden Waldbaum (`node:tree` → `gotthicvania-swamp/umgebung/trees.png`). Ein leerer Platz und die Wachstumszeit stehen nicht im Snapshot.
- **Stufen:** Keines der fünf Gebäude hat `levels` in `data/buildings.json`. Die Regel nennt Farm-Stufen („erhöhen Plätze oder Tempo, Werte mit B-099“, `materialien-gebaeude.md` § 1), umgesetzt sind sie nicht.
- **Hub-Plätze** (`data/hub.json`): Lager −26 (Insel-Platz), Kaserne +24, Treppe runter +8, Treppe hoch +16 (ab Tiefe 1; in Tiefe 0 steht dort der Händler), Farm ±52 zwischen Tor 1 (±48) und Turm 2 (≥ ±56). Nachbarn: Taverne −32, Werkstatt −16, Burg ±4, Schmiede +28.
- Seit GEB3/GEB4 gibt es `tools/gebaeude` mit Vorlage (§ 5), Stufen (§ 6), Vorschau und Stufenreihe; Spiel-Dichte 32 px je Kachel ×1.

## Ziel

Lager, Farm, Kaserne, Treppe hoch und Treppe runter haben einheitliche Grafik aus `tools/gebaeude` in Maßstab 1, die Plantage zeigt auf leeren Plätzen einen Setzling und auf gewachsenen Plätzen einen Baum aus GEB1; die ersetzte Pack-Grafik ist gelöscht. Am Ende sichtbar: Vorschau und Spiel, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

🧑 entscheidet Maße und Darstellung (Offene Fragen) und nimmt Vorschau und Spiel ab; Agenten bauen Daten-JSONs, Zuordnung und Aufräumen; Spieler erkennen die Hub-Gebäude und die nachwachsende Plantage auf einen Blick.

## Anforderungen

B-435 › Anforderungen; Regeln aus `docs/standards/gebaeude-grafik.md` (Maße § 1, Vorlage § 5, Stufen § 6); Aufräum-Regel aus Projekt GEB › Notizen. Sprint-eigen (Maße und Darstellung sind Annahmen, siehe Offene Fragen):
- **Lager** (`lager.json`, `storage`): 6 × 4 Kacheln, Steinsockel mit Holzschuppen, offene Front mit Kisten, Fässern und Säcken; Material Stein (Hub-Stufe 2).
- **Kaserne** (`kaserne.json`, `barracks`): 6 × 4 Kacheln, Steinhaus mit Tor, Waffenständer und Wimpel; Material Stein.
- **Farm** (`farm.json`, `farm`): 4 × 3 Kacheln, Holzscheune mit Zaun; Material Holz (Hub-Stufe 1). Passt zwischen Tor 1 (±48) und Turm 2 (≥ ±56).
- **Plantage** (`setzling.json`, Slot `plantation`): Setzling 1 × 1 Kachel für einen leeren Platz; ein gewachsener Plantage-Baum zeigt die **Pappel** aus GEB1 (`pappel-32.png`). Zwei Phasen (leer → Setzling, gewachsen → Baum), weil der Snapshot keine Wachstumszeit kennt.
- **Treppe hoch** (`treppe-hoch.json`, `stairsUp`): 4 × 4 Kacheln, steinerne Außentreppe, steigt nach rechts; **ein Bild** statt der Kachel-Treppe (`STAIRS_UP_TILES` fällt weg).
- **Treppe runter** (`treppe-runter.json`, `stairsDown`): 4 × 3 Kacheln, Steinbogen mit Stufen in die Tiefe.
- **Stufen:** keine `stufen` in den fünf JSONs, weil keines der Gebäude `levels` hat (Farm-Stufen der Regel sind nicht umgesetzt).
- **Spiel:** Slots `storage`, `farm`, `barracks`, `stairsUp`, `stairsDown`, `plantation` im Register und in `buildingSprites.ts`, Maßstab 1; leere Plantage-Plätze ermittelt der Client aus Farm-Position, `farm.plantation` (`slots`, `stripUnits`) und den Bäumen im Snapshot (reine Funktion, gleiche Formel wie `plotX`).
- **Aufräumen:** ersetzte Grafik samt Verweisen löschen, sofern nichts anderes sie nutzt; ein leerer Pack fällt ganz weg (CREDITS, `grafikPacks.ts`, LICENSE).

## Nicht-Ziele

Ausbaustufen für Lager, Farm, Kaserne oder Treppen (erst wenn `data/buildings.json` sie führt, B-099); feinere Wachstumsphasen der Plantage (bräuchten die Wachstumszeit im Snapshot, SIM/SRV, eigenes Ticket); Waldbäume, Fels, Erz, Truhe, Portal, Rekrutierungslager (GEB6); Verschieben von Hub-Plätzen in `data/hub.json`; Animationen; HP-Balken; Sim- oder Protokolländerungen.

## Regeln und Einschränkungen

- Domänen nacheinander: **DEV** (GEB5.1 Lager und Kaserne, GEB5.2 Farm und Setzling, GEB5.3 Treppen: `public/grafik/gebaeude/`, `tools/gebaeude/` nur bei Bedarf, `src/tools/assetvorschau*.ts`, Regeldatei) → **CLI** (GEB5.4: `src/scenes/`, `data/assets.json`, Löschen alter Grafik samt Fixtures und Mocks). `engine/` und `data/hub.json`/`data/buildings.json` bleiben unberührt.
- Farbtausch und Materialien wie Mauer/Turm/Burg (Stein-Gebäude nutzen die Stein-Farben der Mauer-Stufe 2), damit alles zusammenpasst.
- Determinismus: gleiche JSON → byte-gleiche PNGs; vorhandene PNGs bleiben unverändert (md5 vor und nach `task gebaeude`).
- Keine neue Abhängigkeit. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

- `task gebaeude` mit `lager.json` → `lager-32.png` (192 × 128 px) plus 16/64/128, ohne Stufen-Dateien.
- Farm gebaut, noch kein Baum gewachsen → sechs Setzlinge im Streifen ±3 um die Farm; nach 30 s steht auf jedem Platz eine Pappel.
- Ein Bauer fällt den Plantage-Baum auf Platz 2 → dort steht wieder ein Setzling, bis der Baum nachwächst.
- Tiefe 1, Hub-Stufe 2, Treppe hoch gebaut → ein Bild `treppe-hoch-32.png` (128 × 128 px) bei +16.

## Ausnahme- und Fehlerfälle

- Farm im Bau oder zerstört → keine Setzlinge (die Sim legt Plätze nur für gebaute Farmen an).
- Waldbaum zufällig im Plantage-Streifen, aber nicht auf einer Platz-Position → bleibt ein Waldbaum (Zuordnung nur bei gleicher x-Position, Toleranz im Test festlegen).
- Baustelle eines der Gebäude → Platzhalter wie heute.
- Grafik nicht geladen → bisherige Platzhalter-Form (Treppen: gezeichnete Stufen).

## Akzeptanzkriterien

- **AC-01** Lager, Farm, Kaserne, Treppe hoch und Treppe runter liegen als Daten-JSON und PNG vor.
- **AC-02** Die Plantage (`plantation`) hat Grafik für Wachstumsphasen; sie nutzt die Bäume aus GEB1.
- **AC-03** 🧑 hat die Gebäude abgenommen.
- **AC-04** `lager.json`, `kaserne.json` (je 6 × 4), `farm.json` (4 × 3), `setzling.json` (1 × 1), `treppe-hoch.json` (4 × 4) und `treppe-runter.json` (4 × 3) haben eine Vorlage (§ 5), keine `stufen`, und erzeugen mit `task gebaeude` byte-gleich wiederholbare PNGs in 16/32/64/128; ältere PNGs bleiben unverändert; Maße stehen in der Regeldatei § 1; alle erscheinen in der Vorschau.
- **AC-05** Im Spiel zeigen Lager, Farm, Kaserne, Treppe hoch (ein Bild statt Kacheln) und Treppe runter die neuen PNGs in Maßstab 1; leere Plantage-Plätze zeigen den Setzling, Plantage-Bäume die Pappel, Waldbäume bleiben unverändert; reine Auswahl-Funktionen mit Vitest; Register `storage`, `farm`, `barracks`, `stairsUp`, `stairsDown`, `plantation` zugeordnet.
- **AC-06** Ersetzte Grafik (Kistenstapel, `house-a`, `house-c`, `dungeon-door`, Treppen-Kacheln) ist mit allen Verweisen gelöscht (Suche ohne Treffer außerhalb historischer Doku), sofern nichts anderes sie nutzt; ein leerer Pack fällt ganz weg; `task check` und `task check:go` grün.

## Offene Fragen

Keine blockiert die Freigabe; jede Annahme gilt mit der Freigabe als bestätigt, eine Antwort von 🧑 ersetzt sie vorher.

1. Annahme, bestätigt mit der Freigabe: **Lager 6 × 4** und **Kaserne 6 × 4** Kacheln (Klasse „kleines Gebäude“ wie Werkstatt), beide in Stein.
2. Annahme, bestätigt mit der Freigabe: Die Kaserne (+24, 6 breit: +21…+27) **überdeckt die Schmiede** (+28: +25…+31) um 2 Kacheln; das bleibt so (heute überdeckt `house-c` noch mehr). Hub-Plätze verschieben wäre ein eigenes Ticket (REG, `data/hub.json`).
3. Annahme, bestätigt mit der Freigabe: **Farm 4 × 3** Kacheln (Scheune), damit sie zwischen Tor 1 (±48) und Turm 2 (≥ ±56) passt; die Plantage-Bäume stehen im Streifen ±3 davor.
4. Annahme, bestätigt mit der Freigabe: **Keine sichtbaren Farm-Stufen** (und keine Stufen für Lager, Kaserne, Treppen), weil `data/buildings.json` keine `levels` für sie führt; kommen Farm-Stufen mit B-099, folgt die Grafik dann.
5. Annahme, bestätigt mit der Freigabe: Die **Plantage** zeigt zwei Phasen: Setzling (1 × 1 Kachel, `setzling.json`) auf einem leeren Platz, die **Pappel** aus GEB1 auf einem gewachsenen Platz. Feinere Phasen bräuchten die Wachstumszeit im Snapshot (eigenes Ticket, SIM/SRV).
6. Annahme, bestätigt mit der Freigabe: Die **Treppe hoch wird ein Bild** (4 × 4 Kacheln) statt der Kachel-Treppe; **Treppe runter 4 × 3** Kacheln (Steinbogen mit Abgang).
7. Annahme, bestätigt mit der Freigabe: `phantasy-dungeon-entrance/ebenen/dungeon-door-skull.png` (nirgends im Code genutzt) fällt mit `dungeon-door.png` weg, der Pack damit ganz.
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB5.1 | `GEB5.1-lager-kaserne.md` | Umsetzung | autonom | fertig |
| GEB5.2 | `GEB5.2-farm-plantage.md` | Umsetzung | autonom | fertig |
| GEB5.3 | `GEB5.3-treppen.md` | Umsetzung | autonom | fertig |
| GEB5.4 | `GEB5.4-zuordnung-aufraeumen.md` | Umsetzung | autonom | fertig |
| GEB5.5 | `GEB5.5-review-abnahme.md` | Review | autonom | offen |

- GEB5.1 (DEV) Lager und Kaserne, je 6 × 4 (AC-01, AC-04).
- GEB5.2 (DEV) Farm 4 × 3 und Setzling 1 × 1 für die Plantage (AC-01, AC-02, AC-04).
- GEB5.3 (DEV) Treppe hoch 4 × 4 als ein Bild, Treppe runter 4 × 3 (AC-01, AC-04).
- GEB5.4 (CLI) Zuordnung im Spiel in Maßstab 1, Plantage mit Setzling und Pappel, alte Grafik löschen (AC-02, AC-05, AC-06).
- GEB5.5 Review und Abnahme durch 🧑 (AC-03, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
