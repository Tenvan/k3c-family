import { Tabs } from '@radix-ui/themes';
import { useState } from 'react';
import type { Source } from '../api';
import { loadPref, savePref } from '../lib/prefs';
import { ConsoleView } from './ConsoleView';
import { ErrorsTab } from './ErrorsTab';
import { LogTab } from './LogTab';
import { pickTab, tabEnabled, TABS, type Tab } from './logview';

const LABELS: Record<Tab, string> = { konsole: 'Konsole', log: 'Log', fehler: 'Fehler (verdichtet)' };

/**
 * Rechte Seite von Dienste & Logs: Konsole der gewählten Quelle, Log und Fehler (verdichtet) ihrer Log-Datei
 * (`log`; leer: nur Konsole).
 */
export function SourcePanel({ source, log }: { source: Source; log: string }) {
  const [wanted, setWanted] = useState<Tab>(() => loadPref('logs.tab', TABS, 'konsole'));
  const tab = pickTab(wanted, log !== ''); // der gemerkte Reiter bleibt gemerkt, auch wenn er hier nicht geht
  const choose = (value: string) => {
    const next = TABS.includes(value as Tab) ? (value as Tab) : 'konsole';
    setWanted(next);
    savePref('logs.tab', next);
  };
  return (
    <Tabs.Root className="src-panel" value={tab} onValueChange={choose}>
      <Tabs.List>
        {TABS.map((t) => (
          <Tabs.Trigger key={t} value={t} disabled={!tabEnabled(t, log !== '')}>
            {LABELS[t]}{t !== 'konsole' && log && log !== source.name ? ` · ${log}` : ''}
          </Tabs.Trigger>
        ))}
      </Tabs.List>
      <div className="src-panel-body">
        {tab === 'konsole' && <ConsoleView name={source.name} />}
        {tab === 'log' && <LogTab name={log} />}
        {tab === 'fehler' && <ErrorsTab name={log} />}
      </div>
    </Tabs.Root>
  );
}
