# tools/webkit/templates/

## Responsibility
Templates (`@webkit/templates`): der Rahmen einer Seite ohne Inhalt. Die Seiten selbst liegen in den Programmen (Feature-Ordner unter `src/`) und setzen Templates mit Inhalt zusammen.

## Design
- `index.ts`: Re-Export von Radix `Theme` und `ThemePanel` sowie der eigenen Templates.
- `AppFrame.tsx`: Rahmen jedes Wails-Programms.
  - Props: `title`, `appearance` (`light`/`dark`), optional `tabs` (`HeaderTabs<P>` plus `value`/`onChange`; die Seiten stehen dann als `Tabs.Content` in `children`), `right` (rechte Kopfleiste), `aside` (neben der Seite), `overlay` (über allem), `fixed` (Fenster fest, nur die Seite scrollt; sonst klebt die Kopfzeile oben).
  - Aufbau: `Theme` → mit Reitern `Tabs.Root` (Wechsel nur auf bekannte Seiten) bzw. `div.wk-shell` → `organisms/AppHeader` → `div.wk-body` mit `main.wk-page` und `aside` → `overlay`.
- `ListDetail.tsx` (WB2): Seite mit Kopf über beiden Spalten (Titel, Filter, Hinweise; scrollt nicht mit) und Liste | Detail in voller Höhe, je mit eigenem Scrollbereich; Aussehen in `tools/webkit/theme.css` (`wk-listdetail`).

## Flow
`App.tsx` der Workbench → `<AppFrame title appearance tabs right overlay>` → `AppHeader` und Reiter → aktive Seite → ggf. `<ListDetail header list listWidth>{detail}</ListDetail>` für Listen-Seiten (Listenbreite als CSS-Variable `--wk-list-w`, Vorgabe 320 px).

## Integration
- Importiert nur `../organisms` und `@radix-ui/themes`; `ListDetail` ohne weitere Importe außer `react`.
- Konsumenten: `tools/k3c-workbench/frontend/src/App.tsx` (`AppFrame`; `src/assets/AssetsArea.tsx` liefert Seiten als `Tabs.Content` darin), Asset-Seiten `src/assets/catalog/CatalogPage.tsx` und `src/assets/review/index.tsx` (`ListDetail`).
- Klassen aus `tools/webkit/theme.css` (`wk-shell`, `wk-shell-fixed`, `wk-body`, `wk-page`, `wk-listdetail`).
- Test: `ListDetail.test.tsx`.
