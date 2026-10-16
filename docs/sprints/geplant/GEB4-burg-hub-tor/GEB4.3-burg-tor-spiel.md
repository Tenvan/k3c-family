# GEB4.3 · Burg und Tor im Spiel, alte Grafik gelöscht

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb4/3-burg-tor-spiel
- **Abhängig von:** GEB4.2
- **Tickets:** B-435
- **Kriterien:** AC-05, AC-06

## Ziel

Im Spiel zeigt die Burg die aktuelle Hub-Stufe und wechselt beim Ausbau, jedes Tor zeigt die Stufe der Mauer seiner Linie; die ersetzte Burg- und Tor-Grafik ist samt Verweisen gelöscht.

## Kontext

- **Spec:** GEB4 AC-05, AC-06, › Ausnahme- und Fehlerfälle.
- **Burg heute:** `siteView.drawCastle` setzt die Burg aus Frames von `grafik:fort-tileset` plus `grafik:fort-banner` zusammen (Maßstab 2); `worldRenderer.drawStatic` ruft sie **einmal** auf, mit `hubTexture()` ohne Stufe (Vermutung aus der Planung: Burg wechselt nie). Neu: ein Bild `gebaeude/burg-<n>-32.png` in Maßstab 1, Unterkante am Boden, mittig auf `world.castle.x`; bei Änderung von `World.hubLevel` (fehlt = 1) die Textur tauschen (z. B. in `sync` mit Schlüssel wie `updateSiteView`). `hubTexture(stage)` liefert dann `grafik:hub-<n>`; der HP-Balken der Burg (`castleHp`, y `G - 300`) an die neue Höhe (288 px) anpassen.
- **Tor:** `siteTexture('gate', …)` liefert heute `grafik:gate` (Pack). Neu: Tor in Stufe n = Stufe der gebauten Mauer **seiner Linie** (gleiche Seite relativ zu `World.hubX`, nächste Mauer weiter innen; Tor-Platz ist 4 Units außen vor der Mauer, `data/hub.json` › `wallLines.gateOutsetUnits`); ohne gebaute Mauer Stufe 1. Reine Funktion (z. B. `gateStage(gate, sites, hubX)`) mit Vitest; `showPicture` bekommt für Tore diese Stufe. Cache-Schlüssel in `updateSiteView` um die Tor-Stufe ergänzen (Lehre aus GEB3.4).
- **Register:** `castle`, `hub:1`–`hub:5`, `gate` → neue PNGs (Präferenz 1).
- **Aufräumen (AC-06):** Kandidaten `k3c-paletten/ebenen/fort-tileset.png`, `k3c-paletten/props/fort-banner.png`, `fort-door.png`, `fort-closed-door.png` (danach ist `k3c-paletten` leer → Pack, CREDITS-Zeile, `grafikPacks.ts`-Eintrag und LICENSE weg), `wooden-fortress-and-animated-doors/…/wooden-castle.png`, `opp2017-castle-tiles/…` (nur löschen, was nichts anderes nutzt; leere Packs ganz). Je Kandidat `git grep` vorher und nachher; Mitziehen wie in GEB3.4 (`public/grafik/index.json`, `docs/assets/zuordnung-*.md`, Tests, Mocks).

## Erlaubte Dateien

- `src/scenes/` (`buildingSprites.ts`, `siteView.ts`, `worldRenderer.ts`, neue Regel-Datei mit Test, zugehörige Tests)
- `data/assets.json` (nur `castle`, `hub:*`, `gate`)
- AC-06: die genannten alten PNGs und Pack-Ordner, `public/grafik/index.json`, `public/grafik/CREDITS.md`, `src/tools/grafikPacks.ts`, `docs/assets/zuordnung-*.md`, Tests und Mocks, die genau diese Dateien nennen
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Sim- oder Protokolländerung, keine Änderung an den PNGs, keine Lager/Farm/Kaserne/Treppen (GEB5).

## Schritte

1. `sprint/geb4` auschecken, `Status: in Arbeit`, committen, pushen.
2. Tests zuerst: `hubTexture(1…5)` → `gebaeude/burg-<n>-32.png` (Maßstab 1); Tor-Stufe aus Mauerlinie (beide Seiten, ungebaute Mauer → 1, `level` fehlt → 1).
3. Burg als ein Bild mit Stufenwechsel, Tor mit Stufe, Cache-Schlüssel, HP-Balken-Höhe.
4. Register.
5. AC-06: Suche, löschen, Verweise bereinigen, Suche ohne Treffer belegen.
6. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-05: Vitest für Burg-Stufe und Tor-Stufe; Burg wechselt bei `hubLevel`-Änderung (Schlüssel/Update-Pfad im Code belegt); Register gesetzt.
- [ ] AC-06: alte Grafik gelöscht, Suche ohne Treffer außerhalb historischer Doku.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task check
task check:go
```

Keine manuellen Prüfungen; der Blick ins Spiel gehört GEB4.4.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
