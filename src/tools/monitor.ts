/**
 * Monitoring-Seite (B-282): Ampel je Raum, Verläufe mit Perzentilen und Fehler-Zeitleiste aus `/api/metrics`.
 * Pollt nur, solange die Seite sichtbar ist; zeichnet nur nach einer Antwort oder Bedienung neu.
 */
import { installPageChrome } from '../core/shell';
import { currentLanguage } from '../core/texts';
import { fetchMetrics, nextDelay } from './monitorApi';
import { drawChart, PALETTE, type ChartLine, type ChartMark } from './monitorChart';
import {
  applyDelta,
  emptyState,
  HOUR_MS,
  inWindow,
  nextOf,
  outlierLimit,
  roomSummaries,
  stats,
  SUMMARY_WINDOW_MS,
  windowValues,
  type DiagEvent,
  type EventKind,
  type Stats,
} from './monitorData';
import { nextFocus } from './testTiles';
import { applyTexts, t } from './texts';

installPageChrome();
applyTexts();

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
const TOKEN_KEY = 'k3c-monitor-token';
const VIEWS = { overview: 'overview', history: 'history', events: 'events-view' } as const;
type View = keyof typeof VIEWS;
const KINDS: Record<EventKind, string> = {
  crash: t('monitor.kind.crash'), slow: t('monitor.kind.slow'), drop: t('monitor.kind.drop'), client: t('monitor.kind.client'),
};
const KIND_COLOR: Record<EventKind, string> = { crash: '#ef476f', slow: '#ffd166', drop: '#8a97aa', client: '#b388ff' };

let state = emptyState();
let view: View = 'overview';
let windowMs = SUMMARY_WINDOW_MS;
let cursor: number | undefined;
/** Filter: '' = alle. */
let historyRoom = '';
let evRoom = '';
let evKind = '';
let failures = 0;
let offline = false;
let timer = 0;
let busy = false;

const fmt = (v: number) => v.toLocaleString(currentLanguage(), { maximumFractionDigits: 1 });
const clock = (t: number) => new Date(t).toLocaleTimeString(currentLanguage());

function loadToken(): string {
  try {
    return localStorage.getItem(TOKEN_KEY) ?? '';
  } catch {
    return '';
  }
}

function saveToken(token: string): void {
  try {
    localStorage.setItem(TOKEN_KEY, token);
  } catch {
    // ohne Speicher gilt das Token nur bis zum Neuladen
  }
}

let token = loadToken();

function el(tag: string, text = '', cls = ''): HTMLElement {
  const e = document.createElement(tag);
  e.textContent = text;
  if (cls) e.className = cls;
  return e;
}

function setStatus(text: string, bad = false): void {
  const s = $('status');
  s.textContent = text;
  s.className = bad ? 'bad' : 'dim';
}

// ---------- Ansichten ----------

function showView(next: View): void {
  view = next;
  for (const [v, id] of Object.entries(VIEWS)) $(id).hidden = v !== view;
  for (const b of $('tabs').querySelectorAll<HTMLButtonElement>('button')) b.classList.toggle('on', b.dataset.view === view);
  render();
}

function renderOverview(): void {
  const box = $('overview');
  const rooms = roomSummaries(state);
  box.replaceChildren(el('h2', t('monitor.rooms')));
  if (rooms.length === 0) box.append(el('p', t('monitor.noRoom'), 'dim'));
  const grid = el('div', '', 'rooms');
  for (const r of rooms) {
    const b = el('button') as HTMLButtonElement;
    b.type = 'button';
    const rtt = r.worstRtt === null ? '–' : r.worstRtt < 0 ? t('monitor.noPong') : `${fmt(r.worstRtt)} ms`;
    const tick = r.tickP99 === null ? '–' : `${fmt(r.tickP99)} ms`;
    b.append(el('span', '', `light ${offline ? 'grey' : r.light}`), el('strong', r.code), el('br'));
    b.append(el('span', t('monitor.roomLine', { tick, rtt, errors: r.errors }), 'dim'));
    b.onclick = () => {
      historyRoom = r.code;
      showView('history');
    };
    grid.append(b);
  }
  box.append(grid);
  const last = state.server[state.server.length - 1];
  if (last) {
    const cpu = last.cpu < 0 ? '–' : `${fmt(last.cpu)} %`;
    box.append(el('h2', 'Server'), el('p', t('monitor.serverLine', { heap: fmt(last.heapMB), cpu, goroutines: last.goroutines }), 'dim'));
  }
}

