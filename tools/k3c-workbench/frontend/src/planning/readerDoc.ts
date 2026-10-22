// Aufbereitung eines Planungsdokuments (Antwort von planningGet, wie plan_get) für den Lesemodus (B-439): Pfad und
// H1 abtrennen, in Rubriken (`## `) zerlegen, abgehakte Kriterien zählen. Kein React, damit Vitest es prüft.

export interface ReaderSection {
  /** Anker-ID; leer beim Vorspann vor der ersten Rubrik. */
  id: string;
  title: string;
  text: string;
}

export interface ReaderDoc {
  /** Pfad aus der ersten Zeile (`docs/…`), sonst leer. */
  path: string;
  sections: ReaderSection[];
  /** Abgehakte und alle Checkboxen (`- [x]`, `- [ ]`) außerhalb von Codeblöcken. */
  done: number;
  total: number;
}

const FENCE = /^\s*(```|~~~)/;

/** Anker-ID einer Rubrik; Umlaute bleiben, alles andere wird zu `-`. */
export function slugify(title: string): string {
  return 'rd-' + title.toLowerCase().replace(/[`*_]/g, '').replace(/[^\p{L}\p{N}]+/gu, '-').replace(/^-+|-+$/g, '');
}

/** Rubrik-Titel wie gerendert: Links, `code` und **fett** als reiner Text. */
const plainTitle = (s: string) => s.replace(/\[([^\]]*)\]\([^)]*\)/g, '$1').replace(/[`*]/g, '').trim();

/** Neue Rubrik; ein doppelter Titel bekommt `-2`, `-3` … an die ID. */
function section(title: string, seen: Map<string, number>): ReaderSection {
  const base = slugify(title);
  const n = (seen.get(base) ?? 0) + 1;
  seen.set(base, n);
  return { id: n > 1 ? `${base}-${n}` : base, title, text: '' };
}

/** Zeilen ohne Pfad (erste Zeile `docs/…`), führende Leerzeilen und H1 (steht schon im Kopf des Dialogs). */
function body(raw: string): { path: string; lines: string[] } {
  const lines = raw.replace(/\r\n/g, '\n').split('\n');
  const path = /^docs\/\S+$/.test(lines[0] ?? '') ? (lines.shift() ?? '') : '';
  while (lines.length && lines[0].trim() === '') lines.shift();
  if (/^#\s/.test(lines[0] ?? '')) lines.shift();
  return { path, lines };
}

export function parseDoc(raw: string): ReaderDoc {
  const { path, lines } = body(raw);
  const sections: ReaderSection[] = [{ id: '', title: '', text: '' }];
  const seen = new Map<string, number>();
  const boxes: string[] = [];
  let fence = false;
  for (const line of lines) {
    if (FENCE.test(line)) fence = !fence;
    const h = fence ? null : /^##\s+(.+?)\s*#*\s*$/.exec(line);
    if (h) {
      sections.push(section(plainTitle(h[1]), seen));
      continue;
    }
    const box = fence ? null : /^\s*[-*]\s+\[([ xX])\]/.exec(line);
    if (box) boxes.push(box[1]);
    sections[sections.length - 1].text += `${line}\n`;
  }
  return { path, sections: sections.filter((s) => s.title || s.text.trim()), done: boxes.filter((b) => b !== ' ').length, total: boxes.length };
}
