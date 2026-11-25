// Balancing-Bericht (B-632), Teil 1: Hilfen, Kopf, Kurzauswertung, Ampeln, Ziele-Tabelle mit Filtern und Details.
/* eslint-disable no-unused-vars -- die Teile werden zu einem Skript verkettet (report_html.go), genutzt in report-next.js */
// Daten: #report-data (Insights als JSON, Go: internal/balance/insights.go), Grafiken: #report-assets (data-URIs).
const R = JSON.parse(document.getElementById('report-data').textContent);
const A = JSON.parse(document.getElementById('report-assets').textContent || '{}');
const LIGHTS = { green: 'gut', yellow: 'knapp', red: 'verletzt', grey: 'ungültig' };
const $ = (id) => document.getElementById(id);
// Kompakt-Ansicht (B-649): ?kompakt=1 oder data-view="kompakt"; Standard ist die vollständige Ansicht.
const COMPACT = /[?&]kompakt=1/.test(location.search) || document.documentElement.dataset.view === 'kompakt';

function el(tag, attrs, ...kids) {
  const e = document.createElement(tag);
  for (const [k, v] of Object.entries(attrs || {})) {
    if (v === undefined || v === null || v === false) continue;
    if (k === 'class') e.className = v;
    else if (k.startsWith('on')) e.addEventListener(k.slice(2), v);
    else e.setAttribute(k, v === true ? '' : v);
  }
  for (const k of kids.flat()) if (k !== null && k !== undefined && k !== false) e.append(k.nodeType ? k : String(k));
  return e;
}

function fmt(v, unit) {
  if (v === null || v === undefined || Number.isNaN(v)) return '–';
  const n = Math.abs(v) >= 100 ? Math.round(v) : Math.round(v * 10) / 10;
  return n.toLocaleString('de-DE') + (unit ? ' ' + unit : '');
}

function corridor(t) {
  if (t.lower === null || t.lower === undefined) return '≤ ' + fmt(t.upper, t.unit);
  if (t.upper === null || t.upper === undefined) return '≥ ' + fmt(t.lower, t.unit);
  return fmt(t.lower, t.unit) + ' – ' + fmt(t.upper, t.unit);
}

function dot(light) { return el('span', { class: 'dot l-' + light, title: LIGHTS[light] || light }); }

function icon(key, cls) {
  return A[key] ? el('img', { class: cls || 'ico', src: A[key], alt: '' }) : el('span', { class: cls || 'ico' }, '◆');
}

function fillAssets() {
  for (const img of document.querySelectorAll('img[data-asset]')) {
    const src = A[img.dataset.asset];
    if (src) img.src = src;
    else img.replaceWith(el('span', { class: img.className, title: 'Grafik fehlt' }, '◆'));
  }
}

// Szene im Spiel-Look: Burg zwischen Mauern und Türmen, rechts so viele Gegner wie rote Ziele (höchstens 6).
function renderScene() {
  const box = $('scene');
  const img = (key, h, cls) => A[key] ? el('img', { src: A[key], style: `height:${h}px`, class: cls || '', alt: '' }) : null;
  const enemies = Math.min((R.counts || {}).red || 0, 6);
  box.append(img('fire', 34), img('tower', 80), img('wall', 56), img('castle', 92), img('wall', 56), img('tower', 80),
    el('span', { class: 'grow' }), ...Array.from({ length: enemies }, (_, i) => img(i % 2 ? 'enemy2' : 'enemy', 44, 'walk')));
  box.title = enemies ? enemies + ' Gegner = ' + enemies + ' verletzte Ziele' : 'Keine Gegner: kein Ziel verletzt';
}

function toast(text) {
  const t = el('div', { class: 'toast' }, text);
  document.body.append(t);
  setTimeout(() => t.remove(), 1400);
}

function copy(text) {
  const done = () => toast('Kopiert');
  if (navigator.clipboard && window.isSecureContext) navigator.clipboard.writeText(text).then(done, () => copyFallback(text, done));
  else copyFallback(text, done);
}

