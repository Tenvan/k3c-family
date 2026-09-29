/**
 * Referenzseiten für die Figuren (figuren.html = alle Figuren, aufstellung.html = Rollen im Spiel).
 * Liest alles aus src/data/sprites.json und zeichnet die Sprites auf einfache Canvas, ohne Phaser.
 * Jede Karte spielt im Wechsel: Laufen rechts, Angriff, Stehen, Laufen links, Angriff, Stehen.
 */
import enemiesJson from '../data/enemies.json';
import spritesJson from '../data/sprites.json';
import troopsJson from '../data/troops.json';

type AnimName = 'idle' | 'run' | 'attack';

interface Sheet {
  label: string;
  side: 'ours' | 'enemy' | 'mount';
  frameWidth: number;
  frameHeight: number;
  originX: number;
  originY: number;
  height: number;
  scale: number;
  anims: Partial<Record<AnimName, { frames: number; fps: number }>>;
  pack: string;
  source: string;
}

interface Spec {
  sheet: string;
  tint?: string;
  scale?: number;
  alpha?: number;
}

/** Reittier: Tier-Sheet plus Sattelpunkt pro Frame (Frame-Pixel, Blick nach rechts) */
interface Mount {
  label: string;
  sheet: string;
  scale: number;
  tint?: string;
  saddle: Partial<Record<AnimName, [number, number][]>>;
}

const DATA = spritesJson as unknown as {
  sheets: Record<string, Sheet & { authors?: string; license?: string }>;
  players: Spec[];
  troops: Record<string, Spec>;
  enemies: Record<string, Spec>;
  mounts: Record<string, Mount>;
  riderWaist: number;
};
/** Reiter auf den Referenzseiten */
const RIDER: Spec = { sheet: 'medieval-king' };
const TROOP_NAMES = troopsJson as Record<string, { name: string }>;
const ENEMY_NAMES = enemiesJson as Record<string, { name: string; depth: number }>;

export interface Card {
  title: string;
  note: string;
  spec: Spec;
  /** Reittier (Schlüssel in sprites.json "mounts"), spec ist dann der Reiter */
  mount?: string;
}

export interface Group {
  title: string;
  cards: Card[];
  /** Figuren einzeln einpassen statt im gemeinsamen Maßstab */
  fit?: boolean;
}

function mountCards(): Card[] {
  return Object.entries(DATA.mounts).map(([id, m]) => {
    const s = DATA.sheets[m.sheet];
    return { title: m.label, note: `${s.pack} · ${s.license ?? 'CC0'}${s.authors ? ` · ${s.authors}` : ''}`, spec: RIDER, mount: id };
  });
}

const sheetNote = (s: Sheet) => `${s.pack}`;
const tintNote = (spec: Spec) => (spec.tint ? ` · eingefärbt ${spec.tint}` : '') + (spec.alpha !== undefined ? ' · durchscheinend' : '');

/** Alle Figuren beider Sets, nach Seite getrennt. */
export function allFigures(): Group[] {
  const used = usedSheets();
  const cards = (side: Sheet['side']) =>
    Object.entries(DATA.sheets)
      .filter(([, s]) => s.side === side)
      .map(([id, s]) => ({ title: s.label, note: `${sheetNote(s)}${used.has(id) ? '' : ' · noch ohne Rolle'}`, spec: { sheet: id } }));
  return [
    { title: 'Unsere Seite', cards: cards('ours') },
    { title: 'Gegenseite', cards: cards('enemy') },
    { title: 'Reittiere (mit König 1)', cards: mountCards() },
  ];
}

/** Jede Rolle im Spiel mit dem Sprite aus sprites.json, danach die Figuren ohne Rolle. */
export function lineup(): Group[] {
  const card = (title: string, spec: Spec): Card => ({ title, note: `${DATA.sheets[spec.sheet].label} · ${sheetNote(DATA.sheets[spec.sheet])}${tintNote(spec)}`, spec });
  const used = usedSheets();
  return [
    { title: 'Monarchen', cards: DATA.players.map((s, i) => card(`Spieler ${i + 1}`, s)) },
    { title: 'Truppen', cards: Object.entries(DATA.troops).map(([k, s]) => card(TROOP_NAMES[k]?.name ?? k, s)) },
    { title: 'Gegner', cards: Object.entries(DATA.enemies).map(([k, s]) => card(`${ENEMY_NAMES[k]?.name ?? k} · Tiefe ${ENEMY_NAMES[k]?.depth ?? '?'}`, s)) },
    { title: 'Reittiere (noch nicht im Spiel)', cards: mountCards(), fit: true },
    {
      title: 'Reserve (noch ohne Rolle)',
      fit: true,
      cards: Object.keys(DATA.sheets)
        .filter((id) => !used.has(id) && DATA.sheets[id].side !== 'mount')
        .map((id) => card(DATA.sheets[id].label, { sheet: id })),
    },
  ];
}

function usedSheets(): Set<string> {
  return new Set([...DATA.players, ...Object.values(DATA.troops), ...Object.values(DATA.enemies)].map((s) => s.sheet));
}

