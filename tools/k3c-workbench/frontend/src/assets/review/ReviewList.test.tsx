// @vitest-environment happy-dom
import { afterEach, describe, expect, it } from 'vitest';
import { mount, unmount } from '../testing';
import { ReviewList } from './ReviewList';
import type { ReviewAsset } from './types';

// B-732: Feldliste gewählt bis zur Höhe der Ansicht rechts, sonst zwei Zeilen; der volle Text steht im Tooltip.

afterEach(unmount);

const asset = (id: string): ReviewAsset =>
  ({ id, rig: 'figur', pfad: `public/grafik/figuren/${id}.paper.json`, status: 'geaendert', felder: [{ pfad: 'a.b', alt: 1, neu: 2 }], teile: [], stil: false, pixelAnteil: 0.1 }) as ReviewAsset;

const felder = (host: ParentNode, id: string) => host.querySelector<HTMLElement>(`[data-asset="${id}"] [data-felder]`)!;

describe('Review-Liste: Feldliste (B-732)', () => {
  it('kürzt nicht gewählte Einträge, der gewählte folgt der Höhe rechts', async () => {
    const host = await mount(<ReviewList assets={[asset('a'), asset('b')]} auswahl="a" onSelect={() => undefined} felderHoehe={480} />);
    expect(felder(host, 'a').dataset.zeilen).toBe(String(Math.floor((480 - 64) / 16)));
    expect(felder(host, 'b').dataset.zeilen).toBe('2');
    expect(felder(host, 'b').title).toContain('a.b');
  });
});
