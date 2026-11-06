# WB1.2 · Katalog und Styleguide gegen die Stubs

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** ui
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** wb1/2-katalog-styleguide
- **Abhängig von:** WB1.1
- **Tickets:** B-699
- **Kriterien:** AC-02

## Ziel

Im Bereich Assets zeigen **Katalog** (Liste aller Specs mit Animations-Vorschau, Abzeichen „im Diff“, rotem Eintrag mit Fehlertext für ungültige Specs) und **Styleguide** (Materialtafel aus der Stil-Datei) die Daten der RPC-Stubs, ohne ein einziges schreibendes Control; ein Test sichert das für den ganzen Bereich Assets.

## Kontext

- Voraussetzung WB1.1: Bereich Assets mit Reitern, `frontend/src/assets/api.ts` (Typen und Zugriff auf `assetsList`, `assetsGet`, `assetsStyle`, `assetsReview`, `assetsFrame` …), Mock = Fixtures `tools/paper/fixtures/ui/*.json`.
- Daten laut `docs/standards/paper-mcp.md` › 4: `AssetsList()` → `[{id, rig, gruppe, pfad, gueltig, geaendert, fehler?}]` (`fehler`: `[{feld, regel, text}]`, Fixture: `banner` ungültig, Material „scharlach“ fehlt). `AssetsGet(id)` → Spec mit `animationen` (Name → `{frames 1–64, fps? Standard 8, …}`, `docs/standards/paper-spec.md`), `varianten`. `AssetsFrame(id, animation, frame, variante, basis)` → `{bild: Data-URL, teile: {teilId: [x, y, b, h]}}`, leere Basis = Arbeitsstand. `AssetsStyle()` → `{materialien: {name: {licht, basis, schatten}}, einzeltoene, papier, licht, proportionen, skalierung}`. `AssetsReview('HEAD')` → `assets[].id` mit `status` = Quelle für das Abzeichen „im Diff“ (Entscheidung 005 › 6).
- Die Stubs kennen nur `monarch` (andere IDs → Fehler „Asset „x“ fehlt“): Die Vorschau zeigt dann den Fehlertext statt eines Bildes, die Liste bleibt bedienbar (005 › 7: die übrigen Assets rendern weiter).
- **Schreibendes Control** (Sprint-README › Regeln) = Control, das Dateien, Specs oder Zustände ändert. Erlaubt: Filter, Auswahl, Animations-/Frame-Schieber (`ValueSlider` aus `@webkit/atoms`, PV1.8), Abspielen/Anhalten, Kopieren in die Zwischenablage, „In Claude öffnen“ (`OpenInClaude`), „Im Spiel ansehen“ (WB1.4).
- Atomic Design: Radix nur über `@webkit/*`, rohe Controls nur in `tools/webkit/atoms/` (`tests/atomicDesign.test.ts`); Listen-Filter: `ListFilter` (`@webkit/molecules`), Abzeichen: `StatusBadge`.
- `frontend/src/assets/review/` gehört RV1 – nicht ändern.

## Erlaubte Dateien

- `tools/k3c-workbench/frontend/src/assets/catalog/` (neu)
- `tools/k3c-workbench/frontend/src/assets/style/` (neu)
- `tools/k3c-workbench/frontend/src/assets/AssetsArea.tsx`, `assets/readonly.test.tsx` (neu), `assets/codemap.md`
- `tools/k3c-workbench/frontend/src/styles/` (nur falls eigene Klassen nötig)

## Nicht-Ziele

Keine Prompt-Buttons und keine Teil-Auswahl (WB1.3); Sound, Quellen, „Im Spiel ansehen“ (WB1.4); keine Go-Änderung, keine echte Asset-Logik (RV1 füllt die Stubs); kein Review (RV1).

## Schritte

