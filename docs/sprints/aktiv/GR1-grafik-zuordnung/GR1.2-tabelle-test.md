# GR1.2 · Zuordnung für Gebäude, Gegner, Truppen und Vollständigkeits-Test

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr1/2-tabelle-test
- **Abhängig von:** GR1.1
- **Tickets:** B-161
- **Kriterien:** AC-01, AC-03, AC-04, AC-05

## Ziel

`docs/assets/zuordnung.md` hat zu jeder Objekt-ID aus `data/buildings.json`, `data/enemies.json` und `data/troops.json` eine Zeile mit Stil, Lizenz und Status; ein Test prüft Vollständigkeit, Credits und Lücken-Tickets.

## Kontext

- Spalten laut B-161: Objekt, Herkunft (`data/…` oder Regel), Pack, Datei/Frame, Stil (Raster, Palette, Skalierung), Lizenz, Status (zugeordnet oder Lücke). Lücken **fett** („keine Grafik“, „Stil passt nicht“) mit Ticket (für neue Assets B-162, Suche in GR2).
- Objekt-IDs heute: Gebäude `castle, wall, tower, gate, workshop, storage, farm, barracks, stairsUp, stairsDown`; Gegner `greed, wolf, goblin, goblinArcher, skeleton, bat, caveTroll, zombie, ratSwarm, mineGhost`; Truppen `vagrant, peasant, archer, warrior, eliteArcher, eliteWarrior`. Neue IDs aus parallelen Sprints (z. B. W1) nimmt der Test automatisch mit.
- Figuren: `data/sprites.json` ordnet allen Truppen und Gegnern einen Ordner unter `public/sprites/` zu (Credits in `public/sprites/CREDITS.md`). Gebäude sind heute Rechtecke (`src/scenes/worldRenderer.ts`); Treffer und Nicht-Treffer im Bestand stehen in B-161 › Ausgangslage und `docs/funde/b010-grafik-funde.html` (ohne Treffer: Werkstatt, Farm, Kaserne, Treppen, Mine-Hintergrund, Rekrutierungslager).
- Stil: Pack-Entscheidungen aus GR1.1 (Kopf und Pack-Tabelle in `docs/assets/zuordnung.md`); ein Asset aus einem Pack „passt nicht“ wird Lücke, nicht stillschweigend übernommen.
- Test-Vorbild: `src/tools/grafikPacks.test.ts` (Vitest, `?raw`-Import von Markdown, `import.meta.glob`). Die Datei ≤ 400 Zeilen; bei Bedarf Tabelle nach Gruppen teilen (z. B. `docs/assets/zuordnung-*.md`), der Test liest alle.

## Erlaubte Dateien

- `docs/assets/` (Tabelle)
- `src/tools/` (neuer Test, z. B. `zuordnung.test.ts`)
- `docs/backlog/` (Status, neue Tickets für Lücken, falls B-162 sie nicht abdeckt)
- `docs/sprints/` (nur Status dieser Session)

## Nicht-Ziele

Hub-Stufen, Materialstufen, Materialien, Adern, Plantage, Truhen, Portale, Icons, Hintergründe (GR1.3); neue Assets (GR2); Einbau (GR3); Änderung von `data/`.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Zeilen für alle Gebäude, Gegner und Truppen anlegen, mit Stil aus der Pack-Tabelle und Lizenz aus den CREDITS-Dateien.
3. Test: (a) jede ID aus den drei Daten-Dateien hat eine Zeile; (b) jede Zeile hat Stil und Lizenz; (c) jedes zugeordnete Asset (Pack bzw. Ordner) steht in `public/grafik/CREDITS.md` oder `public/sprites/CREDITS.md`; (d) jede Lücke ist fett und nennt ein Ticket `B-nnn`, das in `docs/backlog/` existiert. Einmal rot sehen (Zeile entfernt), nicht einchecken.
4. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-01: Test belegt eine Zeile mit Status je ID aus `buildings.json`, `enemies.json`, `troops.json`.
- [ ] AC-03: Test belegt einen Credit-Eintrag je zugeordnetem Asset.
- [ ] AC-04: Test belegt fette Markierung und Ticket je Lücke.
- [ ] AC-05: Jede Zeile nennt Stil und Lizenz (Test).
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

## Ergebnis

–
