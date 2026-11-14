import type { Box } from '../api';

/** Breite und Höhe aus dem IHDR-Block einer PNG-Data-URL (Bytes 16–23). */
export function pngGroesse(dataUrl: string): { b: number; h: number } {
  const roh = atob(dataUrl.slice(dataUrl.indexOf(',') + 1, dataUrl.indexOf(',') + 1 + 44));
  const zahl = (i: number) => ((roh.charCodeAt(i) << 24) | (roh.charCodeAt(i + 1) << 16) | (roh.charCodeAt(i + 2) << 8) | roh.charCodeAt(i + 3)) >>> 0;
  return { b: zahl(16), h: zahl(20) };
}

const prozent = (v: number, g: number) => `${(v / g) * 100}%`;

/** Rahmen über einem Bild, in Prozent der Frame-Größe: Pixel-Diff (`data-box`) oder geändertes Teil (`data-teil`). */
export function Boxes({ boxen, groesse, art }: { boxen: [string, Box][]; groesse: { b: number; h: number }; art: 'diff' | 'teil' }) {
  return (
    <>
      {boxen.map(([name, [x, y, b, h]]) => (
        <div key={name} {...(art === 'diff' ? { 'data-box': name } : { 'data-teil': name })} title={art === 'teil' ? name : undefined}
          style={{
            position: 'absolute', left: prozent(x, groesse.b), top: prozent(y, groesse.h), width: prozent(b, groesse.b), height: prozent(h, groesse.h),
            border: `2px ${art === 'diff' ? 'solid var(--red-9)' : 'dashed var(--accent-9)'}`, boxSizing: 'border-box', pointerEvents: 'none',
          }} />
      ))}
    </>
  );
}
