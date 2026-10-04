import { clientLog } from '../core/clientLog';
import { installPageChrome } from '../core/shell';
import { Mixer, type AudioContextLike } from '../audio/mixer';
import {
  crossfadeCurves, fileFor, groupCandidates, keyAction, moveSelection, padActions, parseCandidates, stepVolume,
  type Action, type Candidate, type PadState,
} from './soundtestLogic';

installPageChrome();
// Kein installPadScroll(): Stick und Steuerkreuz wählen hier den Eintrag, die Auswahl scrollt selbst (scrollIntoView).

const FADE_SECONDS = 2;
const $ = (id: string) => document.getElementById(id) as HTMLElement;
const hint = $('hint');
const listEl = $('list');
const statusEl = $('status');

interface Entry {
  cand: Candidate;
  url?: string;
  buffer?: AudioBuffer;
  failed: boolean;
}
interface Voice {
  entry: Entry;
  src: AudioBufferSourceNode;
  gain: GainNode;
}

let entries: Entry[] = [];
let selected = 0;
let ctx: AudioContext | undefined;
let mixer: Mixer | undefined;
let voice: Voice | undefined;

function say(text: string): void {
  statusEl.textContent = text;
}
const volume = (): number => mixer?.getVolume('music') ?? 100;

function render(): void {
  listEl.replaceChildren();
  let i = 0;
  for (const g of groupCandidates(entries.map((e) => e.cand))) {
    const h = document.createElement('h2');
    h.textContent = `${g.title} `;
    const kind = document.createElement('small');
    kind.textContent = g.kind;
    h.append(kind);
    listEl.append(h);
    for (const cand of g.items) {
      const idx = i++;
      const e = entries[idx];
      const div = document.createElement('div');
      div.className = `item${idx === selected ? ' sel' : ''}${voice?.entry === e ? ' playing' : ''}${e.failed ? ' off' : ''}`;
      div.dataset.idx = String(idx);
      div.innerHTML = `<span class="mark"></span><span class="name"></span><span class="meta"></span>`;
      (div.querySelector('.name') as HTMLElement).textContent = e.failed ? `${cand.name} (lädt nicht)` : cand.name;
      (div.querySelector('.meta') as HTMLElement).textContent = `Quelle: ${cand.quelle} · Lizenz: ${cand.lizenz}`;
      div.addEventListener('click', () => {
        selected = idx;
        act('play');
      });
      listEl.append(div);
    }
  }
  listEl.querySelector('.sel')?.scrollIntoView({ block: 'nearest' });
  say(`${voice ? `Spielt: ${voice.entry.cand.name}` : 'Stille'} · Lautstärke ${volume()} %`);
}

async function load(e: Entry): Promise<void> {
  if (e.buffer || e.failed || !ctx) return;
  try {
    if (!e.url) throw new Error('kein abspielbares Format');
    const res = await fetch(e.url);
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    e.buffer = await ctx.decodeAudioData(await res.arrayBuffer());
  } catch (err) {
    e.failed = true;
    clientLog('warn', '🔇 Kandidat lädt nicht', { name: e.cand.name, error: String(err) });
    render();
  }
}

/** Startet `e` mit eigener Gain-Stufe am Bus; `fadeIn` > 0 blendet mit der Kurve ein, sonst sofort. Musik läuft in Schleife. */
function start(e: Entry, fadeIn: number): Voice | undefined {
  if (!ctx || !mixer || !e.buffer) return undefined;
  const src = ctx.createBufferSource();
  src.buffer = e.buffer;
  src.loop = e.cand.bus === 'music';
  const gain = ctx.createGain();
  if (fadeIn > 0) gain.gain.setValueCurveAtTime(crossfadeCurves().in, ctx.currentTime, fadeIn);
  src.connect(gain);
  gain.connect(mixer.node(e.cand.bus) as unknown as AudioNode);
  src.start();
  return { entry: e, src, gain };
}

