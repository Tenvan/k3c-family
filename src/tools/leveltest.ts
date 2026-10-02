import { installPageChrome, openPage } from '../core/shell';
import { BIOMES } from '../model/biome';
import { fetchLevel } from './levelApi';
import { levelModel, seedFromBytes, startTarget, type LevelModel } from './levelView';

installPageChrome();

const byId = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const seedInput = byId<HTMLInputElement>('seed');
const biomeSelect = byId<HTMLSelectElement>('biome');
const startButton = byId<HTMLButtonElement>('start');
const note = byId('note');
const startNote = byId('startNote');
const viewport = byId('viewport');
const strip = byId('strip');
const legend = byId('legend');
const warnings = byId('warnings');

const REPLACES = 'Der Start legt ein neues Spiel mit dem Seed als Namen an und ersetzt einen gleichnamigen Spielstand (die Sicherung bleibt).';
const COLORS: Record<string, string> = {
  castle: '#ffd166',
  portal: '#b388ff',
  exit: '#6bd96b',
  tree: '#3d6b3d',
  bush: '#5f8f4f',
  rock: '#8a97aa',
  chest: '#e8a33d',
  skillPoint: '#4dd0e1',
  recruitCamp: '#ff8fa3',
};
const FALLBACK_COLOR = '#c0c8d4';
const BIG_KINDS = new Set(['castle', 'portal', 'exit']);
const SPECIAL_CHUNKS = new Set(['portal', 'exit', 'edge', 'chest', 'recruitCamp']);
const MIN_SCALE = 2;
const MAX_SCALE = 24;

let scale = 6; // Pixel je Unit
let model: LevelModel | null = null;
let request = 0;

for (const b of BIOMES) biomeSelect.add(new Option(`${b.name} (Tiefe ${b.depth})`, b.id));

const colorOf = (kind: string) => COLORS[kind] ?? FALLBACK_COLOR;

function box(className: string, left: number, width?: number): HTMLDivElement {
  const el = document.createElement('div');
  el.className = className;
  el.style.left = `${left}px`;
  if (width !== undefined) el.style.width = `${width}px`;
  return el;
}

function drawChunks(m: LevelModel): void {
  for (const c of m.chunks) {
    const el = box(`chunk${c.kind === 'hub' ? ' hub' : SPECIAL_CHUNKS.has(c.kind) ? ' special' : ''}`, c.from * scale, (c.to - c.from) * scale);
    el.textContent = `${c.index} ${c.kind}`;
    el.title = `Abschnitt ${c.index}: ${c.kind} (${c.from}–${c.to} Units)`;
    strip.append(el);
  }
}

function drawObjects(m: LevelModel): void {
  for (const o of m.objects) {
    const size = BIG_KINDS.has(o.kind) ? 22 : 10;
    const el = box('marker', o.x * scale - size / 2);
    el.style.cssText += `;top:${BIG_KINDS.has(o.kind) ? 0.5 : 3.25}rem;width:${size}px;height:${size}px;background:${colorOf(o.kind)}`;
    el.title = `${o.kind} bei ${o.x} Units`;
    strip.append(el);
  }
}

function drawLevel(m: LevelModel | null): void {
  strip.replaceChildren();
  legend.replaceChildren();
  warnings.replaceChildren();
  if (!m) return;
  strip.style.width = `${m.widthUnits * scale}px`;
  drawChunks(m);
  drawObjects(m);
  const hub = box('hubline', m.hubCenterUnits * scale);
  hub.title = `Hub-Mitte bei ${m.hubCenterUnits} Units`;
  strip.append(hub);
  for (const [kind, count] of m.counts) {
    const item = document.createElement('li');
    item.style.setProperty('--dot', colorOf(kind));
    item.textContent = `${kind} ${count}`;
    legend.append(item);
  }
  for (const text of m.warnings) {
    const item = document.createElement('li');
    item.textContent = text;
    warnings.append(item);
  }
}

/** „Im Spiel starten“ ist nur für startbare Seeds an; sonst steht der Grund daneben. */
function updateStart(): void {
  const target = startTarget(seedInput.value, biomeSelect.value);
  startButton.disabled = 'reason' in target;
  startNote.textContent = 'reason' in target ? `Im Spiel starten nicht möglich: ${target.reason}.` : REPLACES;
}

