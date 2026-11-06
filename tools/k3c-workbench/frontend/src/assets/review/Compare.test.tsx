// @vitest-environment happy-dom
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { Compare } from './Compare';
import { button, click, fakeApi, fixture, key, mount, unmount } from './testing';
import type { Frame, PixelDiff, Review, ReviewAsset } from './types';

// RV1.3 / AC-03: Vorher/Nachher, Überblendregler, Pixel-Diff-Overlay mit Boxen und Teil-Markierung.

const fake = vi.hoisted(() => ({ api: {} as ReturnType<typeof import('./testing').fakeApi> }));
vi.mock('./api', () => ({ reviewApi: new Proxy({}, { get: (_t, k: string) => (fake.api as Record<string, unknown>)[k] }) }));

beforeEach(() => {
  fake.api = fakeApi();
});
afterEach(unmount);

const asset = (id: string): ReviewAsset => fixture<Review>('AssetsReview').assets.find((a) => a.id === id)!;
const bild = fixture<Frame>('AssetsFrame').bild;

describe('Compare', () => {
  it('zeigt Vorher (Basis) und Nachher (Arbeitsstand)', async () => {
    const host = await mount(<Compare asset={asset('monarch')} basis="HEAD~1" />);
    expect(fake.api.frame).toHaveBeenCalledWith('monarch', 'idle', 0, '', 'HEAD~1');
    expect(fake.api.frame).toHaveBeenCalledWith('monarch', 'idle', 0, '', '');
    expect(host.querySelector('img[alt="Vorher"]')?.getAttribute('src')).toBe(bild);
    expect(host.querySelector('img[alt="Nachher"]')?.getAttribute('src')).toBe(bild);
  });

  it('neu nur Nachher, gelöscht nur Vorher', async () => {
    let host = await mount(<Compare asset={asset('pferd')} basis="HEAD~1" />);
    expect(host.querySelector('img[alt="Vorher"]')).toBeNull();
    expect(host.querySelector('img[alt="Nachher"]')).not.toBeNull();
    expect(fake.api.frame.mock.calls.every((c) => c[4] === '')).toBe(true);
    unmount();
    fake.api = fakeApi();
    host = await mount(<Compare asset={asset('fass')} basis="HEAD~1" />);
    expect(host.querySelector('img[alt="Nachher"]')).toBeNull();
    expect(host.querySelector('img[alt="Vorher"]')).not.toBeNull();
    expect(fake.api.get).not.toHaveBeenCalled();
    expect(fake.api.frame).toHaveBeenCalledWith('fass', '', 0, '', 'HEAD~1');
  });

  it('Überblendregler setzt die Deckkraft des Nachher-Bilds', async () => {
    const host = await mount(<Compare asset={asset('monarch')} basis="HEAD~1" />);
    const nachher = () => host.querySelector<HTMLImageElement>('img[data-ebene="nachher"]')!;
    expect(nachher().style.opacity).toBe('0.5');
    const regler = host.querySelector('[aria-label="Überblendung"] [role="slider"]')!;
    await key(regler, 'Home');
    expect(nachher().style.opacity).toBe('0');
    await key(regler, 'End');
    expect(nachher().style.opacity).toBe('1');
  });

  it('Pixel-Diff-Overlay mit Boxen in Prozent des Frames', async () => {
    const host = await mount(<Compare asset={asset('monarch')} basis="HEAD~1" />);
    await click(button(host, 'Unterschiede'));
    const diff = fixture<PixelDiff>('AssetsPixelDiff');
    expect(fake.api.pixelDiff).toHaveBeenCalledWith('monarch', 'idle', 0, 'HEAD~1');
    expect(host.querySelector('img[data-ebene="overlay"]')?.getAttribute('src')).toBe(diff.overlay);
    const boxen = host.querySelectorAll<HTMLElement>('[data-box]');
    expect(boxen.length).toBe(diff.boxen.length);
    const [x, y, b, h] = diff.boxen[0]!;
    expect(boxen[0]!.style.left).toBe(`${(x / 209) * 100}%`);
    expect(boxen[0]!.style.top).toBe(`${(y / 192) * 100}%`);
    expect(boxen[0]!.style.width).toBe(`${(b / 209) * 100}%`);
    expect(boxen[0]!.style.height).toBe(`${(h / 192) * 100}%`);
  });

  it('markiert geänderte Teile, die im Frame eine Box haben', async () => {
    fake.api.frame.mockImplementation(() => Promise.resolve({ ...fixture<Frame>('AssetsFrame'), teile: { tuch: [10, 20, 30, 40] } }));
    let host = await mount(<Compare asset={asset('banner')} basis="HEAD~1" />);
    expect(host.querySelector('[data-teil="tuch"]')).not.toBeNull();
    unmount();
    fake.api = fakeApi();
    host = await mount(<Compare asset={asset('monarch')} basis="HEAD~1" />);
    expect(host.querySelector('[data-teil]')).toBeNull();
  });
});
