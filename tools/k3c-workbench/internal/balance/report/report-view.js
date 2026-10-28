// Balancing-Bericht (B-649), Teil 4: Ansicht. „Was jetzt tun?“-Kasten, einklappbare Abschnitte mit Ein-Zeilen-Zusammenfassung,
// Verweise statt Doppelungen und die Hilfen der Kompakt-Ansicht (?kompakt=1: Balken je Stellschraube, Ausmaß-Fußnote).

const UNKNOWN_NOTE = 'Wie weit ein Schritt das Ziel verschiebt, ist nicht berechenbar; erst der volle Lauf zeigt es. ' +
  'Balken: Strich = Probierbereich, ● aktuell, ◆ Startvorschlag.';

// Plan-Text je Stellschraube: vollständig mit Wirkungssatz, kompakt ohne (die Fußnote steht einmal im Abschnitt).
function planText(p) { return COMPACT ? p.short + ' Ausmaß unbekannt.' : p.text; }

// Balken: Probierbereich min … max, Marke für den aktuellen Wert und den Startvorschlag (nur kompakt, nur mit Bereich).
function rangeBar(p) {
  const lo = +p.min, hi = +p.max, cur = +p.current, start = +p.start;
  if (!COMPACT || !p.known || !(hi > lo)) return null;
  const pos = (x) => 'calc(' + (100 * (x - lo) / (hi - lo)).toFixed(1) + '% - 5px)';
  return el('div', { class: 'rbar', title: 'Probierbereich ' + p.min + ' … ' + p.max + ', aktuell ' + p.current + ', Start ' + p.start },
    el('div', { class: 'rtrack' }), el('span', { class: 'rcur', style: 'left:' + pos(cur) }, '●'),
    el('span', { class: 'rstart', style: 'left:' + pos(start) }, '◆'), el('span', { class: 'rl' }, p.min), el('span', { class: 'rr' }, p.max));
}

function labelOf(t) { return t.grade === 'normal' ? t.name : t.name + ' (' + t.grade + ')'; }

// Hinweis-Satz zum Ziel (früher in der Liste „Hinweise je Ziel“), jetzt in der aufgeklappten Zeile.
function hintLine(t) {
  const h = (R.hints || []).find((x) => x.target === labelOf(t));
  return h && h.text ? el('p', { class: 'why' }, h.text) : null;
}

// Kasten oben: Urteil, die wirksamsten Stellschrauben mit Startwert, ein Knopf für alle Empfehlungen.
function renderTodo() {
  const box = $('todo');
  const c = R.counts || {};
  box.append(el('h2', {}, icon('fire'), 'Was jetzt tun?'));
  const groups = (R.leverGroups || []).slice(0, 3);
  if (!c.red) {
    box.append(el('p', { class: 'todo-line' }, c.yellow ? 'Kein Ziel verletzt, ' + c.yellow + ' knapp: nichts drehen, beim nächsten Tuning im Blick behalten.'
      : 'Alle Ziele im Korridor: nichts zu tun.'));
    return;
  }
  box.append(el('p', { class: 'todo-line' }, c.red + ' von ' + (R.targets || []).length + ' Zielen verletzt.' +
    (groups.length ? ' Diese ' + groups.length + ' Stellschrauben beheben die meisten:' : ' Keine Stellschraube hinterlegt, Ursache im Replay suchen.')));
  box.append(el('ol', { class: 'todo-list' }, groups.map((g) => el('li', {}, el('b', {}, g.plan.label), ' ',
    g.plan.known ? ['auf ', el('b', {}, g.plan.start), ' (aktuell ' + g.plan.current + ')'] : 'von Hand wählen', ' – verbessert ' + g.targets.length + ': ' + g.targets.join('; '),
    g.conflict ? el('span', { class: 'warn' }, ' ⚠ ' + g.conflict) : null))));
  const rec = ((R.next || {}).commands || []).find((x) => x.label.startsWith('Alle Empfehlungen'));
  if (rec) box.append(el('button', { class: 'btn', type: 'button', onclick: () => copy(rec.text) }, 'Alle Empfehlungen kopieren'));
}

// Abschnitt einklappbar machen: Überschrift wird zur Kopfzeile, danach eine Zusammenfassung in einer Zeile.
function fold(card, text, open) {
  const head = card.querySelector('h2');
  if (!head) return;
  const sum = el('summary', {}, head, el('span', { class: 'sum' }, text));
  const d = el('details', { class: 'fold', open });
  d.append(sum, ...card.childNodes);
  card.append(d);
}

// Schwäche anklicken: Zeile in der Tabelle öffnen und dorthin springen (statt dieselbe Aussage zu wiederholen).
function linkWeaknesses() {
  for (const li of document.querySelectorAll('#weaknesses li:not(.empty)')) {
    const i = (R.targets || []).findIndex((t) => li.textContent.startsWith(labelOf(t) + ':'));
    if (i < 0) continue;
    li.classList.add('link');
    li.title = 'Details in der Tabelle der Ziele';
    li.addEventListener('click', () => {
      const row = document.querySelector('#targets tr.row[data-i="' + i + '"]');
      if (!row) return;
      for (const d of [$('targets').closest('details.fold'), $('targets').closest('details.diag')]) if (d) d.open = true;
      row.hidden = false;
      row.nextElementSibling.hidden = false;
      row.scrollIntoView({ block: 'center' });
    });
  }
}

function initView() {
  const c = R.counts || {};
  const f = R.figures || {};
  const lights = f.lights || [];
  const b = R.baseline || {};
  const cards = (id) => $(id).closest('.card');
  linkWeaknesses();
  fold(cards('strengths'), (R.strengths || []).length + ' Stärken', false);
  fold(cards('weaknesses'), (R.weaknesses || []).length + ' Schwächen, Klick springt zur Tabelle', true);
  fold(cards('targets'), (c.red || 0) + ' verletzt · ' + (c.yellow || 0) + ' knapp · ' + (c.green || 0) + ' gut', true);
  fold(cards('charts'), lights.length + ' Kennzahlen, ' + lights.filter((l) => l.light === 'red').length + ' außerhalb des Richtwerts', false);
  fold(cards('hints'), (R.leverGroups || []).length + ' Stellschrauben', true);
  fold(cards('next'), ((R.next || {}).commands || []).length + ' Befehle', true);
  fold(cards('compare'), b.present ? (b.changes || []).length + ' Ziele mit anderem Status' : 'keine Baseline', !!(b.present && (b.changes || []).length));
  const btn = $('view');
  btn.textContent = COMPACT ? 'Ansicht: kompakt ⇄' : 'Ansicht: vollständig ⇄';
  btn.addEventListener('click', () => {
    const u = new URL(location.href);
    if (COMPACT) u.searchParams.delete('kompakt'); else u.searchParams.set('kompakt', '1');
    location.href = u.toString();
  });
}
