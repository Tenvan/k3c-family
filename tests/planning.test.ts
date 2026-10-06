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
const SPEC = ['Entwurf', 'freigegeben', 'rückwirkend'];
const PRIO = ['hoch', 'mittel', 'niedrig', '?']; // Rangfolge: hoch zuerst
const ENV = ['offline', 'live', '?']; // Umgebung: offline ist worktree-tauglich
const ALLOWED = {
  ticket: { Domäne: DOMAINS, Typ: ['Idee', 'Problem', 'Schuld', 'Frage'], Prio: PRIO, Umgebung: ENV,
    Status: ['offen', 'eingeplant', 'erledigt', 'verworfen'], Spec: SPEC },
  sprint: { Status: ['geplant', 'aktiv', 'erledigt'], Domäne: DOMAINS, Prio: PRIO, Reife: ['Entwurf', 'bereit'], Einschiebbar: ['nein', 'ja'],
    Spec: SPEC },
  session: { Status: ['offen', 'in Arbeit', 'fertig', 'blockiert'], Typ: ['Umsetzung', 'Review', 'Workshop'], Agent: ['autonom', 'Mensch'], Umgebung: ENV },
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

/** Akzeptanzkriterien `- **AC-01** …` im gleichnamigen Abschnitt. */
function criteria(text: string): string[] {
  const section = text.split(/^## Akzeptanzkriterien$/m)[1]?.split(/^## /m)[0] ?? '';
  return [...section.matchAll(/^- \*\*(AC-\d{2})\*\*/gm)].map((m) => m[1]);
}

/** SDD: Revision als Zahl, Freigabe genau bei `freigegeben`, Kriterien lückenlos ab AC-01. */
function checkSpec(text: string, fields: Record<string, string>, where: string): string[] {
  expect(fields.Revision, `${where}: Revision`).toMatch(/^\d+$/);
  expect(fields.Freigabe.startsWith('–'), `${where}: Freigabe passt zu Spec = ${fields.Spec}`).toBe(fields.Spec !== 'freigegeben');
  const acs = criteria(text);
  expect(acs.length, `${where}: mindestens ein Akzeptanzkriterium`).toBeGreaterThan(0);
  expect(acs, `${where}: Kriterien lückenlos ab AC-01`).toEqual(acs.map((_, i) => `AC-${String(i + 1).padStart(2, '0')}`));
  return acs;
}

const isTicket = (f: string) => /^B-\d{3}-.+\.md$/.test(f);
const ticketFiles = (folder: string) =>
  readdirSync(join(DOCS, 'backlog', folder)).filter(isTicket).map((file) => ({ file, folder, rel: folder ? folder + '/' + file : file }));
/** Offene und eingeplante Tickets liegen in backlog/, erledigte und verworfene in backlog/archiv/. */
const tickets = [...ticketFiles(''), ...ticketFiles('archiv')].map((t) => ({ ...t, path: 'backlog/' + t.rel }));
const ticketIds = new Set(tickets.map((t) => t.file.slice(0, 5)));
/** Sprint-Prio = höchste Prio seiner Tickets; ohne Tickets frei wählbar. */
const ticketPrio = (id: string) => meta(read(tickets.find((t) => t.file.startsWith(id))?.path ?? '')).Prio ?? '?';
const sprintPrio = (value: string) =>
  ids(value).map(ticketPrio).reduce((best, p) => (PRIO.indexOf(p) >= 0 && PRIO.indexOf(p) < PRIO.indexOf(best) ? p : best), '?');
const ticketCriteria = (id: string) => criteria(read(tickets.find((t) => t.file.startsWith(id))?.path ?? ''));

describe('Backlog', () => {
  it.each(tickets)('$path folgt der Vorlage', ({ file, folder, path }) => {
    const text = read(path);
    expect(title(text), `${file}: Überschrift`).toBe(file.slice(0, 5));
    const fields = checkTemplate('ticket', text, file);
    const archived = ['erledigt', 'verworfen'].includes(fields.Status);
    expect(folder === 'archiv', `${path}: Ordner passt zum Status ${fields.Status}`).toBe(archived);
    checkSpec(text, fields, file);
  });

  it('Index listet jedes Ticket genau einmal mit gleichem Status und Pfad', () => {
    const rows = [...read('backlog/README.md').matchAll(/^\| \[(B-\d{3})\]\((.+?)\) \|.*\| (\S+) \| \S+ \| .* \|$/gm)];
    expect(rows.map((r) => r[2]).sort()).toEqual(tickets.map((t) => t.rel).sort());
    for (const [, id, file, status] of rows) {
      expect(status, `${id}: Status im Index`).toBe(meta(read('backlog/' + file)).Status);
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

/** Jede Session verweist nur auf Kriterien des Sprints, und jedes Kriterium hat eine Session. */
function checkSessions(path: string, sprintId: string, sprintCriteria: string[]) {
  const rows = sessionRows(read(`${path}/README.md`));
  const files = readdirSync(join(DOCS, path)).filter((f) => f !== 'README.md');
  expect(rows.map((r) => r.file).sort(), `${path}: Session-Tabelle ↔ Dateien`).toEqual(files.sort());
  expect(rows.length, `${path}: mindestens eine Session`).toBeGreaterThan(0);
  const covered = new Set<string>();
  for (const row of rows) {
    const text = read(`${path}/${row.file}`);
    const fields = checkTemplate('session', text, row.file);
    expect(title(text), `${row.file}: Überschrift`).toBe(row.id);
    expect(row.id.startsWith(`${sprintId}.`) && row.file.startsWith(`${row.id}-`), `${row.file}: Name`).toBe(true);
    expect([row.typ, row.agent, row.status], `${row.file}: Tabelle`).toEqual([fields.Typ, fields.Agent, fields.Status]);
    for (const id of ids(fields.Tickets)) expect(ticketIds, `${row.file}: ${id}`).toContain(id);
    const refs = ['alle', '–'].includes(fields.Kriterien) ? [] : fields.Kriterien.split(/,\s*/);
    for (const ref of refs) {
      expect(sprintCriteria, `${row.file}: Kriterium ${ref}`).toContain(ref);
      covered.add(ref);
    }
  }
  for (const ac of sprintCriteria) expect(covered.has(ac), `${path}: ${ac} hat keine Session`).toBe(true);
}

/** Domänen mit mehr als einem aktiven, nicht einschiebbaren Sprint (B-174). */
function crowdedDomains(active: Record<string, string>[]) {
  const count = new Map<string, number>();
  for (const f of active.filter((f) => f.Einschiebbar === 'nein')) count.set(f.Domäne, (count.get(f.Domäne) ?? 0) + 1);
  return [...count].filter(([, n]) => n > 1).map(([d]) => d);
}

/** Sprint wartet nur noch aufs Gerät: alle offenen Sessions sind Mensch-Sessions (Hardware entkoppelt). */
function waitsForDevice(readme: string) {
  const open = sessionRows(readme).filter((r) => !['fertig', 'verworfen'].includes(r.status));
  return open.length > 0 && open.every((r) => r.agent === 'Mensch');
}

describe('Regel: je Domäne ein aktiver Sprint', () => {
  it('ein Sprint, der nur noch auf Mensch-Sessions wartet, sperrt die Domäne nicht', () => {
    const table = (rows: string) => `## Sessions\n\n| Nr. | Datei | Typ | Agent | Status |\n|---|---|---|---|---|\n${rows}\n`;
    expect(waitsForDevice(table('| X1.1 | `a.md` | Umsetzung | autonom | fertig |\n| X1.2 | `b.md` | Workshop | Mensch | offen |'))).toBe(true);
    expect(waitsForDevice(table('| X1.1 | `a.md` | Umsetzung | autonom | offen |\n| X1.2 | `b.md` | Workshop | Mensch | offen |'))).toBe(false);
  });

  it('zwei aktive Sprints verschiedener Domänen sind erlaubt', () => {
    expect(crowdedDomains([{ Domäne: 'SRV', Einschiebbar: 'nein' }, { Domäne: 'INF', Einschiebbar: 'nein' }])).toEqual([]);
  });

  it('zwei nicht einschiebbare Sprints der gleichen Domäne sind ein Fehler', () => {
    const active = [{ Domäne: 'SIM', Einschiebbar: 'nein' }, { Domäne: 'SIM', Einschiebbar: 'nein' }, { Domäne: 'SIM', Einschiebbar: 'ja' }];
    expect(crowdedDomains(active)).toEqual(['SIM']);
  });
});

describe('Sprints', () => {
  it.each(sprints)('$path folgt der Vorlage', ({ state, dir, path }) => {
    const text = read(`${path}/README.md`);
    const fields = checkTemplate('sprint', text, path);
    const acs = checkSpec(text, fields, path);
    const id = title(text) ?? '';
    expect(dir.startsWith(`${id}-`), `${path}: Ordnername beginnt mit ${id}-`).toBe(true);
    expect(text, `${path}: Domäne in der Überschrift`).toMatch(new RegExp(`^# ${id} · ${fields.Domäne} · `));
    expect(fields.Status, `${path}: Status passt zum Ordner`).toBe(state);
    for (const ticket of ids(fields.Tickets)) expect(ticketIds, `${path}: ${ticket}`).toContain(ticket);
    if (ids(fields.Tickets).length) expect(fields.Prio, `${path}: Prio = höchste Prio der Tickets`).toBe(sprintPrio(fields.Tickets));
    for (const [ref, ticket, ac] of text.matchAll(/(B-\d{3})\/(AC-\d{2})/g)) {
      expect(ticketCriteria(ticket), `${path}: ${ref}`).toContain(ac);
    }
    if (fields.Reife === 'bereit') checkSessions(path, id, acs);
  });

  it('je Domäne höchstens ein aktiver Sprint (ohne einschiebbare), nur mit Reife bereit und freigegebener Spec', () => {
    const active = sprints.filter((s) => s.state === 'aktiv').map((s) => read(`${s.path}/README.md`));
    expect(crowdedDomains(active.filter((t) => !waitsForDevice(t)).map(meta))).toEqual([]);
    for (const fields of active.map(meta)) {
      expect(fields.Reife).toBe('bereit');
      expect(fields.Spec, 'aktiver Sprint braucht Spec freigegeben oder rückwirkend').not.toBe('Entwurf');
    }
  });

  it('Fahrplan nennt jeden Sprint-Ordner', () => {
    const overview = read('sprints/README.md');
    for (const { state, dir } of sprints) expect(overview, `${state}/${dir}`).toContain(`${state}/${dir}/`);
  });
});

describe('Glossar', () => {
  it('ist eine Tabelle mit eindeutigen, alphabetisch sortierten Begriffen', () => {
    const text = read('glossar.md');
    expect(text).toContain('| Begriff | Bedeutung | Quelle |');
    const terms = [...text.matchAll(/^\| (.+?) \| .+ \| .+ \|$/gm)].map((m) => m[1].replace(/`/g, '')).slice(1);
    expect(terms.length).toBeGreaterThan(0);
    expect(new Set(terms).size, 'keine Dubletten').toBe(terms.length);
    expect(terms, 'deutsche Sortierung').toEqual([...terms].sort((a, b) => a.localeCompare(b, 'de')));
  });

  it('ist in CLAUDE.md und arbeitsweise.md verpflichtend verlinkt', () => {
    expect(readFileSync(resolve(DOCS, '../CLAUDE.md'), 'utf8')).toContain('docs/glossar.md');
    expect(read('arbeitsweise.md')).toContain('glossar.md');
  });
});