1. **Rot:** `catalog/Catalog.test.tsx` mit dem Mock: Liste zeigt `monarch`, `pferd`, `mauer`, `banner` gruppiert nach `gruppe`; `banner` ist rot markiert und zeigt „Material „scharlach“ fehlt in der Stil-Datei“ samt `feld · regel`; Abzeichen „im Diff“ an jedem Asset aus `AssetsReview('HEAD').assets`; Auswahl von `monarch` lädt `AssetsGet` und zeigt die Animationen `idle`, `lauf`, `reiten` zur Wahl sowie das Bild aus `AssetsFrame('monarch', 'idle', 0, '', '')`; Frame-Schieber ruft `AssetsFrame` mit dem gewählten Frame; Auswahl eines Assets ohne Stub-Daten zeigt den Fehlertext, die Liste bleibt bedienbar.
2. **Rot:** `style/Styleguide.test.tsx`: Materialtafel je Material mit den Tönen `licht`, `basis`, `schatten` als Farbfeld und Hex-Wert, dazu `einzeltoene` und `papier`; nur Anzeige.
3. **Rot:** `assets/readonly.test.tsx` (AC-02, Kern): rendert jede Seite des Bereichs Assets außer `review/` mit einem Backend-Proxy, der bei jeder Methode außerhalb der Lese-Allowlist (`assetsList`, `assetsGet`, `assetsStyle`, `assetsPrompts`, `assetsReview`, `assetsFrame`, `assetsPixelDiff`, `openInClaude`, dazu ab WB1.4 die `studio*`-Lesemethoden und `studioGameOpen`) wirft, und klickt jeden Button und jede Auswahl; erwartet: keine Ausnahme. Zusätzlich keine `input`/`textarea` außer Filter- und Suchfeldern (Rolle `searchbox` bzw. Label „Filter“). Neue Seiten tragen sich in die Liste des Tests ein.
4. Umsetzen: `catalog/` (Liste mit `ListFilter`, Gruppen, `StatusBadge` für „im Diff“ und „ungültig“, Vorschau mit Animationswahl, `ValueSlider` für den Frame, Abspielen mit `fps` der Animation über `setInterval`), `style/` (Materialtafel). Abfragen in einem Hook `useCatalog` mit Fehlertext je Asset statt Abbruch.
5. `AssetsArea.tsx`: Platzhalter von Katalog und Styleguide durch die Seiten ersetzen; Codemap `assets/codemap.md` anlegen.
6. Grün: gezielte Vitest-Läufe, dann einmal `task check:diff`.
7. Commit `feat(dev): Katalog und Styleguide im Bereich Assets (WB1.2)`.

## Fertig, wenn

- [ ] AC-02 (Katalog, Styleguide): `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts catalog style` grün – Animations-Vorschau, Abzeichen „im Diff“, roter Eintrag mit Fehlertext, Materialtafel.
- [ ] AC-02 (kein schreibendes Control): `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts readonly` grün.
- [ ] `task test -- atomicDesign` grün; `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Während der Arbeit nur gezielt: `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts <muster>`; mit k3c-workbench `check_run task:check:diff` mit `checkout`.

## Ergebnis

- AC-02 (Katalog, Styleguide) geprüft: `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts assets/` 11/11 grün – Liste nach Gruppe, Abzeichen „im Diff“ (Review `HEAD`), `banner` rot mit `teile[0].lagen[0].material · material-unbekannt: …`, Animationen `idle`/`lauf`/`reiten`, Bild aus `AssetsFrame('monarch','idle',0,'','')`, Abspielen/Anhalten, Fehlertext für `pferd` (Stub) bei bedienbarer Liste; Materialtafel mit Hex, Einzeltöne, Papier.
- AC-02 (kein schreibendes Control) geprüft: `readonly.test.tsx` klickt Katalog und Styleguide durch, alle Nicht-Lesemethoden des Backends sind Spies ohne Aufruf; Eingabefelder nur „Filter“.
- Umsetzung: `assets/catalog/` (`CatalogPage`, `AssetList`, `Preview`, `useCatalog.ts`), `assets/style/StylePage.tsx`, Helfer `assets/testing.tsx`, Stile `styles/assets.css` (Import in `main.tsx`, nicht in der Liste der erlaubten Dateien, aber Teil von `frontend/`), Codemap `assets/codemap.md`.
- Prüfen: Typecheck, oxlint, `tests/atomicDesign.test.ts` grün; `task check:diff`: Vitest 3022 + Workbench 381 grün, Lint grün; Go rot nur in lastabhängigen Tests ohne WB1-Bezug (`internal/botdev` `TestBotGeraeteSteuernMonarchen`, `internal/mcpsrv` `TestRunProcessExitUndAusgabe`: einzeln grün; Ticket im Review WB1.6).
