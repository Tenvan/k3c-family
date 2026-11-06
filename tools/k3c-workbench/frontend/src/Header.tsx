import { useState } from 'react';
import type { McpState } from './api';
import { loadPref, savePref } from './lib/prefs';
import { StatusBadge, Tip } from '@webkit/atoms';
import { ChipSwitch, SwitchField } from '@webkit/molecules';

export const PAGES = ['dienste', 'tasks', 'planung', 'git', 'mcp', 'monitoring', 'balancing'] as const;
export type Page = (typeof PAGES)[number];

export const LABELS: Record<Page, string> = { dienste: 'Dienste & Logs', tasks: 'Tasks', planung: 'Planung', git: 'Git', mcp: 'MCP', monitoring: 'Monitoring', balancing: 'Balancing' };

/** Bereiche der Kopfleiste (Entscheidung 005 › 5): Development = die Seiten oben, Assets = nur Ansehen (WB1). */
export const AREAS = ['development', 'assets'] as const;
export type Area = (typeof AREAS)[number];
export const AREA_LABELS: Record<Area, string> = { development: 'Development', assets: 'Assets' };

/** Seiten des Bereichs Assets; Review ist die Startseite (RV1 füllt sie). */
export const ASSET_PAGES = ['review', 'katalog', 'styleguide', 'sound', 'quellen'] as const;
export type AssetPage = (typeof ASSET_PAGES)[number];
export const ASSET_LABELS: Record<AssetPage, string> = { review: 'Review', katalog: 'Katalog', styleguide: 'Styleguide', sound: 'Sound', quellen: 'Quellen' };

type AnyPage = Page | AssetPage;

/** Gewählter Bereich und Seite je Bereich, gemerkt unter `app.area`, `app.page`, `app.assetsPage` (WB1.1); `tabs` sind
 *  die Reiter des aktiven Bereichs für AppFrame. */
export function useNavigation() {
  const [area, setArea] = useState<Area>(() => loadPref('app.area', AREAS, 'development'));
  const [page, setPage] = useState<Page>(() => loadPref('app.page', PAGES, 'dienste'));
  const [assetPage, setAssetPage] = useState<AssetPage>(() => loadPref('app.assetsPage', ASSET_PAGES, 'review'));
  const chooseArea = (next: Area) => {
    setArea(next);
    savePref('app.area', next);
  };
  const onChange = (next: AnyPage) => {
    if ((ASSET_PAGES as readonly string[]).includes(next)) {
      setAssetPage(next as AssetPage);
      savePref('app.assetsPage', next);
      return;
    }
    setPage(next as Page);
    savePref('app.page', next);
  };
  const tabs = area === 'assets'
    ? { pages: ASSET_PAGES as readonly AnyPage[], labels: ASSET_LABELS as Record<AnyPage, string>, value: assetPage as AnyPage, onChange }
    : { pages: PAGES as readonly AnyPage[], labels: LABELS as Record<AnyPage, string>, value: page as AnyPage, onChange };
  return { area, page, chooseArea, tabs };
}

interface Props {
  area: Area;
  onArea: (area: Area) => void;
  mcp: McpState | null;
  mock: boolean;
  dark: boolean;
  onDark: (dark: boolean) => void;
}

/** Rechte Seite der Kopfzeile (B-064 › Kopfzeile): Bereichsumschalter (WB1), MCP-Badge, Mock-Badge, Dark-Schalter. Titel und
 *  Reiter setzt AppFrame in App. */
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

function McpBadge({ mcp }: { mcp: McpState | null }) {
  // Noch nicht gestartet (Go: leerer Zustand vor Start(), Mock: die ersten Millisekunden).
  if (!mcp || (!mcp.listening && !mcp.error)) return <StatusBadge tone="neutral">MCP …</StatusBadge>;
  const label = `MCP ${mcp.addr}`;
  if (mcp.listening) {
    return (
      <Tip content={`lauscht an http://${mcp.addr}/mcp`}>
        <StatusBadge tone="ok">{label}</StatusBadge>
      </Tip>
    );
  }
  return (
    <Tip content={mcp.error || 'lauscht nicht'}>
      <StatusBadge tone="error">{label}</StatusBadge>
    </Tip>
  );
}
