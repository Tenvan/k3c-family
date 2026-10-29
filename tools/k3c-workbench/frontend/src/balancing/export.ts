import type { BalanceEntry, BalanceSummary } from '../api';
import type { CompareCell, CompareRow, Comparison } from './compare';

export const num = (v: number | null | undefined, digits = 2) => (v === null || v === undefined ? '–' : String(Number(v.toFixed(digits))));

/** Korridor als Text: „0.6 … ∞“, eine fehlende Grenze bleibt offen. */
export function corridorText(e: Pick<BalanceEntry, 'lower' | 'upper'>): string {
  if (e.lower === null && e.upper === null) return 'kein Ziel';
  return `${e.lower === null ? '−∞' : num(e.lower)} … ${e.upper === null ? '∞' : num(e.upper)}`;
}

const cell = (s: string) => s.replace(/\|/g, '\\|');

/** Bericht als Markdown: Kopf, Ziele mit Korridor, Kennzahlen je Nacht. */
export function summaryMarkdown(s: BalanceSummary, title = 'Balancing-Bericht'): string {
  const out = [`# ${title}`, '', `- Ergebnis: ${s.pass ? 'Pass' : 'Fail'}`, `- Seeds: ${s.seeds}`];
  if (s.varianten?.length) out.push(`- Varianten: ${s.varianten.map((v) => `\`${v}\``).join(', ')}`);
  out.push('', '## Ziele', '', '| Ziel | Wert | Korridor | Status |', '| --- | ---: | --- | --- |');
  for (const e of s.targets) out.push(`| ${cell(e.name)} | ${num(e.value)} | ${corridorText(e)} | ${e.status} |`);
  const f = s.kennzahlen;
  if (f) {
    out.push('', '## Kennzahlen', '', `- Gültige Läufe: ${f.runs}`, `- Erste Mauer (Tick, Median): ${num(f.firstWallTickMedian, 0)}`,
      `- Erste Werkstatt (Tick, Median): ${num(f.firstWorkshopTickMedian, 0)}`, '',
      '| Grad | Nacht | Läufe | Überleben | Gold Tag | Gold Nacht | Wartezeit |', '| --- | ---: | ---: | ---: | ---: | ---: | ---: |');
    for (const n of f.nights) {
      out.push(`| ${n.grade} | ${n.night} | ${n.runs} | ${num(n.survival)} | ${num(n.dayIncome - n.dayExpense)} | ` +
        `${num(n.nightIncome - n.nightExpense)} | ${num(n.idleShare)} |`);
    }
  }
  return out.join('\n') + '\n';
}

/** Bericht als eingerücktes JSON mit Zeilenende (wie `Summary.JSON()` in Go). */
export const summaryJSON = (s: BalanceSummary): string => JSON.stringify(s, null, 2) + '\n';

export const signed = (d: number | null) => (d === null ? '–' : (d > 0 ? '+' : '') + num(d));

const status = (c: CompareCell) => (c.status ? ` [${c.status}]` : '');
const cellText = (c: CompareCell, i: number) =>
  i === 0 ? `${num(c.value)}${status(c)}` : `${num(c.value)} (${signed(c.delta)})${status(c)}${c.tipped ? ' (!)' : ''}`;

const rowLine = (r: CompareRow) => `| ${cell(r.label)} | ${r.corridor ?? ''} | ${r.cells.map(cellText).join(' | ')} |`;

const LETTERS = ['A', 'B', 'C', 'D'];

/** Vergleich als Markdown (A = Basis, die übrigen Spalten mit Abstand zu A). */
export function comparisonMarkdown(c: Comparison, names: string[] = []): string {
  const cols = c.passes.map((_, i) => `${LETTERS[i]}: ${cell(names[i] ?? LETTERS[i])}`);
  const head = `| Wert | Korridor | ${cols.join(' | ')} |`;
  const sep = `| --- | --- |${' ---: |'.repeat(cols.length)}`;
  const out = [`# Vergleich ${LETTERS.slice(0, cols.length).join(' → ')}`, '', `- Ergebnis: ${c.passes.map((p) => (p ? 'Pass' : 'Fail')).join(' → ')}`,
    `- Seeds: ${c.seeds.join(' → ')}`, '', '## Ziele', '', head, sep, ...c.targets.map(rowLine)];
  if (c.figures.length) out.push('', '## Kennzahlen', '', head, sep, ...c.figures.map(rowLine));
  return out.join('\n') + '\n';
}

/** Datei im Browser speichern (Blob, gilt auch im Wails-Fenster). */
export function downloadText(filename: string, text: string, mime: string): void {
  const url = URL.createObjectURL(new Blob([text], { type: `${mime};charset=utf-8` }));
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  a.click();
  URL.revokeObjectURL(url);
}
