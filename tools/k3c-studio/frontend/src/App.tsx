import { Tabs, Theme } from '@radix-ui/themes';
import { useState } from 'react';
import { backend } from './api';
import { GrafikPage } from './grafik/GrafikPage';
import { loadPref, savePref } from './lib/prefs';
import { SoundPage } from './sound/SoundPage';
import { StatusBadge } from './ui/parts';

export const PAGES = ['grafik', 'sound'] as const;
export type Page = (typeof PAGES)[number];

const LABELS: Record<Page, string> = { grafik: 'Grafik', sound: 'Sound' };

/** Rahmen des Asset-Studios (B-459): Kopfzeile mit Navigation Grafik/Sound. Nur die offene Seite ist eingehängt, das
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
          <Tabs.Content value="grafik"><GrafikPage /></Tabs.Content>
          <Tabs.Content value="sound"><SoundPage /></Tabs.Content>
        </main>
      </Tabs.Root>
    </Theme>
  );
}
