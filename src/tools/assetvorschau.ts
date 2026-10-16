/**
 * Asset-Vorschau (B-299, nur Dev-Build): je Kategorie eine Mini-Szene mit den aktiven Zuordnungen aus `data/assets.json`.
 * Aufruf `assetvorschau.html?szene=<kategorie>[&slot=<id>&wahl=<n>]`. Das Register wird importiert, Vite lädt die Seite bei
 * jeder Änderung neu (k3c-dev schreibt es). Grafik im Spielmaßstab, Sounds auf Knopfdruck.
 */
import register from '../../data/assets.json';
import { audioCore } from '../audio/audioCore';
import { CROSSFADE_SECONDS } from '../audio/crossfade';
import { attenuation } from '../audio/events';
import { clientLog } from '../core/clientLog';
import { UNIT_PX as UNIT } from '../core/constants';
import { installPageChrome } from '../core/shell';
import { BIOMES } from '../model/biome';
import {
  baumReihe, figureCard, gebaeudeKarten, GRAFIK_SCALE, hintergrundStreifen, licenseFlag, parseQuery, resolveSlot, SCENES, slotsOf, kachelAnzahl, stufenKarten, vorlageZeilen,
  type AssetSlot, type GebaeudeEintrag, type Query, type Resolved, type SceneId,
} from './assetvorschauLogic';
import { CREDITS } from './credits';
import { renderReference, type Card } from './spriteReference';
import { applyTexts, t, textOf } from './texts';

installPageChrome();
applyTexts();

const SLOTS = register.slots as AssetSlot[];
const core = audioCore();
const $ = (id: string) => document.getElementById(id) as HTMLElement;
const sceneEl = $('scene');
const navEl = $('nav');
const statusEl = $('status');

const sceneTitle = (id: SceneId) => textOf(`av.scene.${id}`, id);

function nav(q: Query): void {
  navEl.replaceChildren(
    ...SCENES.map((s) => {
      const b = document.createElement('button');
      b.type = 'button';
      b.textContent = sceneTitle(s.id);
      b.setAttribute('aria-pressed', String(q.szene === s.id));
      b.addEventListener('click', () => show({ szene: s.id, wahl: 1 }));
      return b;
    }),
  );
}

/** Szene wechseln ohne Seitenwechsel: nur die Adresse dieser Seite und der Inhalt ändern sich. */
function show(q: Query): void {
  const params = new URLSearchParams();
  if (q.szene) params.set('szene', q.szene);
  if (q.slot) params.set('slot', q.slot);
  if (q.slot && q.wahl > 1) params.set('wahl', String(q.wahl));
  history.replaceState(null, '', `?${params}`);
  nav(q);
  sceneEl.replaceChildren();
  if (!q.szene) {
    sceneEl.textContent = t('av.pick');
    return;
  }
  if (q.szene === 'gebaeude') void renderNeueGebaeude();
  const items = slotsOf(SLOTS, q.szene).map((s) => resolveSlot(s, q));
  if (items.length === 0) sceneEl.textContent = t('av.empty');
  else if (SCENES.find((s) => s.id === q.szene)?.art === 'sound') renderSounds(items);
  else renderGraphics(q.szene, items);
}

// ---------------------------------------------------------------- Grafik

/** Rahmen einer Karte: Slot-ID, Beschreibung, Datei, Präferenz und NC-Kennzeichen */
function caption(r: Resolved): HTMLElement {
  const cap = document.createElement('figcaption');
  const flag = r.choice && licenseFlag(r.choice.datei, CREDITS);
  cap.innerHTML = `<b></b> <span class="nc"></span><br><span class="dim"></span>`;
  (cap.querySelector('b') as HTMLElement).textContent = `${r.slot.id} · ${r.slot.beschreibung}`;
  (cap.querySelector('.nc') as HTMLElement).textContent = flag ? t('av.nc') : '';
  const pref = r.pref > 1 ? ` · ${t('av.pref', { n: r.pref })}` : '';
  (cap.querySelector('.dim') as HTMLElement).textContent = r.choice ? `${r.choice.datei}${pref}` : t('av.gap', { id: r.slot.id });
  return cap;
}

