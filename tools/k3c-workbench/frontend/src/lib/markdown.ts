// Markdown-Teilmenge für den Instructions-Dialog (B-065) und die Planungs-Dokumente: Überschriften, Listen (auch nummeriert, Häkchen ☐/☑), Tabellen,
// Absätze, Codeblöcke, `code`, **fett** und [Links] (nur als Text, die Seite navigiert nicht). Ergibt einen Baum, den React als Elemente rendert, nie einen HTML-String.

export interface Inline {
  kind: 'text' | 'code' | 'bold';
  text: string;
}

export type Block =
  | { kind: 'h'; level: 1 | 2 | 3 | 4; inline: Inline[] }
  | { kind: 'table'; head: Inline[][]; rows: Inline[][][] }
  | { kind: 'p'; inline: Inline[] }
  | { kind: 'ul'; items: Inline[][] }
  | { kind: 'ol'; items: Inline[][] }
  | { kind: 'pre'; text: string };

const INLINE = /`([^`]+)`|\*\*([^*]+)\*\*|\[([^\]]+)\]\([^)]*\)/g;
const HEADING = /^(#{1,4})\s+(.*)$/;
const ROW = /^\s*\|(.*)\|\s*$/;
const RULE = /^[\s|:-]+$/; // Trennzeile einer Tabelle
const ITEM = /^\s*[-*]\s+(.*)$/;
const NUMBERED = /^(\d+)\.\s+(.*)$/; // nur am Zeilenanfang, nicht verschachtelt
const CHECK = /^\[([ xX])\]\s/;

/** Zerlegt eine Zeile in Text, `code` und **fett**. */
export function parseInline(text: string): Inline[] {
  const out: Inline[] = [];
  let last = 0;
  for (const m of text.matchAll(INLINE)) {
    if (m.index > last) out.push({ kind: 'text', text: text.slice(last, m.index) });
    if (m[1] !== undefined) out.push({ kind: 'code', text: m[1] });
    else if (m[2] !== undefined) out.push({ kind: 'bold', text: m[2] });
    else out.push({ kind: 'text', text: m[3] });
    last = m.index + m[0].length;
  }
  if (last < text.length) out.push({ kind: 'text', text: text.slice(last) });
  return out;
}

interface State {
  blocks: Block[];
  para: string[]; // Zeilen des offenen Absatzes
  items: string[]; // Punkte der offenen Liste
  list: 'ul' | 'ol'; // Art der offenen Liste
  rows: string[][]; // Zeilen der offenen Tabelle, die erste ist der Kopf
}

function flush(st: State) {
  if (st.para.length) st.blocks.push({ kind: 'p', inline: parseInline(st.para.join(' ')) });
  if (st.items.length) st.blocks.push({ kind: st.list, items: st.items.map(parseInline) });
  if (st.rows.length) {
    const [head, ...rows] = st.rows;
    st.blocks.push({ kind: 'table', head: head.map(parseInline), rows: rows.map((r) => r.map(parseInline)) });
  }
  st.para = [];
  st.items = [];
  st.rows = [];
}

/** `[ ] ` bzw. `[x] ` am Anfang eines Listenpunkts wird ☐ bzw. ☑. */
function check(text: string): string {
  return text.replace(CHECK, (_, x: string) => (x === ' ' ? '☐ ' : '☑ '));
}

/** Listenpunkt der Zeile mit Art; eine Zahl mitten in einem Absatz beginnt nur mit `1.` eine Liste (wie CommonMark). */
function listItem(st: State, raw: string): { list: 'ul' | 'ol'; text: string } | null {
  const ul = ITEM.exec(raw);
  if (ul) return { list: 'ul', text: ul[1] };
  const ol = NUMBERED.exec(raw);
  if (!ol || (st.para.length && ol[1] !== '1')) return null;
  return { list: 'ol', text: ol[2] };
}

/** Eine Zeile außerhalb eines Codeblocks. */
function line(st: State, raw: string) {
  const heading = HEADING.exec(raw);
  const item = listItem(st, raw);
  const row = ROW.exec(raw);
  if (row) {
    if (!st.rows.length) flush(st);
    if (!RULE.test(row[1])) st.rows.push(row[1].split('|').map((c) => c.trim()));
  } else if (st.rows.length) {
    flush(st); // die Tabelle endet vor der ersten Zeile, die keine Zeile ist
    line(st, raw);
  } else if (raw.trim() === '') {
    flush(st);
  } else if (heading) {
    flush(st);
    st.blocks.push({ kind: 'h', level: heading[1].length as 1 | 2 | 3 | 4, inline: parseInline(heading[2]) });
  } else if (item) {
    if (st.para.length || (st.items.length && st.list !== item.list)) flush(st);
    st.list = item.list;
    st.items.push(check(item.text));
  } else if (st.items.length && /^\s+/.test(raw)) {
    st.items[st.items.length - 1] += ` ${raw.trim()}`; // eingerückte Folgezeile eines Punkts
  } else {
    if (st.items.length) flush(st);
    st.para.push(raw.trim());
  }
}

/** Zerlegt den Text in Blöcke. */
export function parseMarkdown(src: string): Block[] {
  const st: State = { blocks: [], para: [], items: [], list: 'ul', rows: [] };
  let code: string[] | null = null;
  for (const raw of src.replace(/\r\n/g, '\n').split('\n')) {
    if (raw.trimStart().startsWith('```')) {
      if (code) {
        st.blocks.push({ kind: 'pre', text: code.join('\n') });
        code = null;
      } else {
        flush(st);
        code = [];
      }
    } else if (code) {
      code.push(raw);
    } else {
      line(st, raw);
    }
  }
  if (code) st.blocks.push({ kind: 'pre', text: code.join('\n') });
  flush(st);
  return st.blocks;
}
