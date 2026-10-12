/** Crossfade des Audio-Kerns (B-250): reine Kurven ohne Browser, damit sie testbar bleiben. */

/** Standard-Dauer eines Stückwechsels in Sekunden (Q16, Beispiel Abend → Nacht). */
export const CROSSFADE_SECONDS = 2;

/** Kurzes Ausblenden statt hartem Abschneiden (kein Knacken), wenn ein Stück ohne Crossfade ersetzt oder gestoppt wird. */
export const QUICK_FADE_SECONDS = 0.03;

/**
 * Crossfade-Kurven mit gleicher Leistung (cos/sin), `n` Stützpunkte für `setValueCurveAtTime`.
 * `out` startet bei 1 und endet bei 0, `in` umgekehrt; benachbarte Werte springen nie (kein Knacken).
 */
export function crossfadeCurves(n = 64): { out: Float32Array; in: Float32Array } {
  const out = new Float32Array(n);
  const inn = new Float32Array(n);
  for (let i = 0; i < n; i++) {
    const x = (i / (n - 1)) * (Math.PI / 2);
    out[i] = Math.cos(x);
    inn[i] = Math.sin(x);
  }
  return { out, in: inn };
}

/** Rein: Ausblend-Kurve, die beim aktuellen Pegel `level` (0–1) beginnt (Umlenken mitten im Übergang ohne Sprung). */
export function fadeOutFrom(level: number, n = 64): Float32Array {
  const scale = Number.isFinite(level) ? Math.min(1, Math.max(0, level)) : 1;
  return crossfadeCurves(n).out.map((v) => v * scale);
}
