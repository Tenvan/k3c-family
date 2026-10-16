# GEB3.4 · Mauer und Turm im Spiel, alte Grafik gelöscht

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb3/4-zuordnung-aufraeumen
- **Abhängig von:** GEB3.3
- **Tickets:** B-435
- **Kriterien:** AC-02, AC-06

## Ziel

Mauer und Turm zeigen im Spiel in allen Stufen 1–5 die neuen Grafiken in Maßstab 1; die ersetzte Pack-Grafik ist mit allen Verweisen gelöscht.

## Kontext

- **Spec:** GEB3 AC-02, AC-06; Muster aus GEB2.3 (`textureScale` = 1 für Dateien unter `gebaeude/`).
- `src/scenes/buildingSprites.ts`: `BUILDING_TEXTURES` (`grafik:wall-2…5`, `grafik:tower-2…4`), `STAGES` (`wall`/`tower`, Stufe 1 und Turm 5 sind heute `null`), `BASE` (ohne Stufe im Snapshot → Stufe 2, so lassen). Neu: `grafik:wall-1…5` → `gebaeude/mauer-<n>-32.png`, `grafik:tower-1…5` → `gebaeude/turm-<n>-32.png`, keine `null` mehr.
- `src/scenes/siteView.ts` › `createSiteView`: Die Mauer ist heute ein `TileSprite` (16 × 80, gestapelte Kachel). Mit eigenem Bild wird sie ein normales `Image` wie die anderen Gebäude; `SITE_SIZE` (Platzhalter) bleibt. `showPicture` setzt Textur und Maßstab schon (GEB2.3).
- `data/assets.json`: Slots `wall:1`–`wall:5`, `tower:1`–`tower:5` → Präferenz 1 = neue PNG.
- **Aufräumen (AC-06):** Kandidaten `public/grafik/gothicvania-town/tileset-einzeln/wall.png`, `public/grafik/k3c-paletten/tileset-einzeln/wall-*.png` (auch `wall-b-*`), `public/grafik/k3c-paletten/props/tower-*.png`. Vor dem Löschen je Datei `rg -n "<name>" --glob '!docs/sprints/erledigt/**' --glob '!.claude/**'`; nur löschen, was danach kein Code, keine Daten und kein Test mehr nutzt. Mitziehen: `public/grafik/index.json`, `docs/assets/zuordnung-objekte.md`, `docs/assets/zuordnung-welt.md`, Test-Fixtures und Mocks, CREDITS (ein leerer Pack fällt ganz weg). Historische Belege (`docs/funde/`) bleiben.

## Erlaubte Dateien

- `src/scenes/buildingSprites.ts`, `src/scenes/buildingSprites.test.ts`, `src/scenes/siteView.ts`
- `data/assets.json` (nur Mauer- und Turm-Slots)
- AC-06: die oben genannten alten PNGs, `public/grafik/index.json`, `public/grafik/CREDITS.md`, `docs/assets/zuordnung-*.md`, Test-Fixtures und Mocks, die genau diese Dateien nennen
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Kein Tor (GEB4), keine Burg, keine Änderung an `engine/`, an Stufen-Logik oder an den PNGs.

## Schritte

1. `sprint/geb3` auschecken, `Status: in Arbeit`, committen, pushen.
2. Test zuerst in `buildingSprites.test.ts`: `siteTexture('wall'|'tower', 'built', 1…5)` liefert je Stufe einen Schlüssel auf `gebaeude/<mauer|turm>-<n>-32.png`, Maßstab 1; keine `null` mehr; bestehende Erwartungen (Lücken) anpassen.
3. Zuordnung umsetzen; Mauer in `createSiteView` als `Image`.
4. Register `data/assets.json`.
5. AC-06: Suche je Kandidat, löschen und Verweise bereinigen, Suche ohne Treffer belegen.
6. Code-Review, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-02: Vitest prüft Zuordnung aller zehn Stufen und Maßstab 1; Register gesetzt; `assets.test.ts` grün.
- [ ] AC-06: alte Mauer- und Turm-PNGs gelöscht, Suche nach ihren Namen ohne Treffer außerhalb historischer Doku.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task check
task check:go
```

Keine manuellen Prüfungen; der Blick ins Spiel gehört GEB3.5.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
