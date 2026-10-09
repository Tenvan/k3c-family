import { describe, expect, it, vi } from 'vitest';

vi.mock('phaser', () => ({ default: {} }));
const { StageEffects } = await import('./effectsView');

/** Szene-Attrappe: zählt Texte, Tweens laufen nie ab (wie ein gehaltener Skill innerhalb von 700 ms). */
function fakeScene() {
  const texts: { text: string; x: number }[] = [];
  const obj = (o: { text: string; x: number }) => Object.assign(o, { setOrigin: () => o, setDepth: () => o });
  const scene = { add: { text: (x: number, _y: number, text: string) => (texts.push({ text, x }), obj({ text, x })) }, tweens: { add: () => undefined } };
  return { scene, texts };
}

describe('StageEffects.label (S9.4, B-319/B-321)', () => {
  it('gehaltener Skill ohne Ziel: je Stelle nur ein „kein Ziel“, an anderer Stelle ein eigenes', () => {
    const { scene, texts } = fakeScene();
    const fx = new StageEffects(scene as never, { add: () => undefined } as never);
    for (let i = 0; i < 5; i++) fx.spawn({ kind: 'noTarget', x: 320, y: 0, text: 'kein Ziel' });
    fx.spawn({ kind: 'noTarget', x: 1280, y: 0, text: 'kein Ziel' });
    expect(texts.map((t) => t.x)).toEqual([320, 1280]);
  });
});
