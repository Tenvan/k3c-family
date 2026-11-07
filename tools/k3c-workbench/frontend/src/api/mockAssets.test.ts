import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import { mockAssets } from './mockAssets';

// WB1.1: Der Mock der Asset-Methoden liefert genau die Beispielantworten aus tools/paper/fixtures/ui (PV1.4/PV1.8).
const fixture = (name: string) =>
  JSON.parse(readFileSync(resolve(__dirname, '../../../../paper/fixtures/ui', `${name}.json`), 'utf-8')) as unknown;

describe('Asset-Methoden im Mock (WB1.1)', () => {
  const m = mockAssets();
  it('liefert je Methode die Fixture', async () => {
    expect(await m.assetsList()).toEqual(fixture('AssetsList'));
    expect(await m.assetsGet('monarch')).toEqual(fixture('AssetsGet'));
    expect(await m.assetsStyle()).toEqual(fixture('AssetsStyle'));
    expect(await m.assetsPrompts()).toEqual(fixture('AssetsPrompts'));
    expect(await m.assetsReview('HEAD~1')).toEqual(fixture('AssetsReview'));
    expect(await m.assetsFrame('monarch', 'idle', 0, '', '')).toEqual(fixture('AssetsFrame'));
    expect(await m.assetsPixelDiff('monarch', 'idle', 0, 'HEAD')).toEqual(fixture('AssetsPixelDiff'));
  });
  it('kennt wie der Stub nur monarch', async () => {
    await expect(m.assetsGet('pferd')).rejects.toThrow('Asset „pferd“ fehlt');
    await expect(m.assetsFrame('mauer', 'idle', 0, '', '')).rejects.toThrow('Asset „mauer“ fehlt');
  });
});
