import { describe, expect, it } from 'vitest';

// B-658/AC-01 und B-659/AC-01: Die Container Query der Balancing-Seite fragt einen Vorfahren ab (nie das Element, das
// `container-type` setzt), bricht bei 1.100 px um, und nur Liste und Detail scrollen.

// Vitest ersetzt CSS-Importe (auch `?raw`) durch einen leeren Text, und @types/node fehlt im Frontend: Die Dateien
// kommen deshalb über ein dynamisch geladenes `node:fs`.
const NODE_FS = 'node:fs';
const { readFileSync } = (await import(/* @vite-ignore */ NODE_FS)) as { readFileSync: (url: URL, encoding: 'utf8') => string };
const NAMES = Object.keys(import.meta.glob('../styles/*.css')).map((p) => p.replace('../styles/', ''));
const css = (name: string) => readFileSync(new URL(`../styles/${name}`, import.meta.url), 'utf8');

interface Block {
  head: string;
  body: string;
}

/** Blöcke der obersten Ebene (Regeln und At-Regeln) ohne Kommentare. */
function blocks(source: string): Block[] {
  const text = source.replace(/\/\*[\s\S]*?\*\//g, '');
  const out: Block[] = [];
  let depth = 0;
  let head = '';
  let from = 0;
  let headFrom = 0;
  for (let i = 0; i < text.length; i++) {
    if (text[i] === '{') {
      if (depth === 0) {
        head = text.slice(headFrom, i).trim();
        from = i + 1;
      }
      depth++;
    } else if (text[i] === '}') {
      depth--;
      if (depth === 0) {
        out.push({ head, body: text.slice(from, i) });
        headFrom = i + 1;
      }
    }
  }
  return out;
}

const selectors = (head: string) => head.split(',').map((s) => s.trim()).filter(Boolean);
/** Das Element, auf das ein Selektor zielt: der letzte zusammengesetzte Teil (`.a .b[x]` → `.b[x]`). */
const subject = (selector: string) => /[^\s>+~]+$/.exec(selector)?.[0] ?? selector;

const containerSubjects = (source: string) =>
  blocks(source)
    .filter((b) => !b.head.startsWith('@') && /(^|[;\s])container(-type)?\s*:/.test(b.body))
    .flatMap((b) => selectors(b.head).map(subject));

const queries = (source: string) => blocks(source).filter((b) => b.head.startsWith('@container'));
const condition = (b: Block) => /\(.*\)/.exec(b.head)?.[0] ?? '';

describe('Container Queries (B-658)', () => {
  it('keine Regel einer Container Query zielt auf das Element, das den Container setzt', () => {
    let checked = 0;
    for (const name of NAMES) {
      const containers = containerSubjects(css(name));
      for (const q of queries(css(name))) {
        for (const rule of blocks(q.body)) {
          for (const sel of selectors(rule.head)) {
            expect(containers, `${name}: ${q.head} { ${sel} }`).not.toContain(subject(sel));
            checked++;
          }
        }
      }
    }
    expect(NAMES).toContain('balancing.css');
    expect(checked).toBeGreaterThan(10); // balancing, tasks und planning haben Container Queries
  });

  it('erkennt den Fehler von B-658: Regel auf dem Element, das container-type setzt', () => {
    const bad = '.bal-page { container-type: inline-size; } @container (max-width: 1000px) { .bal-page { grid-template-columns: 1fr; } }';
    const containers = containerSubjects(bad);
    const hits = blocks(queries(bad)[0].body).flatMap((r) => selectors(r.head)).filter((sel) => containers.includes(subject(sel)));
    expect(hits).toEqual(['.bal-page']);
  });

  it('die Balancing-Seite hat einen Container und bricht bei 1.100 px um, unter 760 px zeigt sie Liste oder Detail', () => {
    const source = css('balancing.css');
    expect(containerSubjects(source)).toEqual(['.bal-wrap']);
    const found = queries(source);
    expect(found.map(condition)).toEqual(['(max-width: 1100px)', '(max-width: 760px)']);
    const stacked = blocks(found[0].body).flatMap((r) => selectors(r.head));
    expect(stacked).toContain('.bal-body');
    expect(source).not.toMatch(/max-width:\s*1000px/);
  });
});

describe('Feste Fläche (B-659)', () => {
  const rule = (selector: string) => blocks(css('balancing.css')).find((b) => selectors(b.head).includes(selector))?.body ?? '';

  it('nur Liste und Detail scrollen, die Seite selbst nie', () => {
    expect(rule('.bal-scroll')).toMatch(/overflow:\s*auto/);
    for (const sel of ['.bal-wrap', '.bal-page', '.bal-body', '.bal-pane']) expect(rule(sel), sel).not.toMatch(/overflow:\s*(auto|scroll)/);
    expect(rule('.bal-wrap')).toMatch(/height:\s*calc\(100vh/);
  });

  it('unter 1.100 px behalten Liste und Detail mindestens 160 px Höhe', () => {
    const [stacked] = queries(css('balancing.css'));
    const body = blocks(stacked.body).find((r) => selectors(r.head).includes('.bal-body'))!.body;
    expect(body).toMatch(/grid-template-rows:\s*minmax\(160px,\s*40%\)\s*minmax\(160px,\s*1fr\)/);
  });
});
