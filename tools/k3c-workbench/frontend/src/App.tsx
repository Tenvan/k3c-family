import { useEffect, useState } from 'react';
import { backend, type McpState } from './api';
import { HeaderRight, useNavigation, type Page } from './Header';
import { AssetsArea } from './assets/AssetsArea';
import { loadPref, savePref } from './lib/prefs';
import { BalancingPage } from './balancing/BalancingPage';
import { GitPage } from './git/GitPage';
import { McpPage } from './mcp/McpPage';
import { MonitoringPage } from './monitoring/MonitoringPage';
import { PlanningPage } from './planning/PlanningPage';
import { ServicesPage } from './services/ServicesPage';
import { TasksPage } from './tasks/TasksPage';
import { WorldPage } from './world/WorldPage';
import { Notices } from './ui/organisms/Notices';
import { TicketProvider } from '@webkit/lib/ticketContext';
import { Tabs } from '@webkit/organisms';
import { AppFrame } from '@webkit/templates';

const createTicket = (input: Parameters<typeof backend.ticketCreate>[0]) => backend.ticketCreate(input);
const recentTickets = () => backend.ticketRecent(5);

const MODES = ['dark', 'light'] as const;

/** Ohne gemerkte Wahl gilt die Einstellung des Systems. */
const systemMode = (): (typeof MODES)[number] =>
  window.matchMedia?.('(prefers-color-scheme: light)').matches ? 'light' : 'dark';

export function App() {
  const nav = useNavigation();
  const [mode, setMode] = useState(() => loadPref('app.mode', MODES, systemMode()));
  const [mcp, setMcp] = useState<McpState | null>(null);
  const [version, setVersion] = useState('');

  useEffect(() => {
    const off = backend.on('mcp:state', setMcp);
    backend.info().then((info) => {
      setMcp((cur) => cur ?? info.mcp);
      setVersion(info.version);
    }, () => undefined);
    return off;
  }, []);

  useEffect(() => {
    savePref('app.mode', mode);
  }, [mode]);

  // Dienste, Tasks, Git und Monitoring bleiben eingehängt (Zustand, laufende Ansichten), auch im Bereich Assets; Planung
  // und MCP bauen sich bei jedem Wechsel neu auf und laden dabei frisch. forceMount hält den Inhalt, hidden und display
  // verbergen ihn.
  const shown = (v: Page) => nav.area === 'development' && nav.page === v;
  const keep = (v: Page) => ({ forceMount: true as const, hidden: !shown(v), style: shown(v) ? undefined : { display: 'none' } });

  return (
    <TicketProvider app="k3c-workbench" version={version} defaultDomain="DEV" defaultProject="WZG" create={createTicket} recent={recentTickets}>
    <AppFrame title="K3C Workbench" appearance={mode} tabs={nav.tabs}
      right={<HeaderRight area={nav.area} onArea={nav.chooseArea} mcp={mcp} mock={backend.mock} dark={mode === 'dark'} onDark={(d) => setMode(d ? 'dark' : 'light')} />}
      overlay={<Notices />}>
      <Tabs.Content value="dienste" {...keep('dienste')}>
        <ServicesPage active={shown('dienste')} />
      </Tabs.Content>
      <Tabs.Content value="tasks" {...keep('tasks')}>
        <TasksPage active={shown('tasks')} />
      </Tabs.Content>
      <Tabs.Content value="planung">
        <PlanningPage />
      </Tabs.Content>
      <Tabs.Content value="git" {...keep('git')}>
        <GitPage active={shown('git')} />
      </Tabs.Content>
      <Tabs.Content value="mcp">
        <McpPage />
      </Tabs.Content>
      <Tabs.Content value="monitoring" {...keep('monitoring')}>
        <MonitoringPage active={shown('monitoring')} />
      </Tabs.Content>
      <Tabs.Content value="balancing" {...keep('balancing')}>
        <BalancingPage active={shown('balancing')} />
      </Tabs.Content>
      <Tabs.Content value="welten">
        <WorldPage />
      </Tabs.Content>
      <AssetsArea />
    </AppFrame>
    </TicketProvider>
  );
}