function placeholder(text: string): HTMLElement {
  const box = document.createElement('div');
  box.className = 'gap';
  box.textContent = text;
  return box;
}

/** Bild (oder Ausschnitt) im Spielmaßstab auf ein Canvas; lädt es nicht, Platzhalter mit Pfad und Log ❌. */
function imageCard(r: Resolved): HTMLElement {
  const fig = document.createElement('figure');
  fig.append(caption(r));
  if (!r.choice) {
    fig.prepend(placeholder(t('av.gap', { id: r.slot.id })));
    return fig;
  }
  const { datei, ausschnitt } = r.choice;
  const img = new Image();
  img.onload = () => {
    const cut = ausschnitt ?? { x: 0, y: 0, w: img.width, h: img.height };
    const canvas = document.createElement('canvas');
    canvas.width = cut.w * GRAFIK_SCALE;
    canvas.height = cut.h * GRAFIK_SCALE;
    const g = canvas.getContext('2d')!;
    g.imageSmoothingEnabled = false;
    g.drawImage(img, cut.x, cut.y, cut.w, cut.h, 0, 0, canvas.width, canvas.height);
    fig.prepend(canvas);
  };
  img.onerror = () => {
    clientLog('error', '❌ Asset lädt nicht', { slot: r.slot.id, datei });
    fig.prepend(placeholder(t('av.missing', { file: datei })));
  };
  img.src = datei;
  return fig;
}

function renderGraphics(szene: SceneId, items: Resolved[]): void {
  const strip = document.createElement('div');
  strip.className = `strip ${szene}`;
  const figures: Card[] = [];
  for (const r of items) {
    const c = r.choice;
    if (szene === 'figuren' && c?.sheet) figures.push(figureCard(r.slot, { ...c, sheet: c.sheet }));
    else strip.append(imageCard(r));
  }
  if (figures.length > 0) {
    const root = document.createElement('div');
    sceneEl.append(root);
    renderReference(root, [{ title: sceneTitle(szene), cards: figures }], true);
  }
  if (strip.childElementCount > 0) sceneEl.append(strip);
}

/** Streifen „Neue Gebäude (GEB)“ aus `grafik/gebaeude/index.json` vor den Slots: je Gebäude 32 px ×1 und 16 px ×2. */
async function renderNeueGebaeude(): Promise<void> {
  const datei = 'grafik/gebaeude/index.json';
  const res = await fetch(datei).catch(() => null);
  if (!res?.ok) {
    clientLog('error', '❌ Gebäude-Index lädt nicht', { datei });
    return;
  }
  const index = (await res.json()) as GebaeudeEintrag[];
  if (parseQuery(location.search).szene !== 'gebaeude') return; // inzwischen andere Szene gewählt
  const strip = document.createElement('div');
  strip.className = 'strip';
  strip.style.alignItems = 'flex-start'; // Bilder per Rand auf eine Bodenlinie, Vorlagen-Text fließt darunter
  const byId = new Map(index.map((e) => [e.id, e]));
  if (baumReihe(index).length > 0) strip.append(baumFigur(index, 32), baumFigur(index, 16));
  for (const k of gebaeudeKarten(index)) {
    const n = kachelAnzahl(byId.get(k.id)!);
    const fig = bildKarte(k.datei, k.scale, `${k.id} · ${k.dichte} px ×${k.scale}`, index, n);
    strip.append(fig);
    if (k.dichte !== 32) continue;
    vorlageBlock(fig, byId.get(k.id)!);
    for (const s of stufenKarten(byId.get(k.id)!)) {
      const sf = bildKarte(s.datei, 1, `${s.id} · Stufe ${s.stufe} · 32 px ×1`, index, n);
      sf.append(Object.assign(document.createElement('p'), { textContent: s.aenderung, style: 'max-width:10rem;font-size:0.75rem;margin:0.3rem 0 0' }));
      strip.append(sf);
    }
  }
  const title = document.createElement('h2');
  title.textContent = t('av.geb');
  const hg = hintergrundBloecke(index);
  sceneEl.prepend(title, strip, ...(hg ? [Object.assign(document.createElement('h2'), { textContent: t('av.hg') }), hg] : []));
}

