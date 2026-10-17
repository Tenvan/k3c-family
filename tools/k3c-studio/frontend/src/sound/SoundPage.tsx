import { Tabs } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { AssetsPage } from '../assets/AssetsPage';
import { loadPref, savePref } from '../lib/prefs';
import { stopAll } from './audio';
import { Hoerprobe } from './Hoerprobe';
import { SoundScenes } from './SoundScenes';

const TABS = ['slots', 'szenen', 'hoerprobe'] as const;
type Tab = (typeof TABS)[number];

/** Seite Sound des Asset-Studios (B-459): Slots mit Hörprobe je Zuordnung, Szenen (frühere Asset-Vorschau) und Hörprobe
 *  (soundtest.html). Verlassen der Seite stoppt alle Busse. */
export function SoundPage() {
  const [tab, setTab] = useState<Tab>(() => loadPref('sound.tab', TABS, 'slots'));
  useEffect(() => stopAll, []);
  const choose = (v: string) => {
    const next = TABS.find((t) => t === v) ?? 'slots';
    setTab(next);
    savePref('sound.tab', next);
  };
  return (
    <Tabs.Root value={tab} onValueChange={choose} className="sub-tabs">
      <Tabs.List size="1">
        <Tabs.Trigger value="slots">Slots</Tabs.Trigger>
        <Tabs.Trigger value="szenen">Szenen</Tabs.Trigger>
        <Tabs.Trigger value="hoerprobe">Hörprobe</Tabs.Trigger>
      </Tabs.List>
      <Tabs.Content value="slots"><AssetsPage art="sound" /></Tabs.Content>
      <Tabs.Content value="szenen"><SoundScenes /></Tabs.Content>
      <Tabs.Content value="hoerprobe"><Hoerprobe /></Tabs.Content>
    </Tabs.Root>
  );
}
