import { Flex, Switch, Tabs, Text } from '@radix-ui/themes';
import type { McpState } from './api';
import { StatusBadge, Tip } from './ui/parts';

export const PAGES = ['dienste', 'tasks', 'planung', 'git', 'logs', 'mcp'] as const;
export type Page = (typeof PAGES)[number];

const LABELS: Record<Page, string> = { dienste: 'Dienste', tasks: 'Tasks', planung: 'Planung', git: 'Git', logs: 'Logs', mcp: 'MCP' };

interface Props {
  mcp: McpState | null;
  mock: boolean;
  dark: boolean;
  onDark: (dark: boolean) => void;
}

/** Kopfzeile (B-064 › Kopfzeile). Liegt in Tabs.Root von App, damit die Reiter die Seiten umschalten. */
export function Header({ mcp, mock, dark, onDark }: Props) {
  return (
    <header className="header">
      <span className="header-title">K3C Dev</span>
      <Tabs.List className="header-tabs">
        {PAGES.map((p) => (
          <Tabs.Trigger key={p} value={p}>
            {LABELS[p]}
          </Tabs.Trigger>
        ))}
      </Tabs.List>
      <div className="header-right">
        <McpBadge mcp={mcp} />
        {mock && (
          <Tip content="Keine Wails-Laufzeit: erfundene Daten">
            <StatusBadge tone="warn">Mock</StatusBadge>
          </Tip>
        )}
        <Text as="label" size="2">
          <Flex gap="2" align="center">
            <Switch checked={dark} onCheckedChange={onDark} />
            Dark
          </Flex>
        </Text>
      </div>
    </header>
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
