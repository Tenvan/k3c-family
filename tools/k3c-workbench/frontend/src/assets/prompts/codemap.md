# tools/k3c-workbench/frontend/src/assets/prompts/

## Responsibility

Prompt-Vorlagen der Studio-Seiten: Platzhalter füllen, Teil per Klick in die Vorschau wählen und den gefüllten Prompt
kopieren oder als neue Claude-Session öffnen (Entscheidung 005 › 5, WB1.3).

## Design

- `fill.ts` (rein): `PLATZHALTER` (`asset`, `rig`, `teil`, `pfad`, `zustand`, `basis`, `eingabe`, `material`, `ton`,
  `slot`, `datei`, `pack`, `lizenz`, `treffer`), `fillPrompt(vorlage, werte)` wirft bei fehlendem Pflichtwert (`teil` bei
  Ort `teil`, `eingabe` bei Ort `review`), fehlendem Wert eines benutzten oder unbekanntem Platzhalter; nie ein halb
  gefüllter Prompt. `zustandVon` liefert „gültig, Version n“ oder die Verstöße `feld · regel`.
- `hit.ts` (rein): `teilAn` wählt das Teil unter dem Punkt (kleinste Box gewinnt, bei Gleichstand das später gezeichnete,
  D-05), `bildPunkt` rechnet Anzeige- in Bildpixel um.
- `TeilLayer.tsx`: Klickfläche über dem Vorschaubild, rahmt das gewählte Teil in Prozent der Bildgröße.
- `PromptButtons.tsx`: je Vorlage ein Knopf; der Titel kopiert den Prompt in die Zwischenablage, ✦ öffnet ihn über
  `backend.openInClaude` als neue Code-Session (B-701 D-12). Kein schreibendes Control.
- `useVorlagen.ts`: lädt `data/asset-prompts.json` einmal über `assetsPrompts` und liefert `jeOrt(ort)`; ein Ladefehler
  steht in `error`.

## Flow

1. Seite lädt Vorlagen (`useVorlagen` bzw. `catalogStore`) -> `PromptButtons vorlagen werte`.
2. Klick ins Bild -> `TeilLayer` -> `bildPunkt` -> `teilAn` -> `onSelect(teil)` -> Werte `{teil}`, `{pfad}` …
3. Knopf -> `fillPrompt` -> `navigator.clipboard.writeText` oder `backend.openInClaude`.

## Integration

- Konsumenten: `../catalog/CatalogPage.tsx`, `../style/StylePage.tsx`, `../sound/SoundPage.tsx`,
  `../sources/SourcesPage.tsx`, `../review/prompt.ts`, `../review/PromptButtons.tsx` (nutzen `fill.ts`).
- Abhängigkeiten: Typen `Vorlage`, `Box`, `Spec`, `AssetEintrag` aus `../api.ts`; `backend` aus `src/api`.
- Tests: `fill.test.ts`, `hit.test.ts`.
