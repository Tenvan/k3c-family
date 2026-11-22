import { useEffect, useState } from 'react';
import { backend, type McpState } from './api';
import { HeaderRight, useNavigation } from './Header';
import { AssetsArea } from './assets/AssetsArea';
import { BalancingPage } from './balancing/BalancingPage';
import { loadPref, savePref } from './lib/prefs';
import { PlannerPage } from './planner/PlannerPage';
import { WorldPage } from './world/WorldPage';
import { Tabs } from '@webkit/organisms';
import { AppFrame } from '@webkit/templates';

const MODES = ['dark', 'light'] as const;

/** Ohne gemerkte Wahl gilt die Einstellung des Systems. */
const systemMode = (): (typeof MODES)[number] =>
  window.matchMedia?.('(prefers-color-scheme: light)').matches ? 'light' : 'dark';

/** Rahmen von k3c-studio (WZ63.3): die sieben Studio-Seiten als Reiter in der Kopfzeile wie früher in der Workbench; die
 *  Seiten kommen unverändert von dort. */
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

  // Balancing bleibt eingehängt (laufende Läufe, Auswahl) wie früher in der Workbench; forceMount hält den Inhalt,
  // hidden und display verbergen ihn.
  const balancing = nav.page === 'balancing';

  return (
    <AppFrame title="K3C Studio" appearance={mode} tabs={nav.tabs}
      right={<HeaderRight mcp={mcp} mock={backend.mock} dark={mode === 'dark'} onDark={(d) => setMode(d ? 'dark' : 'light')} />}>
      <Tabs.Content value="balancing" forceMount hidden={!balancing} style={balancing ? undefined : { display: 'none' }}>
        <BalancingPage active={balancing} />
      </Tabs.Content>
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
