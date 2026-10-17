import enemiesJson from '@data/enemies.json';
import spritesJson from '@data/sprites.json';
import { BIOMES } from '@game/model/biome';
import { ANIMS, SHEETS, gap, type Anim, type FigureSpec, type Gap } from '../abspiel/figure';
import type { FigureItem } from '../abspiel/figureView';
import { DEFAULT_MOUNT, monarchItems, moveOf } from '../monarch/monarchLogic';

// Reine Logik der Gegner-Seite (B-464, WZ11.1): Gegnertypen nach Biom und Tiefe, Karten mit Merkmalen und Lücken,
// Animationen mit den Parametern des Spiels (tint, scale, alpha, y). Werte stammen aus `data/enemies.json`,
// `data/biomes/*.json` und `data/sprites.json`; gezeichnet wird über die Bausteine aus abspiel/.

export interface EnemyDef {
  name: string;
  depth: number;
  tier: string;
  hp: number;
  damage: number;
  speed: number;
  range: number;
  attacksPerSecond: number;
  traits: string[];
}

const DEFS = enemiesJson as unknown as Record<string, EnemyDef>;
const SPRITES = spritesJson as unknown as { enemies: Record<string, FigureSpec> };

/** Alle Gegnertypen aus `data/enemies.json` (heute 17) */
export const ENEMY_IDS: readonly string[] = Object.keys(DEFS);

/** Grafik, die das Spiel für Gegner ohne eigene Zuordnung nimmt (`goblin`, wie `ENEMY_SPRITES[type] ?? ENEMY_SPRITES.goblin`) */
export const FALLBACK_ID = 'goblin';

export interface TraitView {
  trait: string;
  /** Lücke, wenn das Spiel das Merkmal nicht darstellt */
  gap: Gap | null;
}

export interface EnemyCard {
  id: string;
  /** null = der Gegner steht im Biom, fehlt aber in `data/enemies.json` */
  def: EnemyDef | null;
  spec: FigureSpec;
  traits: TraitView[];
  /** Lücken der Karte: fehlende Daten, Rückfall-Grafik */
  gaps: Gap[];
}

/** Das Spiel zeichnet kein Merkmal besonders (nur `src/model` kennt die Merkmale); `flying` zeigt sich über die Höhe `y`
 *  der Grafik. Alles andere ist eine Lücke. */
function traitView(trait: string, spec: FigureSpec): TraitView {
  if (trait === 'flying' && (spec.y ?? 0) > 0) return { trait, gap: null };
  return { trait, gap: gap('display', `Lücke: Merkmal ${trait} ohne Darstellung im Spiel`) };
}

/** Karte eines Gegners: Daten, Grafik (Rückfall `goblin` mit Lücke „keine eigene Grafik“), Merkmale. Ein Gegner ohne
 *  Eintrag in `data/enemies.json` ergibt eine Karte mit Lücke, keinen Absturz. */
export function cardOf(id: string): EnemyCard {
  const def = DEFS[id] ?? null;
  const own = SPRITES.enemies[id];
  const spec = own ?? SPRITES.enemies[FALLBACK_ID] ?? { sheet: FALLBACK_ID };
  const gaps: Gap[] = [];
  if (!def) gaps.push(gap('display', 'Lücke: Gegner fehlt in data/enemies.json'));
  if (!own) gaps.push(gap('sheet', 'Lücke: keine eigene Grafik'));
  return { id, def, spec, traits: (def?.traits ?? []).map((t) => traitView(t, spec)), gaps };
}

export interface Group {
  id: string;
  title: string;
  /** Tiefe des Bioms, null bei „ohne Biom“ */
  depth: number | null;
  ids: string[];
}

/** Gruppen je Biom nach Tiefe: erst die Gegner des Portals, dann die der Nacht (`enemies.portal`, `enemies.night`); danach
 *  die Gruppe „ohne Biom“ mit den Typen, die kein Biom nennt (z. B. `alphaWolf`). Ein Gegner, den ein Biom nennt, den die
 *  Daten aber nicht kennen, steht trotzdem in der Gruppe (Karte mit Lücke). */
export function groups(): Group[] {
  const named = new Set<string>();
  const out: Group[] = [...BIOMES].sort((a, b) => a.depth - b.depth).map((b) => {
    const ids = [...b.enemies.portal, ...b.enemies.night].filter((id, i, all) => all.indexOf(id) === i);
    ids.forEach((id) => named.add(id));
    return { id: b.id, title: b.name, depth: b.depth, ids };
  });
  out.push({ id: 'none', title: 'ohne Biom', depth: null, ids: ENEMY_IDS.filter((id) => !named.has(id)) });
  return out;
}

/** Animationen der Gegner-Karten: alle des Spiels (`idle`, `run`, `attack`); fehlt eine im Sheet, zeigt die Karte `idle` mit Lücke */
export const ENEMY_ANIMS: readonly Anim[] = ANIMS;

export type EnemyActionId = 'hit' | 'kill';

export interface EnemyAction {
  id: EnemyActionId;
  label: string;
  effect: { kind: 'hit' | 'kill' };
  gaps: Gap[];
}

/** Getroffen und Sterben wie im Spiel: Effekt `hit` bzw. `kill`; eigene Animationen gibt es nicht (Lücke). */
export const ENEMY_ACTIONS: readonly EnemyAction[] = [
  { id: 'hit', label: 'Getroffen', effect: { kind: 'hit' }, gaps: [gap('anim', 'Lücke: Treffer-Animation fehlt (die Sheets haben idle, run, attack)')] },
  { id: 'kill', label: 'Sterben', effect: { kind: 'kill' }, gaps: [gap('anim', 'Lücke: Sterbe-Animation fehlt (die Sheets haben idle, run, attack)')] },
];

/** Abstand der Gegner nebeneinander in Units */
export const SPACING = 7;

export interface View {
  anim: Anim;
  action?: EnemyAction;
  /** Monarch 0 beritten auf dem Standard-Reittier links neben den Gegnern, auf derselben Bodenlinie */
  withMonarch?: boolean;
  /** Ersatz-Sheet je Gegner-ID (Grafik-Auswahl); fehlt ein Eintrag, gilt die Grafik des Spiels */
  sheets?: Readonly<Record<string, string | undefined>>;
}

/** Figuren einer Gruppe: die Karten nebeneinander mit den Parametern des Spiels (`tint`, `scale`, `alpha`, `y`). */
export function enemyItems(cards: readonly EnemyCard[], view: View): FigureItem[] {
  const items: FigureItem[] = cards.map((c, i) => ({
    id: `enemy-${c.id}`,
    x: i * SPACING,
    spec: { ...c.spec, sheet: view.sheets?.[c.id] ?? c.spec.sheet },
    anim: view.anim,
    ...(view.action ? { effect: view.action.effect } : {}),
  }));
  if (view.withMonarch) {
    const monarch = monarchItems(DEFAULT_MOUNT, moveOf('stand'), 'idle')[0]!;
    items.unshift({ ...monarch, id: 'monarch-vergleich', x: -SPACING });
  }
  return items;
}

/** Sind alle Sheets einer Karte bekannt? (Lücke, wenn nicht) */
export const knownSheet = (sheet: string): boolean => sheet in SHEETS;
