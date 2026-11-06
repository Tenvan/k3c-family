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
  roter Eintrag mit `feld · regel: text` für ungültige Specs), `Preview` (Animation per `ChipSwitch`, Frame-Schieber
  `ValueSlider`, Abspielen mit `fps`; Fehler je Asset statt Abbruch), Hooks in `useCatalog.ts`.
- `style/StylePage.tsx`: Materialtafel (`licht`, `basis`, `schatten` mit Hex), Einzeltöne, Papier.
- `review/`: gehört RV1.
- Tests: `readonly.test.tsx` klickt jede Seite durch und erlaubt nur Lesemethoden (AC-02); Helfer `testing.tsx`.
  Stile: `../styles/assets.css` (Präfix `as-`).
