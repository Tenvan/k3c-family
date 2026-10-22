import { useEffect, useState } from 'react';
import { backend, type McpState } from './api';
import { HeaderRight, LABELS, PAGES, type Page } from './Header';
import { loadPref, savePref } from './lib/prefs';
import { McpPage } from './mcp/McpPage';
import { PlanningPage } from './planning/PlanningPage';
import { ServicesPage } from './services/ServicesPage';
import { TasksPage } from './tasks/TasksPage';
import { Notices } from './ui/organisms/Notices';
import { Tabs } from '@webkit/organisms';
import { AppFrame } from '@webkit/templates';

const MODES = ['dark', 'light'] as const;

/** Ohne gemerkte Wahl gilt die Einstellung des Systems. */
const systemMode = (): (typeof MODES)[number] =>
  window.matchMedia?.('(prefers-color-scheme: light)').matches ? 'light' : 'dark';

export function App() {
  const [page, setPage] = useState<Page>(() => loadPref('app.page', PAGES, 'dienste'));
  const [mode, setMode] = useState(() => loadPref('app.mode', MODES, systemMode()));
  const [mcp, setMcp] = useState<McpState | null>(null);

  useEffect(() => {
    const off = backend.on('mcp:state', setMcp);
    backend.info().then((info) => setMcp((cur) => cur ?? info.mcp), () => undefined);
    return off;
  }, []);

  useEffect(() => {
    savePref('app.mode', mode);
  }, [mode]);

  const choose = (next: Page) => {
    setPage(next);
    savePref('app.page', next);
  };

  return (
    <AppFrame title="K3C Workbench" appearance={mode} tabs={{ pages: PAGES, labels: LABELS, value: page, onChange: choose }}
      right={<HeaderRight mcp={mcp} mock={backend.mock} dark={mode === 'dark'} onDark={(d) => setMode(d ? 'dark' : 'light')} />}
      overlay={<Notices />}>
      <Tabs.Content value="dienste">
        <ServicesPage />
      </Tabs.Content>
      <Tabs.Content value="tasks">
        <TasksPage />
      </Tabs.Content>
      <Tabs.Content value="planung">
        <PlanningPage />
      </Tabs.Content>
      <Tabs.Content value="mcp">
        <McpPage />
      </Tabs.Content>
    </AppFrame>
  );
}
