import { useState } from 'react';
import { StatusBadge } from '@webkit/atoms';
import { TicketProvider } from '@webkit/lib/ticketContext';
import { Tabs } from '@webkit/organisms';
import { AppFrame } from '@webkit/templates';
import { backend } from './api';
import { ChatPanel, ChatToggle } from './chat/ChatPanel';
import { GraphicsPage } from './graphics/GraphicsPage';
import { loadPref, savePref } from './lib/prefs';
import { EnemiesPage } from './enemies/EnemiesPage';
import { MonarchPage } from './monarch/MonarchPage';
import { PlannerPage } from './planner/PlannerPage';
import { SoundPage } from './sound/SoundPage';
import { WorldPage } from './world/WorldPage';

const createTicket = (input: Parameters<typeof backend.ticketCreate>[0]) => backend.ticketCreate(input);
const recentTickets = () => backend.ticketRecent(5);

export const PAGES = ['grafik', 'sound', 'welt', 'planer', 'monarch', 'gegner'] as const;
export type Page = (typeof PAGES)[number];

const LABELS: Record<Page, string> = { grafik: 'Grafik', sound: 'Sound', welt: 'Welten Editor', planer: 'Gebäude-Planer', monarch: 'Monarch', gegner: 'Gegner' };

/** Rahmen des Asset-Studios (B-459): Kopfzeile mit Navigation Grafik/Sound/Welt-Betrachter/Gebäude-Planer/Monarch/Gegner. Nur die offene Seite ist eingehängt, das
 *  Verlassen der Seite Sound stoppt deshalb alle Busse. Der Chat (B-488) steht rechts neben jeder Seite. */
export function App({ initial }: { initial?: Page }) {
  const [page, setPage] = useState<Page>(() => initial ?? loadPref('app.page', PAGES, 'grafik'));
  const choose = (next: Page) => {
    setPage(next);
    savePref('app.page', next);
  };
  const right = (
    <>
      {backend.mock && <StatusBadge tone="warn">Mock</StatusBadge>}
      <ChatToggle />
    </>
  );
  return (
    <TicketProvider app="k3c-studio" defaultDomain="DEV" defaultProject="WZG" create={createTicket} recent={recentTickets}>
    <AppFrame title="Asset-Studio" appearance="dark" tabs={{ pages: PAGES, labels: LABELS, value: page, onChange: choose }} right={right} aside={<ChatPanel page={page} />} fixed>
      <Tabs.Content value="grafik"><GraphicsPage /></Tabs.Content>
      <Tabs.Content value="sound"><SoundPage /></Tabs.Content>
      <Tabs.Content value="welt"><WorldPage /></Tabs.Content>
      <Tabs.Content value="planer"><PlannerPage /></Tabs.Content>
      <Tabs.Content value="monarch"><MonarchPage /></Tabs.Content>
      <Tabs.Content value="gegner"><EnemiesPage /></Tabs.Content>
    </AppFrame>
    </TicketProvider>
  );
}
