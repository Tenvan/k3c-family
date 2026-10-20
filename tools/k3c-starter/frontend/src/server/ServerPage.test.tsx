// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import type { ServerState } from '../api';
import { ServerPage } from './ServerPage';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const stopped: ServerState = { running: false, starting: false, port: '8080', urls: [], message: 'Server ist gestoppt.', log: [] };
const running: ServerState = {
  running: true, starting: false, port: '8080', message: 'Server läuft.', log: ['a', 'b'],
  urls: ['http://localhost:8080/', 'http://192.168.178.20:8080/'],
};
const starting: ServerState = { ...stopped, starting: true, message: 'Server startet …' };

const fake = vi.hoisted(() => ({ current: undefined as unknown, start: vi.fn(), stop: vi.fn() }));
vi.mock('../api', () => ({
  backend: { mock: true, state: () => Promise.resolve(fake.current), start: () => fake.start(), stop: () => fake.stop() },
}));

async function render(state: ServerState): Promise<HTMLElement> {
  fake.current = state;
  const host = document.createElement('div');
  document.body.append(host);
  await act(async () => createRoot(host).render(<Theme><ServerPage /></Theme>));
  await act(async () => undefined);
  return host;
}

const button = (host: HTMLElement, text: string) => [...host.querySelectorAll('button')].find((b) => b.textContent === text)!;

beforeEach(() => {
  vi.useFakeTimers();
  fake.start.mockResolvedValue(undefined);
  fake.stop.mockResolvedValue(undefined);
});
afterEach(() => {
  vi.useRealTimers();
  document.body.replaceChildren();
  vi.clearAllMocks();
});

describe('Starter-Seite', () => {
  it('gestoppt: Ton error, keine Adressen, Starten frei, Beenden gesperrt', async () => {
    const host = await render(stopped);
    expect(host.querySelector('.wk-status')?.getAttribute('data-tone')).toBe('error');
    expect(host.textContent).toContain('Server ist gestoppt.');
    expect(host.textContent).not.toContain('Adressen im Heimnetz');
    expect(button(host, 'Starten').disabled).toBe(false);
    expect(button(host, 'Beenden').disabled).toBe(true);
  });

  it('startet: Ton warn, Starten gesperrt, Beenden frei', async () => {
    const host = await render(starting);
    expect(host.querySelector('.wk-status')?.getAttribute('data-tone')).toBe('warn');
    expect(button(host, 'Starten').disabled).toBe(true);
    expect(button(host, 'Beenden').disabled).toBe(false);
  });

  it('läuft: Ton ok, Adressen, Log, Starten gesperrt, Beenden frei', async () => {
    const host = await render(running);
    expect(host.querySelector('.wk-status')?.getAttribute('data-tone')).toBe('ok');
    expect(host.textContent).toContain('http://192.168.178.20:8080/');
    expect(host.querySelector('pre')?.textContent).toBe('a\nb');
    expect(button(host, 'Starten').disabled).toBe(true);
    expect(button(host, 'Beenden').disabled).toBe(false);
  });

  it('Starten und Beenden rufen das Backend', async () => {
    const host = await render(stopped);
    await act(async () => button(host, 'Starten').click());
    expect(fake.start).toHaveBeenCalledTimes(1);
    const second = await render(running);
    await act(async () => button(second, 'Beenden').click());
    expect(fake.stop).toHaveBeenCalledTimes(1);
  });

  it('fragt jede Sekunde neu ab', async () => {
    const host = await render(stopped);
    fake.current = running;
    await act(async () => vi.advanceTimersByTimeAsync(1000));
    expect(host.textContent).toContain('Server läuft.');
  });
});

describe('Adressen kopieren', () => {
  const clipboard = (writeText: () => Promise<void>) =>
    Object.defineProperty(navigator, 'clipboard', { value: { writeText }, configurable: true });

  it('zeigt 1,5 s „Kopiert“ und legt die Adresse ab', async () => {
    const writeText = vi.fn().mockResolvedValue(undefined);
    clipboard(writeText);
    const host = await render(running);
    await act(async () => button(host, 'Kopieren').click());
    expect(writeText).toHaveBeenCalledWith('http://localhost:8080/');
    expect(host.textContent).toContain('Kopiert');
    await act(async () => vi.advanceTimersByTimeAsync(1500));
    expect(host.textContent).not.toContain('Kopiert');
    expect(host.textContent).toContain('Kopieren');
  });

  it('zeigt „Nicht möglich“, wenn die Zwischenablage ablehnt', async () => {
    clipboard(() => Promise.reject(new Error('gesperrt')));
    const host = await render(running);
    await act(async () => button(host, 'Kopieren').click());
    expect(host.textContent).toContain('Nicht möglich');
  });
});
