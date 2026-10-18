import type { LevelModel } from './levelView';

// Gemeinsame Ansicht des Welt-Betrachters (B-477, WZ16.1): Position und Zoom gelten für Chunk-Streifen und Live-Vorschau
// gleich. Reine Funktionen, kein DOM. Der Maßstab ist „Pixel je Unit“; die Position ist die linke Kante in Units.

export const MIN_SCALE = 2;
export const MAX_SCALE = 48;
export const DEFAULT_SCALE = 12;
/** Feste Zoomstufen in Prozent des Standard-Maßstabs */
export const ZOOM_PERCENTS: readonly number[] = [25, 50, 100, 200, 400];

export interface View {
  /** linke Kante in Units */
  left: number;
  /** Pixel je Unit */
  scale: number;
}

export const clampScale = (scale: number): number => Math.min(MAX_SCALE, Math.max(MIN_SCALE, scale));

export const scaleOfPercent = (percent: number): number => clampScale((DEFAULT_SCALE * percent) / 100);

/** Linke Kante so begrenzt, dass die Ansicht (viewUnits breit) in der Welt (widthUnits) bleibt; ist sie breiter, gilt 0. */
export function clampLeft(left: number, widthUnits: number, viewUnits: number): number {
  return Math.max(0, Math.min(left, widthUnits - viewUnits));
}

/** Ansicht nach einem Wechsel auf `scale`: die Mitte der Ansicht bleibt an ihrer Stelle in der Welt. */
export function zoomView(view: View, scale: number, widthUnits: number, viewPx: number): View {
  const next = clampScale(scale);
  const center = view.left + viewPx / view.scale / 2;
  return { scale: next, left: clampLeft(center - viewPx / next / 2, widthUnits, viewPx / next) };
}

/** Linke Kante, die `x` (Units) in die Mitte der Ansicht stellt. */
export const leftFor = (x: number, widthUnits: number, viewUnits: number): number => clampLeft(x - viewUnits / 2, widthUnits, viewUnits);

export interface JumpPoint {
  id: string;
  label: string;
  /** Ziel in Units; null = im Level nicht vorhanden (Knopf ausgegraut) */
  x: number | null;
}

const first = (xs: readonly number[]): number | null => (xs.length > 0 ? xs[0] : null);

/** Sprungpunkte: ganz links, Mitte, ganz rechts, Hub, Tor, Portale, Ausgang. `gates` sind die X-Positionen der Tore im Zustand der Live-Vorschau. */
export function jumpPoints(model: LevelModel | null, gates: readonly number[]): JumpPoint[] {
  const width = model?.widthUnits ?? 0;
  const xs = (kind: string) => (model?.objects ?? []).filter((o) => o.kind === kind).map((o) => o.x);
  const portals = xs('portal');
  return [
    { id: 'links', label: 'Anfang', x: model ? 0 : null },
    { id: 'mitte', label: 'Mitte', x: model ? width / 2 : null },
    { id: 'rechts', label: 'Ende', x: model ? width : null },
    { id: 'hub', label: 'Hub', x: model && model.hubCenterUnits > 0 ? model.hubCenterUnits : null },
    { id: 'tor', label: 'Tor', x: first([...gates].sort((a, b) => a - b)) },
    { id: 'portal1', label: 'Portal 1', x: first(portals) },
    { id: 'portal2', label: 'Portal 2', x: portals.length > 1 ? portals[1] : null },
    { id: 'ausgang', label: 'Ausgang', x: first(xs('exit')) },
  ];
}
