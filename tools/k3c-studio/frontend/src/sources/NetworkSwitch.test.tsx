// @vitest-environment happy-dom
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it } from 'vitest';
import { pickValue, selectedText, selectedValue } from '../../../../webkit/testing/select';
import { backend } from '../api';
import { resetMockSources, setMockSourcesOffline } from '../api/mockSources';
import { NETWORK_CHANGED, NetworkSwitch } from './NetworkSwitch';

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;
const flush = () => act(async () => { await new Promise((r) => setTimeout(r, 0)); });

async function mount(): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><NetworkSwitch /></Theme>));
  await flush();
  return host;
}

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  resetMockSources();
  localStorage.clear();
});

describe('Netz-Schalter in der Kopfzeile', () => {
  it('auto zeigt die Probe, online/offline setzen das Studio fest und melden den Wechsel', async () => {
    setMockSourcesOffline(true); // Probe meldet offline
    const h = await mount();
    expect(selectedValue(h, 'Netz-Modus')).toBe('auto');
    expect(selectedText(h, 'Netz-Modus')).toContain('offline');
    let changed = 0;
    const count = () => changed++;
    window.addEventListener(NETWORK_CHANGED, count);
    await pickValue(h, 'Netz-Modus', 'online');
    await flush();
    window.removeEventListener(NETWORK_CHANGED, count);
    expect((await backend.sources()).offline).toBe(false);
    expect(changed).toBe(1);
    expect(localStorage.getItem('k3c-studio.app.network')).toContain('online');
  });
});
