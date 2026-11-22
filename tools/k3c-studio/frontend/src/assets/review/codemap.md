# tools/k3c-studio/frontend/src/assets/review/

## Responsibility

Seite **Review** im Bereich Studio (RV1, Vertrag `docs/standards/paper-mcp.md` › Oberfläche): jede Asset-Änderung gegen
eine wählbare Git-Basis mit Vorher/Nachher, Überblendung, Pixel-Diff-Overlay und markierten Teilen. Ändert nichts.

## Design

- `index.tsx`: `ReviewPage`; links `ReviewList`, rechts `Compare` oder `SizeCompare` (Größenvergleich mit dem Monarchen,
  B-733), darunter `PromptButtons`. Basis aus `BasisSelect`; misst die Höhe der rechten Ansicht, damit die Feldliste
  links nicht höher wird.
- `BasisSelect.tsx`: Basis `HEAD` (Standard), `HEAD~1`, `origin/main` oder eine Commit-SHA; geprüft wird im Server.
- `ReviewList.tsx`: geänderte Assets mit Status; ungültige rot mit Fehlertext, Feldliste gekürzt (voller Text im Tooltip).
- `Compare.tsx`: Vorher/Nachher nebeneinander (B-853), synchron abspielbar über `AnimationSync`, Überblendregler,
  Pixel-Diff (`assetsPixelDiff`) nur bei geänderten Assets, Rahmen über `Boxes`; Zoom über `useZoom` (`../zoom.tsx`).
  Animationen aus der Spec des Arbeitsstands (bei gelöschten Assets die der Basis).
- `AnimationSync.tsx`: gemeinsame Steuerung (Animation, Frame-Schieber, Abspielen im fps-Takt) über `FramePlayer` aus webkit.
- `Boxes.tsx`: `pngGroesse` liest Breite/Höhe aus dem IHDR einer PNG-Data-URL; `Boxes` zeichnet Diff- oder Teil-Rahmen in Prozent.
- `prompt.ts`: `felderText`, `zustandVon`, `reviewWerte` liefern die Platzhalter-Werte für Vorlagen mit Ort `review`;
  `PromptButtons.tsx` füllt sie mit `fillPrompt` (`../prompts/fill.ts`) und kopiert den Prompt.
- `reviewStore.ts`: Modul-Cache (B-732) für Ergebnis je Basis (`loadReview`, `cachedReview`), `specCache`, `frameCache`,
  `diffCache`; `clearReviewCache` leert alles samt `clearSizeCache`. Fehler des Ergebnisses werden nicht gecacht.
- `fehlertext.ts`: `text(e)` gibt den Server-Text unverändert weiter.
- `testing.tsx`: Test-Helfer (Fixtures aus `tools/paper/fixtures/ui/`, `fakeApi` für die `backend.assets*`-Methoden).

## Flow

1. `AssetsArea` ruft beim Start `loadReview('HEAD')` im Hintergrund; `ReviewPage` liest dann `cachedReview` sofort.
2. Basiswechsel -> `loadReview(basis)` -> `assetsReview(basis)` -> `ReviewList`.
3. Auswahl -> `Compare` -> `loadSpec` (`assetsGet`) -> je Frame `assetsFrame(id, anim, nr, '', basis)` für Vorher und
   Arbeitsstand für Nachher -> bei Bedarf `assetsPixelDiff` -> `Boxes`.
4. „Neu laden“ -> `clearReviewCache` -> erneut `loadReview`; das alte Ergebnis bleibt stehen, bis das neue kommt.

## Integration

- Eingehängt von `../AssetsArea.tsx` (Reiter `review`); `../catalog/catalogStore.ts` nutzt `assetsReview('HEAD')` für
  das Abzeichen „im Diff“.
- Abhängigkeiten: `../api.ts`, `../prompts/fill.ts`, `../size/`, `../zoom.tsx`, `@webkit/atoms|molecules`.
- Backend: Bindings `AssetsReview`, `AssetsFrame`, `AssetsPixelDiff` in `tools/k3c-studio/app_assets.go`
  (Paket `internal/assetsui`).