/** Streifen „Hintergründe“ (GEB7): je Biom die drei Schichten fern → nah auf der Himmelsfarbe, zwei Kacheln nebeneinander (Naht sichtbar), 32 px ×1 und 16 px ×2. */
function hintergrundBloecke(index: readonly GebaeudeEintrag[]): HTMLElement | null {
  const streifen = hintergrundStreifen(index, BIOMES);
  if (streifen.length === 0) return null;
  const box = document.createElement('div');
  box.className = 'strip';
  box.style.cssText = 'flex-direction:column;align-items:flex-start;overflow-x:auto';
  for (const s of streifen) {
    const row = document.createElement('div');
    row.style.cssText = 'display:flex;gap:1rem;align-items:flex-start';
    for (const [dichte, scale] of [[32, 1], [16, 2]] as const) {
      const fig = document.createElement('figure');
      fig.append(Object.assign(document.createElement('figcaption'), { textContent: `${s.biom} · ${s.schichten.map((l) => l.id).join(' + ')} · ${dichte} px ×${scale}` }));
      const canvas = document.createElement('canvas');
      fig.prepend(canvas);
      void hintergrundZeichnen(canvas, s.himmel, s.schichten.map((l) => l.dateien[dichte]!), scale);
      row.append(fig);
    }
    box.append(row);
  }
  return box;
}

async function hintergrundZeichnen(canvas: HTMLCanvasElement, himmel: string, dateien: string[], scale: number): Promise<void> {
  try {
    const imgs = await Promise.all(dateien.map(ladeBild));
    const w = imgs[0]!.width * scale;
    canvas.width = w * 2;
    canvas.height = imgs[0]!.height * scale;
    const g = canvas.getContext('2d')!;
    g.imageSmoothingEnabled = false;
    g.fillStyle = himmel;
    g.fillRect(0, 0, canvas.width, canvas.height);
    for (const img of imgs) for (let i = 0; i < 2; i++) g.drawImage(img, i * w, 0, w, canvas.height);
  } catch (err) {
    clientLog('error', '❌ Asset lädt nicht', { datei: (err as Error).message });
  }
}

function ladeBild(src: string): Promise<HTMLImageElement> {
  return new Promise((ok, fail) => {
    const img = new Image();
    img.onload = () => ok(img);
    img.onerror = () => fail(new Error(src));
    img.src = src;
  });
}

/** Karte mit einem Bild in ganzzahligem Maßstab auf der Bodenlinie und Beschriftung darunter. */
function bildKarte(datei: string, scale: number, text: string, index: readonly GebaeudeEintrag[], kacheln = 1): HTMLElement {
  const fig = document.createElement('figure');
  fig.append(Object.assign(document.createElement('figcaption'), { textContent: text }));
  const img = new Image();
  img.onload = () => {
    const canvas = document.createElement('canvas');
    canvas.width = img.width * scale * kacheln;
    canvas.height = img.height * scale;
    const g = canvas.getContext('2d')!;
    g.imageSmoothingEnabled = false;
    for (let i = 0; i < kacheln; i++) g.drawImage(img, i * img.width * scale, 0, img.width * scale, canvas.height);
    fig.prepend(aufBodenlinie(canvas, index));
  };
  img.onerror = () => clientLog('error', '❌ Asset lädt nicht', { datei });
  img.src = datei;
  return fig;
}

/** Vorlage unter der Master-Karte: Motiv als Titel, Teile und Zusatzfelder als Liste (GEB8/AC-03). */
function vorlageBlock(fig: HTMLElement, e: GebaeudeEintrag): void {
  const v = vorlageZeilen(e);
  if (!v) return;
  const box = document.createElement('details');
  box.open = true;
  box.style.cssText = 'max-width:16rem;font-size:0.75rem;margin-top:0.3rem';
  const sum = document.createElement('summary');
  sum.textContent = `${e.id}: ${v.titel}`;
  const ul = document.createElement('ul');
  ul.style.cssText = 'margin:0.2rem 0 0;padding-left:1rem';
  for (const z of v.zeilen) ul.append(Object.assign(document.createElement('li'), { textContent: z }));
  box.append(sum, ul);
  fig.append(box);
}

