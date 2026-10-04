import { describe, expect, it } from 'vitest';

/**
 * S5.3 AC-04 (B-172): Spieltexte stehen in `src/core/texts.*.ts`. In `src/scenes` und `src/online` darf kein Zeichenketten-Literal
 * Umlaute, ß oder mindestens zwei Wörter mit Leerzeichen enthalten. Ausgenommen: Kommentare, CSS-Werte (mit `;`), Zeilen mit `clientLog(` oder
 * `console.`, das Debug-Overlay (Entwickler-Text).
 */
const sources = import.meta.glob(['./*.ts', '../online/*.ts'], { query: '?raw', import: 'default', eager: true }) as Record<string, string>;

const GERMAN = /[äöüßÄÖÜ]|[A-Za-z]{2,}\s+[A-Za-z]{2,}/;
const DEV_FILES = /(^|\/)debug[^/]*\.ts$/;

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

/** Statische Teile aller Zeichenketten (' " `) einer Quelle; Kommentare und Zeilen mit Log-Aufrufen werden übersprungen. */
function literals(src: string): string[] {
  const out: string[] = [];
  let inBlock = false;
  for (const line of src.split('\n')) {
    if (/clientLog\(|console\./.test(line)) continue;
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

const germanLiterals = (src: string): string[] => literals(src).filter((t) => GERMAN.test(t) && !t.includes(';'));

describe('Keine deutschen Text-Literale außerhalb der Textdateien (AC-04)', () => {
  const files = Object.entries(sources).filter(([name]) => !name.endsWith('.test.ts') && !DEV_FILES.test(name));

  it('findet die Dateien', () => {
    expect(files.map(([n]) => n)).toEqual(expect.arrayContaining(['./HudScene.ts', './LobbyScene.ts', '../online/clientConnection.ts']));
  });

  it.each(files)('%s', (_name, text) => {
    expect(germanLiterals(text)).toEqual([]);
  });

  it('erkennt ein deutsches Literal, nicht aber Kommentar, Log oder Code-Wert', () => {
    const src = [
      "const a = 'Nacht naht!'; // schöner Kommentar mit Umlaut",
      'const b = `${x} wartet auf Bauer`;',
      "clientLog('warn', 'Nachricht des Servers nicht lesbar');",
      "const c = { fontFamily: 'sans-serif', color: '#ffffff', k: t('hud.night') };",
      "/* ein Block mit zwei Wörtern */ const d = 'wood';",
    ].join('\n');
    expect(germanLiterals(src)).toEqual(['Nacht naht!', ' wartet auf Bauer']);
  });
});
