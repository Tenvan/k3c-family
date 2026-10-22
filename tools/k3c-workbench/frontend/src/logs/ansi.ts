// ANSI-Farben der Konsole (B-064 › Konsole): SGR-Sequenzen zerlegen, 16 Standardfarben als Klassen, 256 Farben und
// RGB als Stil. Andere Steuersequenzen (Cursor, Zeile löschen) fallen weg. Der Text bleibt Text, nie HTML.

export interface Span {
  text: string;
  className: string;
  style?: { color?: string; backgroundColor?: string };
}

/** Eine Farbe: Index 0–15 (Klasse) oder ein fertiger CSS-Wert (256 Farben ab 16, RGB). */
type Color = number | string | null;

interface Pen {
  fg: Color;
  bg: Color;
  bold: boolean;
  dim: boolean;
}

const PLAIN: Pen = { fg: null, bg: null, bold: false, dim: false };
// eslint-disable-next-line no-control-regex -- ESC ist hier gewollt
const CSI = /\u001b\[([0-9;?]*)([A-Za-z])/g;
// eslint-disable-next-line no-control-regex -- übrige Steuerzeichen außer Tab
const STRAY = /[\u0000-\u0008\u000b-\u001f\u007f]/g;

const byte = (n: number | undefined) => Math.min(255, Math.max(0, Math.trunc(n ?? 0)));

/** Farbe aus der 256er-Palette: 0–15 als Index, 16–231 Würfel, 232–255 Grautöne. */
export function color256(n: number): Color {
  const i = byte(n);
  if (i < 16) return i;
  if (i >= 232) {
    const g = 8 + (i - 232) * 10;
    return `rgb(${g}, ${g}, ${g})`;
  }
  const c = i - 16;
  const level = (v: number) => (v === 0 ? 0 : 55 + v * 40);
  return `rgb(${level(Math.floor(c / 36))}, ${level(Math.floor(c / 6) % 6)}, ${level(c % 6)})`;
}

/** Erweiterte Farbe ab codes[i] (38 oder 48): liefert die Farbe und wie viele Codes sie verbraucht. */
function extended(codes: number[], i: number): [Color, number] {
  if (codes[i + 1] === 5) return [color256(codes[i + 2]), 2];
  if (codes[i + 1] === 2) return [`rgb(${byte(codes[i + 2])}, ${byte(codes[i + 3])}, ${byte(codes[i + 4])})`, 4];
  return [null, 0];
}

/** Ein einfacher SGR-Code; null, wenn er unbekannt ist. */
function simple(pen: Pen, c: number): Pen | null {
  if (c === 0) return PLAIN;
  if (c === 1) return { ...pen, bold: true };
  if (c === 2) return { ...pen, dim: true };
  if (c === 22) return { ...pen, bold: false, dim: false };
  if (c === 39) return { ...pen, fg: null };
  if (c === 49) return { ...pen, bg: null };
  if (c >= 30 && c <= 37) return { ...pen, fg: c - 30 };
  if (c >= 90 && c <= 97) return { ...pen, fg: c - 90 + 8 };
  if (c >= 40 && c <= 47) return { ...pen, bg: c - 40 };
  if (c >= 100 && c <= 107) return { ...pen, bg: c - 100 + 8 };
  return null;
}

/** Wendet die Parameter einer SGR-Sequenz („1;31“) auf den Stift an. */
export function applySgr(pen: Pen, params: string): Pen {
  const codes = params === '' ? [0] : params.split(';').map((p) => (p === '' ? 0 : Number(p)));
  let next = pen;
  for (let i = 0; i < codes.length; i++) {
    const c = codes[i];
    if (c === 38 || c === 48) {
      const [col, used] = extended(codes, i);
      next = c === 38 ? { ...next, fg: col } : { ...next, bg: col };
      i += used;
      continue;
    }
    next = simple(next, c) ?? next;
  }
  return next;
}

function span(text: string, pen: Pen): Span {
  const cls: string[] = [];
  const style: Span['style'] = {};
  if (typeof pen.fg === 'number') cls.push(`ansi-fg-${pen.fg}`);
  else if (pen.fg) style.color = pen.fg;
  if (typeof pen.bg === 'number') cls.push(`ansi-bg-${pen.bg}`);
  else if (pen.bg) style.backgroundColor = pen.bg;
  if (pen.bold) cls.push('ansi-bold');
  if (pen.dim) cls.push('ansi-dim');
  return { text, className: cls.join(' '), ...(style.color || style.backgroundColor ? { style } : {}) };
}

/** Zerlegt eine Zeile in Abschnitte mit gleicher Farbe; leere Abschnitte fallen weg. */
export function parseAnsi(line: string): Span[] {
  const spans: Span[] = [];
  let pen = PLAIN;
  let last = 0;
  const push = (text: string) => {
    const clean = text.replace(STRAY, '');
    if (clean) spans.push(span(clean, pen));
  };
  for (const m of line.matchAll(CSI)) {
    push(line.slice(last, m.index));
    if (m[2] === 'm') pen = applySgr(pen, m[1]);
    last = m.index + m[0].length;
  }
  push(line.slice(last));
  return spans;
}

/** Zeile ohne Steuersequenzen, z. B. für die Suche nach WARN und ERROR. */
export function stripAnsi(line: string): string {
  return line.replace(CSI, '').replace(STRAY, '');
}
