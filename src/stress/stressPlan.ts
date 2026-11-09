/**
 * Stress-Szene (B-765, RX1.3): Szenario-Datei prüfen, Stufen der Rampe berechnen und die Figuren je Stufe festlegen.
 * Phaser-frei und deterministisch: Jede Figur bekommt ihre Werte aus einem Seed-Zähler (kein `Math.random()`), damit gleicher Seed
 * auf allen Geräten dieselbe Szene ergibt und eine höhere Stufe die Figuren der niedrigeren unverändert enthält.
 */

export type Mix = 'typical' | 'worstCase';
export type Pan = 'none' | 'sweep';

export interface Entry {
  type: string;
  count: number;
}

/** Hintergrund und Vordergrund der Szene: Biom (Schlüssel wie in `data/biomes/`), Parallax-Hintergrund an/aus, Zahl der Vordergrund-Objekte */
export interface Scenery {
  biome: string;
  background: boolean;
  foreground: number;
}

export interface Scenario {
  name: string;
  scenery: Scenery;
  players: 1 | 2;
  camera: { x: number; zoom: number; pan: Pan };
  enemies: Entry[];
  troops: Entry[];
  sprites: { mix: Mix; distinctSheets: number };
  ramp: { everyMs: number; steps: number; factor: number };
}

/** Bekannte Typen (Schlüssel aus `enemies.json`/`troops.json`); der Aufrufer liest sie aus den Daten */
export interface KnownTypes {
  enemies: readonly string[];
  troops: readonly string[];
  /** Biome mit Hintergrund-Bildern */
  biomes: readonly string[];
}

/** Kürzeste Stufe: länger als die Aufwärmzeit (2 s), sonst misst sie fast nichts */
export const MIN_STAGE_MS = 4000;

const isObj = (v: unknown): v is Record<string, unknown> => typeof v === 'object' && v !== null && !Array.isArray(v);
const num = (v: unknown): v is number => typeof v === 'number' && Number.isFinite(v);

function entries(raw: unknown, known: readonly string[], where: string, errors: string[]): Entry[] {
  if (!Array.isArray(raw)) {
    errors.push(`${where}: Liste fehlt`);
    return [];
  }
  const out: Entry[] = [];
  for (const e of raw as unknown[]) {
    if (!isObj(e) || typeof e.type !== 'string' || !num(e.count)) {
      errors.push(`${where}: Eintrag ohne type und count`);
      continue;
    }
    if (!known.includes(e.type)) errors.push(`${where}: unbekannter Typ ${e.type}`);
    if (!Number.isInteger(e.count) || e.count < 0) errors.push(`${where}: count von ${e.type} muss eine ganze Zahl ≥ 0 sein`);
    out.push({ type: e.type, count: e.count });
  }
  return out;
}

const int = (v: unknown, min: number): boolean => Number.isInteger(v) && (v as number) >= min;

function sceneryErrors(s: Record<string, unknown>, biomes: readonly string[]): string[] {
  const out: string[] = [];
  if (typeof s.biome !== 'string' || !biomes.includes(s.biome)) out.push(`scenery.biome muss einer von ${biomes.join(', ')} sein`);
  if (typeof s.background !== 'boolean') out.push('scenery.background muss true oder false sein');
  if (!int(s.foreground, 0)) out.push('scenery.foreground muss eine ganze Zahl ≥ 0 sein');
  return out;
}

const obj = (v: unknown): Record<string, unknown> => (isObj(v) ? v : {});

function cameraErrors(c: Record<string, unknown>): string[] {
  const ok = num(c.x) && num(c.zoom) && c.zoom > 0 && (c.pan === 'none' || c.pan === 'sweep');
  return ok ? [] : ['camera braucht x, zoom > 0 und pan none|sweep'];
}

