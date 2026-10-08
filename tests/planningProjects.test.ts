// Prüft Projekte, Rang und Domäne je Session (B-355, B-356; docs/arbeitsweise.md › Projekte und Rang, Domänen).
import { existsSync, readdirSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { checkTemplate, DOCS, meta, none, read, section, sessionRows, sprints, title, waitsForDevice } from './planningDocs';

type Project = { id: string; status: string; rang: string; sprints: string[] };
type SprintInfo = { id: string; projekt: string; active: boolean; domains: string[]; sessionDomains: string[] };
type SessionInfo = { id: string; domain: string; status: string };

/** Sprint-Tabelle eines Projekts: erste Spalte jeder Zeile außer Kopf und Trenner. */
const projectSprints = (text: string) =>
  [...section(text, 'Sprints').matchAll(/^\| (\S+) \| .+ \| .+ \|$/gm)].map((m) => m[1]).filter((id) => /^[A-Z]+\d+$/.test(id));

/** Ränge der aktiven Projekte lückenlos ab 1, ruhende und erledigte ohne Rang. */
function rankErrors(projects: Project[]): string[] {
  const errors = projects.filter((p) => p.status !== 'aktiv' && !none(p.rang)).map((p) => `${p.id}: ${p.status} mit Rang`);
  const ranks = projects.filter((p) => p.status === 'aktiv').map((p) => p.rang).sort((a, b) => Number(a) - Number(b));
  if (ranks.join() !== ranks.map((_, i) => String(i + 1)).join()) errors.push(`Ränge ${ranks.join(', ')} statt 1 … ${ranks.length}`);
  return errors;
}

/** Sprint ↔ Sprint-Tabelle des Projekts in beide Richtungen. */
function linkErrors(projects: Project[], list: SprintInfo[]): string[] {
  const errors: string[] = [];
  for (const s of list.filter((s) => !none(s.projekt))) {
    const p = projects.find((p) => p.id === s.projekt);
    if (!p) errors.push(`${s.id}: Projekt ${s.projekt} fehlt`);
    else if (!p.sprints.includes(s.id)) errors.push(`${s.id}: fehlt in der Tabelle von ${p.id}`);
  }
  for (const p of projects)
    for (const id of p.sprints) {
      const s = list.find((s) => s.id === id);
      if (s?.projekt !== p.id) errors.push(`${p.id}: ${id} ${s ? `nennt Projekt ${s.projekt}` : 'fehlt'}`);
    }
  return errors;
}

/** Projekte mit mehr als einem aktiven Sprint (Sprints, die nur aufs Gerät warten, zählen nicht). */
function crowdedProjects(list: SprintInfo[]): string[] {
  const count = new Map<string, number>();
  for (const s of list.filter((s) => s.active && !none(s.projekt))) count.set(s.projekt, (count.get(s.projekt) ?? 0) + 1);
  return [...count].filter(([, n]) => n > 1).map(([p]) => p);
}

/** Sprint-Feld `Domäne` = Domänen seiner Sessions in Reihenfolge des ersten Auftretens. */
const domainListOk = (s: SprintInfo) => s.domains.join() === [...new Set(s.sessionDomains)].join();

/** Domänen mit mehr als einer Session `in Arbeit` (Sperre je Domäne). */
function busyDomains(sessions: SessionInfo[]): string[] {
  const count = new Map<string, number>();
  for (const s of sessions.filter((s) => s.status === 'in Arbeit')) count.set(s.domain, (count.get(s.domain) ?? 0) + 1);
  return [...count].filter(([, n]) => n > 1).map(([d]) => d);
}

const P = (id: string, status: string, rang: string, list: string[] = []): Project => ({ id, status, rang, sprints: list });
const S = (id: string, projekt: string, active = false, domains = ['SIM'], sessionDomains = domains): SprintInfo =>
  ({ id, projekt, active, domains, sessionDomains });

describe('Regeln für Projekte (Beispiele)', () => {
  it('Ränge lückenlos ab 1, nur aktive Projekte haben einen', () => {
    expect(rankErrors([P('GRA', 'aktiv', '1'), P('SND', 'aktiv', '2'), P('BAL', 'ruht', '–')])).toEqual([]);
    expect(rankErrors([P('GRA', 'aktiv', '1'), P('SND', 'aktiv', '1')])).toHaveLength(1);
    expect(rankErrors([P('GRA', 'aktiv', '1'), P('SND', 'aktiv', '3')])).toHaveLength(1);
    expect(rankErrors([P('GRA', 'aktiv', '1'), P('BAL', 'ruht', '2')])).toEqual(['BAL: ruht mit Rang']);
  });

  it('Sprint und Sprint-Tabelle des Projekts nennen sich gegenseitig', () => {
    expect(linkErrors([P('GRA', 'aktiv', '1', ['GR7'])], [S('GR7', 'GRA'), S('K3', '–')])).toEqual([]);
    expect(linkErrors([P('GRA', 'aktiv', '1')], [S('GR7', 'GRA')])).toEqual(['GR7: fehlt in der Tabelle von GRA']);
    expect(linkErrors([], [S('GR7', 'GRA')])).toEqual(['GR7: Projekt GRA fehlt']);
    expect(linkErrors([P('GRA', 'aktiv', '1', ['GR7', 'X9'])], [S('GR7', 'GRA')])).toEqual(['GRA: X9 fehlt']);
    expect(linkErrors([P('GRA', 'aktiv', '1', ['K3'])], [S('K3', 'KMP')])).toHaveLength(2);
  });

  it('je Projekt höchstens ein aktiver Sprint', () => {
    expect(crowdedProjects([S('GR7', 'GRA', true), S('SO2', 'SND', true), S('M10', 'GRA')])).toEqual([]);
    expect(crowdedProjects([S('GR7', 'GRA', true), S('M10', 'GRA', true)])).toEqual(['GRA']);
    expect(crowdedProjects([S('A1', '–', true), S('B1', '–', true)])).toEqual([]);
  });

  it('Sprint-Domänen folgen den Sessions', () => {
    expect(domainListOk(S('K3', '–', false, ['SIM', 'SRV'], ['SIM', 'SIM', 'SRV', 'SRV']))).toBe(true);
    expect(domainListOk(S('K3', '–', false, ['SIM'], ['SIM', 'CLI']))).toBe(false);
    expect(domainListOk(S('K3', '–', false, ['SRV', 'SIM'], ['SIM', 'SRV']))).toBe(false);
  });

  it('höchstens eine Session je Domäne in Arbeit', () => {
    const s = (id: string, domain: string, status: string) => ({ id, domain, status });
    expect(busyDomains([s('A.1', 'CLI', 'in Arbeit'), s('B.1', 'SIM', 'in Arbeit'), s('C.1', 'CLI', 'offen')])).toEqual([]);
    expect(busyDomains([s('A.1', 'CLI', 'in Arbeit'), s('B.2', 'CLI', 'in Arbeit')])).toEqual(['CLI']);
  });
});

const projectFiles = existsSync(join(DOCS, 'projekte'))
  ? readdirSync(join(DOCS, 'projekte')).filter((f) => f.endsWith('.md') && f !== 'README.md')
  : [];
const projects: Project[] = projectFiles.map((file) => {
  const text = read(`projekte/${file}`);
  const f = meta(text);
  return { id: title(text) ?? file, status: f.Status, rang: f.Rang, sprints: projectSprints(text) };
});
const sprintInfos: (SprintInfo & { reife: string })[] = sprints.map(({ path }) => {
  const readme = read(`${path}/README.md`);
  const f = meta(readme);
  const sessionDomains = sessionRows(readme).map((r) => meta(read(`${path}/${r.file}`)).Domäne);
  return { id: title(readme) ?? path, projekt: f.Projekt, reife: f.Reife, domains: f.Domäne.split(/,\s*/), sessionDomains,
    active: path.startsWith('sprints/aktiv/') && !waitsForDevice(readme) };
});
const sessions: SessionInfo[] = sprints.flatMap(({ path }) =>
  sessionRows(read(`${path}/README.md`)).map((r) => ({ id: r.id, domain: meta(read(`${path}/${r.file}`)).Domäne, status: r.status })));
const ticketDir = (folder: string) => readdirSync(join(DOCS, 'backlog', folder)).filter((f) => /^B-\d{3}-.+\.md$/.test(f))
  .map((f) => `backlog/${folder ? folder + '/' : ''}${f}`);

describe('Projekte', () => {
  it.each(projectFiles.map((file) => ({ file })))('projekte/$file folgt der Vorlage', ({ file }) => {
    const text = read(`projekte/${file}`);
    checkTemplate('projekt', text, file);
    const id = title(text) ?? '';
    expect(id, `${file}: Kürzel aus drei Großbuchstaben`).toMatch(/^[A-Z]{3}$/);
    expect(file.startsWith(`${id}-`), `${file}: Dateiname beginnt mit ${id}-`).toBe(true);
  });

  it('Übersicht nennt jedes Projekt', () => {
    const overview = read('projekte/README.md');
    for (const file of projectFiles) expect(overview, file).toContain(file);
  });

  it('Ränge, Zuordnung und ein aktiver Sprint je Projekt', () => {
    expect(rankErrors(projects)).toEqual([]);
    expect(linkErrors(projects, sprintInfos)).toEqual([]);
    expect(crowdedProjects(sprintInfos)).toEqual([]);
  });

  it('Tickets nennen nur bestehende Projekte', () => {
    const known = new Set(projects.map((p) => p.id));
    for (const path of [...ticketDir(''), ...ticketDir('archiv')]) {
      const projekt = meta(read(path)).Projekt;
      if (!none(projekt)) expect(known, `${path}: Projekt ${projekt}`).toContain(projekt);
    }
  });
});

describe('Domäne je Session', () => {
  it('Sprints mit Reife bereit nennen die Domänen ihrer Sessions', () => {
    const wrong = sprintInfos.filter((s) => s.reife === 'bereit' && !domainListOk(s));
    expect(wrong.map((s) => `${s.id}: ${s.domains.join(', ')} ≠ ${[...new Set(s.sessionDomains)].join(', ')}`)).toEqual([]);
  });

  it('höchstens eine Session je Domäne in Arbeit', () => {
    expect(busyDomains(sessions)).toEqual([]);
  });
});
