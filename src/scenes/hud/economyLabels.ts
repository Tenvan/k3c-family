import { nameOf, t, type TextKey } from '../../core/texts';
import { BUILDINGS } from '../../model/data';
import type { Site, World } from '../../model/types';

/**
 * Wirtschaft als Text (W6.1, B-117): Wartegrund, Lagerstand und Hub-Stufe aus den Server-Feldern (docs/protocol.md ›
 * Wirtschaft). Reine Funktionen ohne Phaser und ohne Regel-Rechnung; fehlt ein Feld (älterer Server), kommt `null` bzw. [].
 * `compact`: kurze Form für Split-Viertel (3 bis 4 Spieler), lang sonst.
 */

export type WaitReason = 'worker' | 'material' | 'danger';
/** Wartegrund mit Symbolname (die Zeichnung wählt das Symbol) und Text. */
export interface WaitInfo {
  reason: WaitReason;
  icon: string;
  text: string;
}

type WaitState = 'waitingMaterial' | 'waitingWorker' | string | undefined;
type CostMap = Partial<Record<string, number>>;
type LevelData = { levels?: { cost: CostMap }[] };

/** Feste Reihenfolge der Rohstoffe; weitere Schlüssel des Servers folgen dahinter. */
const RESOURCE_ORDER = ['wood', 'stone', 'copper', 'iron', 'crystal'];

export const resName = (res: string): string => nameOf('res', res, res);

/** Rohstoffe einer Kostenliste (ohne Gold) in fester Reihenfolge. */
function materialsOf(cost: CostMap | undefined): string[] {
  const keys = Object.keys(cost ?? {}).filter((k) => k !== 'gold' && (cost?.[k] ?? 0) > 0);
  return ordered(keys);
}

function ordered(keys: string[]): string[] {
  const known = RESOURCE_ORDER.filter((r) => keys.includes(r));
  return [...known, ...keys.filter((k) => !RESOURCE_ORDER.includes(k))];
}

/** Wartegrund aus dem Server-Zustand: `waitingWorker` heißt bei `danger` Gefahr. `resources` nennt das Material. */
export function waitInfo(state: WaitState, danger: boolean | undefined, resources: string[], compact: boolean): WaitInfo | null {
  if (state === 'waitingWorker') {
    const reason: WaitReason = danger ? 'danger' : 'worker';
    const key: TextKey = danger ? (compact ? 'wait.dangerShort' : 'wait.danger') : compact ? 'wait.workerShort' : 'wait.worker';
    return { reason, icon: `wait-${reason}`, text: t(key) };
  }
  const names = resources.map(resName).join(', ');
  if (state === 'waitingNextIsland') return { reason: 'material', icon: 'wait-material', text: t(compact ? 'wait.nextIslandShort' : 'wait.nextIsland', { res: names }) };
  if (state !== 'waitingMaterial') return null;
  const text = compact ? t('wait.materialShort') : names ? t('wait.material', { res: names }) : t('wait.materialAny');
  return { reason: 'material', icon: 'wait-material', text };
}

/**
 * Baukosten eines unbezahlten Platzes laut Daten: der Eintrag der Biom-Tiefe `depth` in `costByDepth` (Treppen, B-631),
 * fehlt er oder ist er leer, `cost` der Art. Spiegel von `siteCost` in engine/sim/hub_level.go, nur für die Anzeige.
 */
export function buildCost(kind: string, depth: number | undefined): CostMap {
  const data = BUILDINGS[kind];
  const byDepth = depth === undefined ? undefined : data?.costByDepth?.[depth];
  return byDepth && Object.keys(byDepth).length > 0 ? byDepth : (data?.cost ?? {});
}

/** Wartegrund eines Bauplatzes: erst der Bau (`state`), sonst der laufende Ausbau (`upgrade`). Material aus den Daten, `depth` = Biom-Tiefe. */
export function siteWait(site: Pick<Site, 'kind' | 'state' | 'upgrade' | 'level'>, danger: boolean | undefined, compact: boolean, depth?: number): WaitInfo | null {
  const data = BUILDINGS[site.kind] as (typeof BUILDINGS)[string] & LevelData | undefined;
  if (site.state !== 'built') return waitInfo(site.state, danger, materialsOf(buildCost(site.kind, depth)), compact);
  // Ausbau zielt auf Stufe level+1, das ist Eintrag `level` (0-basiert) der Daten
  return waitInfo(site.upgrade, danger, materialsOf(data?.levels?.[site.level ?? 1]?.cost), compact);
}

