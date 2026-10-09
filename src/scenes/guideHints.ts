/**
 * Geführte erste Nacht (S6.3, B-148): welcher Hinweis über welchem Objekt steht, ohne Phaser, damit es getestet werden kann.
 * Alles kommt aus dem Snapshot (Münzen, Bauplätze mit Zustand, Phase aus `cycle`, Truppen); der Client rechnet keine Regel.
 * Ein Hinweis gilt als gesehen, sobald seine Handlung im Snapshot sichtbar ist; ignoriert bleibt er stehen.
 */
import type { Player, World } from '../model/types';
import { PRICE_TAG_RANGE } from './viewRules';

export type GuideId = 'coin' | 'pay' | 'dusk' | 'recruit';

/** Reihenfolge, wenn mehrere passen (B-148 › Anforderungen) */
export const GUIDE_ORDER: readonly GuideId[] = ['coin', 'pay', 'dusk', 'recruit'];

/** Bild vor dem Hinweistext: Textur-Key und feste Pixel-Art-Skalierung (Zuordnung `docs/assets/zuordnung-welt.md` › `guide:coin`) */
export interface GuideImage {
  image: string;
  scale: number;
}

/** Bild vor dem Text (B-294): Münze mit dem Münzbild; die Nacht bekommt erst mit einem Mond-Bild eins (B-370) */
export function guideImage(id: GuideId): GuideImage | null {
  return id === 'coin' ? { image: 'grafik:coinIcon', scale: 4 } : null; // Münze 6 px im 16-px-Bild → 24 px, etwa Texthöhe
}

export interface GuideHint {
  id: GuideId;
  /** Ort in der Welt (Units), über dem der Hinweis steht */
  x: number;
  /** Münze, Bauplatz bzw. Landstreicher, an dem die Handlung erkannt wird; `null` bei „Nacht naht“ */
  target: number | null;
}

export type GuideWorld = Pick<World, 'coins' | 'sites' | 'troops' | 'cycle'>;

/** Nächstes Objekt in Reichweite des Preisschilds (`viewRules.ts`), sonst `undefined` */
function nearest<T extends { id: number; x: number }>(list: readonly T[], x: number, ok: (o: T) => boolean): T | undefined {
  let best: T | undefined;
  for (const o of list) if (ok(o) && Math.abs(o.x - x) < PRICE_TAG_RANGE && (!best || Math.abs(o.x - x) < Math.abs(best.x - x))) best = o;
  return best;
}

function candidate(id: GuideId, w: GuideWorld, x: number): GuideHint | null {
  const at = (o: { id: number; x: number } | undefined): GuideHint | null => (o ? { id, x: o.x, target: o.id } : null);
  switch (id) {
    case 'coin':
      return at(nearest(w.coins, x, () => true));
    case 'pay':
      return at(nearest(w.sites, x, (s) => s.state === 'unpaid'));
    case 'dusk':
      return w.cycle.phase === 'dusk' ? { id, x, target: null } : null;
    case 'recruit':
      return at(nearest(w.troops, x, (t) => t.kind === 'vagrant'));
  }
}

/** Nächster ungesehener Hinweis für einen Spieler oder keiner; ein gefallener Monarch bekommt keinen */
export function nextGuide(w: GuideWorld, p: Pick<Player, 'x' | 'respawnIn'>, seen: ReadonlySet<string>): GuideHint | null {
  if (p.respawnIn > 0) return null;
  for (const id of GUIDE_ORDER) {
    const h = seen.has(id) ? null : candidate(id, w, p.x);
    if (h) return h;
  }
  return null;
}

/** Handlung erledigt: Münze aufgehoben, Bauplatz bezahlt, Nacht da, Landstreicher ist Bauer */
export function guideDone(h: GuideHint, w: GuideWorld): boolean {
  switch (h.id) {
    case 'coin':
      return !w.coins.some((c) => c.id === h.target);
    case 'pay':
      return w.sites.some((s) => s.id === h.target && s.state !== 'unpaid');
    case 'dusk':
      return w.cycle.phase === 'night';
    case 'recruit':
      return w.troops.some((t) => t.id === h.target && t.kind !== 'vagrant');
  }
}

/** Ein Frame einer Zelle: Ist der zuletzt gezeigte Hinweis erledigt, meldet `markSeen` ihn; dann kommt der nächste. */
export function stepGuide(prev: GuideHint | null, w: GuideWorld, p: Pick<Player, 'x' | 'respawnIn'>, seen: ReadonlySet<string>, markSeen: (id: GuideId) => void): GuideHint | null {
  if (prev && !seen.has(prev.id) && guideDone(prev, w)) markSeen(prev.id);
  return nextGuide(w, p, seen);
}
