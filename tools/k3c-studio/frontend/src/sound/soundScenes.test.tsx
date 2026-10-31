// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { Proben } from './Proben';
import { SoundScenes } from './SoundScenes';

// B-459/AC-06, WZ30.2 im DOM mit dem Mock: Sound-Szenen und Proben (die Hörprobe geht seit WZ33 in die Abspielleiste der Assets-Seite auf).
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

describe('Sound-Szenen (B-459)', () => {
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

});

describe('Sound-Szenen und Proben (WZ30.2)', () => {
  it('Szenen: Skills nach Linie mit Herkunft (Linie, Rückfall) und Lücken-Hinweis (WZ30.2)', async () => {
    localStorage.setItem('k3c-studio.sound.szene', 'skills');
    const h = await mount(<SoundScenes />);
    const row = (t: string) => [...h.querySelectorAll('.sd-row')].find((r) => r.textContent?.includes(t))?.textContent ?? '';
    expect(row('groupHeal')).toContain('sfx:skill:healer');
    expect(row('groupHeal')).toContain('Linien-Slot');
    expect(row('fireball')).toContain('Rückfall sfx:skill'); // Linien-Slot der Zauberer ist leer
    expect(row('healingAura')).toContain('still (kein Slot)'); // passiv ohne eigenen Slot: im Spiel still (SO8.2, WZ29.2)
    const playable = (r: Element) => [...r.querySelectorAll('button')].some((b) => b.textContent === 'Abspielen');
    expect([...h.querySelectorAll('.sd-row')].every((r) => playable(r) || r.textContent?.includes('still (kein Slot)'))).toBe(true);
  });

  it.each([['gegner', 'sfx:hit'], ['bauen', 'sfx:hub'], ['reiten', 'sfx:ride'], ['ui', 'sfx:uiSelect'], ['warnungen', 'sfx:warn']])('Szenen: %s zeigt Slot %s abspielbar', async (szene, slotId) => {
    localStorage.setItem('k3c-studio.sound.szene', szene);
    const h = await mount(<SoundScenes />);
    expect(h.textContent).toContain(slotId);
    expect([...h.querySelectorAll('button')].some((b) => b.textContent === 'Abspielen')).toBe(true);
  });

  it('Szenen: UI-Bestätigen ohne Zuordnung zeigt Lücke statt Fehler', async () => {
    localStorage.setItem('k3c-studio.sound.szene', 'ui');
    const h = await mount(<SoundScenes />);
    expect(h.textContent).toContain('Lücke: sfx:uiConfirm hat keine Zuordnung');
  });

  it('Proben: Misch-, Lautheits- und Abstandsprobe sind bedienbar (WZ30.2)', async () => {
    const h = await mount(<Proben />);
    const names = () => [...h.querySelectorAll('button')].map((b) => b.textContent ?? '');
    expect(h.textContent).toContain('35 %'); // Ducking-Pegel aus DUCK
    expect(names()).toEqual(expect.arrayContaining(['Musik starten', 'Effekt dazu', 'Kampf-Mischung']));
    expect(names().some((n) => n.startsWith('nah (im Bild)') && n.endsWith('100 %'))).toBe(true);
    expect(names().find((n) => n.startsWith('halbe Breite'))).toContain('25 %');
    const play = () => [...h.querySelectorAll('button')].find((b) => b.textContent?.startsWith('Nacheinander'))!;
    expect(play().disabled).toBe(true);
    await act(async () => h.querySelector<HTMLElement>('button[role="checkbox"][aria-label="sfx:hit"]')!.click());
    expect(play().disabled).toBe(false);
  });


});
