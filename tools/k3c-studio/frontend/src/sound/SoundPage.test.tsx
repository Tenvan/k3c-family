// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { AssetsPage } from '../assets/AssetsPage';
import { Hoerprobe } from './Hoerprobe';
import { SoundScenes } from './SoundScenes';

// B-459/AC-02, AC-06 im DOM mit dem Mock: Hörprobe am Slot (auch Atlas-Ton mit „fern“), Szenen und Hörprobe-Liste.
// credits.ts liest public/**/CREDITS.md?raw; das Vitest des Studios liefert nichts außerhalb seiner Wurzel aus.
vi.mock('@game/tools/credits', () => ({ CREDITS: [] }));

(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

async function mount(el: React.ReactElement): Promise<HTMLElement> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme>{el}</Theme>));
  await act(async () => { for (let i = 0; i < 5; i++) await new Promise((r) => setTimeout(r, 0)); });
  return host;
}

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  localStorage.clear();
});

describe('Seite Sound (B-459)', () => {
  it('Slots: Atlas-Ton sfx:coin hat Hörprobe mit „fern“, keine Ausschnitt-Felder', async () => {
    localStorage.setItem('k3c-studio.sound.selected', 'sfx:coin');
    const h = await mount(<AssetsPage art="sound" />);
    const buttons = [...h.querySelectorAll('button')].map((b) => b.textContent);
    expect(buttons).toContain('Abspielen');
    expect(buttons).toContain('fern');
    expect(h.querySelector('.as-cut')).toBeNull();
    expect(h.textContent).not.toContain('figuren');
  });

  it('Szenen: Ereignis-SFX mit Abspielen und Stopp', async () => {
    localStorage.setItem('k3c-studio.sound.szene', 'sfx');
    const h = await mount(<SoundScenes />);
    expect(h.textContent).toContain('sfx:coin · Münze');
    expect(h.textContent).toContain('audio/atlas.ogg#coin');
    expect([...h.querySelectorAll('button')].some((b) => b.textContent === 'Stopp')).toBe(true);
  });

  it('Szenen: gemerkter Filter „Lücke“ blendet zugeordnete Slots aus, Zurücksetzen holt sie zurück (B-599)', async () => {
    localStorage.setItem('k3c-studio.sound.szene', 'sfx');
    localStorage.setItem('k3c-studio.sound.sfx.filter', JSON.stringify({ query: '', chips: { status: ['Lücke'] }, sort: 'spiel', group: '' }));
    const h = await mount(<SoundScenes />);
    const counter = () => [...h.querySelectorAll('[role="status"]')].map((e) => e.textContent).find((t) => / von /.test(t ?? ''));
    expect(counter()).toMatch(/^0 von [1-9]\d*$/);
    expect(h.querySelectorAll('.sd-row')).toHaveLength(0);
    await act(async () => [...h.querySelectorAll('button')].find((b) => b.textContent === 'Filter zurücksetzen')!.click());
    expect(h.textContent).toContain('sfx:coin · Münze');
    expect(JSON.parse(localStorage.getItem('k3c-studio.sound.sfx.filter')!).chips).toEqual({});
  });

  it('Hörprobe: ganze Dateien nach Musik-Zustand, Atlas-Töne fehlen wie in soundtest.html', async () => {
    const h = await mount(<Hoerprobe />);
    expect(h.querySelector('h3')?.textContent).toContain('Tag');
    expect(h.textContent).toContain('probe-a (music:tag, 1)');
    expect(h.textContent).not.toContain('coin');
  });

  it('Hörprobe: Auswahlwechsel mit scrollIntoView, das etwas zurückgibt, zerlegt die Seite nicht (leerer Bildschirm)', async () => {
    const original = Element.prototype.scrollIntoView;
    Element.prototype.scrollIntoView = () => ({ ignoriert: true }) as unknown as undefined; // wie die WebView des Studios
    try {
      const h = await mount(<Hoerprobe />);
      const warn = vi.spyOn(console, 'error').mockImplementation(() => {});
      const pick = (i: number) => act(async () => { h.querySelectorAll<HTMLButtonElement>('.hp-item')[i]!.click(); });
      await pick(1); // Auswahl wechselt: der Effekt läuft erneut
      await pick(2);
      await act(async () => root!.render(<Theme><div /></Theme>)); // Aufräumen aller Effekte
      const bad = warn.mock.calls.filter((c) => String(c[0]).includes('must not return anything'));
      warn.mockRestore();
      expect(bad).toEqual([]);
    } finally {
      Element.prototype.scrollIntoView = original;
    }
  });
});
