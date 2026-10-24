import type { McpState } from './api';
import { StudioLink } from './StudioLink';
import { StatusBadge, Tip } from '@webkit/atoms';
import { SwitchField } from '@webkit/molecules';

export const PAGES = ['dienste', 'tasks', 'planung', 'git', 'mcp', 'monitoring'] as const;
export type Page = (typeof PAGES)[number];

export const LABELS: Record<Page, string> = { dienste: 'Dienste & Logs', tasks: 'Tasks', planung: 'Planung', git: 'Git', mcp: 'MCP', monitoring: 'Monitoring' };

interface Props {
  mcp: McpState | null;
  mock: boolean;
  dark: boolean;
  onDark: (dark: boolean) => void;
}

/** Rechte Seite der Kopfzeile (B-064 › Kopfzeile): Studio-Verweis, MCP-Badge, Mock-Badge, Dark-Schalter. Titel und
 *  Reiter setzt AppFrame in App. */
export function HeaderRight({ mcp, mock, dark, onDark }: Props) {
  return (
    <>
      <StudioLink />
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
