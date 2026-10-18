import type { LevelModel } from './levelView';

// Ansicht des Welt-Betrachters (B-477, WZ16.1; B-485, WZ17.4): Chunk-Streifen und Live-Vorschau teilen die Weltposition
// der Bildmitte, der Zoom gilt je Vorschau. Reine Funktionen, kein DOM. Der Maßstab ist „Pixel je Unit“; `View` ist die
// Ansicht einer Vorschau mit linker Kante in Units.

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

/** Die beiden Vorschauen des Welt-Betrachters */
export type Pane = 'chunk' | 'live';

/** Ansicht beider Vorschauen (B-485/AC-05): eine gemeinsame Weltposition der Bildmitte, ein Zoom je Vorschau. */
export interface Views {
  /** Weltposition der Bildmitte in Units, gilt für beide Vorschauen */
  center: number;
  /** Pixel je Unit der Chunk-Vorschau */
  chunk: number;
  /** Pixel je Unit der Live-Vorschau */
  live: number;
}

/** Erlaubter Bereich der Mitte: die engste Ansicht (größter Maßstab) reicht bis an die Ränder der Welt; ist die Welt
 *  schmaler, gilt ihre Mitte. Eine weitere Ansicht hält `paneView` am Rand fest, nur dort weicht ihre Bildmitte ab. */
export function centerRange(scales: readonly number[], widthUnits: number, viewPx: number): [number, number] {
  const half = viewPx / Math.max(...scales) / 2;
  return widthUnits <= 2 * half ? [widthUnits / 2, widthUnits / 2] : [half, widthUnits - half];
}

export function clampCenter(center: number, scales: readonly number[], widthUnits: number, viewPx: number): number {
  const [lo, hi] = centerRange(scales, widthUnits, viewPx);
  return Math.max(lo, Math.min(center, hi));
}

const settle = (v: Views, widthUnits: number, viewPx: number): Views => ({ ...v, center: clampCenter(v.center, [v.chunk, v.live], widthUnits, viewPx) });

/** Linke Kante und Maßstab einer Vorschau: ihre Bildmitte liegt auf `center`. */
export function paneView(v: Views, pane: Pane, widthUnits: number, viewPx: number): View {
  const scale = v[pane];
  return { left: clampLeft(v.center - viewPx / scale / 2, widthUnits, viewPx / scale), scale };
}

/** Neuer Zoom für die genannten Vorschauen (`to` bekommt den alten Maßstab); die Mitte bleibt, soweit die Welt reicht. */
export function zoomPanes(v: Views, panes: readonly Pane[], to: (scale: number) => number, widthUnits: number, viewPx: number): Views {
  const next = { ...v };
  for (const p of panes) next[p] = clampScale(to(v[p]));
  return settle(next, widthUnits, viewPx);
}

/** Bildlauf um `units` (Welt-Strecke): beide Vorschauen wandern gleich weit. */
export const panViews = (v: Views, units: number, widthUnits: number, viewPx: number): Views => settle({ ...v, center: v.center + units }, widthUnits, viewPx);

/** Mitte auf `x` (Units) stellen, am Rand begrenzt. */
export const centerViews = (v: Views, x: number, widthUnits: number, viewPx: number): Views => settle({ ...v, center: x }, widthUnits, viewPx);

/** `own`: nur die Vorschau unter dem Zeiger zoomen (Alt + Rad, B-485/AC-05) */
export type WheelAction = { kind: 'pan'; px: number } | { kind: 'zoom'; factor: number; own?: true } | null;

/** Was ein Mausrad-Ereignis über einer Vorschau bewirkt: Strg + Rad zoomt, Alt + Rad zoomt nur diese Vorschau, Umschalt + Rad
 *  oder seitliches Rad scrollt nach links und rechts; das gewöhnliche Rad bewirkt nichts, damit die Seite senkrecht scrollen kann. */
export function wheelAction(e: { deltaX: number; deltaY: number; ctrlKey: boolean; shiftKey: boolean; altKey?: boolean }): WheelAction {
  const factor = e.deltaY < 0 ? 1.25 : 0.8;
  if (e.ctrlKey) return e.deltaY === 0 ? null : { kind: 'zoom', factor };
  if (e.altKey) return e.deltaY === 0 ? null : { kind: 'zoom', factor, own: true };
  const px = Math.abs(e.deltaX) > Math.abs(e.deltaY) ? e.deltaX : e.shiftKey ? e.deltaY : 0;
  return px === 0 ? null : { kind: 'pan', px };
}

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