function copyFallback(text, done) {
  const ta = el('textarea', { style: 'position:fixed;left:-9999px' });
  ta.value = text;
  document.body.append(ta);
  ta.select();
  document.execCommand('copy');
  ta.remove();
  done();
}

function renderHead() {
  document.title = 'K3C · Balancing ' + (R.title || '');
  $('sub').textContent = [R.run, R.generated, R.seeds + ' Seeds', (R.varianten || []).length ? 'Varianten: ' + R.varianten.join(', ') : '']
    .filter(Boolean).join(' · ');
  const v = $('verdict');
  const goal = R.goal && R.goal.total > 0 ? R.goal : null;
  v.textContent = goal ? (goal.reached ? '✔ Ziel erreicht' : '✖ Ziel nicht erreicht') : (R.pass ? '✔ bestanden' : '✖ nicht bestanden');
  v.className = 'verdict ' + (R.pass ? 'pass' : 'fail');
  renderResult();
  const ul = $('summary');
  for (const s of R.summary || []) ul.append(el('li', {}, s));
  $('foot').textContent = 'Erzeugt von k3c-workbench (B-632) · Richtwerte: data/balance-hints.json · Korridore: data/balance-targets.json';
}

// Ergebnis, Meilenstein-Leiter und Blocker (B-648): steht oben, die Diagnose darunter.
function renderResult() {
  if (!R.goal) return;
  $('result').hidden = false;
  $('goal').textContent = (R.goal.reached ? '✅ ' : '❌ ') + (R.goalText || '');
  if (R.quality) $('quality').textContent = 'Qualität: ' + R.quality.passed + ' von ' + R.quality.total + ' Korridoren im Korridor.';
  if (R.progress) renderProgress(R.progress);
}

function configText(c) {
  const opt = (label, v) => (v ? ' · ' + label + ' ' + v : '');
  return 'Welt ' + c.world + ' · Bot ' + c.bot + ' · ' + c.players + ' Spieler · ' + c.days + ' Tage' + opt('Grad', c.grade) + opt('Hilfen', c.helps);
}

function ladderRow(r) {
  const done = r.runs > 0 && r.reached === r.runs;
  const mark = done ? '✔ ' : r.reached ? '◐ ' : '○ ';
  const day = r.medianDay == null ? '–' : 'Tag ' + r.medianDay;
  return el('tr', { class: done ? 'done' : (r.reached ? 'part' : 'open') },
    el('td', { class: 'arrow' }, r.arrow || '·'), el('td', {}, mark + r.label),
    el('td', { class: 'num' }, r.reached + '/' + r.runs), el('td', { class: 'num' }, day));
}

function renderProgress(p) {
  $('cfg').textContent = 'Konfiguration: ' + configText(p.config) + ' – ' + p.testMatch + '.';
  const tbl = $('ladder');
  tbl.append(el('thead', {}, el('tr', {}, ['', 'Meilenstein', 'Seeds', 'Median-Tag'].map((h) => el('th', {}, h)))));
  tbl.append(el('tbody', {}, p.ladder.map(ladderRow)));
  if (R.previous) $('ladder-note').textContent = 'Pfeile (▲ besser, ▼ schlechter, = gleich) gegen den letzten Lauf gleicher Konfiguration: ' + R.previous;
  if (p.blocker) $('blocker').textContent = 'Blocker: ' + p.blocker;
}

function renderLights() {
  const box = $('lights');
  const c = R.counts || {};
  for (const light of ['green', 'yellow', 'red', 'grey']) {
    const b = el('div', { class: 'light-box', 'data-light': light, title: 'Ziele filtern' },
      el('b', { class: 'l-' + light }, c[light] || 0), dot(light), LIGHTS[light]);
    b.addEventListener('click', () => toggleLight(light));
    box.append(b);
  }
}

function renderBullets(id, items, emptyText) {
  const ul = $(id);
  if (!items || !items.length) ul.append(el('li', { class: 'empty' }, emptyText));
  for (const s of items || []) ul.append(el('li', {}, s));
}