function spriteErrors(s: Record<string, unknown>): string[] {
  const out: string[] = [];
  if (s.mix !== 'typical' && s.mix !== 'worstCase') out.push('sprites.mix muss typical oder worstCase sein');
  if (!int(s.distinctSheets, 1)) out.push('sprites.distinctSheets muss eine ganze Zahl ≥ 1 sein');
  return out;
}

function rampErrors(r: Record<string, unknown>): string[] {
  const out: string[] = [];
  if (!num(r.everyMs) || r.everyMs < MIN_STAGE_MS) out.push(`ramp.everyMs muss ≥ ${MIN_STAGE_MS} sein`);
  if (!int(r.steps, 1)) out.push('ramp.steps muss eine ganze Zahl ≥ 1 sein');
  if (!num(r.factor) || r.factor < 1) out.push('ramp.factor muss ≥ 1 sein');
  return out;
}

/** Kamera, Sprites und Rampe prüfen; gibt die Fehlermeldungen zurück */
const settings = (raw: Record<string, unknown>, known: KnownTypes): string[] => [...cameraErrors(obj(raw.camera)), ...spriteErrors(obj(raw.sprites)), ...rampErrors(obj(raw.ramp)), ...sceneryErrors(obj(raw.scenery), known.biomes)];

/** Prüft ein Szenario; bei Fehlern wirft sie mit allen Meldungen, sonst das typisierte Szenario. */
export function parseScenario(id: string, raw: unknown, known: KnownTypes): Scenario {
  if (!isObj(raw)) throw new Error(`${id}: kein Objekt`);
  const errors: string[] = [];
  if (typeof raw.name !== 'string' || raw.name === '') errors.push('name fehlt');
  if (raw.players !== 1 && raw.players !== 2) errors.push('players muss 1 oder 2 sein');
  errors.push(...settings(raw, known));
  const lists: string[] = [];
  const enemies = entries(raw.enemies, known.enemies, `${id}.enemies`, lists);
  const troops = entries(raw.troops, known.troops, `${id}.troops`, lists);
  if (enemies.length + troops.length === 0) errors.push('keine Figuren');
  const all = [...errors.map((m) => `${id}: ${m}`), ...lists];
  if (all.length > 0) throw new Error(all.join('; '));
  return raw as unknown as Scenario;
}

/** Alle Szenarien einer Datei (Schlüssel ohne `_`) prüfen */
export function parseScenarios(file: Record<string, unknown>, known: KnownTypes): Record<string, Scenario> {
  const out: Record<string, Scenario> = {};
  for (const [id, raw] of Object.entries(file)) if (!id.startsWith('_')) out[id] = parseScenario(id, raw, known);
  if (Object.keys(out).length === 0) throw new Error('keine Szenarien');
  return out;
}

/** `?bg=0` schaltet den Hintergrund, `?fg=0` den Vordergrund ab (Vergleichslauf: Figuren allein gegen Figuren mit Szene) */
export function withScenery(s: Scenario, search: string): Scenario {
  const p = new URLSearchParams(search);
  return { ...s, scenery: { ...s.scenery, background: s.scenery.background && p.get('bg') !== '0', foreground: p.get('fg') === '0' ? 0 : s.scenery.foreground } };
}

/** Zahl der Stufen: ohne Rampe eine, sonst `ramp.steps` */
export const stageCount = (s: Scenario, ramp: boolean): number => (ramp ? s.ramp.steps : 1);

/** Anzahl eines Eintrags in Stufe `stage` (ab 1): `ceil(count × factor^(stage−1))` */
export const countAt = (count: number, factor: number, stage: number): number => Math.ceil(count * Math.pow(factor, stage - 1) - 1e-9);

/** Eine Figur der Szene; `key` bestimmt ihre Werte und bleibt über alle Stufen gleich */
export interface Figure {
  kind: 'enemy' | 'troop';
  type: string;
  key: number;
  player: number;
}

type PoolEntry = { kind: Figure['kind']; type: string };

