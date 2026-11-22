import { useState } from 'react';
import type { McpState } from './api';
import { loadPref, savePref } from './lib/prefs';
import { StatusBadge, Tip } from '@webkit/atoms';
import { McpBadge, SwitchField } from '@webkit/molecules';

/** Seiten der Workbench (nur Development). Welten-Editor, Assets und Gebäude-Planer sind seit WZ63.3 im Studio
 *  (k3c-studio), die Balancing-Seite seit WZ63.6. */
export const PAGES = ['dienste', 'tasks', 'planung', 'git', 'mcp', 'monitoring'] as const;
export type Page = (typeof PAGES)[number];

export const LABELS: Record<Page, string> = {
  dienste: 'Dienste & Logs', tasks: 'Tasks', planung: 'Planung', git: 'Git', mcp: 'MCP', monitoring: 'Monitoring',
};

/** Gewählte Seite, gemerkt unter `app.page`; `tabs` sind die Reiter für AppFrame. */
export function useNavigation() {
  const [page, setPage] = useState<Page>(() => loadPref('app.page', PAGES, 'dienste'));
  const onChange = (next: Page) => {
    setPage(next);
    savePref('app.page', next);
  };
  return { page, tabs: { pages: PAGES, labels: LABELS, value: page, onChange } };
}

interface Props {
  mcp: McpState | null;
  mock: boolean;
  dark: boolean;
  onDark: (dark: boolean) => void;
}

/** Rechte Seite der Kopfzeile (B-064 › Kopfzeile): MCP-Badge, Mock-Badge, Dark-Schalter. Titel und Reiter setzt AppFrame in App. */
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
