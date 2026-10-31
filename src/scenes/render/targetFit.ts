import { UNIT_PX } from '../../core/constants';

/**
 * Zielmaß einer Grafik (B-685): Die Grafik (nach Ausschnitt) wird in das Zielmaß in Units eingepasst, ohne Verzerrung;
 * der Aufrufer stellt sie mit Ursprung unten mittig auf den Boden. Rein, ohne Phaser: Spiel und Studio-Bühne rechnen hier.
 */

export interface PxSize {
  w: number;
  h: number;
}

/** Ergebnis: Maßstab, belegte Größe in Spiel-Pixeln und ob weich skaliert wird */
export interface TargetFit {
  scale: number;
  w: number;
  h: number;
  /** false = Nearest (Pixel scharf), true = Linear */
  smooth: boolean;
}

/** Gültiges Zielmaß: beide Seiten endlich und > 0; sonst gilt das heutige Maß (Spec „Zielmaß 0 oder fehlend“) */
export function validTarget(t: { w?: unknown; h?: unknown } | null | undefined): PxSize | null {
  const ok = (v: unknown): v is number => typeof v === 'number' && Number.isFinite(v) && v > 0;
  return t && ok(t.w) && ok(t.h) ? { w: t.w, h: t.h } : null;
}

/** Scharf bleibt Pixelgrafik bei ganzzahliger Vergrößerung (×1, ×2 …); Verkleinern und krumme Faktoren skalieren weich */
export function crispScale(scale: number): boolean {
  return scale >= 1 && Math.abs(scale - Math.round(scale)) < 1e-6;
}

/**
 * Passt `source` (Pixel der Quelle) in `target` (Units) ein: Maßstab = kleineres Verhältnis von Breite und Höhe.
 * Ohne gültiges Zielmaß gilt `fallbackScale` (heutiger Maßstab der Datei).
 */
export function fitTarget(source: PxSize, target: { w?: unknown; h?: unknown } | null | undefined, fallbackScale = 1): TargetFit {
  const t = validTarget(target);
  const scale = t && source.w > 0 && source.h > 0 ? Math.min((t.w * UNIT_PX) / source.w, (t.h * UNIT_PX) / source.h) : fallbackScale;
  return { scale, w: source.w * scale, h: source.h * scale, smooth: !crispScale(scale) };
}
