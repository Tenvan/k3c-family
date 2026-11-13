// @vitest-environment happy-dom
import { useState } from 'react';
import { afterEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../../api';
import { buttons, click, mount, unmount } from '../testing';
import { until } from './catalogTesting';
import { Preview } from './Preview';
import { TeilSicht } from './TeilSicht';

// B-734: Teil aus- und einblenden; die Vorschau rendert dann ohne das Teil.

afterEach(() => {
  unmount();
  vi.restoreAllMocks();
});

function Probe({ teil }: { teil: string }) {
  const [ohne, setOhne] = useState<string[]>([]);
  return (
    <>
      <TeilSicht teil={teil} ohne={ohne} onOhne={setOhne} />
      <output data-ohne>{ohne.join(',')}</output>
    </>
  );
}

const ohne = (host: ParentNode) => host.querySelector('[data-ohne]')?.textContent;

describe('Teil ausblenden (B-734)', () => {
  it('blendet das gewählte Teil aus und wieder ein', async () => {
    const host = await mount(<Probe teil="krone" />);
    await click(buttons(host, 'krone ausblenden')[0]!);
    expect(ohne(host)).toBe('krone');
    await click(buttons(host, 'krone einblenden')[0]!);
    expect(ohne(host)).toBe('');
  });

  it('Vorschau holt den Frame ohne die ausgeblendeten Teile', async () => {
    const spy = vi.spyOn(backend, 'assetsFrameOhne');
    const host = await mount(<Preview id="monarch" ohne={['krone']} />);
    await until('Bild ohne krone', () => host.querySelector('img[alt="monarch idle Frame 1"]'));
    expect(spy).toHaveBeenCalledWith('monarch', 'idle', 0, ['krone']);
  });
});
