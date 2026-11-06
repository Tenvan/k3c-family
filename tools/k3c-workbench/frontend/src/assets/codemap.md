# tools/k3c-workbench/frontend/src/assets/

## Responsibility

Bereich **Assets** der Workbench (Entscheidung 005 › 5, Sprint WB1): Review (RV1), Katalog, Styleguide, Sound, Quellen.
Nur Ansehen – kein schreibendes Control; Assets ändern nur die MCP-Tools `assets_*`. Daten über die RPC-Methoden laut
`docs/standards/paper-mcp.md` › 4.

## Design

- `AssetsArea.tsx`: die fünf Seiten als `Tabs.Content` im `Tabs.Root` von AppFrame (`App.tsx`, Bereich aus `Header.tsx`).
- `api.ts`: Zugriff auf die sieben Asset-Methoden (`assetsList` … `assetsPixelDiff`) samt Typen (`api/typesAssets.ts`);
  gemeinsam mit RV1 (Sprint-README WB1 › D-02).
- `catalog/`: `CatalogPage` (Liste + Vorschau), `AssetList` (Filter, Gruppen, Abzeichen „im Diff“ aus `assetsReview('HEAD')`,
  roter Eintrag mit `feld · regel: text` für ungültige Specs), Prompt-Buttons am Asset und am gewählten Teil (WB1.3), `Preview` (Animation per `ChipSwitch`, Frame-Schieber
  `ValueSlider`, Abspielen mit `fps`; Fehler je Asset statt Abbruch), Hooks in `useCatalog.ts`.
- `prompts/`: `fill.ts` (`fillPrompt`: alle Platzhalter aus `paper-mcp.md` › 3, Pflicht `{teil}` bzw. `{eingabe}`, nie halb gefüllt; `zustandVon`), `hit.ts` (`teilAn`: kleinste Box, bei Gleichstand späteres Teil, D-05; `bildPunkt`), `TeilLayer.tsx` (Klickfläche über dem Bild, Rahmen des Teils), `PromptButtons.tsx` (Titel kopiert, ✦ öffnet in Claude). RV1 importiert `fill.ts` (D-02).
- `style/StylePage.tsx`: Materialtafel (`licht`, `basis`, `schatten` mit Hex), Einzeltöne, Papier.
- `review/`: gehört RV1.
- Tests: `readonly.test.tsx` klickt jede Seite durch und erlaubt nur Lesemethoden (AC-02); Helfer `testing.tsx`.
  Stile: `../styles/assets.css` (Präfix `as-`).