/** Überlappende Baum-Reihe in Dichte d, auf 32 px je Kachel skaliert (GEB1/AC-05). */
function baumFigur(index: readonly GebaeudeEintrag[], d: number): HTMLElement {
  const reihe = baumReihe(index);
  const byId = new Map(index.map((e) => [e.id, e]));
  const fig = document.createElement('figure');
  const cap = document.createElement('figcaption');
  cap.textContent = `${reihe.map((b) => b.id).join(', ')} · ${d} px ×${UNIT / d}`;
  fig.append(cap);
  const hoehe = Math.max(...reihe.map((b) => byId.get(b.id)!.kacheln.h));
  const canvas = document.createElement('canvas');
  canvas.width = (reihe.at(-1)!.x + 1) * UNIT;
  canvas.height = hoehe * UNIT;
  const g = canvas.getContext('2d')!;
  g.imageSmoothingEnabled = false;
  void Promise.all(reihe.map((b) => ladeBild(byId.get(b.id)!.dateien[d]!)))
    .then((imgs) => imgs.forEach((img, i) => {
      const e = byId.get(reihe[i]!.id)!;
      g.drawImage(img, reihe[i]!.x * UNIT, (hoehe - e.kacheln.h) * UNIT, e.kacheln.b * UNIT, e.kacheln.h * UNIT);
    }))
    .catch((err: Error) => clientLog('error', '❌ Asset lädt nicht', { datei: err.message }));
  fig.prepend(aufBodenlinie(canvas, index));
  return fig;
}

/** Rückt ein Bild so weit nach unten, dass seine Unterkante auf der Bodenlinie des höchsten Gebäudes steht. */
function aufBodenlinie(canvas: HTMLCanvasElement, index: readonly GebaeudeEintrag[]): HTMLCanvasElement {
  const boden = Math.max(...index.map((e) => e.kacheln.h)) * UNIT;
  canvas.style.marginTop = `${Math.max(0, boden - canvas.height)}px`;
  return canvas;
}

// ---------------------------------------------------------------- Sound

/** Abstand der anderen Bildhälfte im Split-Screen: Quelle beim zweiten Spieler, gehört vom ersten (wie im Spiel). */
const FAR_GAIN = attenuation(100, [{ center: 20, span: 20 }], false);

async function playSound(r: Resolved, far: boolean): Promise<void> {
  await core.onInput();
  const c = r.choice;
  if (!c || !core.running) return;
  let ok: boolean;
  if (c.sprite) ok = core.play(c.sprite, 'sfx', far ? FAR_GAIN : 1);
  else if (r.slot.bus === 'music') ok = await core.playFile(c.datei, { fade: CROSSFADE_SECONDS });
  else ok = await core.playFile(c.datei, { bus: r.slot.bus === 'ambient' ? 'ambient' : 'sfx', loop: r.slot.bus === 'ambient' });
  if (!ok) clientLog('error', '❌ Sound lädt nicht', { slot: r.slot.id, datei: c.datei });
  statusEl.textContent = ok ? `${r.slot.beschreibung} · ${c.datei}${c.sprite ? `#${c.sprite}` : ''}` : t('av.missing', { file: c.datei });
}

function button(label: string, onClick: () => void): HTMLButtonElement {
  const b = document.createElement('button');
  b.type = 'button';
  b.textContent = label;
  b.addEventListener('click', onClick);
  return b;
}

function renderSounds(items: Resolved[]): void {
  sceneEl.append(
    button(t('av.stop'), () => {
      core.stop('music');
      core.stop('sfx');
      core.stop('ambient');
    }),
  );
  for (const r of items) {
    const row = document.createElement('div');
    row.className = 'sound';
    row.append(caption(r));
    if (r.choice) {
      row.append(button(t('av.play'), () => void playSound(r, false)));
      // Dämpfung wie in der anderen Bildhälfte geht nur mit Atlas-Tönen (eigene Lautstärke je Ton).
      if (r.choice.sprite) row.append(button(t('av.playFar'), () => void playSound(r, true)));
    }
    sceneEl.append(row);
  }
}

addEventListener('pointerdown', () => void core.onInput());
addEventListener('keydown', () => void core.onInput());
show(parseQuery(location.search));
