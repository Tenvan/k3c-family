// Balancing-Bericht (B-632), Teil 3: Tuning-Hinweise, nächster Lauf, Baseline-Vergleich, Sortierung, Thema, Start.
// Tuning: je Stellschraube einmal mit den Verletzungen, die sie verbessern soll. Die Hinweise je Ziel stehen in der
// aufgeklappten Tabellenzeile (hintLine); nur ohne Stellschrauben (z. B. nur knappe Ziele) bleibt die Liste je Ziel.
function renderHints() {
  const ul = $('hints');
  const hints = R.hints || [];
  const groups = R.leverGroups || [];
  if (!hints.length) ul.append(el('li', { class: 'empty' }, 'Keine Hinweise: alle Ziele im Korridor.'));
  if (COMPACT && groups.length) ul.append(el('li', { class: 'foot-note' }, UNKNOWN_NOTE));
  for (const g of groups) {
    ul.append(el('li', { class: 'hint h-red' }, el('div', {}, el('b', {}, g.direction === '+' ? '▲ ' : '▼ '), g.plan.label, ' ', el('code', {}, g.path)),
      el('div', {}, 'Soll ' + g.targets.length + ' Verletzung(en) verbessern: ' + g.targets.join('; ')),
      el('div', { class: 'why' }, g.plan.short, g.conflict ? ' ' + g.conflict : ''), rangeBar(g.plan)));
  }
  if (groups.length) return;
  for (const h of hints) {
    ul.append(el('li', { class: 'hint h-' + h.light }, el('div', {}, dot(h.light), el('b', {}, h.target), ' – ', h.text),
      h.why ? el('div', { class: 'why' }, h.why) : null, leverList(h.levers, h.plans)));
  }
}

// Wohin ein kopierter Befehl gehört (Command.where, Go: insights_next.go).
const WHERE = {
  chat: ['💬 Chat', 'In den Chat an Claude einfügen: Claude ruft das Werkzeug (sim_test, replay_run) auf und wertet aus.'],
  shell: ['⌨ Terminal', 'In ein Terminal im Checkout einfügen (Ordner mit Taskfile.yml).'],
  workbench: ['🧰 Workbench', 'In der Workbench auf der Balancing-Seite unter „Abstimmen“ so eintragen.'],
};

function cmdRow(c) {
  const [badge, help] = WHERE[c.where] || ['', ''];
  return el('div', {}, el('div', { class: 'note' }, c.label, ' ', badge ? el('span', { class: 'where', title: help }, badge) : null),
    el('div', { class: 'cmd' }, el('pre', {}, c.text), el('button', { class: 'btn small', type: 'button', title: help, onclick: () => copy(c.text) }, 'Kopieren')));
}

// Kompakt: nur „Alle Empfehlungen“ und „Voller Lauf“ offen, die Wege zum Nachspielen und Abstimmen hinter „Weitere Wege“.
const PRIMARY = /^(Alle Empfehlungen|Voller Lauf)/;

function renderNext() {
  const n = R.next || {};
  const box = $('next');
  const cmds = n.commands || [];
  const shown = COMPACT ? cmds.filter((c) => PRIMARY.test(c.label)) : cmds;
  const more = COMPACT ? cmds.filter((c) => !PRIMARY.test(c.label)) : [];
  const help = () => el('p', { class: 'where-help' }, Object.values(WHERE).map(([b, h]) => el('span', {}, el('b', {}, b), ' ', h)));
  if (n.steps && n.steps.length) box.append(el('ol', { class: 'steps' }, n.steps.map((s) => el('li', {}, s))));
  if (cmds.length && !COMPACT) box.append(help());
  for (const c of shown) box.append(cmdRow(c));
  if (more.length) box.append(el('details', { class: 'more' }, el('summary', {}, 'Weitere Wege (' + more.length + '): Replay, Abstimmen'), help(), more.map(cmdRow)));
  if (!box.children.length) box.append(el('p', { class: 'empty' }, 'Kein Vorschlag: der Lauf ist im Korridor.'));
}

function renderCompare() {
  const b = R.baseline || {};
  const box = $('compare');
  if (!b.present) {
    box.append(el('p', { class: 'empty' }, b.note || 'Keine Baseline: kein Trend.'));
    return;
  }
  if (b.note) box.append(el('p', { class: 'note' }, b.note));
  if (!(b.changes || []).length) {
    box.append(el('p', {}, 'Kein Ziel hat gegenüber der Baseline den Status gewechselt.'));
    return;
  }
  const tbl = el('table', { class: 'tbl sortable' },
    el('thead', {}, el('tr', {}, ['Ziel', 'Vorher', 'Nachher', 'Kippt'].map((h) => el('th', {}, h)))),
    el('tbody', {}, b.changes.map((c) => el('tr', {}, el('td', { 'data-v': c.name }, c.name), el('td', { 'data-v': c.before }, c.before),
      el('td', { 'data-v': c.after }, c.after), el('td', { 'data-v': c.tipped ? 1 : 0 }, c.tipped ? '⚠ ja' : 'nein')))));
  box.append(tbl);
}

// Sortierung: Klick auf eine Spaltenüberschrift; Detailzeilen (tr.detail) wandern mit ihrer Zeile.
function sortBy(th) {
  const tbl = th.closest('table'), body = tbl.tBodies[0], col = th.cellIndex;
  const dir = th.dataset.dir === 'asc' ? 'desc' : 'asc';
  for (const h of tbl.tHead.rows[0].cells) delete h.dataset.dir;
  th.dataset.dir = dir;
  const rows = [...body.rows].filter((r) => !r.classList.contains('detail'));
  const val = (r) => { const v = r.cells[col].dataset.v; return v !== undefined && v !== '' && !Number.isNaN(+v) ? +v : (v || '').toLowerCase(); };
  rows.sort((a, b) => { const x = val(a), y = val(b); return (x < y ? -1 : x > y ? 1 : 0) * (dir === 'asc' ? 1 : -1); });
  for (const r of rows) {
    const d = r.nextElementSibling && r.nextElementSibling.classList.contains('detail') ? r.nextElementSibling : null;
    body.append(r);
    if (d) body.append(d);
  }
}

function initSorting() {
  document.addEventListener('click', (e) => {
    const th = e.target.closest('table.sortable th');
    if (th) sortBy(th);
  });
}

function initTheme() {
  const key = 'k3c-balance-theme';
  let theme = 'night';
  try { theme = localStorage.getItem(key) || (matchMedia('(prefers-color-scheme: light)').matches ? 'day' : 'night'); } catch { /* kein Speicher */ }
  document.documentElement.dataset.theme = theme;
  $('theme').addEventListener('click', () => {
    theme = document.documentElement.dataset.theme === 'night' ? 'day' : 'night';
    document.documentElement.dataset.theme = theme;
    try { localStorage.setItem(key, theme); } catch { /* kein Speicher */ }
  });
}

function main() {
  initTheme();
  fillAssets();
  renderHead();
  renderScene();
  renderLights();
  renderBullets('strengths', R.strengths, 'Keine Stärken erkannt.');
  renderBullets('weaknesses', R.weaknesses, 'Keine Schwächen erkannt.');
  renderFilters();
  renderTargets();
  renderFigureLights();
  renderCharts();
  renderNights();
  renderHints();
  renderNext();
  renderCompare();
  initSorting();
  renderTodo();
  initView();
  applyFilter();
}

main();