// ---------- Ziele: Filter, Tabelle, Details ----------
const filter = { light: '', grade: '', scenario: '', q: '', green: !COMPACT };

function toggleLight(light) {
  filter.light = filter.light === light ? '' : light;
  for (const b of document.querySelectorAll('.light-box')) b.classList.toggle('active', b.dataset.light === filter.light);
  for (const c of document.querySelectorAll('.chip[data-light]')) c.classList.toggle('on', c.dataset.light === filter.light);
  applyFilter();
}

function select(name, label, values) {
  const s = el('select', { 'aria-label': label }, el('option', { value: '' }, label + ': alle'),
    values.map((v) => el('option', { value: v }, v)));
  s.addEventListener('change', () => { filter[name] = s.value; applyFilter(); });
  return s;
}

function renderFilters() {
  const f = $('filters');
  for (const light of ['green', 'yellow', 'red', 'grey']) {
    f.append(el('button', { class: 'chip', type: 'button', 'data-light': light, onclick: () => toggleLight(light) }, dot(light), LIGHTS[light]));
  }
  const uniq = (key) => [...new Set((R.targets || []).map((t) => t[key]))].sort();
  if (COMPACT && (R.counts || {}).green) {
    f.append(el('button', { class: 'chip', type: 'button', onclick: (e) => { filter.green = !filter.green; e.currentTarget.classList.toggle('on', filter.green); applyFilter(); } },
      dot('green'), (R.counts.green) + ' im Korridor anzeigen'));
  }
  f.append(select('grade', 'Grad', uniq('grade')), select('scenario', 'Szenario', uniq('scenario')));
  const q = el('input', { type: 'search', placeholder: 'Suche …', 'aria-label': 'Ziele durchsuchen' });
  q.addEventListener('input', () => { filter.q = q.value.toLowerCase(); applyFilter(); });
  f.append(q);
}

function applyFilter() {
  for (const tr of document.querySelectorAll('#targets tr.row')) {
    const t = R.targets[+tr.dataset.i];
    const ok = (filter.green || t.light !== 'green' || filter.light === 'green') && (!filter.light || t.light === filter.light) && (!filter.grade || t.grade === filter.grade) &&
      (!filter.scenario || t.scenario === filter.scenario) && (!filter.q || (t.name + ' ' + t.measure).toLowerCase().includes(filter.q));
    tr.hidden = !ok;
    if (!ok) tr.nextElementSibling.hidden = true;
  }
}

function corridorBar(t) {
  const lo = t.lower ?? Math.min(0, t.value), hi = t.upper ?? Math.max(t.value, (t.lower || 0) * 2, 1);
  const span = Math.max(hi - lo, 1e-9);
  const min = Math.min(lo - span * 0.5, t.value, t.trend ? t.trend.before : t.value);
  const max = Math.max(hi + span * 0.5, t.value, t.trend ? t.trend.before : t.value);
  const pos = (x) => (100 * (x - min) / (max - min || 1)).toFixed(1) + '%';
  const bar = el('div', { class: 'bar', title: 'Korridor ' + corridor(t) },
    el('div', { class: 'band', style: `left:${pos(t.lower ?? min)};right:calc(100% - ${pos(t.upper ?? max)})` }),
    el('div', { class: 'mark', style: `left:calc(${pos(t.value)} - 2px)` }));
  if (t.trend) bar.append(el('div', { class: 'base', style: `left:${pos(t.trend.before)}`, title: 'Baseline ' + fmt(t.trend.before, t.unit) }));
  return bar;
}

function trendCell(t) {
  if (!t.trend) return el('span', { class: 'trend same' }, '–');
  const d = t.trend.delta, better = t.trend.better;
  const cls = Math.abs(d) < 1e-9 ? 'same' : better ? 'up' : 'down';
  return el('span', { class: 'trend ' + cls, title: 'Baseline: ' + fmt(t.trend.before, t.unit) + ' (' + t.trend.beforeStatus + ')' },
    (d > 0 ? '▲ +' : d < 0 ? '▼ ' : '● ') + fmt(d, t.unit === '%' ? 'Pkt.' : t.unit));
}

