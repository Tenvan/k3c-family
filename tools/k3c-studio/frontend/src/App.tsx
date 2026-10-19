import { Tabs, Theme } from '@radix-ui/themes';
import { useState } from 'react';
import { backend } from './api';
import { GraphicsPage } from './graphics/GraphicsPage';
import { loadPref, savePref } from './lib/prefs';
import { EnemiesPage } from './enemies/EnemiesPage';
import { MonarchPage } from './monarch/MonarchPage';
import { PlannerPage } from './planner/PlannerPage';
import { SoundPage } from './sound/SoundPage';
import { WorldPage } from './world/WorldPage';
import { StatusBadge } from '@webkit/parts';

export const PAGES = ['grafik', 'sound', 'welt', 'planer', 'monarch', 'gegner'] as const;
export type Page = (typeof PAGES)[number];

const LABELS: Record<Page, string> = { grafik: 'Grafik', sound: 'Sound', welt: 'Welten Editor', planer: 'Gebäude-Planer', monarch: 'Monarch', gegner: 'Gegner' };

/** Rahmen des Asset-Studios (B-459): Kopfzeile mit Navigation Grafik/Sound/Welt-Betrachter/Gebäude-Planer/Monarch/Gegner. Nur die offene Seite ist eingehängt, das
 *  Verlassen der Seite Sound stoppt deshalb alle Busse. */
export function App({ initial }: { initial?: Page }) {
  const [page, setPage] = useState<Page>(() => initial ?? loadPref('app.page', PAGES, 'grafik'));
  const choose = (value: string) => {
    const next = PAGES.find((p) => p === value) ?? 'grafik';
    setPage(next);
    savePref('app.page', next);
  };
  return (
    <Theme appearance="dark">
      <Tabs.Root value={page} onValueChange={choose} className="shell">
        <header className="header">
          <span className="header-title">Asset-Studio</span>
          <Tabs.List>
            {PAGES.map((p) => (
              <Tabs.Trigger key={p} value={p}>
                {LABELS[p]}
              </Tabs.Trigger>
            ))}
          </Tabs.List>
          {backend.mock && <StatusBadge tone="warn">Mock</StatusBadge>}
        </header>
        <main className="page">
          <Tabs.Content value="grafik"><GraphicsPage /></Tabs.Content>
          <Tabs.Content value="sound"><SoundPage /></Tabs.Content>
          <Tabs.Content value="welt"><WorldPage /></Tabs.Content>
          <Tabs.Content value="planer"><PlannerPage /></Tabs.Content>
          <Tabs.Content value="monarch"><MonarchPage /></Tabs.Content>
          <Tabs.Content value="gegner"><EnemiesPage /></Tabs.Content>
        </main>
      </Tabs.Root>
    </Theme>
  );
}
