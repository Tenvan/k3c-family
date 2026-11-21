// Eingabe von Werten in Tabellen (B-1173, WZ62.7): Art eines Werts, Prüfung beim Tippen, Schritt mit den Pfeiltasten und
// Abweichung zum Standardwert. Reine Funktionen ohne React; das Molecule `ValueField` zeigt sie an. Die endgültige
// Prüfung bleibt beim Schreiben im Backend (Typ, Bereich, Nachfolger-Welten).

export type ValueKind = 'number' | 'boolean' | 'text' | 'json';

export type Parsed = { ok: true; value: unknown } | { ok: false; error: string };

/** Art des Basiswerts: bestimmt Eingabefeld und Prüfung. Listen und Objekte bleiben JSON. */
export function valueKind(base: unknown): ValueKind {
  if (typeof base === 'number') return 'number';
  if (typeof base === 'boolean') return 'boolean';
  if (typeof base === 'string') return 'text';
  return 'json';
}

/** Wert als Text im Feld: Text bleibt Text, sonst JSON; ohne Wert „–“. */
export const valueText = (v: unknown): string => (v === undefined || v === null ? '–' : typeof v === 'string' ? v : JSON.stringify(v));

/** Eingabe als Wert mit Prüfung beim Tippen. Zahlen nehmen Komma und Punkt; Ja/Nein-Werte `true`/`false` (auch ja/nein);
 *  Listen und Objekte müssen gültiges JSON sein, und zwar vom selben Typ wie der Basiswert (Liste bleibt Liste). */
export function parseEntry(text: string, base: unknown): Parsed {
  const t = text.trim();
  switch (valueKind(base)) {
    case 'text':
      return { ok: true, value: text };
    case 'number': {
      const n = Number(t.replace(',', '.'));
      return t !== '' && Number.isFinite(n) ? { ok: true, value: n } : { ok: false, error: 'Zahl erwartet' };
    }
    case 'boolean': {
      const v = t.toLowerCase();
      if (v === 'true' || v === 'ja') return { ok: true, value: true };
      if (v === 'false' || v === 'nein') return { ok: true, value: false };
      return { ok: false, error: 'true oder false erwartet' };
    }
    default:
      try {
        const v = JSON.parse(t) as unknown;
        if (Array.isArray(base) !== Array.isArray(v) || (v === null || typeof v !== 'object')) {
          return { ok: false, error: Array.isArray(base) ? 'Liste erwartet, z. B. [1, 2]' : 'Objekt erwartet' };
        }
        return { ok: true, value: v };
      } catch {
        return { ok: false, error: 'kein gültiges JSON' };
      }
  }
}

/** Schrittweite einer Zahl: ganze Zahlen 1, Dezimalzahlen 0,1 (Umschalt ×10). */
export const stepSize = (base: unknown, big: boolean): number => (Number.isInteger(base) ? 1 : 0.1) * (big ? 10 : 1);

/** Zahl im Feld um einen Schritt ändern; ohne gültige Zahl im Feld gilt `fallback`. Dezimalstellen werden gerundet,
 *  damit 0,1 + 0,2 nicht 0,30000000000000004 ergibt. */
export function stepText(text: string, fallback: number, base: unknown, dir: 1 | -1, big: boolean): string {
  const cur = parseEntry(text, 0);
  const from = cur.ok ? (cur.value as number) : fallback;
  const next = from + dir * stepSize(base, big);
  return String(Math.round(next * 1e6) / 1e6);
}

/** Abweichung des Werts vom Standardwert als Text: Zahlen mit Unterschied und Prozent („+4 · +33 %“), sonst „geändert“. */
export function deviation(base: unknown, value: unknown): string {
  if (JSON.stringify(base) === JSON.stringify(value)) return '';
  if (typeof base !== 'number' || typeof value !== 'number') return 'geändert';
  const d = Math.round((value - base) * 1e6) / 1e6;
  const sign = d > 0 ? '+' : d < 0 ? '−' : '';
  const diff = `${sign}${Math.abs(d)}`;
  if (base === 0) return diff;
  const pct = Math.round(((value - base) / Math.abs(base)) * 100);
  return `${diff} · ${pct > 0 ? '+' : pct < 0 ? '−' : ''}${Math.abs(pct)} %`;
}
