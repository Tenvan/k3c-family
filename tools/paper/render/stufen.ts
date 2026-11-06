// Gebäude-Stufen und Ausgabedateien (paper-spec.md › 3 `stufen`, › 7 Ausgabe-Layout).
import type { Animation, Spec } from '../schema/types';
import { tausche } from './stil';

/** Spec der Stufe n (1 = Basis): stufen[0..n-2] nacheinander, je Stufe erst Zusätze, dann Tausch. */
export function stufenSpec(spec: Spec, n: number): Spec {
  let s: Spec = structuredClone(spec);
  for (const st of (spec.stufen ?? []).slice(0, Math.max(0, n - 1))) {
    s = { ...s, teile: [...s.teile, ...structuredClone(st.zusaetze ?? [])] };
    if (st.tausch) s = tausche(s, st.tausch);
  }
  return s;
}

/** Animationen der Spec; ohne `animationen` gilt `stand` mit einem Frame. */
export function animationen(spec: Spec): Record<string, Animation> {
  return spec.animationen && Object.keys(spec.animationen).length ? spec.animationen : { stand: { frames: 1 } };
}

export type Ausgabe = { schluessel: string; animation: string; variante?: string; stufe: number };

/** Alle Ausgabedateien (Schlüssel = Dateiname ohne .png): Animationen alphabetisch, ohne Variante zuerst, Stufen aufsteigend. */
export function ausgaben(spec: Spec): Ausgabe[] {
  const varianten: (string | undefined)[] = [undefined, ...Object.keys(spec.varianten ?? {}).sort()];
  const stufen = Array.from({ length: (spec.stufen?.length ?? 0) + 1 }, (_, i) => i + 1);
  const out: Ausgabe[] = [];
  for (const animation of Object.keys(animationen(spec)).sort())
    for (const variante of varianten)
      for (const stufe of stufen) {
        const schluessel = `${animation}${variante ? `@${variante}` : ''}${stufe > 1 ? `.s${stufe}` : ''}`;
        out.push(variante ? { schluessel, animation, variante, stufe } : { schluessel, animation, stufe });
      }
  return out;
}

/** Spec einer Ausgabe: Stufe anwenden, dann den Tausch der Variante. */
export function ausgabeSpec(spec: Spec, a: Ausgabe): Spec {
  const s = stufenSpec(spec, a.stufe);
  return a.variante ? tausche(s, spec.varianten![a.variante].tausch) : s;
}
