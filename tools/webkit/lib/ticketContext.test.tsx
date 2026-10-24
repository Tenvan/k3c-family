// @vitest-environment happy-dom
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { TicketProvider, useTicket, useTicketPage } from './ticketContext';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  root = null;
  document.body.innerHTML = '';
});

let seen = { page: '', selection: '' };

function Page({ page, selection, active }: { page: string; selection?: string; active?: boolean }) {
  useTicketPage(page, selection, active);
  seen = useTicket().page;
  return null;
}

const create = async () => ({ id: 'B-1', pfad: '', titel: '' });

async function render(page: string, selection?: string, active?: boolean) {
  const host = document.createElement('div');
  document.body.append(host);
  root ??= createRoot(host);
  await act(async () =>
    root!.render(
      <TicketProvider app="a" defaultDomain="DEV" create={create}>
        <Page page={page} selection={selection} active={active} />
      </TicketProvider>,
    ),
  );
}

describe('useTicketPage', () => {
  it('setzt Seite und Auswahl im Provider und folgt Änderungen', async () => {
    await render('grafik', 'Slot x');
    expect(seen).toEqual({ page: 'grafik', selection: 'Slot x' });
    await render('daten');
    expect(seen).toEqual({ page: 'daten', selection: '' });
  });

  it('meldet nichts, solange die Seite nicht aktiv ist', async () => {
    await render('git', 'main');
    await render('tasks', 'x', false);
    expect(seen).toEqual({ page: 'git', selection: 'main' });
  });
});
