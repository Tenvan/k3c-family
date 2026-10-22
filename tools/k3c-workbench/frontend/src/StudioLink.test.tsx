// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot } from 'react-dom/client';
import { describe, expect, it, vi } from 'vitest';
import { PAGES } from './Header';
import { StudioLink } from './StudioLink';

const studioOpen = vi.fn();
vi.mock('./api', () => ({ backend: { studioOpen: () => studioOpen() } }));

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

async function clickOpen(): Promise<HTMLElement> {
  const host = document.createElement('div');
  document.body.append(host);
  const root = createRoot(host);
  await act(async () => root.render(<Theme><StudioLink /></Theme>));
  const button = [...host.querySelectorAll('button')].find((b) => b.textContent === 'Asset-Studio öffnen')!;
  await act(async () => button.click());
  await act(async () => { await new Promise((r) => setTimeout(r, 0)); });
  return host;
}

describe('Workbench ohne Assets-Seite (B-459/AC-03)', () => {
  it('die Kopfzeile hat keinen Reiter Assets mehr', () => {
    expect(PAGES).not.toContain('assets');
  });

  it('fehlt die EXE, steht der Hinweis aus Go neben dem Verweis', async () => {
    studioOpen.mockRejectedValueOnce('Asset-Studio nicht gebaut (`task k3c-studio:build`)'); // Wails lehnt mit dem Text ab
    const host = await clickOpen();
    expect(studioOpen).toHaveBeenCalledTimes(1);
    expect(host.textContent).toContain('Asset-Studio nicht gebaut (`task k3c-studio:build`)');
  });

  it('startet das Studio ohne Hinweis', async () => {
    studioOpen.mockResolvedValueOnce(undefined);
    const host = await clickOpen();
    expect(host.textContent).not.toContain('nicht gebaut');
  });
});
