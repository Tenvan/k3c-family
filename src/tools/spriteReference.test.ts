// @vitest-environment happy-dom
import { afterEach, describe, expect, it, vi } from 'vitest';
import { renderReference } from './spriteReference';

// B-471/AC-01: Nach der Abmeldung (oder wenn `root` aus dem Dokument verschwindet) fragt die Schleife kein Bild mehr an.
afterEach(() => vi.unstubAllGlobals());

function frames() {
  const queue: FrameRequestCallback[] = [];
  vi.stubGlobal('requestAnimationFrame', (cb: FrameRequestCallback) => { queue.push(cb); return queue.length; });
  const run = () => { const cb = queue.shift(); cb?.(0); };
  return { queue, run };
}

describe('renderReference beendet seine Schleife (B-471)', () => {
  it('die Abmeldung beendet sie', () => {
    const { queue, run } = frames();
    const root = document.createElement('div');
    document.body.append(root);
    const stop = renderReference(root, [], true);
    expect(queue).toHaveLength(1);
    run();
    expect(queue).toHaveLength(1); // läuft weiter
    stop();
    run();
    expect(queue).toHaveLength(0);
  });

  it('sie endet von selbst, wenn root nicht mehr im Dokument hängt', () => {
    const { queue, run } = frames();
    const root = document.createElement('div');
    document.body.append(root);
    renderReference(root, [], true);
    root.remove();
    run();
    expect(queue).toHaveLength(0);
  });
});
