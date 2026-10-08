/**
 * Dungeon-Master-Seite `/dm` (B-232): Raumliste, Live-Diagnose eines Raums und Dev-Aktionen vom Handy.
 * Läuft außerhalb der Shell (eigene URL), daher ohne installPageChrome().
 */
import { applyTexts, t } from './texts';
import { DM_ACTIONS, fetchDiagnose, fetchRooms, sendAction, type DmDiagnose, type DmRoom } from './dmApi';

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const POLL_MS = 1000;

let current: string | null = null;
let dev = false;

applyTexts();

function el(tag: string, text: string, cls?: string): HTMLElement {
  const e = document.createElement(tag);
  e.textContent = text;
  if (cls) e.className = cls;
  return e;
}

function setMode(text: string, bad = false): void {
  const m = $('mode');
  m.textContent = text;
  m.className = bad ? 'bad' : 'dim';
}

function renderList(rooms: DmRoom[]): void {
  const list = $('list');
  list.replaceChildren(el('h2', t('dm.rooms')));
  if (rooms.length === 0) list.append(el('p', t('dm.noRoom'), 'dim'));
  const grid = el('div', '', 'grid');
  for (const r of rooms) {
    const b = el('button', `${r.code} · ${r.name} · ${r.taken} 👑${r.running ? '' : t('dm.idle')}`) as HTMLButtonElement;
    b.onclick = () => openRoom(r.code);
    grid.append(b);
  }
  list.append(grid);
}

function row(label: string, value: string): HTMLElement {
  const tr = document.createElement('tr');
  tr.append(el('td', label), el('td', value));
  return tr;
}

function renderDiag(d: DmDiagnose): void {
  $('title').textContent = t('dm.room', { code: d.code });
  const devices = d.devices.map((x) => `${x.id} ${x.connected ? '🔌' : '👋'} [${x.slots.join(',')}]`).join(' · ');
  const stages = d.stages.map((s) => `T${s.depth} ${s.phase} (${s.players.length} 👑)`).join(' · ');
  const troops = Object.entries(d.troops).map(([k, n]) => `${k} ${n}`).join(', ');
  $('diag').replaceChildren(
    row(t('dm.row.tick'), `${d.tick}`),
    row(t('dm.row.time'), t('dm.timeValue', { day: d.day, phase: d.phase, speed: d.paused ? t('dm.pause') : `${d.timescale}×` })),
    row(t('dm.row.wave'), t('dm.waveValue', { wave: d.wave, enemies: d.enemies })),
    row(t('dm.row.castle'), `${Math.round(d.castleHp)} HP`),
    row(t('dm.row.gold'), d.gold.join(' · ') || '–'),
    row(t('dm.row.troops'), troops || '–'),
    row(t('dm.row.devices'), devices || '–'),
    row(t('dm.row.stages'), stages),
  );
  const slot = $<HTMLSelectElement>('slot');
  if (slot.options.length !== d.gold.length) {
    slot.replaceChildren(...d.gold.map((_, i) => new Option(t('dm.monarch', { n: i + 1 }), `${i}`)));
  }
  for (const b of $('actions').querySelectorAll('button')) {
    const a = DM_ACTIONS[Number(b.dataset.i)].action;
    b.classList.toggle('on', a.action === 'timescale' ? a.factor === d.timescale : a.action === 'pause' && a.paused === d.paused);
  }
}

function buildActions(): void {
  $('actions').replaceChildren(
    ...DM_ACTIONS.map((a, i) => {
      const b = el('button', a.label) as HTMLButtonElement;
      b.dataset.i = `${i}`;
      b.onclick = async () => {
        if (!current) return;
        const body = a.needsSlot ? { ...a.action, slot: Number($<HTMLSelectElement>('slot').value) } : a.action;
        const err = await sendAction(current, body);
        $('msg').textContent = err ? `❌ ${err}` : `✅ ${a.label}`;
        void poll();
      };
      return b;
    }),
  );
}

function openRoom(code: string | null): void {
  current = code;
  $('list').hidden = code !== null;
  $('room').hidden = code === null;
  $('msg').textContent = '';
  void poll();
}

async function poll(): Promise<void> {
  if (current === null) {
    const res = await fetchRooms();
    if (!res) return setMode(`❌ ${t('dm.unreachable')}`, true);
    dev = res.dev;
    renderList(res.rooms);
  } else {
    const res = await fetchDiagnose(current);
    if (res === 'gone') return openRoom(null);
    if (!res) return setMode(`❌ ${t('dm.unreachable')}`, true);
    dev = res.dev;
    renderDiag(res.room);
  }
  setMode(t(dev ? 'dm.devOn' : 'dm.devOff'));
  for (const b of $('actions').querySelectorAll('button')) b.disabled = !dev;
}

buildActions();
$('back').onclick = () => openRoom(null);
openRoom(null);
setInterval(() => void poll(), POLL_MS);
