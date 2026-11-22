import { useCallback, useState } from 'react';
import { useNavigateRequests } from '@webkit/lib/navigate';
import type { McpState } from './api';
import { loadPref, savePref } from './lib/prefs';
import { StatusBadge, Tip } from '@webkit/atoms';
import { McpBadge, SwitchField } from '@webkit/molecules';

/** Seiten des Studios als eine flache Reiterleiste in der Kopfzeile wie in der Workbench (B-892): Review bleibt die
 *  Startseite (RV1). */
export const PAGES = ['styleguide', 'katalog', 'review', 'sound', 'welten', 'planer', 'balancing'] as const;
export type Page = (typeof PAGES)[number];
export const LABELS: Record<Page, string> = {
  styleguide: 'Styleguide', katalog: 'Katalog', review: 'Review', sound: 'Sound & Quellen',
  welten: 'Welten-Editor', planer: 'Gebäude-Planer', balancing: 'Balancing',
};

/** Frühere Zielnamen von navigateTo (Bereiche der kurzzeitigen Umsetzung WZ63.6) führen auf die heutigen Seiten. */
const ALIASES: Record<string, Page> = { hub: 'planer', assets: 'review' };

/** Gewählte Seite, gemerkt unter `app.page`; `tabs` sind die Reiter der Kopfzeile für AppFrame. */
export function useNavigation() {
  const [page, setPage] = useState<Page>(() => loadPref('app.page', PAGES, 'review'));
  const onChange = (next: Page) => {
    setPage(next);
    savePref('app.page', next);
  };
  // Seitenwechsel auf Wunsch einer Seite (z. B. Welten-Editor → Katalog, WZ62.4)
  useNavigateRequests(useCallback((next: string) => {
    const target = PAGES.find((p) => p === next) ?? ALIASES[next];
    if (target) onChange(target);
  }, [])); // eslint-disable-line react-hooks/exhaustive-deps
  const tabs = { pages: PAGES as readonly Page[], labels: LABELS, value: page, onChange };
  return { page, tabs };
}

interface Props {
  mcp: McpState | null;
  mock: boolean;
  dark: boolean;
  onDark: (dark: boolean) => void;
}

/** Rechte Seite der Kopfzeile: MCP-Badge, Mock-Badge, Dark-Schalter. Titel und Reiter setzt AppFrame in App. */
export function HeaderRight({ mcp, mock, dark, onDark }: Props) {
  return (
    <>
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
