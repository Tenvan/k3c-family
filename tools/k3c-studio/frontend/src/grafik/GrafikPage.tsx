import { Tabs } from '@radix-ui/themes';
import { useState } from 'react';
import { AssetsPage } from '../assets/AssetsPage';
import { loadPref, savePref } from '../lib/prefs';
import { PacksView } from './PacksView';
import { ScenesView } from './ScenesView';

const TABS = ['slots', 'szenen', 'packs'] as const;
type Tab = (typeof TABS)[number];

/** Seite Grafik des Asset-Studios (B-459): Slots (wie die frühere Workbench-Seite), Szenen (frühere Asset-Vorschau),
 *  Packs (grafiken.html). */
export function GrafikPage() {
  const [tab, setTab] = useState<Tab>(() => loadPref('grafik.tab', TABS, 'slots'));
  const choose = (v: string) => {
    const next = TABS.find((t) => t === v) ?? 'slots';
    setTab(next);
    savePref('grafik.tab', next);
  };
  return (
    <Tabs.Root value={tab} onValueChange={choose} className="sub-tabs">
      <Tabs.List size="1">
        <Tabs.Trigger value="slots">Slots</Tabs.Trigger>
        <Tabs.Trigger value="szenen">Szenen</Tabs.Trigger>
        <Tabs.Trigger value="packs">Packs</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="slots"><AssetsPage art="grafik" /></Tabs.Content>
      <Tabs.Content value="szenen"><ScenesView /></Tabs.Content>
      <Tabs.Content value="packs"><PacksView /></Tabs.Content>
    </Tabs.Root>
  );
}
