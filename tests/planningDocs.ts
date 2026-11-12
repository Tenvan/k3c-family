// Gemeinsame Hilfen der Planungstests: Planungs-Dateien lesen und gegen die Vorlagen in docs/vorlagen/ prüfen.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { expect } from 'vitest';

export const DOCS = resolve(__dirname, '../docs');
// Zeilenenden vereinheitlichen: unter Windows checkt Git mit CRLF aus.
export const read = (path: string) => readFileSync(join(DOCS, path), 'utf8').replace(/\r\n/g, '\n');
export const dirs = (path: string) =>
  existsSync(join(DOCS, path)) ? readdirSync(join(DOCS, path)).filter((n) => statSync(join(DOCS, path, n)).isDirectory()) : [];

/** Felder aus der Liste `- **Feld:** Wert` vor der ersten Unterüberschrift. */
export function meta(text: string): Record<string, string> {
  const head = text.split(/^## /m)[0];
  return Object.fromEntries([...head.matchAll(/^- \*\*(.+?):\*\* (.*)$/gm)].map((m) => [m[1], m[2].trim()]));
}
export const headings = (text: string) => [...text.matchAll(/^## (.+)$/gm)].map((m) => m[1].trim());
export const title = (text: string) => /^# (\S+) · /.exec(text)?.[1];
export const ids = (value: string) => value.match(/B-\d{3}/g) ?? [];
/** `–` (auch mit Erläuterung dahinter) heißt: kein Wert. */
export const none = (value: string | undefined) => !value || value.startsWith('–');
/** Abschnitt `## Name` bis zur nächsten Unterüberschrift. */
export const section = (text: string, name: string) => text.split(new RegExp(`^## ${name}$`, 'm'))[1]?.split(/^## /m)[0] ?? '';

export const DOMAINS = ['REG', 'SIM', 'SRV', 'CLI', 'PLAT', 'INF', 'DEV', 'PAP'];
export const SPEC = ['Entwurf', 'freigegeben', 'rückwirkend'];
export const PRIO = ['hoch', 'mittel', 'niedrig', '?']; // Rangfolge: hoch zuerst
const ENV = ['offline', 'live', '?']; // Umgebung: offline ist worktree-tauglich
const ALLOWED = {
  ticket: { Domäne: DOMAINS, Typ: ['Idee', 'Problem', 'Schuld', 'Frage'], Prio: PRIO, Umgebung: ENV,
    Status: ['offen', 'eingeplant', 'erledigt', 'verworfen'], Spec: SPEC },
  sprint: { Status: ['geplant', 'aktiv', 'erledigt'], Reife: ['Entwurf', 'bereit'], Spec: SPEC },
  session: { Status: ['offen', 'in Arbeit', 'fertig', 'blockiert', 'verworfen'], Typ: ['Umsetzung', 'Review', 'Workshop'],
    Agent: ['autonom', 'Mensch'], Domäne: DOMAINS, Umgebung: ENV, Modell: ['–', 'haiku', 'sonnet', 'opus', 'fable'] },
  projekt: { Status: ['aktiv', 'ruht', 'erledigt'] },
} as const;
type Kind = keyof typeof ALLOWED;

/** Felder, die in einer Datei fehlen dürfen (ältere Sessions ohne Lane und Modell, PV1.1); die Reihenfolge bleibt die der Vorlage. */
const OPTIONAL: Partial<Record<Kind, readonly string[]>> = { session: ['Lane', 'Modell'] };
/** Lane einer Session: `–` oder Kurzname aus a-z, 0-9 und `-`. */
export const laneOk = (value: string) => value === '–' || /^[a-z0-9-]+$/.test(value);

/** Gleiche Felder und Überschriften wie die Vorlage (optionale dürfen fehlen), erlaubte Werte in den Auswahlfeldern. */
export function checkTemplate(kind: Kind, text: string, where: string): Record<string, string> {
  const template = read(`vorlagen/${kind}.md`);
  const fields = meta(text);
  const optional = OPTIONAL[kind] ?? [];
  const expected = Object.keys(meta(template)).filter((f) => !optional.includes(f) || f in fields);
  expect(Object.keys(fields), `${where}: Felder`).toEqual(expected);
  expect(headings(text), `${where}: Überschriften`).toEqual(headings(template));
  for (const [field, values] of Object.entries(ALLOWED[kind])) {
    if (optional.includes(field) && !(field in fields)) continue;
    expect(values as readonly string[], `${where}: ${field} = "${fields[field]}"`).toContain(fields[field]);
  }
  if ('Lane' in fields && optional.includes('Lane')) expect(laneOk(fields.Lane), `${where}: Lane = "${fields.Lane}"`).toBe(true);
  return fields;
}

const STATES = ['geplant', 'aktiv', 'erledigt'] as const;
export const sprints = STATES.flatMap((state) => dirs(`sprints/${state}`).map((dir) => ({ state, dir, path: `sprints/${state}/${dir}` })));

/** Session-Tabelle der Sprint-README: Nr., Datei, Typ, Agent, Status. */
export function sessionRows(text: string) {
  return [...section(text, 'Sessions').matchAll(/^\| (\S+) \| `(.+?)` \| (.+?) \| (.+?) \| (.+?) \|$/gm)].map((m) => ({
    id: m[1], file: m[2], typ: m[3], agent: m[4], status: m[5],
  }));
}

/** Stichtag der Planungsangaben (B-784, Merge von WZ44): Tickets, die danach freigegeben werden, tragen sie. */
export const PLAN_LINES_SINCE = '2026-11-12';
const PLAN_LINES = ['Bereich', 'Abhängig von', 'Braucht 🧑', 'Größe', 'Sperre'];

/** Mängel der Planungsangaben (docs/arbeitsweise.md › Planungsangaben) eines Tickets, das nach dem Stichtag
 *  freigegeben wurde; für den Bestand (früher freigegeben oder nicht freigegeben) leer. Volle Prüfung: plan_check. */
export function planLineDefects(text: string): string[] {
  const fields = meta(text);
  const dates = fields.Freigabe?.match(/\d{4}-\d{2}-\d{2}/g) ?? [];
  const date = dates[dates.length - 1];
  if (fields.Spec !== 'freigegeben' || !date || date <= PLAN_LINES_SINCE) return [];
  const rules = section(text, 'Regeln und Einschränkungen');
  const value = (name: string) => new RegExp(`^- \\*\\*${name}:\\*\\* *(.*)$`, 'm').exec(rules)?.[1].trim();
  const out = PLAN_LINES.filter((name) => value(name) === undefined).map((name) => `${name} fehlt`);
  const area = value('Bereich');
  if (area !== undefined && none(area)) out.push('Bereich leer');
  if (fields.Umgebung === '?') out.push('Umgebung: ?');
  if (value('Größe') === 'L') out.push('Größe: L');
  return out;
}

/** Sprint wartet nur noch aufs Gerät: alle offenen Sessions sind Mensch-Sessions (Hardware entkoppelt). */
export function waitsForDevice(readme: string) {
  const open = sessionRows(readme).filter((r) => !['fertig', 'verworfen'].includes(r.status));
  return open.length > 0 && open.every((r) => r.agent === 'Mensch');
}
