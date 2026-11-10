/**
 * Papierschnitt-Karten für die Kachelseiten (B-768): gleiche Sprache wie die Grafik des Spiels (`public/grafik/stil.paper.json`):
 * flache Teile aus Papier mit weichem Schlagschatten, Licht von rechts oben, je Teil drei Töne (licht, basis, schatten), leicht
 * unregelmäßig geschnittene Kanten. Reine Geometrie und Farben ohne Phaser; `hubView.ts` zeichnet, `paperBackdrop.ts` den Hintergrund.
 */

/** Drei Töne eines Materials (wie `materialien` in `stil.paper.json`) */
export interface Tone {
  licht: number;
  basis: number;
  schatten: number;
}

/** Materialien aus `stil.paper.json`; ein Test hält sie gleich mit der Datei */
export const MATERIAL = {
  gold: { licht: 0xffe36e, basis: 0xf2c33a, schatten: 0xc48a1e },
  smaragd: { licht: 0x5fd18a, basis: 0x2f9a5c, schatten: 0x1d6640 },
  koenigsblau: { licht: 0x8796f2, basis: 0x4f5fd0, schatten: 0x33368f },
  koenigsrot: { licht: 0xe2574c, basis: 0xb8312f, schatten: 0x7c1f2a },
  purpur: { licht: 0xa865b8, basis: 0x7c3a8a, schatten: 0x5a2466 },
  kristall: { licht: 0xa8e8f2, basis: 0x5fcde4, schatten: 0x2f8fb0 },
  kupfer: { licht: 0xe8a070, basis: 0xc0703c, schatten: 0x8a4a24 },
  stoff: { licht: 0xaaa4b2, basis: 0x857f8e, schatten: 0x625c6b },
  leinen: { licht: 0xf3ead4, basis: 0xe0d2b0, schatten: 0xbba98a },
} as const satisfies Record<string, Tone>;

export type MaterialId = keyof typeof MATERIAL;

/** Linienfarbe und Schattenfarbe des Stils (`einzeltoene.linie`, `papier.lage.farbe`) */
export const LINE = 0x2a1d2e;
export const LINE_TEXT = '#2a1d2e';
export const SHADOW = 0x1a0f14;
/** Schrift auf dem Leinen-Schild, ausgegraut */
export const LINE_TEXT_OFF = '#7a7060';

/** Ton zu einer Grundfarbe aus `MATERIAL` (ein Eintrag der Kacheln); unbekannte Farbe: Stoff */
export function toneOfColor(color: number): Tone {
  return Object.values(MATERIAL).find((m) => m.basis === color) ?? MATERIAL.stoff;
}

export interface Pt {
  x: number;
  y: number;
}

/** Deterministische Zufallszahlen (mulberry32), damit eine Kachel immer gleich geschnitten ist */
export function rng(seed: number): () => number {
  let a = seed >>> 0;
  return () => {
    a = (a + 0x6d2b79f5) >>> 0;
    let t = Math.imul(a ^ (a >>> 15), 1 | a);
    t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
    return (((t ^ (t >>> 14)) >>> 0) % 100000) / 100000;
  };
}

/** Zahl aus einem Text (FNV-1a) als Startwert für `rng` */
export function seedOf(text: string): number {
  let h = 2166136261;
  for (let i = 0; i < text.length; i++) h = Math.imul(h ^ text.charCodeAt(i), 16777619);
  return h >>> 0;
}

/**
 * Rechteck als von Hand geschnittenes Papier: Punkte entlang der Kanten im Abstand von etwa `step`, jeder senkrecht um höchstens
 * `wobble` verschoben, Ecken leicht abgeschrägt (`chamfer`). Alle Punkte liegen innerhalb von `wobble` um das Rechteck.
 */
export function cutPolygon(x: number, y: number, w: number, h: number, seed: number, step = 90, wobble = 2.2, chamfer = 7): Pt[] {
  const rand = rng(seed);
  const c = chamfer;
  // Ecken im Uhrzeigersinn: oben, Ecke, rechts, Ecke, unten, Ecke, links, Ecke; die geraden Kanten (gerader Index) wackeln senkrecht
  const corners: Pt[] = [
    { x: x + c, y },
    { x: x + w - c, y },
    { x: x + w, y: y + c },
    { x: x + w, y: y + h - c },
    { x: x + w - c, y: y + h },
    { x: x + c, y: y + h },
    { x, y: y + h - c },
    { x, y: y + c },
  ];
  const normal: readonly (readonly [number, number])[] = [[0, -1], [0, 0], [1, 0], [0, 0], [0, 1], [0, 0], [-1, 0], [0, 0]];
  const out: Pt[] = [];
  corners.forEach((from, i) => {
    const to = corners[(i + 1) % corners.length]!;
    const [nx, ny] = normal[i]!;
    const parts = nx === 0 && ny === 0 ? 1 : Math.max(1, Math.round(Math.hypot(to.x - from.x, to.y - from.y) / step));
    out.push(from);
    for (let k = 1; k < parts; k++) {
      const t = k / parts;
      const j = (rand() * 2 - 1) * wobble;
      out.push({ x: from.x + (to.x - from.x) * t + nx * j, y: from.y + (to.y - from.y) * t + ny * j });
    }
  });
  return out;
}

/** Punkte verschoben (Schatten) */
export const shifted = (points: readonly Pt[], dx: number, dy: number): Pt[] => points.map((p) => ({ x: p.x + dx, y: p.y + dy }));

/** Schlagschatten wie `papier.lage` (Versatz nach rechts unten, weich): drei Schichten von weit und blass bis nah und dunkel */
export const SHADOW_LAYERS: readonly { d: number; alpha: number }[] = [
  { d: 15, alpha: 0.1 },
  { d: 9, alpha: 0.15 },
  { d: 4, alpha: 0.24 },
];

/** Höhe des Leinen-Schilds unter der Beschriftung: Text plus Rand */
export const STRIP_PAD = 18;
export const STRIP_INSET = 14;
export const stripHeight = (textHeight: number): number => Math.round(textHeight + 2 * STRIP_PAD);
