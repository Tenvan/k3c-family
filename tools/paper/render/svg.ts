// Lagen und Teile als SVG; Filter (Schatten, Papierstruktur) aus der Stil-Datei (paper-spec.md › 2, 3, 6).
// Transformationen als Kette von rotate(…) statt matrix(…): so gilt die 2-Nachkommastellen-Regel ohne Verzerrung.
import { createHash } from 'node:crypto';
import { num } from '../schema/hash';
import type { Lage, Spec, Stil, Teil } from '../schema/types';
import { farbe } from './stil';

/** Seed der Papierstruktur: erste 4 Byte von SHA-256(id) als uint32, % 1000 + 1. */
export function seed(id: string): number {
  return (createHash('sha256').update(id, 'utf8').digest().readUInt32BE(0) % 1000) + 1;
}

/** Gegenfarbe der Papierstruktur (Wert des Prototyps; die Stil-Datei hat dafür kein Feld). */
const STRUKTUR_INTERCEPT = 0.86;

/** Faktor von Prototyp-Pixeln auf die Pixeldichte der Ausgabe (px je Einheit); ohne Dichte 1, geklemmt auf 1…4 (RD2.1). */
export function dichteFaktor(stil: Stil, dichte?: number): number {
  if (!dichte || dichte <= 0) return 1;
  return Math.min(4, Math.max(1, stil.papier.bezug_px_je_einheit / dichte));
}

/** Filter der Stil-Datei; Werte gelten bei `bezug_px_je_einheit` und werden auf die Ausgabe-Dichte umgerechnet. */
export function defs(stil: Stil, id: string, dichte?: number): string {
  const { lage, auflage, struktur } = stil.papier;
  const k = dichteFaktor(stil, dichte);
  const schatten = (name: string, s: { dx: number; dy: number; unschaerfe: number; deckkraft: number }) =>
    `<filter id="${name}" x="-30%" y="-30%" width="160%" height="160%"><feDropShadow dx="${num(s.dx * k)}" dy="${num(s.dy * k)}" ` +
    `stdDeviation="${num(s.unschaerfe * k)}" flood-color="${lage.farbe}" flood-opacity="${num(s.deckkraft)}"/></filter>`;
  const f = (k: string) => `<feFunc${k} type="linear" slope="${num(struktur.staerke)}" intercept="${STRUKTUR_INTERCEPT}"/>`;
  const papier = `<filter id="paper" x="-5%" y="-5%" width="110%" height="110%">` +
    `<feTurbulence type="fractalNoise" baseFrequency="${num(struktur.frequenz / k)}" numOctaves="${struktur.oktaven}" seed="${seed(id)}" result="n"/>` +
    `<feColorMatrix in="n" type="saturate" values="0" result="g"/>` +
    `<feComponentTransfer in="g" result="t">${f('R')}${f('G')}${f('B')}<feFuncA type="linear" slope="0" intercept="1"/></feComponentTransfer>` +
    `<feComposite in="SourceGraphic" in2="t" operator="arithmetic" k1="1" k2="0" k3="0" k4="0"/></filter>`;
  return `<defs>${schatten('ds', lage)}${schatten('ds2', auflage)}${papier}</defs>`;
}

export function lageSvg(l: Lage, stil: Stil): string {
  const fill = farbe(stil, l.material, l.ton);
  if (l.art === 'flaeche') return `<path d="${l.form}" fill="${fill}" filter="url(#${l.auflage ? 'ds2' : 'ds'})"/>`;
  if (l.art === 'falz')
    return `<path d="${l.form}" fill="none" stroke="${fill}" stroke-width="${num(l.breite)}" stroke-linecap="round" ` +
      `opacity="${num(stil.papier.falz_deckkraft)}"/>`;
  return `<circle cx="${num(l.mitte[0])}" cy="${num(l.mitte[1])}" r="${num(l.radius)}" fill="${fill}" stroke="none"/>`;
}

/** Kette der Drehungen von der Wurzel bis zum Teil (Winkel 0 entfallen); leer = keine Transformation. */
export function kette(teile: Teil[], winkel: Record<string, number>): Map<string, string> {
  const nachId = new Map(teile.map((t) => [t.id, t]));
  const out = new Map<string, string>();
  const von = (t: Teil): string => {
    const da = out.get(t.id);
    if (da !== undefined) return da;
    const eltern = t.eltern ? nachId.get(t.eltern) : undefined;
    const w = winkel[t.id] ?? 0;
    const eigen = num(w) === '0' ? '' : `rotate(${num(w)} ${num(t.pivot[0])} ${num(t.pivot[1])})`;
    const k = [eltern ? von(eltern) : '', eigen].filter(Boolean).join(' ');
    out.set(t.id, k);
    return k;
  };
  for (const t of teile) von(t);
  return out;
}

/** Teil-ID und alle Nachkommen (z. B. zum Ausblenden einer Bein-Kette). */
export function mitNachkommen(teile: Teil[], id: string): Set<string> {
  const out = new Set([id]);
  for (let neu = true; neu;) {
    neu = false;
    for (const t of teile)
      if (t.eltern && out.has(t.eltern) && !out.has(t.id)) {
        out.add(t.id);
        neu = true;
      }
  }
  return out;
}

export type TeileOptionen = { einschub?: (id: string) => string; ausblenden?: Set<string> };

/** Alle Teile in Listenreihenfolge; `einschub(id)` steht vor dem Teil (Beinpfad, Reiter). */
export function teileSvg(spec: Pick<Spec, 'teile'>, stil: Stil, winkel: Record<string, number>, opt: TeileOptionen = {}): string {
  const k = kette(spec.teile, winkel);
  return spec.teile.map((t) => {
    const vor = opt.einschub?.(t.id) ?? '';
    if (opt.ausblenden?.has(t.id) || t.lagen.length === 0) return vor;
    const tf = k.get(t.id) ? ` transform="${k.get(t.id)}"` : '';
    return `${vor}<g${tf}>${t.lagen.map((l) => lageSvg(l, stil)).join('')}</g>`;
  }).join('');
}
