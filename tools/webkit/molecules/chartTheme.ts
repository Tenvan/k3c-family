// Gemeinsame Farben der SVG-Diagramme: nur Radix-CSS-Variablen, damit helles und dunkles Theme lesbar bleiben.
export const SERIES_COLORS = ['var(--accent-9)', 'var(--orange-9)', 'var(--green-9)', 'var(--purple-9)', 'var(--cyan-9)', 'var(--red-9)'];
export const TONE_FILL = { ok: 'var(--green-9)', warn: 'var(--amber-9)', fail: 'var(--red-9)' } as const;
export type ChartTone = keyof typeof TONE_FILL;
export const GRID = 'var(--gray-a6)';
export const AXIS_TEXT = 'var(--gray-11)';
export const BAND_FILL = 'var(--green-a4)';
export const BAND_LINE = 'var(--green-9)';

/** Zielkorridor eines Diagramms; fehlt eine Seite, ist sie offen. */
export interface ChartBand {
  lower?: number;
  upper?: number;
}
