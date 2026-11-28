// Flags wie das Go-Paket `flag` (Orakel): -name und --name, -name=wert, -name wert (nicht bei Schaltern); das erste
// Argument ohne Strich oder `--` beendet die Flags. Ein Fehler bricht ab, bevor irgendetwas läuft.

export type FlagKind = 'bool' | 'string';

export interface Flags {
  values: Record<string, string | boolean>;
  rest: string[];
}

/** FlagError beendet das Programm mit `code` (2 Fehler, 0 Hilfe). */
export class FlagError extends Error {
  readonly code: number;
  constructor(message: string, code = 2) {
    super(message);
    this.code = code;
  }
}

const TRUE = new Set(['1', 't', 'T', 'TRUE', 'true', 'True']);
const FALSE = new Set(['0', 'f', 'F', 'FALSE', 'false', 'False']);

function boolValue(name: string, value: string | undefined): boolean {
  if (value === undefined || TRUE.has(value)) return true;
  if (FALSE.has(value)) return false;
  throw new FlagError(`invalid boolean value "${value}" for -${name}: parse error`);
}

function splitFlag(arg: string): { name: string; value: string | undefined } {
  const body = arg.slice(arg.startsWith('--') ? 2 : 1);
  if (body === '' || body.startsWith('-') || body.startsWith('=')) throw new FlagError(`bad flag syntax: ${arg}`);
  const eq = body.indexOf('=');
  if (eq < 0) return { name: body, value: undefined };
  return { name: body.slice(0, eq), value: body.slice(eq + 1) };
}

export function parseFlags(argv: string[], spec: Record<string, FlagKind>): Flags {
  const values: Flags['values'] = {};
  let i = 0;
  for (; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--') {
      i++;
      break;
    }
    if (arg.length < 2 || !arg.startsWith('-')) break;
    const { name, value } = splitFlag(arg);
    const kind = Object.hasOwn(spec, name) ? spec[name] : undefined;
    if (!kind) {
      if (name === 'h' || name === 'help') throw new FlagError('', 0);
      throw new FlagError(`flag provided but not defined: -${name}`);
    }
    if (kind === 'bool') {
      values[name] = boolValue(name, value);
      continue;
    }
    const text = value ?? argv[++i];
    if (text === undefined) throw new FlagError(`flag needs an argument: -${name}`);
    values[name] = text;
  }
  return { values, rest: argv.slice(i) };
}
