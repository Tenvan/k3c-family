// @vitest-environment happy-dom
import { Theme } from '@webkit/templates';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { fitTarget } from '@game/scenes/render/targetFit';
import { mockAssets } from '../api/mockAssets';
import type { AssetSlot, AssetTarget } from '../api';
import { fittedSource } from './MiniScene';
import { TargetField, targetSummary } from './TargetField';

// B-685/AC-03: Zielmaß im Studio bearbeiten; die Bühne rechnet wie das Spiel (`fitTarget`).
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

const slot: AssetSlot = {
  id: 'hub:5', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Hub-Stufen: hub:5', status: 'zugeordnet', ziel: { w: 8, h: 9 },
  wahl: [{ datei: 'grafik/gebaeude/burg-5-128.png', ausschnitt: { x: 0, y: 0, w: 1024, h: 1152 } }],
};
let root: Root | null = null;
let host: HTMLElement | null = null;

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  root = host = null;
});

async function mount(onTarget: (id: string, ziel: AssetTarget | null) => Promise<boolean>): Promise<void> {
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><TargetField slot={slot} onTarget={onTarget} /></Theme>));
}

function type(label: string, value: string): void {
  const input = host!.querySelector<HTMLInputElement>(`input[aria-label="${label}"]`)!;
  const set = Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value')!.set!;
  act(() => {
    set.call(input, value);
    input.dispatchEvent(new Event('input', { bubbles: true }));
  });
}
const btn = (text: string) => [...host!.querySelectorAll('button')].find((b) => b.textContent === text)!;

describe('Zielmaß im Studio (B-685)', () => {
  it('zeigt die Wirkung im Spiel: 128-px-Quelle füllt 8×9 Units = 256×288 px', async () => {
    await mount(async () => true);
    expect(host!.textContent).toContain('Quelle 1024×1152 px → 256×288 px im Spiel (×0.25, weich)');
  });

  it('speichert das geänderte Zielmaß und setzt mit „Heutiges Maß“ zurück', async () => {
    const onTarget = vi.fn(async () => true);
    await mount(onTarget);
    type('Zielmaß Breite in Units', '10');
    type('Zielmaß Höhe in Units', '12');
    await act(async () => btn('Zielmaß speichern').click());
    expect(onTarget).toHaveBeenCalledWith('hub:5', { w: 10, h: 12 });
    await act(async () => btn('Heutiges Maß').click());
    expect(onTarget).toHaveBeenLastCalledWith('hub:5', null);
  });

  it('ohne gültiges Maß ist Speichern gesperrt', async () => {
    await mount(async () => true);
    type('Zielmaß Breite in Units', '0');
    expect(btn('Zielmaß speichern').disabled).toBe(true);
  });

  it('Bühne: 32- und 128-px-Quelle belegen dieselbe Fläche wie im Spiel', () => {
    const ziel = { w: 8, h: 9 };
    expect(fittedSource({ w: 256, h: 288 }, ziel)).toEqual(fittedSource({ w: 1024, h: 1152 }, ziel));
    const fit = fitTarget({ w: 1024, h: 1152 }, ziel);
    expect(fittedSource({ w: 1024, h: 1152 }, ziel).w * 2).toBe(fit.w); // ×GAME_SCALE in sceneLayout = Spielgröße
    expect(targetSummary({ w: 256, h: 288 }, null)).toContain('heutiges Maß');
  });

  it('Mock: Zielmaß setzen und entfernen mit Schreibregel', async () => {
    const m = mockAssets(() => undefined);
    const v = await m.assetsList();
    const id = v.slots.find((s) => s.art === 'grafik')!.id;
    const saved = await m.assetsTargetSet(id, { w: 3, h: 4 }, v.stand);
    expect(saved.slot.ziel).toEqual({ w: 3, h: 4 });
    await expect(m.assetsTargetSet(id, null, v.stand)).rejects.toThrow();
    expect((await m.assetsTargetSet(id, null, saved.stand)).slot.ziel).toBeUndefined();
  });
});
