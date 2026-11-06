// Hash-Regel der Papierschnitt-Grafik (docs/standards/paper-spec.md › Hash und Golden): specHash und stilHash über die
// kanonische JSON-Form, renderKey als Cache- und Golden-Schlüssel. Zeilenenden und Schlüsselreihenfolge ändern nichts.
import { createHash } from 'node:crypto';

/** Version von Renderer und resvg; jede Änderung, die Pixel ändern kann, erhöht sie (RD1). */
export const RENDERER_VERSION = 'paper-0+resvg-js-2.6.2';

/** Kanonische JSON-Form: Schlüssel sortiert (Codepunkte), ohne Leerraum, CRLF in Texten als LF. */
export function canonical(wert: unknown): string {
  if (typeof wert === 'string') return JSON.stringify(wert.replace(/\r\n/g, '\n'));
  if (Array.isArray(wert)) return `[${wert.map(canonical).join(',')}]`;
  if (wert && typeof wert === 'object') {
    const keys = Object.keys(wert).sort();
    return `{${keys.map((k) => `${JSON.stringify(k)}:${canonical((wert as Record<string, unknown>)[k])}`).join(',')}}`;
  }
  return JSON.stringify(wert);
}

const sha256 = (text: string) => createHash('sha256').update(text, 'utf8').digest('hex');

/** SHA-256 der kanonischen Form; Text wird als JSON gelesen. `$schema` zählt nicht mit. */
export function specHash(spec: unknown): string {
  const wert = typeof spec === 'string' ? JSON.parse(spec) : spec;
  if (wert && typeof wert === 'object' && !Array.isArray(wert)) {
    const { $schema: _ignoriert, ...rest } = wert as Record<string, unknown>;
    return sha256(canonical(rest));
  }
  return sha256(canonical(wert));
}

/** Wie specHash, für die Stil-Datei. */
export const stilHash = specHash;

/** Cache- und Golden-Schlüssel: SHA-256 über `specHash:stilHash:rendererVersion`. */
export function renderKey(spec: string, stil: string, renderer = RENDERER_VERSION): string {
  return sha256(`${spec}:${stil}:${renderer}`);
}

/** Koordinate für die SVG-Ausgabe: höchstens 2 Nachkommastellen, ohne Nullen am Ende, nie `-0`. */
export function num(v: number): string {
  const r = Math.round(v * 100) / 100;
  return String(Object.is(r, -0) ? 0 : r);
}