function targetRow(t, i) {
  const dist = t.gap || '–';
  return el('tr', { class: 'row', 'data-i': i, title: 'Details auf-/zuklappen' },
    el('td', { 'data-v': ['green', 'yellow', 'red', 'grey'].indexOf(t.light) }, dot(t.light), t.status),
    el('td', { 'data-v': t.name }, t.name, ' ', el('span', { class: 'tag' }, t.grade)),
    el('td', { 'data-v': t.scenario }, el('span', { class: 'tag' }, t.scenario)),
    el('td', { class: 'num', 'data-v': t.value }, fmt(t.value, t.unit)),
    el('td', { class: 'num', 'data-v': t.lower ?? -1e9 }, corridor(t)),
    el('td', { 'data-v': t.value }, corridorBar(t)),
    el('td', { class: 'num', 'data-v': t.distance ?? 0, title: 'Abstand zum Korridor: außerhalb in Prozentpunkten (Pkt.) bzw. in der Einheit des Werts bis zur verletzten Grenze, im Korridor die Reserve bis zur nächsten Grenze' }, dist),
    el('td', { class: 'num', 'data-v': t.trend ? t.trend.delta : 0 }, trendCell(t)));
}

function leverList(levers, plans) {
  if (!levers || !levers.length) return el('p', { class: 'empty' }, 'Keine Hebel hinterlegt.');
  return el('div', {}, levers.map((l, i) => el('div', { class: 'lever' },
    el('b', { class: l.direction === '+' ? 'l-green' : 'l-red' }, l.direction === '+' ? '▲' : '▼'),
    el('span', {}, l.label, ' ', el('code', {}, l.path), plans && plans[i] ? el('div', { class: 'why' }, planText(plans[i])) : null, plans && plans[i] ? rangeBar(plans[i]) : null))));
}

function targetDetail(t) {
  const seeds = (list, label) => list && list.length
    ? el('div', {}, el('b', {}, label + ' (' + list.length + ')'), el('div', { class: 'seeds' }, list.map((s) => el('span', { class: 'seed' }, s))))
    : null;
  return el('tr', { class: 'detail', hidden: true }, el('td', { colspan: 8 }, el('div', { class: 'detail-grid' },
    el('div', {}, el('p', { class: 'why' }, t.plain, t.trendText ? ' ' + t.trendText : ''),
      el('div', {}, el('span', { class: 'tag' }, t.measure), ' ', el('span', { class: 'tag' }, t.kind === 'share' ? 'Anteil' : 'Median'),
      ' ', el('code', {}, t.rule)),
    hintLine(t), seeds(t.failSeeds, 'Seeds zum Nachspielen'), seeds(t.invalidSeeds, 'Ungültige Seeds'),
    t.failSeeds && t.failSeeds.length ? el('button', { class: 'btn small', type: 'button', onclick: () => copy(t.failSeeds.join(',')) }, 'Seeds kopieren') : null),
    el('div', {}, el('b', {}, 'Hebel'), leverList(t.levers, t.plans)))));
}

function renderTargets() {
  const tbl = $('targets');
  const heads = [['Ampel', ''], ['Ziel', ''], ['Szenario', ''], ['Wert', 'num'], ['Korridor', 'num'], ['Lage', ''], ['Abstand', 'num'], ['Trend', 'num']];
  tbl.append(el('thead', {}, el('tr', {}, heads.map(([h, c]) => el('th', { class: c }, h)))));
  const body = el('tbody');
  (R.targets || []).forEach((t, i) => body.append(targetRow(t, i), targetDetail(t)));
  body.addEventListener('click', (e) => {
    const tr = e.target.closest('tr.row');
    if (tr && !e.target.closest('button')) tr.nextElementSibling.hidden = !tr.nextElementSibling.hidden;
  });
  tbl.append(body);
}
