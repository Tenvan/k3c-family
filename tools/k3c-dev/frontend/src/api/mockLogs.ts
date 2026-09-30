import type { ConsoleLine, Source } from './types';

// Erfundene Quellen und Konsolen für den Mock (B-064): Vite schreibt laufend, das eigene Log spiegelt Einträge,
// ein Lauf check:npm:test wechselt blau → grün/rot und schreibt dabei farbige Zeilen.

const ESC = '\u001b[';
const CAPACITY = 2000;
const RUN = 'check:npm:test';

type EmitLines = (lines: ConsoleLine[]) => void;
type EmitSource = (src: Source) => void;

const VITE_LINES = [
  `${ESC}32m➜${ESC}0m  ${ESC}1mLocal${ESC}0m:   ${ESC}36mhttp://localhost:5173/${ESC}0m`,
  `${ESC}2m17:03:12${ESC}0m ${ESC}36m[vite]${ESC}0m hmr update ${ESC}2m/src/scenes/GameScene.ts${ESC}0m`,
  `${ESC}2m17:03:15${ESC}0m ${ESC}33m[vite] WARN${ESC}0m Sourcemap für ${ESC}38;5;208mphaser.js${ESC}0m fehlt`,
  `${ESC}2m17:03:20${ESC}0m ${ESC}36m[vite]${ESC}0m page reload ${ESC}2mgame.html${ESC}0m`,
];

const LOG_LINES = [
  'time=17:03:01 level=INFO msg="aufruf beendet" ns=mcp tool=logs_query ms=12',
  'time=17:03:04 level=WARN msg="lauf beendet" ns=check target=npm:test exit=1',
  'time=17:03:09 level=INFO msg="dienst Vite: läuft" ns=svc pid=41232',
  'time=17:03:11 level=ERROR msg="dienst Heimnetz fehlgeschlagen: Port 8080 bereits belegt" ns=svc',
];

const TEST_LINES = [
  ` ${ESC}32m✓${ESC}0m src/world/levelGenerator.test.ts ${ESC}2m(500 tests)${ESC}0m ${ESC}33m812ms${ESC}0m`,
  ` ${ESC}32m✓${ESC}0m src/world/sim/campaign.test.ts ${ESC}2m(24 tests)${ESC}0m`,
  ` ${ESC}31m×${ESC}0m tests/planning.test.ts > Sprints > folgt der Vorlage`,
  `   ${ESC}31mERROR${ESC}0m ${ESC}38;2;255;107;107mexpected 'bereit' to be 'Entwurf'${ESC}0m`,
  ` ${ESC}1m${ESC}42m PASS ${ESC}0m Tests ${ESC}1m${ESC}32m175 passed${ESC}0m`,
];

export function mockLogs(emitLines: EmitLines, emitSource: EmitSource) {
  const buffers = new Map<string, ConsoleLine[]>();
  const seqs = new Map<string, number>();
  const add = (source: string, text: string, stream = 'stdout') => {
    const seq = (seqs.get(source) ?? 0) + 1;
    seqs.set(source, seq);
    const line = { source, stream, text, seq };
    const buf = buffers.get(source) ?? [];
    buf.push(line);
    if (buf.length > CAPACITY) buf.shift();
    buffers.set(source, buf);
    emitLines([line]);
  };
  let run: Source = { name: RUN, kind: 'run', state: 'ok', detail: 'Exit 0 · 4,2 s · 17:02:40' };
  VITE_LINES.forEach((l) => add('Vite', l));
  LOG_LINES.forEach((l) => add('k3c-dev', l, 'log'));
  TEST_LINES.forEach((l) => add(RUN, l));

  let tick = 0;
  setInterval(() => {
    tick++;
    add('Vite', VITE_LINES[1 + (tick % 3)]);
    if (tick % 3 === 0) add('k3c-dev', LOG_LINES[tick % LOG_LINES.length], 'log');
    if (tick % 16 === 0) {
      buffers.set(RUN, []); // wie Reset in Go: neuer Lauf, Seq läuft weiter
      run = { ...run, state: 'running', detail: 'läuft seit eben' };
      emitSource(run);
    }
    if (run.state === 'running') add(RUN, TEST_LINES[tick % TEST_LINES.length]);
    if (tick % 16 === 8) {
      run = tick % 32 === 8
        ? { ...run, state: 'failed', detail: 'Exit 1 · 5,1 s · eben' }
        : { ...run, state: 'ok', detail: 'Exit 0 · 4,8 s · eben' };
      emitSource(run);
    }
  }, 700);

  const logs: Source[] = [
    { name: 'k3c-dev', kind: 'log', state: 'entries', detail: 'logs/k3c-dev.jsonl · 48 KB' },
    { name: 'server', kind: 'log', state: 'empty', detail: 'noch keine Einträge' },
  ];
  return {
    /** Log-Dateien und Läufe; die Dienste steuert mockServices bei. */
    logSources: (): Source[] => [...logs, run],
    consoleTail: async (source: string): Promise<ConsoleLine[]> => [...(buffers.get(source) ?? [])],
    knows: (source: string) => buffers.has(source) || logs.some((l) => l.name === source),
  };
}