// ---------------------------------------------------------------- Zeichnen

const images = new Map<string, Promise<HTMLImageElement>>();
function loadImage(src: string): Promise<HTMLImageElement> {
  let p = images.get(src);
  if (!p) {
    p = new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = reject;
      img.src = src;
    });
    images.set(src, p);
  }
  return p;
}

/** Streifen laden und wie Phaser (Multiplikation) einfärben. */
async function strip(sheet: string, anim: AnimName, tint?: string): Promise<CanvasImageSource> {
  const img = await loadImage(`sprites/${sheet}/${anim}.png`);
  if (!tint) return img;
  const c = document.createElement('canvas');
  c.width = img.width;
  c.height = img.height;
  const g = c.getContext('2d')!;
  g.drawImage(img, 0, 0);
  g.globalCompositeOperation = 'multiply';
  g.fillStyle = tint;
  g.fillRect(0, 0, c.width, c.height);
  g.globalCompositeOperation = 'destination-in';
  g.drawImage(img, 0, 0);
  return c;
}

const PHASES: { anim: AnimName; dir: 1 | -1; ms: number }[] = [
  { anim: 'run', dir: 1, ms: 1600 },
  { anim: 'attack', dir: 1, ms: 0 },
  { anim: 'idle', dir: 1, ms: 1000 },
  { anim: 'run', dir: -1, ms: 1600 },
  { anim: 'attack', dir: -1, ms: 0 },
  { anim: 'idle', dir: -1, ms: 1000 },
];

interface Live {
  canvas: HTMLCanvasElement;
  sheet: Sheet;
  spec: Spec;
  strips: Partial<Record<AnimName, CanvasImageSource>>;
  /** Pixel pro Frame-Pixel auf der Karte */
  zoom: number;
  offset: number;
  mount?: { data: Mount; sheet: Sheet; strips: Partial<Record<AnimName, CanvasImageSource>>; zoom: number };
}

const CANVAS_W = 320;
const CANVAS_H = 220;
const GROUND = 200;
/** So weit gleitet eine Figur beim Laufen über die Karte */
const DRIFT = 60;

/**
 * Baut die Seite. sameScale = true: alle Figuren im gleichen Maßstab wie im Spiel (Größen vergleichbar),
 * sonst wird jede Figur einzeln auf die Karte eingepasst.
 */
export function renderReference(root: HTMLElement, groups: Group[], sameScale: boolean): void {
  const lives: Live[] = [];
  const inGame = (spec: Spec) => DATA.sheets[spec.sheet].scale * (spec.scale ?? 1);
  /** Höhe und halbe Breite im Spiel (px), bei Reittieren samt Reiter */
  const size = (c: Card): [number, number] => {
    const s = DATA.sheets[c.spec.sheet];
    if (!c.mount) return [s.originY * s.frameHeight * inGame(c.spec), s.frameWidth * Math.max(s.originX, 1 - s.originX) * inGame(c.spec)];
    const m = DATA.mounts[c.mount];
    const ms = DATA.sheets[m.sheet];
    const top = Math.min(...Object.values(m.saddle).flatMap((pts) => pts.map((p) => p[1])));
    const h = (ms.originY * ms.frameHeight - top) * m.scale + s.height * DATA.riderWaist * inGame(c.spec);
    return [h, ms.frameWidth * Math.max(ms.originX, 1 - ms.originX) * m.scale];
  };
  /** Kartenpixel pro Spielpixel, damit die Figur (samt Gleiten beim Laufen) auf die Karte passt, höchstens 1 */
  const fit = (c: Card) => {
    const [h, w] = size(c);
    return Math.min(1, (GROUND - 10) / h, (CANVAS_W / 2 - DRIFT / 2) / w);
  };
  // gemeinsamer Maßstab: so, dass die größte Figur der gemeinsamen Gruppen auf die Karte passt
  const common = Math.min(1, ...groups.filter((g) => !g.fit).flatMap((g) => g.cards).map(fit));

  for (const group of groups) {
    const section = document.createElement('section');
    section.innerHTML = `<h2>${group.title} <small>${group.cards.length}</small></h2>`;
    const grid = document.createElement('div');
    grid.className = 'grid';
    section.append(grid);
    root.append(section);
    for (const card of group.cards) {
      const s = DATA.sheets[card.spec.sheet];
      const el = document.createElement('article');
      const canvas = document.createElement('canvas');
      canvas.width = CANVAS_W;
      canvas.height = CANVAS_H;
      el.append(canvas);
      const id = card.mount ? DATA.mounts[card.mount].sheet : card.spec.sheet;
      el.insertAdjacentHTML('beforeend', `<h3>${card.title}</h3><p>${card.note}</p><p class="dim">${id} · im Spiel ${Math.round(size(card)[0])} px hoch</p>`);
      grid.append(el);
      const k = sameScale && !group.fit ? common : fit(card);
      const live: Live = { canvas, sheet: s, spec: card.spec, strips: {}, zoom: inGame(card.spec) * k, offset: lives.length * 370 };
      load(live.strips, card.spec.sheet, s, card.spec.tint);
      if (card.mount) {
        const m = DATA.mounts[card.mount];
        live.mount = { data: m, sheet: DATA.sheets[m.sheet], strips: {}, zoom: m.scale * k };
        load(live.mount.strips, m.sheet, live.mount.sheet, m.tint);
      }
      lives.push(live);
    }
  }

  const start = performance.now();
  const frame = (now: number) => {
    for (const l of lives) if (isVisible(l.canvas)) draw(l, now - start + l.offset);
    requestAnimationFrame(frame);
  };
  requestAnimationFrame(frame);
}

