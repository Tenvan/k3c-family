import type { BalanceEntry, BalanceSummary } from '../api';
import type { Comparison, CompareRow } from './compare';

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

const rowLine = (r: CompareRow) =>
  `| ${cell(r.label)} | ${num(r.a)} | ${num(r.b)} | ${signed(r.delta)} | ` +
  `${r.statusA && r.statusB ? `${r.statusA} → ${r.statusB}${r.tipped ? ' (!)' : ''}` : ''} |`;

/** Vergleich als Markdown (A = Basis, B = neu). */
export function comparisonMarkdown(c: Comparison, nameA = 'A', nameB = 'B'): string {
  const head = `| Wert | ${cell(nameA)} | ${cell(nameB)} | Differenz | Status |`;
  const sep = '| --- | ---: | ---: | ---: | --- |';
  const out = [`# Vergleich ${nameA} → ${nameB}`, '', `- Ergebnis: ${c.passA ? 'Pass' : 'Fail'} → ${c.passB ? 'Pass' : 'Fail'}`,
    `- Seeds: ${c.seedsA} → ${c.seedsB}`, '', '## Ziele', '', head, sep, ...c.targets.map(rowLine)];
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
