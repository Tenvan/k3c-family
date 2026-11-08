# CU1.2 · Spiel-Client lädt nur noch Papier

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Lane:** –
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** sprint/cu1
- **Abhängig von:** CU1.1
- **Tickets:** B-700
- **Kriterien:** AC-04, AC-05

## Ziel

Das Spiel lädt nur noch Papier-Assets: Gerüste und Plantage aus den Papier-Gruppen, keine Datei mehr aus `public/sprites/`, `public/atlas/`, alten Packs in `public/grafik/`; Maße aus `data/paper.json` statt `data/sprites.json`, Sound-Slots aus `data/sounds.json` statt `data/assets.json`.

## Kontext

- Gerüste/Plantage: `src/scenes/render/buildingSprites.ts` (Zeilen 39–57: `scaffoldKey`, `grafik:hub-geruest`, `grafik:setzling`, `grafik:pappel-niedrig` → `gebaeude/*-32.png`), `scaffoldView.ts`, `hubGrowth.ts`, `worldRenderer.ts:250`. Papier-Assets aus CU1.1; Zuordnung wie bei den übrigen Gebäuden/Objekten über `data/paper.json` › `gebaeude` bzw. `objekte` (Frame, Rahmen, Frames). Papier-Sheets sind Streifen, nicht Einzelbilder.
- `src/scenes/render/sprites.ts`: liest `data/sprites.json` (Größe, Höhe über Boden, Deckkraft je Figur, `mounts`, `riderWaist`), `preloadSheet` lädt noch `sprites/<sheet>/<anim>.png`. `paperAssets.ts` nennt `mounts`-Schlüssel. Werte nach `data/paper.json` (Abschnitte `spieler`, `beritten`, `reittiere`, `truppen`, `gegner` existieren), `sheets` entfällt; `preloadSheet` und Atlas-Wege entfernen.
- `data/embed_test.go:16` listet `sprites.json`; Kommentar `engine/sim/monarch_data.go:40` nennt `sprites.json › mounts` (Go liest die Datei nicht, nur Kommentar anpassen).
- `src/scenes/render/buildingSize.ts` und `siteView.ts` lesen Gebäude-Slots aus `data/assets.json` (B-685) → auf `data/paper.json` › `gebaeude` umstellen.
- `worldSprites.ts`, `natureSprites.ts`: Restpfade auf alte Packs unter `public/grafik/` entfernen.
- Sound: `src/audio/events.ts`, `ui.ts`, `music.ts` importieren `data/assets.json`. Die 32 Slots mit `art: sound` 1:1 nach neuer Datei `data/sounds.json` kopieren (gleiches Format `{_comment, slots}`); `data/assets.json` bleibt bis CU1.5 liegen, weil `tools/soundatlas` und die Workbench es noch lesen.
- Tests mit Bezug: `src/scenes/loadLogic.test.ts`, `mountData.test.ts`, `render/monarchAnim.test.ts`, `mountPose.test.ts`, `buildingSprites.test.ts`, `scaffoldView.test.ts`, `mineSignView.test.ts`, `paperAssets`-Tests, `tests/sprites.test.ts`, `src/audio/*.test.ts`.

## Erlaubte Dateien

- `src/scenes/**` (Render, Load, Tests)
- `src/audio/**`
- `data/paper.json`, `data/sounds.json` (neu), `data/sprites.json` (löschen am Ende, wenn ungenutzt), `data/embed_test.go`, `data/codemap.md`
- `engine/sim/monarch_data.go` (nur Kommentar)
- `tests/sprites.test.ts` (umstellen oder löschen)
- `src/scenes/codemap.md`, `src/scenes/render/codemap.md`, `src/audio/codemap.md`
- `docs/backlog/B-701-pap-gesammelte-fragen.md` (D-Zeilen)

## Nicht-Ziele

`src/tools/` (CU1.3); `data/assets.json` löschen und `tools/soundatlas` (CU1.5); Dateien unter `public/` löschen (CU1.5); D-US2-04.

## Schritte

1. Test (rot): Gerüst einer Klasse und `setzling`/`pappel-niedrig` liefern Papier-Texturen aus `gebaeude-paper` bzw. `objekte`; keine Ladeliste enthält `gebaeude/*-32.png`, `sprites/` oder `atlas/`.
2. Gerüste und Plantage auf Papier umstellen (Frame/Origin/Zoom wie die übrigen Gebäude).
3. Gebrauchte Werte aus `data/sprites.json` nach `data/paper.json`; `sprites.ts`, `paperAssets.ts` und Tests umstellen; `preloadSheet` entfernen; `data/sprites.json` löschen, wenn `git grep sprites.json -- src data engine tests` leer ist.
4. `buildingSize.ts`/`siteView.ts` ohne `data/assets.json`.
5. `data/sounds.json` anlegen, `src/audio` umstellen; Test: gleiche Slot-Ids wie die Sound-Slots in `data/assets.json`.
6. Fehlendes Papier-Asset → einmal 🤒-Warnung, kein Rückfall.
7. Review-Agent (Sonnet) parallel zu `task check:diff` (Loader-Code, über 150 Zeilen).

## Fertig, wenn

- [ ] AC-04: Test aus Schritt 1 grün; im Spiel (headless-Probe mit `window.game` oder Lade-Test) zeigt ein Bauplatz im Bau das Papier-Gerüst.
- [ ] AC-05: `git grep -n -E "sprites/|atlas/|data/sprites.json|data/assets.json|grafik-db32" -- src/scenes src/audio data engine tests` ohne Treffer außer Kommentaren, die auf CU1.5 verweisen.
- [ ] AC-05: `task test -- audio` grün, Slot-Liste in `data/sounds.json` = Sound-Slots im Register.
- [ ] `task check` grün.

## Prüfen

```bash
task check:diff
```

Review-Agent vor dem Commit (Loader- und Datenpfade).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
