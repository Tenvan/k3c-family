import { Tabs } from '@radix-ui/themes';
import { useState } from 'react';
import type { Source } from '../api';
import { loadPref, savePref } from '../lib/prefs';
import { ConsoleView } from './ConsoleView';
import { ErrorsTab } from './ErrorsTab';
import { LogTab } from './LogTab';
import { pickTab, tabEnabled, TABS, type Tab } from './logview';

const LABELS: Record<Tab, string> = { konsole: 'Konsole', log: 'Log', fehler: 'Fehler (verdichtet)' };

/** Rechte Seite der Logs-Seite: Reiter Konsole, Log, Fehler (verdichtet) für die gewählte Quelle. */
export function SourcePanel({ source }: { source: Source }) {
  const [wanted, setWanted] = useState<Tab>(() => loadPref('logTab', TABS, 'konsole'));
  const tab = pickTab(wanted, source.kind); // der gemerkte Reiter bleibt gemerkt, auch wenn er hier nicht geht
  const choose = (value: string) => {
    const next = TABS.includes(value as Tab) ? (value as Tab) : 'konsole';
    setWanted(next);
    savePref('logTab', next);
  };
  return (
    <Tabs.Root className="src-panel" value={tab} onValueChange={choose}>
      <Tabs.List>
        {TABS.map((t) => (
          <Tabs.Trigger key={t} value={t} disabled={!tabEnabled(t, source.kind)}>
            {LABELS[t]}
          </Tabs.Trigger>
        ))}
      </Tabs.List>
      <div className="src-panel-body">
        {tab === 'konsole' && <ConsoleView name={source.name} />}
        {tab === 'log' && <LogTab name={source.name} />}
        {tab === 'fehler' && <ErrorsTab name={source.name} />}
      </div>
    </Tabs.Root>
  );
}
