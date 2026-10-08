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

export const DOMAINS = ['REG', 'SIM', 'SRV', 'CLI', 'PLAT', 'INF', 'DEV'];
export const SPEC = ['Entwurf', 'freigegeben', 'rückwirkend'];
export const PRIO = ['hoch', 'mittel', 'niedrig', '?']; // Rangfolge: hoch zuerst
const ENV = ['offline', 'live', '?']; // Umgebung: offline ist worktree-tauglich
const ALLOWED = {
  ticket: { Domäne: DOMAINS, Typ: ['Idee', 'Problem', 'Schuld', 'Frage'], Prio: PRIO, Umgebung: ENV,
    Status: ['offen', 'eingeplant', 'erledigt', 'verworfen'], Spec: SPEC },
  sprint: { Status: ['geplant', 'aktiv', 'erledigt'], Reife: ['Entwurf', 'bereit'], Spec: SPEC },
  session: { Status: ['offen', 'in Arbeit', 'fertig', 'blockiert', 'verworfen'], Typ: ['Umsetzung', 'Review', 'Workshop'],
    Agent: ['autonom', 'Mensch'], Domäne: DOMAINS, Umgebung: ENV },
  projekt: { Status: ['aktiv', 'ruht', 'erledigt'] },
} as const;
type Kind = keyof typeof ALLOWED;

/** Gleiche Felder und Überschriften wie die Vorlage, erlaubte Werte in den Auswahlfeldern. */
export function checkTemplate(kind: Kind, text: string, where: string): Record<string, string> {
  const template = read(`vorlagen/${kind}.md`);
  const fields = meta(text);
  expect(Object.keys(fields), `${where}: Felder`).toEqual(Object.keys(meta(template)));
  expect(headings(text), `${where}: Überschriften`).toEqual(headings(template));
  for (const [field, values] of Object.entries(ALLOWED[kind])) {
    expect(values as readonly string[], `${where}: ${field} = "${fields[field]}"`).toContain(fields[field]);
  }
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

/** Sprint wartet nur noch aufs Gerät: alle offenen Sessions sind Mensch-Sessions (Hardware entkoppelt). */
export function waitsForDevice(readme: string) {
  const open = sessionRows(readme).filter((r) => !['fertig', 'verworfen'].includes(r.status));
  return open.length > 0 && open.every((r) => r.agent === 'Mensch');
}
