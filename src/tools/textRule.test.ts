import { describe, expect, it } from 'vitest';

/**
 * PL2 AC-01 (B-322/AC-01): Die Werkzeug-Seiten holen ihre Texte aus `src/tools/texts.*.ts`. Wie `src/scenes/textRule.test.ts`:
 * Kein Zeichenketten-Literal mit Umlaut, ß oder zwei Wörtern mit Leerzeichen; HTML-Tags im Literal zählen nicht.
 * Ausgenommen: Kommentare, CSS-Werte (mit `;`), Zeilen mit `clientLog(`, `console.`, `querySelector` (Selektor) oder `.font =`, die Textdateien selbst.
 */
const sources = import.meta.glob(['./*.ts'], { query: '?raw', import: 'default', eager: true }) as Record<string, string>;

/** Noch nicht umgestellt (PL2.3 leert die Liste). */
const OFFEN = [
  './audioProbe.ts', './dev.ts', './devTiles.ts', './gamepadTest.ts', './selection.ts', './spriteReference.ts', './testing.ts',
  './testScenarios.ts', './testTiles.ts',
];
/** Reine Daten, keine Oberflächen-Texte. */
const DATEN: Record<string, string> = {
  './grafikPacks.ts': 'Urheber, Lizenzen und Recherche-Notizen der Grafik-Packs',
  './soundtestLogic.ts': 'Gruppen-Namen der Kandidatenliste public/audio/kandidaten.json',
};

const GERMAN = /[äöüßÄÖÜ]|[A-Za-z]{2,}\s+[A-Za-z]{2,}/;
const TEXT_FILES = /\/texts(\.\w+)?\.ts$/;

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
    if (/clientLog\(|console\.|querySelector|\.font = /.test(line)) continue;
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
  literals(src).filter((t) => !t.includes(';') && GERMAN.test(t.replace(/<[^>]*>/g, ' ')));

describe('Keine deutschen Text-Literale in den Werkzeug-Seiten (PL2 AC-01)', () => {
  const all = Object.entries(sources).filter(([name]) => !name.endsWith('.test.ts') && !TEXT_FILES.test(name));
  const checked = all.filter(([name]) => !OFFEN.includes(name) && !(name in DATEN));

  it('findet die Dateien, und jeder Eintrag in OFFEN und DATEN existiert', () => {
    expect(checked.map(([n]) => n)).toEqual(expect.arrayContaining(['./leveltest.ts', './soundtest.ts']));
    for (const name of [...OFFEN, ...Object.keys(DATEN)]) expect(sources[name], name).toBeDefined();
  });

  it.each(checked)('%s', (_name, text) => {
    expect(germanLiterals(text)).toEqual([]);
  });

  it.each(OFFEN)('%s ist noch offen (sonst aus OFFEN streichen)', (name) => {
    expect(germanLiterals(sources[name]!).length).toBeGreaterThan(0);
  });

  it('erkennt ein deutsches Literal, nicht aber Kommentar, Log, HTML-Gerüst oder Code-Wert', () => {
    const src = [
      "const a = 'Nacht naht!'; // schöner Kommentar mit Umlaut",
      'const b = `${x} wartet auf Bauer`;',
      "clientLog('warn', 'Nachricht des Servers nicht lesbar');",
      "const c = { fontFamily: 'sans-serif', color: '#ffffff', k: t('level.load') };",
      'el.innerHTML = `<span class="mark"></span><span class="name"></span>`;',
      "const items = document.querySelectorAll('main button, main input'); ctx.font = '12px system-ui, sans-serif';",
      "/* ein Block mit zwei Wörtern */ const d = 'wood';",
    ].join('\n');
    expect(germanLiterals(src)).toEqual(['Nacht naht!', ' wartet auf Bauer']);
  });
});
