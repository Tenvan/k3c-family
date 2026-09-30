import { Tabs, Theme } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { backend, type McpState } from './api';
import { Header, PAGES, type Page } from './Header';
import { loadPref, savePref } from './lib/prefs';
import { ServicesPage } from './services/ServicesPage';
import { NoticeCard } from './ui/parts';

const MODES = ['dark', 'light'] as const;

export function App() {
  const [page, setPage] = useState<Page>(() => loadPref('page', PAGES, 'dienste'));
  const [mode, setMode] = useState(() => loadPref('mode', MODES, 'dark'));
  const [mcp, setMcp] = useState<McpState | null>(null);

  useEffect(() => {
    const off = backend.on('mcp:state', setMcp);
    backend.info().then((info) => setMcp((cur) => cur ?? info.mcp), () => undefined);
    return off;
  }, []);

  useEffect(() => {
    document.documentElement.dataset.mode = mode;
    savePref('mode', mode);
  }, [mode]);

  const choose = (value: string) => {
    const next = PAGES.includes(value as Page) ? (value as Page) : 'dienste';
    setPage(next);
    savePref('page', next);
  };

  return (
    <Theme appearance={mode} accentColor="amber" grayColor="slate" radius="medium" hasBackground={false}>
      <Tabs.Root value={page} onValueChange={choose} className="shell">
        <Header mcp={mcp} mock={backend.mock} dark={mode === 'dark'} onDark={(d) => setMode(d ? 'dark' : 'light')} />
        <main className="page">
          <Tabs.Content value="dienste">
            <ServicesPage />
          </Tabs.Content>
          <Tabs.Content value="logs">
            <NoticeCard title="Logs">Quellenleiste und Konsole folgen in M4.3, die Reiter Log und Fehler in M4.4.</NoticeCard>
          </Tabs.Content>
          <Tabs.Content value="mcp">
            <NoticeCard title="MCP">Die MCP-Seite mit Monitoren und Statistik folgt mit B-065 (M5).</NoticeCard>
          </Tabs.Content>
        </main>
      </Tabs.Root>
    </Theme>
  );
}
