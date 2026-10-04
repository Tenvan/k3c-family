# GR3.2 · Ressourcen, Adern, Plantage und Parallax je Biom, Credits prüfen

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Branch:** gr3/2-ressourcen-parallax
- **Abhängig von:** GR3.1
- **Tickets:** B-010
- **Kriterien:** AC-01, AC-02, AC-05, AC-06

## Ziel

Ressourcen (Bäume, Steine, Erz, später Adern und Plantage), Portale, Truhen und Münzen sowie die Parallax-Ebenen von Wald, Höhle und Mine werden mit den zugeordneten Grafiken gezeichnet; die Credits aller eingebauten Grafiken stehen in `public/`.

## Kontext

- **Zuordnung:** wie GR3.1 (`docs/assets/zuordnung.md`, Status je Zeile). Lücken (laut B-010 › Notizen u. a. Mine-Hintergrund) bekommen einen **dokumentierten Platzhalter** (AC-05): im Ergebnis nennen, welche Ebene Platzhalter ist, und das Ticket aus der Zuordnung verlinken.
- **Ressourcen heute:** `WorldRenderer.createNode()` in `src/scenes/worldRenderer.ts` zeichnet `tree`, `rock`, `copperOre` und Busch als Formen; `drawStatic()` Portal (`ellipse`), Ausgang (`exit`), Rekrutierungslager und Busch; Pickups in `createPickup()`. Adern und Plantage sind noch nicht im Snapshot (B-114, W2): erst wenn sie dort stehen, Sprites anbinden (Pack `various-stones-and-oregem-veins-16x16`), vorher nur die Zuordnungszeile nutzen und nichts erfinden.
- **Parallax heute:** `src/scenes/stageView.ts` zeichnet zwei Silhouetten (`ridge()`, Scroll-Faktor 0,3 und 0,6) in den Palettenfarben `world.biome.palette` (`sky`, `far`, `near`, `ground`; `data/biomes/forest.json`, `cave.json`, `mine.json`). Die Ebenen aus den Packs (`forest-background`, `sunnyland-*`, `blue-cave-background`, `gothicvania-*`, `warped-caves-pixel-art-pack`; Bildliste `public/grafik/index.json`, Gruppe `ebenen`) ersetzen sie je Biom; als `TileSprite` mit `setScrollFactor` kacheln, damit sie über `widthUnits * UNIT_PX` reichen. Palettenbruch ist bei Hintergründen erlaubt (Q13).
- **Laden:** Einzelne Bilder in `LoadScene.preload()` wie in GR3.1; wird der Ladebalken zu lang, nur das Biom laden, das gebraucht wird. Kaltstart-Budget aus GR4: an der Xbox noch nicht gemessen (angenommen, `docs/plan-weiterentwicklung.md` § 11.6).
- **Credits:** Quelle ist `public/grafik/CREDITS.md` (und `public/sprites/CREDITS.md`), die Seite `lizenzen.html` liest sie über `src/tools/credits.ts`; `src/tools/credits.test.ts` prüft Vollständigkeit (GR6). Warped Caves ist **CC BY 3.0**: die Namensnennung steht dort schon und muss erhalten bleiben. Der Satz „Stand der Verwendung: noch nicht im Spiel“ im Kopf von `public/grafik/CREDITS.md` wird mit dieser Session falsch und ist anzupassen.
- **Regeln:** `src/scenes` rechnet nichts; 2 Spieler im Split-Screen (Parallax je Kamera, `showOnly`); Datei ≤ 400 Zeilen, `worldRenderer.ts` nicht verlängern.

## Erlaubte Dateien

- `src/scenes/worldRenderer.ts`, `src/scenes/stageView.ts`, `src/scenes/LoadScene.ts`, neue Dateien in `src/scenes/` mit Tests daneben
- `public/grafik/CREDITS.md` (nur Credits und Verwendungsstand)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Gebäude und Stufen (GR3.1), neue Grafiken (GR2), Atlas-Werkzeug (GR4), Effekte (GR5), Reittiere.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. Zuordnung für Ressourcen, Portale, Truhen, Münzen und Hintergründe lesen.
2. Ressourcen, Portal, Truhe und Münze auf Sprites umstellen (Auswahlfunktion aus GR3.1 mitnutzen; Rückfall Platzhalter-Form).
3. Parallax je Biom (Wald, Höhle, Mine) aus den zugeordneten Ebenen; wo Lücke: Platzhalter (bestehende `ridge()`) und im Ergebnis dokumentieren.
4. Credits prüfen: jedes eingebaute Pack steht in den CREDITS-Dateien, `credits.test.ts` grün; Verwendungsstand im Kopf aktualisieren.
5. Alle drei Biome im Browser-Pane ansehen (Wald, Höhle, Mine, z. B. über `leveltest.html` oder die Dev-Aktionen aus `src/scenes/debugActions.ts`), Screenshot je Biom; 2 Spieler im Split-Screen.
6. `task check`. Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [ ] AC-02: Jedes eingebaute Pack hat Urheber, Lizenz und Quelle in `public/grafik/CREDITS.md`; `src/tools/credits.test.ts` ist grün.
- [ ] AC-05: Wald, Höhle und Mine zeigen je eigene Parallax-Ebenen im Browser-Pane (Screenshot) oder der Platzhalter ist im Ergebnis mit Ticket dokumentiert.
- [ ] AC-06: `task check` grün, mit 2 Spielern im Split-Screen keine Darstellungsfehler (Browser-Pane).
- [ ] AC-01: Ressourcen, Portale, Truhen, Münzen und Parallax-Ebenen sind Sprites, wo die Zuordnung `zugeordnet` sagt (Screenshot je Biom), sonst Platzhalter.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

–
