# GEB11.4 · Spiel zeichnet die Burg nach Hub-Stufe und Ausbau-Stand

- **Status:** offen
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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
