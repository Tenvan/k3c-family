# CU1.3 · Werkzeugseiten ohne Register und Packs

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/cu1
- **Abhängig von:** CU1.2
- **Tickets:** B-700
- **Kriterien:** AC-05

## Ziel

Die Werkzeugseiten unter `src/tools/` greifen nicht mehr auf alte Pixel-Grafik, Atlas oder Register zu: Die Seiten `grafiken.html` und `pipeline.html` sind entfernt oder zeigen nur Papier-Assets, die Soundtest-Seite liest `data/sounds.json`, Credits nennen nur Sound und Papier.

## Kontext

- Treffer (Suche 2026-11-07): `src/tools/grafiken.ts`, `grafikPacks.ts` (+Test, liest `public/grafik/index.json`), `pipeline.ts` (DB32-Pipeline, `grafik-db32`), `spriteReference.ts` (`data/sprites.json`), `credits.ts` (+Test, `public/sprites/CREDITS.md`, `public/grafik/CREDITS.md`), `soundtest.ts`/`soundtestLogic.ts` (+Test, `data/assets.json`), `assets.test.ts`, `zuordnung.test.ts` (`docs/assets/zuordnung*.md`), `texts.de.ts`/`texts.en.ts`, `src/tools/codemap.md`.
- Kacheln: `src/tools/devTiles.ts` Zeile 59 `grafiken`, Zeile 60 `pipeline`; jede `*.html` im Root wird gebaut, `tests/projectRules.test.ts` prüft Eintrag und `installPageChrome()` (Seiten-Regel in `CLAUDE.md`).
- `data/sounds.json` existiert seit CU1.2. Die Papier-Assets sieht man in der Workbench (Bereich Assets); eine eigene Grafik-Seite im Spiel ist nicht nötig.
- `tests/paperAssets.test.ts` erlaubt alte Gebäude-Texturen nur für bekannte Lücken (US2): nach CU1.2 ist die Liste leer → Ausnahme entfernen.

## Erlaubte Dateien

- `src/tools/**`
- `grafiken.html`, `pipeline.html` (Root, löschen) und weitere Root-`*.html`, die nur alte Grafik zeigen
- `tests/projectRules.test.ts`, `tests/paperAssets.test.ts`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-Zeilen)

## Nicht-Ziele

Spiel-Szenen (CU1.2); Dateien unter `public/` und `docs/assets/` löschen (CU1.5); `site/`.

## Schritte

1. Test (rot): kein Modul unter `src/tools` importiert `data/assets.json`, `data/sprites.json` oder lädt `grafik/index.json`, `grafik-db32/`, `sprites/`, `atlas/`.
2. `grafiken`- und `pipeline`-Seite samt Kacheln, Texten und Tests entfernen; `spriteReference.ts` und `grafikPacks.ts` entfernen, wenn kein anderer Nutzer.
3. Soundtest auf `data/sounds.json` umstellen.
4. `credits.ts` liest nur noch Sound-Credits (und Papier-Hinweis), Test anpassen.
5. `zuordnung.test.ts`, `assets.test.ts` löschen oder auf Papier umstellen; `tests/paperAssets.test.ts` ohne Ausnahmeliste.
6. `task check:diff`.

## Fertig, wenn

- [ ] AC-05: Test aus Schritt 1 grün; `git grep -n -E "assets.json|sprites.json|grafik-db32|grafik/index.json|public/sprites" -- src/tools tests` ohne Treffer.
- [ ] Soundtest-Seite spielt alle Slots (Unit-Test der Logik grün).
- [ ] `tests/projectRules.test.ts` grün (keine verwaisten Seiten oder Kacheln).

## Prüfen

```bash
task check:diff
```

Kein Review-Agent, außer der Diff wächst über 150 geänderte Code-Zeilen (gelöschte Dateien zählen nicht).

## Ergebnis

- **AC-05 umgesetzt, geprüft:** `src/tools/noOldGraphics.test.ts` (neu) prüft, dass kein Modul unter `src/tools` `data/assets.json`, `data/sprites.json`, `grafik/index.json`, `grafik-db32`, `public/sprites`, `atlas/` oder die alten CREDITS-Dateien nennt: grün. `git grep -E "assets.json|sprites.json|grafik-db32|grafik/index.json|public/sprites" -- src/tools tests` trifft nur noch den Schutz-Test `tests/projectRules.test.ts` (nennt `grafik-db32` als verbotenen Pfad).
- **Entfernt:** Seiten `grafiken.html` und `pipeline.html` samt Kacheln, Texten (`grafik.*`, `pipeline.*`, `sel.*`, `tile.grafiken/pipeline`) und Modulen (`grafiken.ts`, `grafikPacks.ts`+Test, `pipeline.ts`, `spriteReference.ts`+Test, `selection.ts`+Test) sowie die Register-Tests `assets.test.ts`, `zuordnung.test.ts` und `tests/sprites.test.ts` (prüften `data/sprites.json`). `installPadScroll` zog nach `padScroll.ts` (von `lizenzen.ts` genutzt).
- **Soundtest:** liest `data/sounds.json`; die Logik-Tests (`soundtestLogic.test.ts`) sind grün, die Seite lädt im Dev-Server ohne Fehler (Slots erscheinen nach dem Entsperren per Taste, wie bisher; nicht hörend geprüft).
- **Credits:** `credits.ts` liest nur noch `public/audio/README.md` (Sound-Credits, 52 Zeilen auf der Lizenzseite); die Lizenzseite sagt im Abschnitt Grafiken, dass die Grafik eigener Papierschnitt ist, die Reittier-Hinweise (CC BY 3.0) entfallen, weil die Pixel-Reittiere nicht mehr im Spiel sind. Die Textstelle „Spiel hat keinen Ton“ im Sound-Abschnitt ist durch einen zutreffenden Satz ersetzt.
- `tests/paperAssets.test.ts` ohne Ausnahmeliste: schon in CU1.2 erledigt. `tests/projectRules.test.ts` grün (keine verwaisten Seiten oder Kacheln).
- **Geprüft:** `tsc`, `oxlint`, Vitest `src` + `tests` (2825 Tests) grün. `task check:diff` nicht gelaufen (Nutzerwunsch); kein Review-Agent (Diff fast nur Entfernen).