async function load(): Promise<void> {
  const mine = ++request;
  note.textContent = 'Lade …';
  const result = await fetchLevel(seedInput.value, biomeSelect.value);
  if (mine !== request) return; // eine neuere Anfrage läuft schon
  updateStart();
  if ('error' in result) {
    model = null;
    drawLevel(null);
    note.textContent = result.error;
    return;
  }
  model = levelModel(result.level);
  drawLevel(model);
  const biome = BIOMES.find((b) => b.id === model!.biomeId)?.name ?? model.biomeId;
  note.textContent = `Seed „${model.seed}“ · ${biome} · ${model.widthUnits} Units · ${model.chunks.length} Abschnitte · ${model.objects.length} Objekte`;
}

function zoom(factor: number): void {
  const next = Math.min(MAX_SCALE, Math.max(MIN_SCALE, scale * factor));
  if (next === scale) return;
  const center = (viewport.scrollLeft + viewport.clientWidth / 2) / scale;
  scale = next;
  drawLevel(model);
  viewport.scrollLeft = center * scale - viewport.clientWidth / 2;
}

byId('load').addEventListener('click', () => void load());
byId('random').addEventListener('click', () => {
  seedInput.value = seedFromBytes(crypto.getRandomValues(new Uint8Array(8)));
  void load();
});
startButton.addEventListener('click', () => {
  const target = startTarget(seedInput.value, biomeSelect.value);
  if ('url' in target) openPage(target.url);
});
seedInput.addEventListener('input', updateStart);
seedInput.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') void load();
});
biomeSelect.addEventListener('change', () => void load());

// Tastatur: Pfeile scrollen, + und − zoomen (nicht, solange im Eingabefeld getippt wird).
document.addEventListener('keydown', (e) => {
  if (e.target instanceof HTMLInputElement || e.target instanceof HTMLSelectElement) return;
  if (e.key === 'ArrowLeft') viewport.scrollLeft -= 120;
  else if (e.key === 'ArrowRight') viewport.scrollLeft += 120;
  else if (e.key === '+' || e.key === '=') zoom(1.25);
  else if (e.key === '-') zoom(0.8);
});

// Controller: Steuerkreuz hoch/runter wählt, A klickt, linker Stick scrollt, LB/RB zoomen.
// B bleibt frei (Edge-Zurück), View + Menu übernimmt die Shell.
const PAD = { A: 0, LB: 4, RB: 5, UP: 12, DOWN: 13 } as const;
const STICK = 0.3;
const SCROLL_PER_FRAME = 24;
let held = new Set<string>();

function padKeys(): { keys: Set<string>; x: number } {
  const keys = new Set<string>();
  let x = 0;
  for (const pad of navigator.getGamepads?.() ?? []) {
    if (!pad) continue;
    const on = (i: number) => pad.buttons[i]?.pressed;
    if (on(PAD.A)) keys.add('a');
    if (on(PAD.UP)) keys.add('prev');
    if (on(PAD.DOWN)) keys.add('next');
    if (on(PAD.LB)) keys.add('zoomOut');
    if (on(PAD.RB)) keys.add('zoomIn');
    const stick = pad.axes[0] ?? 0;
    if (Math.abs(stick) > Math.abs(x)) x = stick;
  }
  return { keys, x: Math.abs(x) > STICK ? x : 0 };
}

function pollPad(): void {
  const { keys, x } = padKeys();
  const edge = (name: string) => keys.has(name) && !held.has(name);
  const focusable = [...document.querySelectorAll<HTMLElement>('input, select, button')].filter((el) => !(el as HTMLButtonElement).disabled);
  const at = Math.max(0, focusable.indexOf(document.activeElement as HTMLElement));
  if (edge('next')) focusable[Math.min(focusable.length - 1, at + 1)]?.focus();
  if (edge('prev')) focusable[Math.max(0, at - 1)]?.focus();
  if (edge('a') && document.activeElement instanceof HTMLButtonElement) document.activeElement.click();
  if (edge('zoomIn')) zoom(1.25);
  if (edge('zoomOut')) zoom(0.8);
  if (x) viewport.scrollLeft += x * SCROLL_PER_FRAME;
  held = keys;
  requestAnimationFrame(pollPad);
}

updateStart();
void load();
byId('load').focus();
requestAnimationFrame(pollPad);
