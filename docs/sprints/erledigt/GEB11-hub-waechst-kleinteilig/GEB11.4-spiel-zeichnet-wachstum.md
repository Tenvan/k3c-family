# GEB11.4 · Spiel zeichnet die Burg nach Hub-Stufe und Ausbau-Stand

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** geb11/4-spiel-zeichnet-wachstum
- **Abhängig von:** GEB11.3
- **Tickets:** B-458
- **Kriterien:** AC-04

## Ziel

Das Spiel zeichnet die Burg nach `hubLevel` und Ausbau-Stand (`hubUpgrade`) mit dem Look aus der Wachstumsreihe, während des Baus mit Gerüst; ohne Grafik fällt es auf die Burg der Hub-Stufe zurück.

## Kontext

- `src/scenes/render/buildingSprites.ts` (`BUILDING_TEXTURES`, `hubTexture`), `siteView.ts` (`createCastleView`, `updateCastleView`), `worldRenderer.ts` (Aufruf mit `world.hubLevel ?? 1`).
- Reihe und Ableitung: `src/scenes/render/hubGrowth.ts` (GEB11.3), Regel `materialien-gebaeude.md` § 2.1.
- Register `data/assets.json`, Doku `docs/assets/zuordnung-welt.md`.

## Erlaubte Dateien

- `src/scenes/render/buildingSprites.ts`, `siteView.ts`, `worldRenderer.ts`, `hubGrowth.ts` und Tests
- `data/assets.json` (Register), `docs/assets/zuordnung-welt.md`, `docs/standards/gebaeude-grafik.md` § 6

## Nicht-Ziele

`engine/`, Protokoll, Kosten, Bauplatz-Freischaltung.

## Schritte

1. Test zuerst: Ableitung Look aus Hub-Stufe und Ausbau-Stand inklusive Rückfälle (alter Server ohne `hubUpgrade`, letzte Stufe, unbekannte Stufe).
2. Texturen registrieren, Burg-Ansicht mit Gerüst-Bild.
3. Register und Zuordnungs-Doku.
4. `task check:diff`.

## Fertig, wenn

- [ ] AC-04 (Teil Spiel): Zuordnungs-Tests grün; die Abnahme durch 🧑 ist GEB11.6.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-04 (Teil Spiel) umgesetzt und geprüft: `hubLookOf` (`src/scenes/render/hubGrowth.ts`) leitet den Look aus `World.hubLevel` und `World.hubUpgrade` ab (Hub-Stufe 1 vor vollem Gold: Lagerfeuer bei 0, Zelt unter der Hälfte, Hütte ab der Hälfte; sonst Burg der Stufe; `scaffold` bei `waitingWorker`; ohne `hubUpgrade` oder ohne Goldkosten die Burg, ohne `hubLevel` Stufe 1). `buildingSprites.ts` registriert `grafik:lagerfeuer-1..3` und `grafik:hub-geruest` (aus `HUB_GROWTH`), `hubLookTexture`, `HUB_SCAFFOLD_TEXTURE`; `siteView.ts` › `updateCastleView` bekommt die Welt statt der Stufe, zeigt den Look und das Gerüst als drittes Bild im Container, fällt ohne Grafik auf die Burg der Hub-Stufe und dann auf die Platzhalter-Form zurück; `worldRenderer.ts` reicht `world` durch. Tests: `hubGrowth.test.ts` (Schwellen 0/1/24/25/49/50, Gerüst, Rückfälle, Reihe = Vorschau) und `buildingSprites.test.ts` (Texturen, Dateien unter `public/grafik/`) grün. Register `data/assets.json` (`hub:camp:1..3`, `hub:scaffold`), `docs/assets/zuordnung-welt.md` und `gebaeude-grafik.md` § 6 nachgezogen. Abweichung: Namen und Bedingungen der Looks stehen nicht im Spielcode (`textRule.test.ts` verbietet deutsche Literale in `src/scenes/`), sondern im Studio (`scenesLogic.ts` › `WACHSTUM_TEXT`); der Spielcode hält nur die Reihe.

Prüfung: Client-Vitest 2356/2356 und Studio-Vitest 471/471 grün, `tsc --noEmit` (Client und Studio), `oxlint` ohne Befund in den geänderten Dateien, `go test ./tools/buildings` grün. `task check:diff` als Ganzes kam nicht durch: ein Lauf brach mit „parallel golangci-lint is running“ ab (zweiter Agent in anderem Worktree), der zweite am 10-Minuten-Zeitlimit (go test bis dahin grün); dieser Sprint ändert keinen Go-Code. Kein Reviewer-Agent: unter 150 Code-Zeilen, nicht SIM/SRV, kein Protokoll. Bekannt, ungeprüft im Spiel: Der HP-Balken der Burg steht fest 310 px über dem Boden und schwebt beim Lagerfeuer höher (nur bei Schaden sichtbar). Nicht im Browser geprüft; Abnahme AC-04 durch 🧑 in GEB11.6.
