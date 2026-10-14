import { describe, expect, it } from 'vitest';
import { refollow, type FollowCam } from './cameraFollow';

function fakeCam() {
  const calls: string[] = [];
  const cam: FollowCam = {
    startFollow: (v) => void calls.push(`follow:${(v as { name: string }).name}`),
    stopFollow: () => void calls.push('stop'),
  };
  return { cam, calls };
}

const view = (name: string) => ({ name });

describe('refollow (B-392)', () => {
  it('folgt der Figur, sobald ihre View nach dem Layout erscheint', () => {
    const { cam, calls } = fakeCam();
    let current = refollow(cam, undefined, undefined); // View noch nicht da
    current = refollow(cam, current, view('p2'));
    expect(calls).toEqual(['follow:p2']);
    expect(current).toEqual(view('p2'));
  });

  it('startet nicht jeden Frame neu, solange das Ziel gleich bleibt', () => {
    const { cam, calls } = fakeCam();
    const v = view('p2');
    let current = refollow(cam, undefined, v);
    current = refollow(cam, current, v);
    current = refollow(cam, current, v);
    expect(calls).toEqual(['follow:p2']);
  });

  it('folgt der neuen View, wenn die Figur neu gezeichnet wird', () => {
    const { cam, calls } = fakeCam();
    let current = refollow(cam, undefined, view('alt'));
    current = refollow(cam, current, view('neu'));
    expect(calls).toEqual(['follow:alt', 'follow:neu']);
    expect(current).toEqual(view('neu'));
  });

  it('hört auf zu folgen, wenn die Figur verschwindet', () => {
    const { cam, calls } = fakeCam();
    let current = refollow(cam, undefined, view('p2'));
    current = refollow(cam, current, undefined);
    expect(calls).toEqual(['follow:p2', 'stop']);
    expect(current).toBeUndefined();
  });

  it('zwei Kameras folgen ihren eigenen Figuren unabhängig', () => {
    const a = fakeCam();
    const b = fakeCam();
    refollow(a.cam, undefined, view('p1'));
    refollow(b.cam, undefined, view('p2'));
    expect([a.calls, b.calls]).toEqual([['follow:p1'], ['follow:p2']]);
  });
});