/** Pool der Typen: `typical` die Einträge der Datei (höchstens `distinctSheets`), `worstCase` auch weitere bekannte bis `distinctSheets` */
function pool(s: Scenario, known: KnownTypes): PoolEntry[] {
  const listed: PoolEntry[] = [...s.enemies.map((e) => ({ kind: 'enemy' as const, type: e.type })), ...s.troops.map((e) => ({ kind: 'troop' as const, type: e.type }))];
  if (s.sprites.mix === 'typical') return listed.slice(0, s.sprites.distinctSheets);
  const all: PoolEntry[] = [...known.enemies.map((type) => ({ kind: 'enemy' as const, type })), ...known.troops.map((type) => ({ kind: 'troop' as const, type }))];
  const seen = new Set(listed.map((l) => `${l.kind}:${l.type}`));
  return [...listed, ...all.filter((a) => !seen.has(`${a.kind}:${a.type}`))].slice(0, s.sprites.distinctSheets);
}

/** Figuren der Stufe `stage` (ab 1). Die Figuren einer Stufe sind eine Vorsilbe der nächsten (gleiche `key`). */
export function figuresAt(s: Scenario, known: KnownTypes, stage: number, players: 1 | 2): Figure[] {
  const p = pool(s, known);
  const f = s.ramp.factor;
  const list = [...s.enemies.map((e) => ({ ...e, kind: 'enemy' as const })), ...s.troops.map((e) => ({ ...e, kind: 'troop' as const }))];
  if (s.sprites.mix === 'worstCase') {
    const total = list.reduce((n, e) => n + countAt(e.count, f, stage), 0);
    return Array.from({ length: total }, (_, n) => ({ ...p[n % p.length]!, key: n, player: n % players }));
  }
  const allowed = new Set(p.map((q) => `${q.kind}:${q.type}`));
  const out: Figure[] = [];
  list.forEach((e, i) => {
    if (!allowed.has(`${e.kind}:${e.type}`)) return;
    for (let j = 0, n = countAt(e.count, f, stage); j < n; j++) out.push({ kind: e.kind, type: e.type, key: i * 100_000 + j, player: 0 });
  });
  return out.map((fig) => ({ ...fig, player: fig.key % players }));
}

/** Zahl verschiedener Figuren-Typen in einer Liste (die „Sheets“ im Log) */
export const distinctTypes = (figs: readonly Figure[]): number => new Set(figs.map((x) => `${x.kind}:${x.type}`)).size;

/** Seed-Zähler (mulberry32): gleiche Zahl, gleiche Folge; die Rückgabe liegt in [0, 1) */
export function seeded(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return ((t ^ (t >>> 14)) >>> 0) / 4294967296;
  };
}

/** Bewegung einer Figur: Mitte, Hub, Tempo, Phase, Bahn (Tiefe) */
export interface Motion {
  x: number;
  amp: number;
  speed: number;
  phase: number;
  lane: number;
}

/** Werte einer Figur aus Seed und Schlüssel; `view` ist die Breite des Bildausschnitts, `centerX` die Mitte ihres Spielers */
export function motionFor(seed: number, key: number, centerX: number, view: number): Motion {
  const r = seeded((Math.imul(seed, 2654435761) + Math.imul(key, 40503)) >>> 0);
  return { x: centerX + (r() - 0.5) * view * 0.9, amp: 40 + r() * 160, speed: 0.4 + r() * 1.2, phase: r() * Math.PI * 2, lane: (r() - 0.5) * 70 };
}

/** FNV-1a über den Text, 8 Hex-Zeichen (Kennung der Szenario-Datei im Log) */
export function hashText(text: string): string {
  let h = 0x811c9dc5;
  for (let i = 0; i < text.length; i++) h = Math.imul(h ^ text.charCodeAt(i), 0x01000193) >>> 0;
  return h.toString(16).padStart(8, '0');
}
