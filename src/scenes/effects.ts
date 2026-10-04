import { GROUND_Y, UNIT_PX } from '../core/constants';
import type { GameEvent } from '../model/types';

/** Art eines Effekts; je Feedback-Event genau eine (B-164, GR5.1). */
export type EffectKind = 'hit' | 'kill' | 'coin' | 'coinGive' | 'build' | 'built' | 'down';

export interface Effect {
  kind: EffectKind;
  /** Ort in Pixeln der Stufen-Ebene */
  x: number;
  y: number;
}

/** Aussehen und Grenzen aller Effekte an einer Stelle (GR5.2 hängt hier Blitzgrenze und Schalter ein). */
export const EFFECT_CONFIG: Record<EffectKind, { color: number; radius: number; durationMs: number; particles: number; flash: boolean }> = {
  hit: { color: 0xffffff, radius: 22, durationMs: 160, particles: 0, flash: true },
  kill: { color: 0xff5533, radius: 34, durationMs: 380, particles: 6, flash: false },
  coin: { color: 0xffd23c, radius: 10, durationMs: 300, particles: 3, flash: false },
  coinGive: { color: 0xffe680, radius: 12, durationMs: 300, particles: 3, flash: false },
  build: { color: 0xc8b078, radius: 26, durationMs: 300, particles: 4, flash: false },
  built: { color: 0x7be07b, radius: 44, durationMs: 500, particles: 8, flash: false },
  down: { color: 0xb02030, radius: 40, durationMs: 600, particles: 8, flash: false },
};

/** Höchstzahl gleichzeitig lebender Effekte; weitere Ereignisse werden verworfen (kein Wachstum der Objektzahl). */
export const MAX_LIVE_EFFECTS = 48;

/** Höhe über dem Boden, in der Treffer und Münzen erscheinen */
const BODY_PX = 40;

/** Merkt sich den Ort des letzten Baufortschritts je Bauart, denn `built` trägt kein `x`. */
export type BuildSpots = Map<string, number>;

/**
 * Ereignis → Effekt. Reine Zuordnung: liest nur das Ereignis (und `playerX` für `playerDown`), ändert keinen Spielzustand.
 * Unbekannte Typen und Ereignisse ohne bekannten Ort ergeben `null`.
 */
export function effectFor(e: GameEvent, playerX: (player: number) => number | undefined, spots: BuildSpots): Effect | null {
  const at = (kind: EffectKind, xUnits: number | undefined, up = BODY_PX): Effect | null =>
    xUnits === undefined ? null : { kind, x: xUnits * UNIT_PX, y: GROUND_Y - up };
  switch (e.type) {
    case 'hit': return at('hit', e.x);
    case 'kill': return at('kill', e.x);
    case 'coinPickup': return at('coin', e.x);
    case 'coinGive': return at('coinGive', e.x);
    case 'buildProgress': spots.set(e.kind, e.x); return at('build', e.x, 60);
    case 'built': return at('built', spots.get(e.kind), 60);
    case 'playerDown': return at('down', playerX(e.player));
    default: return null;
  }
}