function statsRow(label: string, s: Stats | null, unit: string): HTMLElement {
  const tr = el('tr');
  const cell = (v: number | undefined) => el('td', v === undefined ? '–' : `${fmt(v)} ${unit}`);
  tr.append(el('td', label), cell(s?.p50), cell(s?.p95), cell(s?.p99), cell(s?.max));
  return tr;
}

function marks(room: string): ChartMark[] {
  const events = inWindow(state.events, state.now, windowMs).filter((e) => !room || e.room === room || !e.room);
  return [
    ...events.map((e) => ({ t: e.t, color: KIND_COLOR[e.kind] })),
    ...state.restarts.map((at) => ({ t: at, color: '#e8edf4', label: t('monitor.restart') })),
  ];
}

function historyLines(room: string): { tick: ChartLine[]; rtt: ChartLine[]; server: ChartLine[] } {
  const win = <T extends { t: number }>(pts: T[]) => inWindow(pts, state.now, windowMs);
  const tick = Object.entries(state.rooms)
    .filter(([code]) => !room || code === room)
    .map(([code, pts], i) => ({ label: code, color: PALETTE[i % PALETTE.length], points: win(pts).map((p) => ({ t: p.t, v: p.maxMs })) }));
  const rtt = Object.entries(state.devices)
    .filter(([, d]) => !room || d.room === room)
    .map(([id, d], i) => ({
      label: id,
      color: PALETTE[i % PALETTE.length],
      points: win(d.points).filter((p) => p.rttMs >= 0).map((p) => ({ t: p.t, v: p.rttMs })),
    }));
  const srv = win(state.server);
  const server = [
    { label: t('monitor.heapMB'), color: PALETTE[0], points: srv.map((p) => ({ t: p.t, v: p.heapMB })) },
    { label: 'CPU %', color: PALETTE[1], points: srv.filter((p) => p.cpu >= 0).map((p) => ({ t: p.t, v: p.cpu })) },
  ];
  return { tick, rtt, server };
}

/** Raum-Codes mit Punkten oder Ereignissen, sortiert, mit '' = alle vorn. */
function roomChoices(): string[] {
  return ['', ...[...new Set([...Object.keys(state.rooms), ...state.events.map((e) => e.room)])].filter(Boolean).sort()];
}

const roomLabel = (room: string) => t('monitor.roomFilter', { room: room || t('monitor.all') });

function renderHistory(): void {
  const room = historyRoom;
  $('room').textContent = roomLabel(room);
  $('window').textContent = t(windowMs === HOUR_MS ? 'monitor.window1h' : 'monitor.window5m');
  const v = windowValues(state, windowMs, room);
  const head = el('tr');
  head.append(...['', 'p50', 'p95', 'p99', 'Max'].map((h) => el('th', h)));
  const over = el('tr');
  over.append(el('td', t('monitor.overBudget')), el('td', t('monitor.ticks', { n: v.over })));
  over.lastElementChild?.setAttribute('colspan', '4');
  $('stats').replaceChildren(head, statsRow('Tick', stats(v.tick), 'ms'), statsRow('RTT', stats(v.rtt), 'ms'), over);
  const lines = historyLines(room);
  const base = { from: state.now - windowMs, to: state.now, marks: marks(room), cursor };
  drawChart($('c-tick'), { ...base, lines: lines.tick, unit: 'ms', outlier: outlierLimit(v.tick) });
  drawChart($('c-rtt'), { ...base, lines: lines.rtt, unit: 'ms', outlier: outlierLimit(v.rtt) });
  drawChart($('c-server'), { ...base, lines: lines.server, unit: '' });
}

function jumpTo(e: DiagEvent): void {
  cursor = e.t;
  windowMs = state.now - e.t > SUMMARY_WINDOW_MS ? HOUR_MS : SUMMARY_WINDOW_MS;
  historyRoom = e.room;
  showView('history');
}

