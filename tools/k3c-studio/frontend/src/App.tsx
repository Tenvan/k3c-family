import { useEffect, useState } from 'react';
import { backend, type McpState } from './api';
import { HeaderRight, useNavigation } from './Header';
import { AssetsArea } from './assets/AssetsArea';
import { loadPref, savePref } from './lib/prefs';
import { PlannerPage } from './planner/PlannerPage';
import { WorldPage } from './world/WorldPage';
import { Tabs } from '@webkit/organisms';
import { AppFrame } from '@webkit/templates';

const MODES = ['dark', 'light'] as const;

/** Ohne gemerkte Wahl gilt die Einstellung des Systems. */
const systemMode = (): (typeof MODES)[number] =>
  window.matchMedia?.('(prefers-color-scheme: light)').matches ? 'light' : 'dark';

/** Rahmen von k3c-studio (WZ63.3): Bereiche Welten, Assets und Hub; die Seiten kommen unverändert aus der Workbench. */
export function App() {
  const nav = useNavigation();
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

  return (
    <AppFrame title="K3C Studio" appearance={mode} tabs={nav.tabs}
      right={<HeaderRight area={nav.area} onArea={nav.chooseArea} mcp={mcp} mock={backend.mock} dark={mode === 'dark'} onDark={(d) => setMode(d ? 'dark' : 'light')} />}>
      <Tabs.Content value="welten">
        <WorldPage />
      </Tabs.Content>
      <Tabs.Content value="planer">
        <PlannerPage />
      </Tabs.Content>
      <AssetsArea />
    </AppFrame>
  );
}
