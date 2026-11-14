// Prüft die Planungs-Dateien gegen die Pflicht-Vorlagen in docs/vorlagen/ (siehe docs/arbeitsweise.md).
// Projekte, Rang und Domänen-Sperre: planningProjects.test.ts; gemeinsame Hilfen: planningDocs.ts.
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import {
  checkTemplate, DIGEST, DOCS, DOMAINS, ids, laneOk, meta, none, PLAN_LINES_SINCE, planLineDefects, read, sessionRows, sprints, title,
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
/** Kriterien aller Dateien mit der Nummer (doppelt vergebene wie B-199, B-696 liegen beide im Archiv). */
const ticketCriteria = (id: string) => tickets.filter((t) => t.file.startsWith(id + '-')).flatMap((t) => criteria(read(t.path)));

describe('Backlog', () => {
  it.each(tickets)('$path folgt der Vorlage', ({ file, folder, path }) => {
    const text = read(path);
    expect(title(text), `${file}: Überschrift`).toBe(file.slice(0, 5));
    const fields = checkTemplate('ticket', text, file);
    const archived = ['erledigt', 'verworfen'].includes(fields.Status);
    expect(folder === 'archiv', `${path}: Ordner passt zum Status ${fields.Status}`).toBe(archived);
    for (const id of ids(`${fields['Duplikat von'] ?? ''} ${fields.Ersetzt ?? ''}`)) expect(ticketIds, `${file}: Relation ${id}`).toContain(id);
    if (!none(fields['Duplikat von'])) expect(fields.Status, `${file}: Duplikat ist verworfen`).toBe('verworfen');
    checkSpec(text, fields, file);
  });

  // B-696 Teil B: offen und eingeplant in backlog/README.md, erledigt und verworfen in backlog/archiv/README.md (Links
  // dort relativ zu archiv/); die Archiv-Datei darf vor der Umstellung fehlen.
  it('Index listet jedes Ticket genau einmal mit gleichem Status und Pfad', () => {
    const row = /^\| \[(B-\d{3})\]\((.+?)\) \|.*\| (\S+) \| \S+ \| .* \|$/gm;
    const split = existsSync(join(DOCS, 'backlog/archiv/README.md'));
    const rows = [
      ...[...read('backlog/README.md').matchAll(row)].map(([, id, file, status]) => ({ id, file, status, archive: false })),
      ...(split
        ? [...read('backlog/archiv/README.md').matchAll(row)].map(([, id, file, status]) => ({ id, file: 'archiv/' + file, status, archive: true }))
        : []),
    ];
    expect(rows.map((r) => r.file).sort()).toEqual(tickets.map((t) => t.rel).sort());
    for (const { id, file, status, archive } of rows) {
      expect(status, `${id}: Status im Index`).toBe(meta(read('backlog/' + file)).Status);
      if (split) expect(archive ? ['erledigt', 'verworfen'] : ['offen', 'eingeplant'], `${id}: richtige Index-Datei`).toContain(status);
    }
  });
});

/** Jede Session verweist nur auf Kriterien des Sprints, und jedes Kriterium hat eine Session. */
function checkSessions(path: string, sprintId: string, sprintCriteria: string[]) {
  const rows = sessionRows(read(`${path}/README.md`));
  // ZUSAMMENFASSUNG.md (plan_digest, B-696 Teil C) ist kein Dokument nach Vorlage.
  const files = readdirSync(join(DOCS, path)).filter((f) => f !== 'README.md' && f !== DIGEST);
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

describe('Optionale Session-Felder Lane und Modell (PV1.1)', () => {
  /** Session nach Vorlage mit gültigen Werten; `extra` steht zwischen Domäne und Umgebung wie in der Vorlage. */
  const session = (extra: string[]) => {
    const head = ['Status: offen', 'Typ: Umsetzung', 'Agent: autonom', 'Domäne: PAP', ...extra,
      'Umgebung: offline', 'Branch: x1/1-a', 'Abhängig von: –', 'Tickets: B-699', 'Kriterien: AC-01'];
    const body = read('vorlagen/session.md').split(/^## /m).slice(1).map((s) => '## ' + s).join('');
    const fields = head.map((f) => `- **${f.replace(': ', ':** ')}`).join('\n');
    return `# X1.1 · Titel\n\n${fields}\n\n${body}`;
  };

  it('dürfen fehlen und sind einzeln oder gemeinsam gültig', () => {
    expect(checkTemplate('session', session([]), 'ohne').Domäne).toBe('PAP');
    expect(checkTemplate('session', session(['Lane: mcp', 'Modell: opus']), 'beide').Lane).toBe('mcp');
    expect(checkTemplate('session', session(['Modell: sonnet']), 'nur Modell').Modell).toBe('sonnet');
    expect(checkTemplate('session', session(['Lane: –']), 'Lane leer').Lane).toBe('–');
  });

  it('lehnen ungültige Werte und falsche Reihenfolge ab', () => {
    expect(() => checkTemplate('session', session(['Lane: MCP!']), 'Lane')).toThrow();
    expect(() => checkTemplate('session', session(['Modell: gpt']), 'Modell')).toThrow();
    expect(() => checkTemplate('session', session(['Modell: opus', 'Lane: mcp']), 'Reihenfolge')).toThrow();
    expect([laneOk('mcp'), laneOk('renderer-2'), laneOk('–'), laneOk('Ui'), laneOk('')]).toEqual([true, true, true, false, false]);
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

describe('Planungsangaben für Arbeitswellen (B-784)', () => {
  it('jedes offene Ticket, das nach dem Stichtag freigegeben wurde, trägt sie', () => {
    const open = tickets.filter((t) => !t.folder);
    expect(open.flatMap((t) => planLineDefects(read(t.path)).map((d) => `${t.file}: ${d}`))).toEqual([]);
  });

  it('schlägt bei einem neu freigegebenen Ticket ohne Bereich an, der Bestand bleibt gültig', () => {
    const rules = (area: string | null) =>
      (area === null ? '' : `- **Bereich:** ${area}\n`) + '- **Abhängig von:** –\n- **Braucht 🧑:** –\n- **Größe:** S\n- **Sperre:** –\n';
    const ticket = (freigabe: string, area: string | null) =>
      `# B-999 · Neu\n\n- **Umgebung:** offline\n- **Spec:** freigegeben\n- **Freigabe:** Klasse A, Claude, ${freigabe}\n\n` +
      `## Regeln und Einschränkungen\n\nDomäne DEV.\n\n${rules(area)}\n## Beispiele\n\n–\n`;
    expect(planLineDefects(ticket('2099-01-01', ''))).toEqual(['Bereich leer']);
    expect(planLineDefects(ticket('2099-01-01', null))).toEqual(['Bereich fehlt']);
    expect(planLineDefects(ticket('2099-01-01', '`tools/x/**`'))).toEqual([]);
    expect(planLineDefects(ticket(PLAN_LINES_SINCE, ''))).toEqual([]);
    expect(planLineDefects(ticket('2026-01-01', null))).toEqual([]);
    expect(planLineDefects(ticket('2099-01-01', '').replace('Spec:** freigegeben', 'Spec:** Entwurf'))).toEqual([]);
  });
});
