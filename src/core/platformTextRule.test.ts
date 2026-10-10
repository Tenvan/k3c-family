import { describe, expect, it } from 'vitest';

/**
 * B-215/AC-01, B-369/AC-01: Touch-Overlay, Shell und Landingpage holen ihre Texte aus `src/core/texts.*.ts`. Gleiche Regel wie
 * `src/scenes/textRule.test.ts` (CLI, dort nicht geändert): kein Literal mit Umlaut, ß oder zwei Wörtern; ausgenommen
 * Kommentare, CSS (mit `;` oder in Zeilen mit `font`) und Log-Zeilen. In HTML-Markup (`<…>`) zählen nur Umlaute und ß,
 * weil Tag- und Attributnamen sonst als „zwei Wörter“ gälten.
 */
const sources = import.meta.glob(['../input/touchInput.ts', './shell.ts', '../landing/*.ts', '!../landing/*.test.ts'], {
  query: '?raw',
  import: 'default',
  eager: true,
}) as Record<string, string>;
const indexHtml = (import.meta.glob('../../index.html', { query: '?raw', import: 'default', eager: true }) as Record<string, string>)['../../index.html']!;

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

/** Sichtbarer Text der Seite: Inhalt zwischen Tags und Umlaute im Markup; `<style>`, `<script>` und Kommentare fehlen. */
function germanHtml(html: string): string[] {
  const markup = html.replace(/<!--[\s\S]*?-->|<style[\s\S]*?<\/style>|<script[\s\S]*?<\/script>/g, '');
  const texts = [...markup.matchAll(/>([^<]+)</g)].map((m) => m[1]!.trim()).filter((s) => s && s !== 'Family Three Crowns');
  const tags = [...markup.matchAll(/<[^>]+>/g)].map((m) => m[0]);
  return [...texts.filter((s) => GERMAN.test(s)), ...tags.filter((s) => UMLAUT.test(s))];
}

describe('Keine deutschen Text-Literale in Touch-Overlay, Shell und Landingpage (B-215, B-369)', () => {
  it('findet alle Dateien', () => {
    expect(Object.keys(sources).sort()).toEqual([
      '../input/touchInput.ts',
      '../landing/landing.ts',
      '../landing/pages.ts',
      '../landing/serverCheck.ts',
      './shell.ts',
    ]);
  });

  it('index.html (B-369/AC-01)', () => {
    expect(germanHtml(indexHtml)).toEqual([]);
  });

  it('erkennt deutschen Seitentext, nicht aber Spielname, CSS oder Kommentar', () => {
    const html = '<style>p { content: "Grün Blau" }</style><!-- schön --><h1>Family Three Crowns</h1><p>Couch-Koop für alle</p><b title="Zurück">A</b>';
    expect(germanHtml(html)).toEqual(['Couch-Koop für alle', '<b title="Zurück">']);
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