/** Wartegrund des Hub-Ausbaus (`hubUpgrade.state`). */
export function hubWait(world: Pick<World, 'hubUpgrade' | 'danger'> & { stock?: World['stock'] }, compact: boolean): WaitInfo | null {
  const up = world.hubUpgrade;
  if (!up) return null;
  // Material-Tor: nur nennen, was im Vorrat noch fehlt (sonst stünde „Stein, Kupfer“, obwohl nur Kupfer fehlt; Review GEB17)
  const stock = (world.stock ?? {}) as CostMap;
  const need = (up.material ?? {}) as CostMap;
  const names = materialsOf(up.material);
  const missing = up.state === 'waitingNextIsland' ? names.filter((r) => (stock[r] ?? 0) < (need[r] ?? 0)) : names;
  return waitInfo(up.state, world.danger, missing.length > 0 ? missing : names, compact);
}

/**
 * Lagerstand je Rohstoff: nur Rohstoffe mit Vorrat oder der Primärrohstoff des Bioms. Mit `stockMax` als „n/max“,
 * „voll“ bei Vorrat = Maximum; ohne Maximum (fehlt oder 0) nur die Menge.
 */
export function stockTexts(world: Pick<World, 'stock' | 'stockMax'> & { biome?: { primaryResource?: string } }, compact: boolean): string[] {
  const stock = (world.stock ?? {}) as CostMap;
  const primary = world.biome?.primaryResource;
  const all = Object.keys(stock);
  if (primary && !all.includes(primary)) all.push(primary);
  const keys = ordered(all).filter((r) => (stock[r] ?? 0) > 0 || r === primary);
  const max = world.stockMax ?? 0;
  return keys.map((r) => {
    const params = { res: resName(r), n: stock[r] ?? 0, max };
    if (max <= 0) return t('stock.amount', params);
    if (params.n >= max) return t(compact ? 'stock.fullShort' : 'stock.full', params);
    return t('stock.ofMax', params);
  });
}

/** Kosten des Hub-Ausbaus: Gold (bezahlt/gesamt) und Material in fester Reihenfolge. */
function hubCost(up: NonNullable<World['hubUpgrade']>): string {
  const parts = [t('cost.gold', { paid: up.paid, gold: up.gold })];
  for (const r of materialsOf(up.material)) parts.push(t('cost.item', { n: up.material[r as keyof typeof up.material] ?? 0, res: resName(r) }));
  return parts.join(', ');
}

/** Ausbauschritt als Zahlen (B-592), nur wenn der Server `hubStep` und `hubUpgrade.steps` meldet. */
function hubStepOf(world: Partial<Pick<World, 'hubStep' | 'hubUpgrade'>>): { step: number; steps: number } | null {
  const step = world.hubStep;
  const steps = world.hubUpgrade?.steps;
  return step !== undefined && steps !== undefined ? { step, steps } : null;
}

/** Titel des Ausbau-Schilds an der Burg: „Ausbau 2/3“ mit Schritt (B-592), sonst „Ausbauen“. */
export function hubOfferTitle(world: Partial<Pick<World, 'hubStep' | 'hubUpgrade'>>): string {
  const s = hubStepOf(world);
  return s ? t('hub.step', s) : t('hub.offer');
}

/** Hub-Stufe mit Schritt und Kosten des laufenden Ausbauschritts; höchste Stufe (kein `hubUpgrade`) ohne Kosten; ohne `hubLevel` → null. */
export function hubText(world: Pick<World, 'hubLevel' | 'hubUpgrade'> & Partial<Pick<World, 'hubStep'>>, compact: boolean): string | null {
  const n = world.hubLevel;
  if (n === undefined) return null;
  if (!world.hubUpgrade) return t(compact ? 'hub.levelShort' : 'hub.level', { n });
  const params = { n, next: n + 1, cost: hubCost(world.hubUpgrade) };
  const s = hubStepOf(world);
  if (s) return t(compact ? 'hub.upgradeStepShort' : 'hub.upgradeStep', { ...params, ...s });
  return t(compact ? 'hub.upgradeShort' : 'hub.upgrade', params);
}
