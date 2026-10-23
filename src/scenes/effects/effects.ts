import { GROUND_Y, UNIT_PX } from '../../core/constants';
import { t } from '../../core/texts';
import type { GameEvent } from '../../model/types';

/** Art eines Effekts; je Feedback-Event genau eine (B-164, GR5.1). */
export type EffectKind = 'hit' | 'kill' | 'coin' | 'coinGive' | 'build' | 'built' | 'down' | 'swing' | 'noTarget' | 'skill';

export interface Effect {
  kind: EffectKind;
  /** Ort in Pixeln der Stufen-Ebene */
  x: number;
  y: number;
  /** Nur `noTarget`: Text, der statt der Ringe am Monarchen erscheint */
  text?: string;
  /** Nur `swing`: ID des Spielers, dessen Monarch ausholt (S9.7, B-396) */
  by?: number;
  /** Nur `coinGive`: Index des Spielers, dessen Monarch die Münze wirft (GR9.3) */
  giver?: number;
  /** Nur `skill`: Farbe der Linie statt der Farbe aus `EFFECT_CONFIG` (SK2.3, B-503) */
  color?: number;
  /** Nur `skill`: Aussehen des Skills (`skillLooks.ts`, B-578) statt `EFFECT_CONFIG.skill` */
  look?: SkillLook;
}

/** Bewegung der Partikel eines Skill-Effekts: `burst` im Kreis, `rise` steigt auf, `fall` fällt herab, `wave` am Boden zur Seite. */
export type SkillMotion = 'burst' | 'rise' | 'fall' | 'wave';

/** Aussehen eines Skill-Effekts (B-578): Ring plus Partikel in Farbe, Größe, Dauer und Bewegung des Skills. */
export interface SkillLook {
  color: number;
  radius: number;
  durationMs: number;
  particles: number;
  motion: SkillMotion;
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
  swing: { color: 0xdddddd, radius: 18, durationMs: 140, particles: 0, flash: false },
  noTarget: { color: 0xffffff, radius: 0, durationMs: 700, particles: 0, flash: false },
  skill: { color: 0xffffff, radius: 30, durationMs: 450, particles: 4, flash: false },
};

/** Farbe des Skill-Rings je Linie aus `data/monarch.json › lines` (SK2.3, B-503); Rückfall für Skills ohne eigenen Look (B-578). */
export const LINE_COLORS: Record<string, number> = { tank: 0x6fa8ff, mage: 0xc070ff, healer: 0x70f0a0, thief: 0x9a9a9a };

/** Barrierefreiheit (B-164): höchstens 3 Blitze pro Sekunde; ein Blitz im Sperrfenster wird verworfen, nicht aufgeschoben. */
export const MAX_FLASHES_PER_SECOND = 3;
export const FLASH_MIN_GAP_MS = Math.ceil(1000 / MAX_FLASHES_PER_SECOND);

/** Schütteln der Kamera des getroffenen Spielers */
export const SHAKE = { durationMs: 150, intensity: 0.008 };
/** Vibration des Controllers des getroffenen Spielers */
export const RUMBLE = { duration: 150, strongMagnitude: 0.6, weakMagnitude: 0.3 };

/** Höchstzahl gleichzeitig lebender Effekte; weitere Ereignisse werden verworfen (kein Wachstum der Objektzahl). */
export const MAX_LIVE_EFFECTS = 48;

/** Höhe über dem Boden, in der Treffer und Münzen erscheinen */
const BODY_PX = 40;

/** Merkt sich den Ort des letzten Baufortschritts je Bauart: Rückfall für Server, deren `built` noch kein `x` trägt (B-217). */
export type BuildSpots = Map<string, number>;

/** Ort aus dem Ereignis, sonst der Rückfall (hält `effectFor` unter dem Komplexitäts-Limit). */
const orElse = (x: number | undefined, fallback: () => number | undefined): number | undefined => x ?? fallback();

/**
 * Ereignis → Effekt. Reine Zuordnung: liest nur das Ereignis (und als Rückfall `playerX` für `playerDown` ohne `x`), ändert keinen Spielzustand.
 * Unbekannte Typen und Ereignisse ohne bekannten Ort ergeben `null`.
 */
export function effectFor(e: GameEvent, playerX: (player: number) => number | undefined, spots: BuildSpots): Effect | null {
  const at = (kind: EffectKind, xUnits: number | undefined, up = BODY_PX): Effect | null =>
    xUnits === undefined ? null : { kind, x: xUnits * UNIT_PX, y: GROUND_Y - up };
  switch (e.type) {
    case 'hit': return at('hit', e.x);
    case 'kill': return at('kill', e.x);
    case 'coinPickup': return at('coin', e.x);
    case 'coinGive': { const fx = at('coinGive', e.x); return fx && { ...fx, giver: e.player }; }
    case 'buildProgress': spots.set(e.kind, e.x); return at('build', e.x, 60);
    case 'built': return at('built', orElse(e.x, () => spots.get(e.kind)), 60);
    case 'playerDown': return at('down', orElse(e.x, () => playerX(e.player)));
    // `hit` fehlt nur beim Schlag eines Gegners (Protokoll): nur der Monarch holt aus, der Treffer kommt als eigenes `hit`
    case 'strike': { const fx = e.hit === undefined ? null : at('swing', e.x); return fx && { ...fx, by: e.from }; }
    case 'castFailed': { const fx = at('noTarget', e.x, 90); return fx && { ...fx, text: t('cast.noTarget') }; }
    default: return null;
  }
}
