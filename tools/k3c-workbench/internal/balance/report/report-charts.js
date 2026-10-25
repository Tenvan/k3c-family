// Balancing-Bericht (B-632), Teil 2: Kennzahl-Ampeln, SVG-Diagramme je Nacht mit Tooltips, Tabelle je Nacht.
/* eslint-disable no-unused-vars -- die Teile werden zu einem Skript verkettet (report_html.go), genutzt in report-next.js */
const SVGNS = 'http://www.w3.org/2000/svg';
const SERIES = ['#ffd166', '#4dabf7', '#3fb950', '#ff6b6b', '#c084fc', '#f472b6'];
const RES = ['wood', 'stone', 'copper', 'iron', 'crystal'];
const RES_DE = { wood: 'Holz', stone: 'Stein', copper: 'Kupfer', iron: 'Eisen', crystal: 'Kristall' };

function svg(tag, attrs, ...kids) {
  const e = document.createElementNS(SVGNS, tag);
  for (const [k, v] of Object.entries(attrs || {})) e.setAttribute(k, v);
  for (const k of kids) if (k) e.append(k);
  return e;
}

function showTip(evt, html) {
  const tip = $('tip');
  tip.innerHTML = html;
  tip.classList.add('on');
  const x = Math.min(evt.clientX + 14, window.innerWidth - tip.offsetWidth - 8);
  tip.style.left = x + 'px';
  tip.style.top = (evt.clientY + 14) + 'px';
}

function hideTip() { $('tip').classList.remove('on'); }