function load(into: Live['strips'], id: string, sheet: Sheet, tint?: string): void {
  for (const a of Object.keys(sheet.anims) as AnimName[]) void strip(id, a, tint).then((img) => (into[a] = img));
}

function isVisible(el: Element): boolean {
  const r = el.getBoundingClientRect();
  return r.bottom > 0 && r.top < innerHeight;
}

function phaseDuration(l: Live, i: number): number {
  const p = PHASES[i];
  if (p.ms) return p.ms;
  const a = l.sheet.anims.attack ?? l.sheet.anims.idle!;
  return Math.max(600, (a.frames / a.fps) * 1000 * 2);
}

function frameOf(sheet: Sheet, anim: AnimName, ms: number): [AnimName, number] {
  const name: AnimName = sheet.anims[anim] ? anim : 'idle';
  const a = sheet.anims[name]!;
  return [name, Math.floor((ms / 1000) * a.fps) % a.frames];
}

function draw(l: Live, t: number): void {
  const total = PHASES.reduce((sum, _, i) => sum + phaseDuration(l, i), 0);
  let rest = t % total;
  let i = 0;
  while (rest >= phaseDuration(l, i)) rest -= phaseDuration(l, i++);
  const phase = PHASES[i];

  const g = l.canvas.getContext('2d')!;
  g.clearRect(0, 0, CANVAS_W, CANVAS_H);
  g.fillStyle = '#3a5a40';
  g.fillRect(0, GROUND, CANVAS_W, CANVAS_H - GROUND);
  g.imageSmoothingEnabled = false;
  // beim Laufen ein Stück über die Karte gleiten
  const drift = phase.anim === 'run' ? (rest / phaseDuration(l, i) - 0.5) * DRIFT * phase.dir : 0;
  g.save();
  g.globalAlpha = l.spec.alpha ?? 1;
  g.translate(CANVAS_W / 2 + drift, GROUND);
  if (phase.dir < 0) g.scale(-1, 1);

  const { frameWidth: fw, frameHeight: fh, originX, originY } = l.sheet;
  const z = l.zoom;
  if (!l.mount) {
    const [name, frame] = frameOf(l.sheet, phase.anim, rest);
    const img = l.strips[name];
    if (img) g.drawImage(img, frame * fw, 0, fw, fh, -originX * fw * z, -originY * fh * z, fw * z, fh * z);
    g.restore();
    return;
  }
  // Reittier: Tier läuft oder steht (auch beim Angriff), der Reiter steht oder greift an
  const m = l.mount;
  const [mName, mFrame] = frameOf(m.sheet, phase.anim === 'attack' ? 'idle' : phase.anim, rest);
  const [rName, rFrame] = frameOf(l.sheet, phase.anim === 'run' ? 'idle' : phase.anim, rest);
  const mImg = m.strips[mName];
  const rImg = l.strips[rName];
  const saddle = m.data.saddle[mName]?.[mFrame];
  if (!mImg || !rImg || !saddle) {
    g.restore();
    return;
  }
  const mw = m.sheet.frameWidth;
  const mh = m.sheet.frameHeight;
  const mz = m.zoom;
  const mx = -m.sheet.originX * mw * mz;
  const my = -m.sheet.originY * mh * mz;
  g.drawImage(mImg, mFrame * mw, 0, mw, mh, mx, my, mw * mz, mh * mz);
  // Oberkörper des Reiters: Hüfte auf den Sattelpunkt
  const waist = originY * fh - l.sheet.height * (1 - DATA.riderWaist);
  const sx = mx + saddle[0] * mz;
  const sy = my + saddle[1] * mz;
  g.drawImage(rImg, rFrame * fw, 0, fw, waist, sx - originX * fw * z, sy - waist * z, fw * z, waist * z);
  g.restore();
}

/** Seite mit Stick oder Steuerkreuz scrollen (B bleibt frei). */
export function installPadScroll(): void {
  const tick = () => {
    const pads = typeof navigator.getGamepads === 'function' ? navigator.getGamepads() : [];
    let dy = 0;
    for (const p of pads) {
      if (!p) continue;
      const y = p.axes[1] ?? 0;
      if (Math.abs(y) > 0.2) dy += y * 24;
      if (p.buttons[12]?.pressed) dy -= 16;
      if (p.buttons[13]?.pressed) dy += 16;
    }
    if (dy) scrollBy(0, dy);
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
}
