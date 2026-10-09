import { describe, expect, it } from 'vitest';

/**
 * B-215/AC-01: Touch-Overlay und Shell holen ihre Texte aus `src/core/texts.*.ts`. Gleiche Regel wie
 * `src/scenes/textRule.test.ts` (CLI, dort nicht geändert): kein Literal mit Umlaut, ß oder zwei Wörtern; ausgenommen
 * Kommentare, CSS (mit `;` oder in Zeilen mit `font`) und Log-Zeilen. In HTML-Markup (`<…>`) zählen nur Umlaute und ß,
 * weil Tag- und Attributnamen sonst als „zwei Wörter“ gälten.
 */
const sources = import.meta.glob(['../input/touchInput.ts', './shell.ts'], { query: '?raw', import: 'default', eager: true }) as Record<string, string>;

const GERMAN = /[äöüßÄÖÜ]|[A-Za-z]{2,}\s+[A-Za-z]{2,}/;
const UMLAUT = /[äöüßÄÖÜ]/;
const SKIP_LINE = /clientLog\(|console\.|\bfont\b|font-family|font:/;

/** Liest die Zeichenkette ab `start` (Anführungszeichen); liefert den statischen Text und die Position dahinter. */
function readString(line: string, start: number): [string, number] {
  const q = line[start]!;
  let j = start + 1;
  let text = '';
  let depth = 0;
  while (j < line.length && !(line[j] === q && depth === 0)) {
    if (line[j] === '\\') j += 1;
    else if (q === '`' && line[j] === '$' && line[j + 1] === '{') depth += 1;
    else if (q === '`' && line[j] === '}' && depth > 0) depth -= 1;
    else if (depth === 0) text += line[j];
    j += 1;
  }
  return [text, j + 1];
}

/** Statische Teile aller Zeichenketten (' " `) einer Quelle; Kommentare und übersprungene Zeilen fehlen. */
function literals(src: string): string[] {
  const out: string[] = [];
  let inBlock = false;
  for (const line of src.split('\n')) {
    if (SKIP_LINE.test(line)) continue;
    let i = 0;
    while (i < line.length) {
      const two = line.slice(i, i + 2);
      if (inBlock) {
        if (two === '*/') inBlock = false;
        i += inBlock ? 1 : 2;
      } else if (two === '/*') {
        inBlock = true;
        i += 2;
      } else if (two === '//') {
        break;
      } else if (line[i] === "'" || line[i] === '"' || line[i] === '`') {
        const [text, next] = readString(line, i);
        out.push(text);
        i = next;
      } else i += 1;
    }
  }
  return out;
}

const germanLiterals = (src: string): string[] =>
  literals(src).filter((t) => !t.includes(';') && (t.includes('<') ? UMLAUT.test(t) : GERMAN.test(t)));

describe('Keine deutschen Text-Literale in Touch-Overlay und Shell (B-215)', () => {
  it('findet beide Dateien', () => {
    expect(Object.keys(sources).sort()).toEqual(['../input/touchInput.ts', './shell.ts']);
  });

  it.each(Object.entries(sources))('%s', (_name, text) => {
    expect(germanLiterals(text)).toEqual([]);
  });

  it('erkennt ein deutsches Literal, nicht aber Kommentar, Markup ohne Umlaut oder CSS', () => {
    const src = [
      "b.title = 'Zurück zur Startseite'; // schön",
      "const k = t('shell.homeTitle');",
      'x.innerHTML = `<b aria-label="Münzen">`;',
      'y.innerHTML = `<svg viewBox="0 0 24 24" aria-hidden="true"></svg>`;',
      '  font: 600 18px/1 "Segoe UI", system-ui, sans-serif;',
    ].join('\n');
    expect(germanLiterals(src)).toEqual(['Zurück zur Startseite', '<b aria-label="Münzen">']);
  });
});
