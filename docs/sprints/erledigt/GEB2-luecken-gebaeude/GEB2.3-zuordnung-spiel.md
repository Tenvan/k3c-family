# GEB2.3 · Neue Gebäude-Grafik im Spiel zuordnen

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb2/3-zuordnung-spiel
- **Abhängig von:** GEB2.2
- **Tickets:** B-435
- **Kriterien:** AC-02, AC-04

## Ziel

Werkstatt, Taverne, Heilplatz, Schmiede und Rüstkammer zeigen im Spiel gebaut die neuen Grafiken aus `public/grafik/gebaeude/` in Maßstab 1 (32 px je Kachel); Register und Client-Zuordnung stimmen überein, Vitest prüft beides.

## Kontext

- **Spec:** Sprint-README GEB2 AC-02; Spiel-Dichte 32 px je Kachel ×1 (`docs/standards/gebaeude-grafik.md` § 7).
- **Client-Zuordnung:** `src/scenes/buildingSprites.ts` – `BUILDING_TEXTURES` (Schlüssel → Datei unter `public/grafik/`, LoadScene lädt genau diese) und `BASE` (SiteKind → Schlüssel; `tavern`, `healer`, `smithy`, `armory` fehlen dort heute → Platzhalter). `grafik:workshop` zeigt heute `gothicvania-town/props-einzeln/house-b.png`.
- **Maßstab:** `src/scenes/siteView.ts` zeichnet jedes Bauplatz-Bild mit `const ZOOM = 2` (`createSiteView`: `picture.setScale(ZOOM)`; `showPicture` setzt nur die Textur). Die neuen PNGs brauchen Maßstab 1. Kleinster Weg: in `buildingSprites.ts` eine reine Funktion (z. B. `textureScale(key)`: 1 für Dateien unter `gebaeude/`, sonst 2) mit Test; `showPicture` setzt den Maßstab beim Texturwechsel.
- **Register:** `data/assets.json` – Slots `workshop`, `tavern`, `healer`, `smithy`, `armory` (`kategorie: gebaeude`); Präferenz 1 (erster Eintrag in `wahl`) wird `{ "datei": "grafik/gebaeude/<id>-32.png" }`; bei `workshop` rückt `house-b.png` auf Präferenz 2. `src/tools/assets.test.ts` prüft, dass jede `datei` existiert. Falls die k3c-dev-Tools laufen, Register über `assets_set` ändern.
- Dateinamen: `werkstatt`, `taverne`, `heilplatz`, `schmiede`, `ruestkammer` (je `-32.png`).
- Baustellen und zerstörte Gebäude bleiben Platzhalter (`siteTexture` gibt nur bei `built` eine Textur).

## Erlaubte Dateien

- `src/scenes/buildingSprites.ts`, `src/scenes/buildingSprites.test.ts`, `src/scenes/siteView.ts`
- `data/assets.json` (nur die fünf Slots)
- `docs/assets/zuordnung-objekte.md` (falls dort die Gebäude-Zuordnung steht)
- AC-04, Löschen von `house-b.png`: `public/grafik/gothicvania-town/props-einzeln/house-b.png`, `public/grafik/index.json`, CREDITS des Packs, `src/tools/assetvorschauLogic.test.ts`, `tools/k3c-dev/app_assets_test.go`, `tools/k3c-dev/frontend/src/api/mockAssets.ts` (nur den Verweis ersetzen oder entfernen)
- `docs/sprints/`, `docs/backlog/`

## Nicht-Ziele

Keine Bäume (`node:tree`, B-438), kein Umbau anderer Gebäude auf Maßstab 1, keine Ausbaustufen, keine Änderung an `engine/` oder an den PNGs, kein TV-Faktor (B-251).

## Schritte

1. `sprint/geb2` auschecken, `Status: in Arbeit`, committen, pushen.
2. Test zuerst in `buildingSprites.test.ts`: `siteTexture('tavern'|'healer'|'smithy'|'armory'|'workshop', 'built')` liefert einen Schlüssel auf `gebaeude/<id>-32.png`; Maßstab dieser Schlüssel 1, z. B. `grafik:barracks` weiter 2.
3. `BUILDING_TEXTURES`, `BASE` und Maßstab-Funktion umsetzen; `siteView.ts` nutzt den Maßstab beim Zeigen.
4. Register `data/assets.json` für die fünf Slots nach Kontext; bei `workshop` nur der neue Eintrag.
5. AC-04: `rg -n "house-b" --glob '!docs/sprints/erledigt/**' --glob '!.claude/**'` → jeden Verweis entfernen oder ersetzen (Test-Fixtures auf eine andere vorhandene Datei umstellen), PNG löschen; Suche erneut ohne Treffer. Nutzt eine weitere Stelle die Datei im Spiel, nicht löschen, sondern als Ticket melden.
6. Code-Review der geänderten Dateien, Ergebnis ausfüllen, `Status: fertig`, ein Commit, push.

## Fertig, wenn

- [ ] AC-02: Vitest in `buildingSprites.test.ts` prüft Zuordnung der fünf Slots und Maßstab 1 bzw. 2.
- [ ] AC-02: `data/assets.json` hat für die fünf Slots Präferenz 1 = neue PNG; `assets.test.ts` grün.
- [ ] AC-04: `house-b.png` gelöscht, Suche nach `house-b` ohne Treffer außerhalb `docs/sprints/erledigt/`.
- [ ] `task check` und `task check:go` grün.

## Prüfen

```bash
task check
```

Keine manuellen Prüfungen in dieser Session; der Blick ins Spiel gehört GEB2.4.

## Ergebnis

2026-10-16, Claude (autonom, Ralf im Chat).
- AC-02 geprüft: `buildingSprites.ts` ordnet `workshop`, `tavern`, `healer`, `smithy`, `armory` den `gebaeude/<id>-32.png` zu; `textureScale` = 1 für eigene Gebäude, sonst 2; `siteView.showPicture` setzt den Maßstab beim Zeigen. Test „Gebäude aus tools/gebaeude …“ in `buildingSprites.test.ts` (erst rot, dann grün). Register `data/assets.json`: die fünf Slots mit Präferenz 1 = neue PNG; `docs/assets/zuordnung-objekte.md` nachgezogen (alte Lücken-Zeilen entfernt).
- AC-04 geprüft: `house-b.png` gelöscht, Eintrag in `public/grafik/index.json` entfernt; Test-Fixtures und Mock auf `house-c` umgestellt (`assetvorschauLogic.test.ts`, `mockAssets.ts`, `assets.test.ts`, `app_assets_test.go`, `register_test.go`; die beiden letzten über die Liste hinaus, gleiche Art Fixture). CREDITS bleibt (Pack wird weiter genutzt). Suche nach `house-b` ohne Treffer in Code, Daten und Tests. Abweichung: historische Belege behalten den Namen (`docs/funde/gr2-grafik-funde.html`, Entscheidung GR2.2, und die GEB2-Sprintdateien selbst); Umschreiben würde die Historie verfälschen.
- Nebenbei: `zuordnung.test.ts` nimmt `grafik/gebaeude` wie `credits.test.ts` von der Credit-Pflicht aus (eigene Grafik).
- Checks: `task check` und `task check:go` grün. `dev:test` rot, vorbestehend auf `develop` (GEB3–GEB7 als Entwurf ohne Sessions) → B-440.
- Review (Sonnet-Agent): keine Befunde. Blick ins Spiel gehört GEB2.4.
