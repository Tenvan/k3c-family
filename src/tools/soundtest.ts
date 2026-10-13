import { clientLog } from '../core/clientLog';
import { installPageChrome } from '../core/shell';
import { audioCore } from '../audio/audioCore';
import { CROSSFADE_SECONDS } from '../audio/crossfade';
import {
  fileFor, groupCandidates, keyAction, moveSelection, padActions, parseCandidates, stepVolume,
  type Action, type Candidate, type PadState,
} from './soundtestLogic';
import { applyTexts, t } from './texts';

installPageChrome();
applyTexts();
// Kein installPadScroll(): Stick und Steuerkreuz wählen hier den Eintrag, die Auswahl scrollt selbst (scrollIntoView).

const core = audioCore();
const $ = (id: string) => document.getElementById(id) as HTMLElement;
const hint = $('hint');
const listEl = $('list');
const statusEl = $('status');

interface Entry {
  cand: Candidate;
  url?: string;
  failed: boolean;
}

let entries: Entry[] = [];
let selected = 0;
let playing: Entry | undefined;

function say(text: string): void {
  statusEl.textContent = text;
}
const volume = (): number => core.getVolume('music');

function render(): void {
  listEl.replaceChildren();
  let i = 0;
  for (const g of groupCandidates(entries.map((e) => e.cand))) {
    const h = document.createElement('h2');
    h.textContent = `${g.title} `;
    const kind = document.createElement('small');
    kind.textContent = t(g.kind === 'Zustand' ? 'sound.kindState' : 'sound.kindEvent');
    h.append(kind);
    listEl.append(h);
    for (const cand of g.items) {
      const idx = i++;
      const e = entries[idx];
      const div = document.createElement('div');
      div.className = `item${idx === selected ? ' sel' : ''}${playing === e ? ' playing' : ''}${e.failed ? ' off' : ''}`;
      div.dataset.idx = String(idx);
      div.innerHTML = `<span class="mark"></span><span class="name"></span><span class="meta"></span>`;
      (div.querySelector('.name') as HTMLElement).textContent = e.failed ? t('sound.failed', { name: cand.name }) : cand.name;
      (div.querySelector('.meta') as HTMLElement).textContent = t('sound.meta', { source: cand.quelle, license: cand.lizenz });
      div.addEventListener('click', () => {
        selected = idx;
        act('play');
      });
      listEl.append(div);
    }
  }
  listEl.querySelector('.sel')?.scrollIntoView({ block: 'nearest' });
  const state = playing ? t('sound.playing', { name: playing.cand.name }) : t('sound.silence');
  say(t('sound.status', { state, volume: volume() }));
}

async function load(e: Entry): Promise<void> {
  if (e.failed) return;
  if (!e.url || !(await core.preload(e.url))) {
    e.failed = true;
    clientLog('warn', '🔇 Kandidat lädt nicht', { name: e.cand.name, url: e.url ?? t('sound.noFormat') });
    render();
  }
}

/** Musik läuft in Schleife, Effekte einmal; ein Wechsel blendet mit der Kurve des Audio-Kerns über (kein Sprung der Lautstärke). */
async function play(crossfade: boolean): Promise<void> {
  const e = entries[selected];
  if (!e || !core.running) return;
  await load(e);
  if (e.failed || !e.url || playing === e) return;
  if (playing && playing.cand.bus !== e.cand.bus) core.stop(playing.cand.bus); // eine Stimme zur Zeit, wie zuvor
  const ok = await core.playFile(e.url, {
    bus: e.cand.bus,
    fade: crossfade && playing ? CROSSFADE_SECONDS : 0,
    onEnded: () => {
      if (playing === e) { playing = undefined; render(); }
    },
  });
  if (ok) playing = e;
  render();
}

function stop(): void {
  if (playing) core.stop(playing.cand.bus);
  playing = undefined;
  render();
}

function act(a: Action): void {
  if (!core.running) return;
  switch (a) {
    case 'up': case 'down':
      selected = moveSelection(selected, entries.length, a === 'up' ? -1 : 1);
      return render();
    case 'play': return void play(false);
    case 'fade': return void play(true);
    case 'stop': return stop();
    case 'volUp': case 'volDown': {
      const v = stepVolume(volume(), a === 'volUp' ? 1 : -1);
      core.setVolume('music', v);
      core.setVolume('sfx', v);
      return render();
    }
  }
}

/** Erste Eingabe (Taste, Klick/Touch, Controller): Kontext anlegen und fortsetzen; solange er gesperrt bleibt, steht der Hinweis da. */
async function unlock(): Promise<void> {
  const was = core.running;
  await core.onInput(); // legt Kontext und Mixer an, wirft nie
  const running = core.running;
  if (running && !was) void Promise.all(entries.map(load));
  hint.classList.toggle('on', running);
  hint.textContent = t(running ? 'sound.unlocked' : 'sound.locked');
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
      if (acts.length && !core.running) void unlock();
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
