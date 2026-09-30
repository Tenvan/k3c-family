// Prüft die Planungs-Dateien gegen die Pflicht-Vorlagen in docs/vorlagen/ (siehe docs/arbeitsweise.md).
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const DOCS = resolve(__dirname, '../docs');
// Zeilenenden vereinheitlichen: unter Windows checkt Git mit CRLF aus.
const read = (path: string) => readFileSync(join(DOCS, path), 'utf8').replace(/\r\n/g, '\n');
const dirs = (path: string) =>
  existsSync(join(DOCS, path)) ? readdirSync(join(DOCS, path)).filter((n) => statSync(join(DOCS, path, n)).isDirectory()) : [];

/** Felder aus der Liste `- **Feld:** Wert` vor der ersten Unterüberschrift. */
function meta(text: string): Record<string, string> {
  const head = text.split(/^## /m)[0];
  return Object.fromEntries([...head.matchAll(/^- \*\*(.+?):\*\* (.*)$/gm)].map((m) => [m[1], m[2].trim()]));
}
const headings = (text: string) => [...text.matchAll(/^## (.+)$/gm)].map((m) => m[1].trim());
const title = (text: string) => /^# (\S+) · /.exec(text)?.[1];
const ids = (value: string) => value.match(/B-\d{3}/g) ?? [];

const DOMAINS = ['REG', 'SIM', 'SRV', 'CLI', 'PLAT', 'INF'];
const ALLOWED = {
  ticket: { Domäne: DOMAINS, Typ: ['Idee', 'Problem', 'Schuld', 'Frage'], Prio: ['hoch', 'mittel', 'niedrig', '?'],
    Status: ['offen', 'eingeplant', 'erledigt', 'verworfen'] },
  sprint: { Status: ['geplant', 'aktiv', 'erledigt'], Domäne: DOMAINS, Reife: ['Entwurf', 'bereit'], Einschiebbar: ['nein', 'ja'] },
  session: { Status: ['offen', 'in Arbeit', 'fertig', 'blockiert'], Typ: ['Umsetzung', 'Review', 'Workshop'], Agent: ['autonom', 'Mensch'] },
} as const;
type Kind = keyof typeof ALLOWED;

/** Gleiche Felder und Überschriften wie die Vorlage, erlaubte Werte in den Auswahlfeldern. */
function checkTemplate(kind: Kind, text: string, where: string): Record<string, string> {
  const template = read(`vorlagen/${kind}.md`);
  const fields = meta(text);
  expect(Object.keys(fields), `${where}: Felder`).toEqual(Object.keys(meta(template)));
  expect(headings(text), `${where}: Überschriften`).toEqual(headings(template));
  for (const [field, values] of Object.entries(ALLOWED[kind])) {
    expect(values as readonly string[], `${where}: ${field} = "${fields[field]}"`).toContain(fields[field]);
  }
  return fields;
}

const tickets = readdirSync(join(DOCS, 'backlog')).filter((f) => /^B-\d{3}-.+\.md$/.test(f));
const ticketIds = new Set(tickets.map((f) => f.slice(0, 5)));

describe('Backlog', () => {
  it.each(tickets)('%s folgt der Vorlage', (file) => {
    const text = read(`backlog/${file}`);
    expect(title(text), `${file}: Überschrift`).toBe(file.slice(0, 5));
    checkTemplate('ticket', text, file);
  });

  it('Index listet jedes Ticket genau einmal mit gleichem Status', () => {
    const rows = [...read('backlog/README.md').matchAll(/^\| \[(B-\d{3})\]\((.+?)\) \|.*\| (\S+) \| \S+ \| .* \|$/gm)];
    expect(rows.map((r) => r[2]).sort()).toEqual([...tickets].sort());
    for (const [, id, file, status] of rows) {
      expect(status, `${id}: Status im Index`).toBe(meta(read(`backlog/${file}`)).Status);
    }
  });
});

const STATES = ['geplant', 'aktiv', 'erledigt'] as const;
const sprints = STATES.flatMap((state) => dirs(`sprints/${state}`).map((dir) => ({ state, dir, path: `sprints/${state}/${dir}` })));

/** Session-Tabelle der Sprint-README: Nr., Datei, Typ, Agent, Status. */
function sessionRows(text: string) {
  const section = text.split(/^## Sessions$/m)[1]?.split(/^## /m)[0] ?? '';
  return [...section.matchAll(/^\| (\S+) \| `(.+?)` \| (.+?) \| (.+?) \| (.+?) \|$/gm)].map((m) => ({
    id: m[1], file: m[2], typ: m[3], agent: m[4], status: m[5],
  }));
}

function checkSessions(path: string, sprintId: string) {
  const rows = sessionRows(read(`${path}/README.md`));
  const files = readdirSync(join(DOCS, path)).filter((f) => f !== 'README.md');
  expect(rows.map((r) => r.file).sort(), `${path}: Session-Tabelle ↔ Dateien`).toEqual(files.sort());
  expect(rows.length, `${path}: mindestens eine Session`).toBeGreaterThan(0);
  for (const row of rows) {
    const text = read(`${path}/${row.file}`);
    const fields = checkTemplate('session', text, row.file);
    expect(title(text), `${row.file}: Überschrift`).toBe(row.id);
    expect(row.id.startsWith(`${sprintId}.`) && row.file.startsWith(`${row.id}-`), `${row.file}: Name`).toBe(true);
    expect([row.typ, row.agent, row.status], `${row.file}: Tabelle`).toEqual([fields.Typ, fields.Agent, fields.Status]);
    for (const id of ids(fields.Tickets)) expect(ticketIds, `${row.file}: ${id}`).toContain(id);
  }
}

describe('Sprints', () => {
  it.each(sprints)('$path folgt der Vorlage', ({ state, dir, path }) => {
    const text = read(`${path}/README.md`);
    const fields = checkTemplate('sprint', text, path);
    const id = title(text) ?? '';
    expect(dir.startsWith(`${id}-`), `${path}: Ordnername beginnt mit ${id}-`).toBe(true);
    expect(text, `${path}: Domäne in der Überschrift`).toMatch(new RegExp(`^# ${id} · ${fields.Domäne} · `));
    expect(fields.Status, `${path}: Status passt zum Ordner`).toBe(state);
    for (const ticket of ids(fields.Tickets)) expect(ticketIds, `${path}: ${ticket}`).toContain(ticket);
    if (fields.Reife === 'bereit') checkSessions(path, id);
  });

  it('höchstens ein aktiver Sprint (plus eingeschobene), und nur mit Reife bereit', () => {
    const active = sprints.filter((s) => s.state === 'aktiv').map((s) => meta(read(`${s.path}/README.md`)));
    expect(active.filter((f) => f.Einschiebbar === 'nein').length).toBeLessThanOrEqual(1);
    for (const fields of active) expect(fields.Reife).toBe('bereit');
  });

  it('Fahrplan nennt jeden Sprint-Ordner', () => {
    const overview = read('sprints/README.md');
    for (const { state, dir } of sprints) expect(overview, `${state}/${dir}`).toContain(`${state}/${dir}/`);
  });
});
