# GEB3.7 · Hintergrundmauer im Spiel zeichnen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb3/7-hintergrundmauer-spiel
- **Abhängig von:** GEB3.6
- **Tickets:** B-442
- **Kriterien:** AC-08

## Ziel

Im Spiel steht zwischen je zwei benachbarten gebauten Mauern derselben Seite eine durchgehende Hintergrundmauer hinter den Bauplätzen, in der Stufe der äußeren Mauer.

## Kontext

- **Spec:** GEB3 AC-08, › Beispiele und › Ausnahme- und Fehlerfälle (Revision 2); B-442; Glossar „Hintergrundmauer“.
- **Daten im Snapshot:** `World.sites` (`src/model/types.ts`: `Site.kind === 'wall'`, `x` in Units, `state`, `level` „fehlt = 1“) und `World.hubX` (Hub-Mitte). Seite = Vorzeichen von `x - hubX`; benachbart = nach Abstand zur Hub-Mitte sortiert; „äußere“ = größerer Abstand.
- **Logik:** reine Funktion (z. B. `hintergrundmauern(sites, hubX)` in einer neuen Datei `src/scenes/backWallRules.ts` oder in `buildingSprites.ts`), Ergebnis je Abschnitt `{ von, bis, stufe }` in Units; nur Mauern mit `state === 'built'`. Vitest: Beispiel aus der Spec, beide Seiten, Lücke bei ungebauter Mauer, nur eine Mauer → nichts.
- **Zeichnen:** `src/scenes/worldRenderer.ts` bzw. ein kleines eigenes View-Modul: je Abschnitt ein `TileSprite` mit `gebaeude/hintergrundmauer-<stufe>-32.png` (Maßstab 1, Breite = Abschnitt in px, Höhe 4 Kacheln, Unterkante auf dem Boden `GROUND_Y`), Tiefe 1 (Bauplätze haben 2). Neu zeichnen nur bei geändertem Abschnitts-Schlüssel (wie `updateSiteView`). Texturen in `buildingSprites.ts` (`BUILDING_TEXTURES`) eintragen, damit LoadScene sie lädt.
- **Register:** neuer Slot `wall:back` o. ä. in `data/assets.json` (Kategorie `gebaeude`, Präferenz 1 = Stufe 1), damit der Asset-Manager ihn kennt.
- 2 Spieler (Split-Screen): die Hintergrundmauer ist Weltgrafik und erscheint in beiden Kameras wie die anderen Bauplätze.

## Erlaubte Dateien

- `src/scenes/` (neue Regel-Datei mit Test, `buildingSprites.ts`, `worldRenderer.ts` bzw. neues View-Modul)
- `data/assets.json` (nur der neue Slot)
- `docs/assets/zuordnung-welt.md` (Zeile für die Hintergrundmauer)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Sim- oder Protokolländerung, keine Kollision, kein Schaden an der Hintergrundmauer, keine Änderung an der Grafik.

## Schritte

1. `sprint/geb3` auschecken, `Status: in Arbeit`, committen, pushen.
2. Test zuerst für die Abschnitts-Funktion (Fälle aus dem Kontext).
3. Funktion umsetzen, Zeichnen mit TileSprite hinter den Bauplätzen, Texturen laden.
4. Register-Slot und Zuordnungs-Doku.
5. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-08: Vitest belegt Lage, Stufe der äußeren Mauer und Lücken; die Hintergrundmauer wird mit Tiefe unter den Bauplätzen gezeichnet; Slot im Register.
- [ ] `task check` grün.

## Prüfen

```bash
task check
```

Keine manuellen Prüfungen; der Blick ins Spiel gehört GEB3.5.

## Ergebnis

2026-10-16, Claude (autonom, Ralf im Chat).
- AC-08 umgesetzt und geprüft: `src/scenes/backWallRules.ts` (`backWalls(sites, hubX)`: je Seite nach Abstand zur Hub-Mitte sortierte Mauer-Plätze, Abschnitt nur zwischen zwei benachbarten gebauten, Stufe der äußeren, `level` fehlt = 1) mit Vitest `backWallRules.test.ts` (Spec-Beispiel, beide Seiten, Lücke durch ungebaute Mauer, eine Mauer allein, Turm zählt nicht; erst rot, dann grün). `backWallView.ts`: gekacheltes `TileSprite` in Maßstab 1, Tiefe 1 hinter den Bauplätzen, Unterkante am Boden, Lage und Breite auf ganze Pixel gerundet; fehlt die Textur, bleibt es unsichtbar. `worldRenderer.ts` synchronisiert die Abschnitte als eigene Ebene (id = äußere Mauer), damit beide Kameras sie zeigen. Texturen `grafik:backwall-1…5` in `buildingSprites.ts` (`backWallTexture`, Vitest). Slot `wall:back` in `data/assets.json`, Zeile in `docs/assets/zuordnung-welt.md`.
- Checks: `task check` und `task check:go` grün; `dev:test` rot nur vorbestehend (B-440). Review (Sonnet-Agent): keine schweren Befunde; Rundung auf ganze Pixel übernommen.
- Nicht geprüft: Darstellung im laufenden Spiel (gehört GEB3.5).