function esc(s) { return String(s).replace(/[&<>"]/g, (c) => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]); }

// Kurven: series = [{name, color, points:[{x, y, tip}]}], zones = [{from, to, light}] (Richtwert-Bänder).
function lineChart(title, iconKey, unit, series, zones) {
  const W = 300, H = 160, L = 34, B = 22, T = 8;
  const pts = series.flatMap((s) => s.points);
  if (!pts.length) return null;
  const xs = pts.map((p) => p.x), ys = pts.map((p) => p.y).concat(zones.flatMap((z) => [z.from, z.to]).filter(Number.isFinite));
  const x0 = Math.min(...xs), x1 = Math.max(...xs, x0 + 1);
  let y0 = Math.min(0, ...ys), y1 = Math.max(...ys, y0 + 1);
  y1 += (y1 - y0) * 0.08;
  const sx = (x) => L + (W - L - 6) * (x - x0) / (x1 - x0), sy = (y) => T + (H - T - B) * (1 - (y - y0) / (y1 - y0));
  const g = svg('svg', { viewBox: `0 0 ${W} ${H}`, role: 'img', 'aria-label': title });
  for (const z of zones) {
    const a = sy(Math.min(Math.max(z.to, y0), y1)), b = sy(Math.max(Math.min(z.from, y1), y0));
    g.append(svg('rect', { class: 'zone', x: L, y: a, width: W - L - 6, height: Math.max(b - a, 0), fill: `var(--${z.light})` }));
  }
  axes(g, { W, H, L, B, T, x0, x1, y0, y1, sx, sy, unit });
  series.forEach((s) => drawSeries(g, s, sx, sy, unit));
  const box = el('div', { class: 'chart' }, el('h3', {}, icon(iconKey), title), g);
  box.append(el('div', { class: 'legend' }, series.map((s) => el('span', {}, el('i', { style: 'background:' + s.color }), s.name)),
    zones.length ? el('span', {}, el('i', { style: 'background:var(--green);opacity:.4' }), 'Richtwert') : null));
  return box;
}

function axes(g, c) {
  g.append(svg('line', { class: 'axis', x1: c.L, y1: c.H - c.B, x2: c.W - 6, y2: c.H - c.B }));
  g.append(svg('line', { class: 'axis', x1: c.L, y1: c.T, x2: c.L, y2: c.H - c.B }));
  for (let i = 0; i <= 3; i++) {
    const y = c.y0 + (c.y1 - c.y0) * i / 3;
    const t = svg('text', { class: 'lbl', x: c.L - 4, y: c.sy(y) + 3, 'text-anchor': 'end' });
    t.textContent = fmt(y);
    g.append(t, svg('line', { class: 'axis', x1: c.L, y1: c.sy(y), x2: c.W - 6, y2: c.sy(y), 'stroke-dasharray': '2 4' }));
  }
  for (let x = c.x0; x <= c.x1; x++) {
    const t = svg('text', { class: 'lbl', x: c.sx(x), y: c.H - 6, 'text-anchor': 'middle' });
    t.textContent = 'N' + x;
    g.append(t);
  }
}

function drawSeries(g, s, sx, sy, unit) {
  const d = s.points.map((p, i) => (i ? 'L' : 'M') + sx(p.x).toFixed(1) + ' ' + sy(p.y).toFixed(1)).join(' ');
  g.append(svg('path', { d, fill: 'none', stroke: s.color, 'stroke-width': 2.2, 'stroke-linejoin': 'round', 'stroke-dasharray': s.dash || '' }));
  for (const p of s.points) {
    const c = svg('circle', { class: 'pt', cx: sx(p.x), cy: sy(p.y), r: 4, fill: s.color });
    c.addEventListener('mousemove', (e) => showTip(e, `<b>${esc(s.name)}</b> · Nacht ${p.x}<br>${esc(fmt(p.y, unit))}${p.tip ? '<br>' + esc(p.tip) : ''}`));
    c.addEventListener('mouseleave', hideTip);
    g.append(c);
  }
}

function byGrade(nights) {
  const m = new Map();
  for (const n of nights) {
    const k = n.grade || 'normal';
    if (!m.has(k)) m.set(k, []);
    m.get(k).push(n);
  }
  return [...m.entries()];
}

function zonesFor(key) {
  const b = ((R.figures || {}).bands || {})[key]; // Band [von, bis], null = offen
  if (!b) return [];
  return [{ from: b[0] ?? -1e9, to: b[1] ?? 1e9, light: 'green' }];
}

function chartSeries(groups, pick, tip) {
  return groups.map(([grade, list], i) => ({ name: grade, color: SERIES[i % SERIES.length],
    points: list.map((n) => ({ x: n.night, y: pick(n), tip: tip ? tip(n) : '' })) }));
}

function resourceSeries(groups) {
  const nights = groups.length ? groups[0][1] : [];
  return RES.map((r, i) => ({ name: RES_DE[r] + ' (' + groups[0][0] + ')', color: SERIES[i % SERIES.length],
    points: nights.map((n) => ({ x: n.night, y: (n.resourceIn[r] || 0) - (n.resourceOut[r] || 0),
      tip: `rein ${fmt(n.resourceIn[r])} · raus ${fmt(n.resourceOut[r])}` })) })).filter((s) => s.points.some((p) => p.y !== 0));
}

function renderCharts() {
  const f = R.figures || {};
  const nights = f.nights || [];
  $('figure-note').textContent = f.note || '';
  if (!nights.length) return;
  const groups = byGrade(nights);
  const gold = (n) => n.dayIncome - n.dayExpense + n.nightIncome - n.nightExpense;
  const charts = [
    lineChart('Überleben je Nacht', 'castle', '%', chartSeries(groups, (n) => n.survival, (n) => n.runs + ' Läufe'), zonesFor('survival')),
    lineChart('Gold-Bilanz je Tag', 'coin', '', chartSeries(groups, gold,
      (n) => `Tag +${fmt(n.dayIncome)} / −${fmt(n.dayExpense)}, Nacht +${fmt(n.nightIncome)} / −${fmt(n.nightExpense)}`), zonesFor('goldBalance')),
    lineChart('Rohstoff-Bilanz je Tag', 'ore', '', resourceSeries(groups), []),
    lineChart('Leerlauf der Bauern', 'axe', '%', chartSeries(groups, (n) => n.idleShare), zonesFor('idleShare')),
  ];
  $('charts').append(...charts.filter(Boolean));
}

function renderFigureLights() {
  const box = $('figure-lights');
  const lights = (R.figures || {}).lights || [];
  if (!lights.length) box.append(el('span', { class: 'empty' }, 'Keine Kennzahlen im Bericht: Ampeln grau.'));
  for (const l of lights) {
    box.append(el('div', { class: 'fl', title: l.text || '' }, dot(l.light), el('b', {}, l.name), ' ', fmt(l.value, l.unit),
      l.grade ? el('span', { class: 'tag' }, l.grade) : null));
  }
}

function renderNights() {
  const nights = (R.figures || {}).nights || [];
  const tbl = $('nights');
  if (!nights.length) return;
  const cols = [['Grad', (n) => n.grade || 'normal', ''], ['Nacht', (n) => n.night, 'num'], ['Läufe', (n) => n.runs, 'num'],
    ['Überleben %', (n) => n.survival, 'num'], ['Gold Tag', (n) => n.dayIncome - n.dayExpense, 'num'],
    ['Gold Nacht', (n) => n.nightIncome - n.nightExpense, 'num'], ['Holz', (n) => n.resourceIn.wood - n.resourceOut.wood, 'num'],
    ['Stein', (n) => n.resourceIn.stone - n.resourceOut.stone, 'num'], ['Leerlauf %', (n) => n.idleShare, 'num']];
  tbl.append(el('thead', {}, el('tr', {}, cols.map(([h, , c]) => el('th', { class: c }, h)))));
  tbl.append(el('tbody', {}, nights.map((n) => el('tr', {}, cols.map(([, f, c]) => {
    const v = f(n);
    return el('td', { class: c, 'data-v': v }, typeof v === 'number' ? fmt(v) : v);
  })))));
}