/** Altes Stück über die Kurve ausblenden und danach beenden (kein Sprung der Lautstärke). */
function fadeOut(v: Voice, seconds: number): void {
  if (!ctx) return;
  v.gain.gain.setValueCurveAtTime(crossfadeCurves().out, ctx.currentTime, seconds);
  v.src.stop(ctx.currentTime + seconds);
}

async function play(crossfade: boolean): Promise<void> {
  const e = entries[selected];
  if (!e || !ctx) return;
  await load(e);
  if (e.failed || voice?.entry === e) return;
  const old = voice;
  const seconds = crossfade && old ? FADE_SECONDS : 0;
  voice = start(e, seconds);
  if (old) seconds ? fadeOut(old, seconds) : fadeOut(old, 0.03);
  render();
  const v = voice;
  if (v && e.cand.bus === 'sfx') v.src.onended = () => { if (voice === v) { voice = undefined; render(); } };
}

function stop(): void {
  if (voice) fadeOut(voice, 0.03);
  voice = undefined;
  render();
}

function act(a: Action): void {
  if (!ctx) return;
  switch (a) {
    case 'up': case 'down':
      selected = moveSelection(selected, entries.length, a === 'up' ? -1 : 1);
      return render();
    case 'play': return void play(false);
    case 'fade': return void play(true);
    case 'stop': return stop();
    case 'volUp': case 'volDown': {
      const v = stepVolume(volume(), a === 'volUp' ? 1 : -1);
      mixer?.setVolume('music', v);
      mixer?.setVolume('sfx', v);
      return render();
    }
  }
}

/** Erste Eingabe (Taste, Klick/Touch, Controller): Kontext anlegen und fortsetzen; solange er gesperrt bleibt, steht der Hinweis da. */
async function unlock(): Promise<void> {
  try {
    if (!ctx) {
      ctx = new AudioContext();
      mixer = new Mixer(ctx as unknown as AudioContextLike, window.localStorage);
      void Promise.all(entries.map(load));
    }
    await ctx.resume();
  } catch (e) {
    clientLog('warn', '🔇 Audio nicht startbar', { error: String(e) });
  }
  const running = ctx?.state === 'running';
  hint.classList.toggle('on', running);
  hint.textContent = running ? 'Audio entsperrt' : 'Audio gesperrt: Taste drücken, Bildschirm antippen oder A am Controller';
}

function padState(p: Gamepad): PadState {
  return { buttons: p.buttons.map((b) => b.pressed), stickY: p.axes[1] ?? 0 };
}

function pollPads(): void {
  const prev = new Map<number, PadState>();
  const tick = () => {
    for (const p of typeof navigator.getGamepads === 'function' ? navigator.getGamepads() : []) {
      if (!p) continue;
      const cur = padState(p);
      const acts = padActions(prev.get(p.index) ?? { buttons: [], stickY: 0 }, cur);
      if (acts.length && !ctx?.state.startsWith('run')) void unlock();
      for (const a of acts) act(a);
      prev.set(p.index, cur);
    }
    requestAnimationFrame(tick);
  };
  requestAnimationFrame(tick);
}

async function init(): Promise<void> {
  const raw = await fetch('audio/kandidaten.json').then((r) => r.json()).catch(() => []);
  const canPlay = (mime: string) => new Audio().canPlayType(mime) !== '';
  const cands = parseCandidates(raw);
  // gleiche Reihenfolge wie die Anzeige (nach Gruppen), damit Index und Zeile übereinstimmen
  entries = groupCandidates(cands).flatMap((g) => g.items).map((cand) => ({ cand, url: fileFor(cand, canPlay), failed: false }));
  render();
}

addEventListener('keydown', (ev) => {
  const a = keyAction(ev.code);
  if (a) ev.preventDefault();
  void unlock().then(() => a && act(a));
});
addEventListener('pointerdown', () => void unlock());
$('b-stop').addEventListener('click', () => act('stop'));
$('b-fade').addEventListener('click', () => act('fade'));
$('b-vold').addEventListener('click', () => act('volDown'));
$('b-volu').addEventListener('click', () => act('volUp'));
pollPads();
void init();