function renderEvents(): void {
  const room = evRoom;
  const kind = evKind;
  $('ev-room').textContent = roomLabel(room);
  $('ev-kind').textContent = t('monitor.kindFilter', { kind: kind ? KINDS[kind as EventKind] : t('monitor.all') });
  const list = state.events.filter((e) => (!room || e.room === room) && (!kind || e.kind === kind)).reverse();
  const box = $('events');
  box.replaceChildren(...(list.length === 0 ? [el('p', t('monitor.noEvents'), 'dim')] : []));
  for (const e of list) {
    const b = el('button', `${clock(e.t)} · ${KINDS[e.kind] ?? e.kind}${e.room ? t('monitor.eventRoom', { room: e.room }) : ''} · ${e.text}`);
    (b as HTMLButtonElement).type = 'button';
    b.onclick = () => jumpTo(e);
    box.append(b);
  }
}

function render(): void {
  if (view === 'overview') renderOverview();
  else if (view === 'history') renderHistory();
  else renderEvents();
}

// ---------- Abfrage ----------

function schedule(): void {
  clearTimeout(timer);
  timer = 0;
  if (!document.hidden && token) timer = window.setTimeout(() => void poll(), nextDelay(failures));
}

function askToken(text: string): void {
  setStatus(text, true);
  $('login').hidden = false;
  $('token').focus();
}

async function poll(): Promise<void> {
  timer = 0;
  if (!token) return askToken(t('monitor.tokenAsk'));
  busy = true;
  const res = await fetchMetrics(token, state.now);
  busy = false;
  if (res.kind === 'unauthorized') return askToken(t('monitor.tokenBad'));
  offline = res.kind === 'offline';
  if (res.kind === 'ok') {
    state = applyDelta(state, res.data);
    failures = 0;
    setStatus(t('monitor.ok', { now: clock(state.now), since: clock(state.startedAt) }));
  } else {
    failures++;
    const wait = Math.round(nextDelay(failures) / 1000);
    if (res.kind === 'off') setStatus(t('monitor.off'), true);
    else setStatus(t('monitor.offline', { wait }), true);
  }
  render();
  schedule();
}

// ---------- Bedienung ----------

$('login').addEventListener('submit', (ev) => {
  ev.preventDefault();
  token = $<HTMLInputElement>('token').value.trim();
  if (!token) return;
  saveToken(token);
  $('login').hidden = true;
  failures = 0;
  void poll();
});
for (const b of $('tabs').querySelectorAll<HTMLButtonElement>('button')) b.onclick = () => showView(b.dataset.view as View);
$('window').onclick = () => {
  windowMs = windowMs === HOUR_MS ? SUMMARY_WINDOW_MS : HOUR_MS;
  render();
};
$('room').onclick = () => {
  historyRoom = nextOf(roomChoices(), historyRoom);
  render();
};
$('ev-room').onclick = () => {
  evRoom = nextOf(roomChoices(), evRoom);
  render();
};
$('ev-kind').onclick = () => {
  evKind = nextOf(['', ...Object.keys(KINDS)], evKind);
  render();
};
document.addEventListener('visibilitychange', () => {
  if (!document.hidden && !timer && !busy) void poll();
});
addEventListener('resize', () => render());

// Controller: Steuerkreuz/Stick wandert über die sichtbaren Bedienelemente, A klickt (B bleibt frei).
const PAD = { A: 0, UP: 12, DOWN: 13, LEFT: 14, RIGHT: 15 };
let held = new Set<string>();

function padKeys(): Set<string> {
  const keys = new Set<string>();
  for (const pad of navigator.getGamepads?.() ?? []) {
    if (!pad) continue;
    const on = (i: number) => pad.buttons[i]?.pressed;
    const [x = 0, y = 0] = pad.axes;
    if (on(PAD.A)) keys.add('a');
    if (on(PAD.LEFT) || on(PAD.UP) || Math.min(x, y) < -0.5) keys.add('prev');
    if (on(PAD.RIGHT) || on(PAD.DOWN) || Math.max(x, y) > 0.5) keys.add('next');
  }
  return keys;
}

function pollPad(): void {
  const next = padKeys();
  const edge = (k: string) => next.has(k) && !held.has(k);
  if (next.size > 0) {
    const items = [...document.querySelectorAll<HTMLElement>('main button, main select, main input')].filter((e) => e.offsetParent);
    const at = items.indexOf(document.activeElement as HTMLElement);
    if (edge('next')) items[nextFocus(items.length, at, 'next')]?.focus();
    if (edge('prev')) items[nextFocus(items.length, at, 'prev')]?.focus();
    if (edge('a')) (document.activeElement as HTMLElement | null)?.click();
  }
  held = next;
  requestAnimationFrame(pollPad);
}

requestAnimationFrame(pollPad);
showView('overview');
void poll();
