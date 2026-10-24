// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import type { TicketCreated, TicketInput } from '../lib/ticket';
import { TicketProvider, useTicketPage } from '../lib/ticketContext';
import { pickValue } from '../testing/select';
import { TicketButton } from './TicketButton';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
afterEach(() => {
  act(() => root?.unmount());
  document.body.innerHTML = '';
});

const made: TicketCreated = { id: 'B-607', pfad: 'docs/backlog/B-607-x.md', titel: 'Pilz' };

function Page({ page, selection }: { page: string; selection?: string }) {
  useTicketPage(page, selection);
  return null;
}

async function mount(create: (i: TicketInput) => Promise<TicketCreated>, recent?: () => Promise<TicketCreated[]>) {
  const host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () =>
    root!.render(
      <Theme>
        <TicketProvider app="k3c-studio" version="0.9.3" defaultDomain="DEV" create={create} recent={recent}>
          <Page page="grafik" selection="Slot shardling" />
          <TicketButton />
        </TicketProvider>
      </Theme>,
    ),
  );
  await act(async () => [...host.querySelectorAll('button')].find((b) => b.textContent === 'Ticket')!.click());
}

const dialog = () => document.querySelector<HTMLElement>('[role="dialog"]')!;

async function type(label: string, value: string) {
  const el = dialog().querySelector<HTMLInputElement | HTMLTextAreaElement>(`[aria-label="${label}"]`)!;
  const proto = el instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype;
  await act(async () => {
    Object.getOwnPropertyDescriptor(proto, 'value')!.set!.call(el, value);
    el.dispatchEvent(new Event('input', { bubbles: true }));
  });
}

const submit = () => act(async () => [...dialog().querySelectorAll('button')].find((b) => b.textContent === 'Anlegen')!.click());

describe('TicketDialog', () => {
  it('zeigt den Kontext und sendet Eingabe samt Kontext und ISO-Zeit', async () => {
    const create = vi.fn().mockResolvedValue(made);
    await mount(create);
    expect(dialog().querySelector('[data-testid="ticket-context"]')!.textContent).toBe('Kontext: k3c-studio · grafik · Slot shardling · 0.9.3');
    await type('Titel', ' Pilz ');
    await type('Beschreibung', 'Zeile 1\nZeile 2');
    await pickValue(dialog(), 'Typ', 'Idee');
    await pickValue(dialog(), 'Priorität', 'hoch');
    await submit();
    expect(create).toHaveBeenCalledTimes(1);
    const input = create.mock.calls[0]![0] as TicketInput;
    expect(input).toMatchObject({ titel: 'Pilz', beschreibung: 'Zeile 1\nZeile 2', typ: 'Idee', prio: 'hoch', domaene: 'DEV' });
    expect(input.kontext).toMatchObject({ app: 'k3c-studio', seite: 'grafik', auswahl: 'Slot shardling', version: '0.9.3' });
    expect(new Date(input.kontext.zeit).toISOString()).toBe(input.kontext.zeit);
    expect(dialog().textContent).toContain('B-607');
  });

  it('zeigt den Fehler und behält Titel und Beschreibung', async () => {
    const create = vi.fn().mockRejectedValue(new Error('Backlog nicht beschreibbar'));
    await mount(create);
    await type('Titel', 'Pilz');
    await type('Beschreibung', 'Text');
    await submit();
    expect(dialog().textContent).toContain('Backlog nicht beschreibbar');
    expect(dialog().querySelector<HTMLInputElement>('[aria-label="Titel"]')!.value).toBe('Pilz');
    expect(dialog().querySelector<HTMLTextAreaElement>('[aria-label="Beschreibung"]')!.value).toBe('Text');
  });

  it('listet die letzten Tickets', async () => {
    const recent = vi.fn().mockResolvedValue([{ id: 'B-606', pfad: 'p', titel: 'Vorheriges' }]);
    await mount(vi.fn(), recent);
    expect(dialog().textContent).toContain('B-606 · Vorheriges');
  });
});
