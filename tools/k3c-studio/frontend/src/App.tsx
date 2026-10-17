import { Tabs, Text, Theme } from '@radix-ui/themes';
import { useState } from 'react';

export const PAGES = ['grafik', 'sound'] as const;
export type Page = (typeof PAGES)[number];

const LABELS: Record<Page, string> = { grafik: 'Grafik', sound: 'Sound' };
const PREF = 'studio.page';

/** Gemerkte Seite; fehlt sie oder ist sie unbekannt, gilt Grafik. */
function loadPage(): Page {
  try {
    const v = localStorage.getItem(PREF);
    return PAGES.includes(v as Page) ? (v as Page) : 'grafik';
  } catch {
    return 'grafik';
  }
}

function savePage(page: Page) {
  try {
    localStorage.setItem(PREF, page);
  } catch {
    // ohne Speicher (Test, gesperrt) bleibt die Wahl nur bis zum Neuladen
  }
}

/** Rahmen des Asset-Studios (B-459): Kopfzeile mit Navigation Grafik/Sound; die Seiten füllen WZ6.3 und WZ6.4. */
export function App({ initial }: { initial?: Page }) {
  const [page, setPage] = useState<Page>(() => initial ?? loadPage());
  const choose = (value: string) => {
    const next = PAGES.includes(value as Page) ? (value as Page) : 'grafik';
    setPage(next);
    savePage(next);
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
        </header>
        <main className="page">
          {PAGES.map((p) => (
            <Tabs.Content key={p} value={p}>
              <Text color="gray" size="2">
                Seite {LABELS[p]} folgt.
              </Text>
            </Tabs.Content>
          ))}
        </main>
      </Tabs.Root>
    </Theme>
  );
}
