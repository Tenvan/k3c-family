// Prüft die Planungs-Dateien gegen die Pflicht-Vorlagen in docs/vorlagen/ (siehe docs/arbeitsweise.md).
// Projekte, Rang und Domänen-Sperre: planningProjects.test.ts; gemeinsame Hilfen: planningDocs.ts.
import { readdirSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import {
  checkTemplate, DOCS, DOMAINS, ids, meta, none, read, sessionRows, sprints, title,
} from './planningDocs';

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

describe('Sprints', () => {
  it.each(sprints)('$path folgt der Vorlage', ({ state, dir, path }) => {
    const text = read(`${path}/README.md`);
    const fields = checkTemplate('sprint', text, path);
    const acs = checkSpec(text, fields, path);
    const id = title(text) ?? '';
    expect(dir.startsWith(`${id}-`), `${path}: Ordnername beginnt mit ${id}-`).toBe(true);
    const domains = fields.Domäne.split(/,\s*/);
    for (const d of domains) expect(DOMAINS, `${path}: Domäne "${d}"`).toContain(d);
    expect(new Set(domains).size, `${path}: Domänen ohne Dopplung`).toBe(domains.length);
    expect(text, `${path}: Domäne in der Überschrift`).toMatch(new RegExp(`^# ${id} · ${fields.Domäne} · `));
    expect(fields.Status, `${path}: Status passt zum Ordner`).toBe(state);
    for (const ticket of ids(fields.Tickets)) expect(ticketIds, `${path}: ${ticket}`).toContain(ticket);
    for (const [ref, ticket, ac] of text.matchAll(/(B-\d{3})\/(AC-\d{2})/g)) {
      expect(ticketCriteria(ticket), `${path}: ${ref}`).toContain(ac);
    }
    if (fields.Reife === 'bereit') checkSessions(path, id, acs);
  });

  it('aktive Sprints haben Reife bereit und eine freigegebene Spec', () => {
    for (const s of sprints.filter((s) => s.state === 'aktiv')) {
      const fields = meta(read(`${s.path}/README.md`));
      expect(fields.Reife, s.path).toBe('bereit');
      expect(fields.Spec, `${s.path}: aktiver Sprint braucht Spec freigegeben oder rückwirkend`).not.toBe('Entwurf');
    }
  });

  it('jeder aktive und geplante Sprint und jedes offene Ticket hat ein Projekt (B-359)', () => {
    const open = [
      ...sprints.filter((s) => s.state !== 'erledigt').map((s) => `${s.path}/README.md`),
      ...tickets.filter((t) => !t.folder).map((t) => t.path),
    ];
    expect(open.filter((path) => none(meta(read(path)).Projekt))).toEqual([]);
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
