import { useCallback, useState } from 'react';
import { useNavigateRequests } from '@webkit/lib/navigate';
import type { McpState } from './api';
import { loadPref, savePref } from './lib/prefs';
import { StatusBadge, Tip } from '@webkit/atoms';
import { ChipSwitch, McpBadge, SwitchField } from '@webkit/molecules';

/** Bereiche des Studios (WZ63.3): Welten-Editor, Assets (Styleguide, Katalog, Review, Sound & Quellen) und Hub
 *  (Gebäude-Planer); die Seiten kommen unverändert aus dem früheren Bereich Studio der Workbench. */
export const AREAS = ['welten', 'assets', 'hub'] as const;
export type Area = (typeof AREAS)[number];
export const AREA_LABELS: Record<Area, string> = { welten: 'Welten', assets: 'Assets', hub: 'Hub' };

/** Reiter je Bereich; Review bleibt die Startseite der Assets (RV1). */
export const AREA_PAGES = {
  welten: ['welten'],
  assets: ['styleguide', 'katalog', 'review', 'sound'],
  hub: ['planer'],
} as const satisfies Record<Area, readonly string[]>;
export const PAGES = [...AREA_PAGES.welten, ...AREA_PAGES.assets, ...AREA_PAGES.hub] as const;
export type Page = (typeof PAGES)[number];
export const LABELS: Record<Page, string> = {
  welten: 'Welten-Editor', styleguide: 'Styleguide', katalog: 'Katalog', review: 'Review', sound: 'Sound & Quellen', planer: 'Gebäude-Planer',
};

const ASSET_PAGES: readonly Page[] = AREA_PAGES.assets;

/** Bereich der Seite page. */
function areaOf(page: Page): Area {
  return AREAS.find((a) => (AREA_PAGES[a] as readonly Page[]).includes(page)) ?? 'welten';
}

/** Gewählter Bereich und gemerkte Asset-Seite (`app.area`, `app.assetsPage`); `tabs` sind die Reiter des aktiven
 *  Bereichs für AppFrame. */
export function useNavigation() {
  const [area, setArea] = useState<Area>(() => loadPref('app.area', AREAS, 'welten'));
  const [assetPage, setAssetPage] = useState<Page>(() => loadPref('app.assetsPage', ASSET_PAGES, 'review'));
  const chooseArea = (next: Area) => {
    setArea(next);
    savePref('app.area', next);
  };
  const onChange = (next: Page) => {
    if (!ASSET_PAGES.includes(next)) return; // Welten und Hub haben je einen Reiter
    setAssetPage(next);
    savePref('app.assetsPage', next);
  };
  // Seitenwechsel auf Wunsch einer Seite (z. B. Welten-Editor → Katalog, WZ62.4)
  useNavigateRequests(useCallback((next: string) => {
    const page = PAGES.find((p) => p === next);
    if (!page) return;
    const target = areaOf(page);
    setArea(target);
    savePref('app.area', target);
    onChange(page);
  }, [])); // eslint-disable-line react-hooks/exhaustive-deps
  const page: Page = area === 'assets' ? assetPage : AREA_PAGES[area][0];
  const tabs = { pages: AREA_PAGES[area] as readonly Page[], labels: LABELS, value: page, onChange };
  return { area, page, assetPage, chooseArea, tabs };
}

interface Props {
  area: Area;
  onArea: (area: Area) => void;
  mcp: McpState | null;
  mock: boolean;
  dark: boolean;
  onDark: (dark: boolean) => void;
}

/** Rechte Seite der Kopfzeile wie in der Workbench: Bereichsumschalter, MCP-Badge, Mock-Badge, Dark-Schalter. */
export function HeaderRight({ area, onArea, mcp, mock, dark, onDark }: Props) {
  return (
    <>
      <ChipSwitch label="Bereich" options={AREAS.map((a) => ({ value: a, label: AREA_LABELS[a] }))} value={area} onChange={onArea} />
      <McpBadge mcp={mcp} />
      {mock && (
        <Tip content="Keine Wails-Laufzeit: erfundene Daten">
          <StatusBadge tone="warn">Mock</StatusBadge>
        </Tip>
      )}
      <SwitchField label="Dark" checked={dark} onCheckedChange={onDark} />
    </>
  );
}
