/**
 * Ebenen „Seiten“ des Spielmenüs (B-768, BD11.4): vier Kategorien, darunter eine Kachel je Seite oder Szenario. Die Einträge kommen aus
 * den Daten der Werkzeugseite (`src/tools/devTiles.ts`), nicht aus einer zweiten Liste: jede Seite hat genau eine Kachel. Reine Daten und Logik ohne Phaser; `LobbyScene` zeichnet und führt aus.
 */
import { t } from '../core/texts';
import { runParamsOf, type RunParams } from '../core/runConfig';

export { runParamsOf };
import { DEV_SECTIONS, type DevTile } from '../tools/devTiles';
import { HUB_COLORS } from './pauseHub';

export type CategoryId = 'info' | 'dev' | 'perf' | 'balance';

/** Was ein Eintrag beim Bestätigen tut */
export type EntryTarget =
  /** Seite des Projekts öffnen (bis BD11.5 über die Shell, danach als Panel im Menü) */
  | { kind: 'page'; href: string }
  /** Spiel mit dieser Konfiguration starten, ohne Neuladen (`startRun`) */
  | { kind: 'run'; params: RunParams };

export interface MenuEntry {
  /** Ziel aus den Daten (`href`), eindeutig */
  id: string;
  readonly title: string;
  readonly description: string;
  target: EntryTarget;
}

export interface MenuCategory {
  id: CategoryId;
  entries: readonly MenuEntry[];
}

/** Farbe je Kategorie: Türkis = Info, Blau = Entwicklung, Rosa = Performance, ausgegraut = leer */
export const CATEGORY_COLOR: Record<CategoryId, number> = { info: HUB_COLORS.teal, dev: HUB_COLORS.blue, perf: HUB_COLORS.pink, balance: HUB_COLORS.blue };

/**
 * Entwicklung, Performance und Balancing: vorerst immer sichtbar (B-768, Dev-Modus fest an). Später nur bei Dev-Modus des Servers
 * (`/api/health`).
 */
export const devMenuVisible = (): boolean => true;

/**
 * Ein Szenario mit `fresh=1` legt einen neuen Spielstand an; der Server lehnt das auf einen vorhandenen Namen ab („Spielstand gibt es schon“).
 * Deshalb trägt jeder Start einen eigenen Namen: der feste Name plus die Uhrzeit (Base 36), höchstens 32 Zeichen (`a-z`, `0-9`, `-`).
 */
export function uniqueRun(params: RunParams, now: number = Date.now()): RunParams {
  const save = params.save;
  if (params.fresh !== '1' || !save) return params;
  return { ...params, save: `${save.slice(0, 24)}-${now.toString(36)}`.slice(0, 32) };
}

/** Ziel einer Kachel der Werkzeugseite: Spiel-Szenario oder Seite */
export function targetOf(tile: Pick<DevTile, 'href'>): EntryTarget {
  const params = runParamsOf(tile.href);
  return params ? { kind: 'run', params } : { kind: 'page', href: tile.href };
}

const entryOf = (tile: DevTile): MenuEntry => ({
  id: tile.href,
  get title() {
    return tile.title;
  },
  get description() {
    return tile.description;
  },
  target: targetOf(tile),
});

const licenses: MenuEntry = {
  id: 'lizenzen.html',
  get title() {
    return t('landing.licenses');
  },
  get description() {
    return t('landing.licenses.desc');
  },
  target: { kind: 'page', href: 'lizenzen.html' },
};

/** Alle Kategorien in Anzeigereihenfolge; ohne `dev` nur Info und Hilfe */
export function menuCategories(dev: boolean = devMenuVisible()): readonly MenuCategory[] {
  const tiles = (id: 'dev' | 'perf' | 'balance') => DEV_SECTIONS.find((s) => s.id === id)?.tiles ?? [];
  const info: MenuCategory = { id: 'info', entries: [licenses] };
  if (!dev) return [info];
  return [
    info,
    { id: 'dev', entries: tiles('dev').map(entryOf) },
    { id: 'perf', entries: tiles('perf').map(entryOf) },
    { id: 'balance', entries: tiles('balance').map(entryOf) },
  ];
}

/** Beschriftung der Kategorie-Kachel */
export const categoryLabel = (id: CategoryId): string => t(`menu.cat.${id}`);

/** Zeile unter der Kategorie: Zahl der Einträge, bei keinem „noch leer“ */
export const categorySub = (c: MenuCategory): string => (c.entries.length === 0 ? t('menu.catEmpty') : t('menu.catCount', { n: c.entries.length }));
