import { Button, Checkbox, Flex, Text, TextField } from '@radix-ui/themes';
import { useEffect, useState } from 'react';
import { backend, type GitHubData, type PlanningData, type PlanSprint } from '../api';
import { ProjectsView } from './ProjectsView';
import { rankedCount } from './projects';
import { foldAll } from './fold';
import { errorText } from '../lib/errors';
import { loadText, savePref } from '../lib/prefs';
import { ActionButton, NoticeCard, StatusBadge } from '../ui/parts';
import { BacklogList, SessionDetail } from './Backlog';
import { SprintCard } from './SprintCard';
import { CopyPrompt } from './PromptParts';
import { openSessions, promptSessions } from './prompts';
import { domains, filterSprints, filterTickets, parseFilter, QUICK, sortSprints, toggle, type PlanFilter } from './planning';

const PREF = 'planning.filter';

/** Sprints & Backlog aus planning.Data (dieselben Daten wie plan_list). Lädt bei `planning:changed` neu; Filter und
 *  Auswahl bleiben dabei stehen und werden gemerkt. */
export function SprintsBacklog() {
  const [data, setData] = useState<PlanningData | null>(null);
  const [error, setError] = useState('');
  const [filter, setFilterState] = useState<PlanFilter>(() => parseFilter(loadText(PREF, '{}')));
  const [gh, reloadGh] = useGitHub();
  const [checked, setChecked] = useState<ReadonlySet<string>>(new Set()); // markierte Sessions (Nr.)
  const setFilter = (f: PlanFilter) => {
    setFilterState(f);
    savePref(PREF, JSON.stringify(f));
  };
  const load = () => backend.planningData().then((d) => { setData(d); setError(''); }, (e) => setError(errorText(e)));
  useEffect(() => {
    void load();
    return backend.on('planning:changed', () => void load()); // Wächter in Go: Datei in docs/ geändert
  }, []);
  if (error) return <NoticeCard title="Planung nicht geladen" tone="error">{error}</NoticeCard>;
  if (data === null) return <Text color="gray">Lade Planung …</Text>;
  const sprints = sortSprints(filterSprints(data, filter), gh?.sprints);
  const tickets = filterTickets(data, filter);
  const select = (nr: string) => setFilter({ ...filter, sel: filter.sel === nr ? '' : nr });
  const check = (nr: string, on: boolean) => setChecked((cur) => { const n = new Set(cur); if (on) n.add(nr); else n.delete(nr); return n; });
  // in Planungs-Reihenfolge; fertige oder verschwundene fallen heraus
  const picked = sprints.flatMap(openSessions).filter((it) => checked.has(it.session.nr));
  const card = (s: PlanSprint) => (
    <SprintCard key={s.id} sprint={s} gh={gh?.sprints[s.id.toUpperCase()]} sel={filter.sel} onSelect={select} checked={checked} onCheck={check} />
  );
  return (
    <div className="pl-board">
      <FilterBar data={data} filter={filter} setFilter={setFilter} hits={sprints.length + tickets.length} />
      <GitHubBar gh={gh} reload={reloadGh} />
      <div className="pl-grid">
        <section className="pl-col">
          <ColumnHead data={data} shown={sprints} />
          {picked.length > 0 && (
            <div className="pl-picked">
              <Text size="1" weight="medium">{picked.length} Session{picked.length > 1 ? 's' : ''} markiert</Text>
              <CopyPrompt prompt={promptSessions(picked, data.projects)} what={`${picked.length} markierte Sessions`} />
              <button type="button" className="pl-textlink" onClick={() => setChecked(new Set())}>Auswahl aufheben</button>
            </div>
          )}
          {data.projects?.length
            ? <ProjectsView data={data} sprints={sprints} tickets={tickets} sel={filter.sel} renderSprint={card} reload={load} />
            : sprints.map(card)}
          {sprints.length === 0 && <Text color="gray">Kein Sprint passt zum Filter.</Text>}
        </section>
        <section className="pl-col pl-right">
          <div className="pl-scroll">
            <h2 className="pl-h">Backlog <small>{tickets.length} von {data.tickets.length} offenen Tickets</small></h2>
            <BacklogList data={data} tickets={tickets} sel={filter.sel} />
          </div>
          <SessionDetail data={data} sel={filter.sel} onSelect={select} onClose={() => setFilter({ ...filter, sel: '' })} />
        </section>
      </div>
    </div>
  );
}

/** GitHub-Stand: beim Öffnen und bei `planning:changed` aus dem Zwischenspeicher, per Knopf frisch. */
function useGitHub(): [GitHubData | null, () => Promise<void>] {
  const [gh, setGh] = useState<GitHubData | null>(null);
  const load = (force: boolean) => backend.githubStatus(force).then(setGh, () => setGh(null));
  useEffect(() => {
    void load(false);
    return backend.on('planning:changed', () => void load(false));
  }, []);
  return [gh, () => load(true)];
}

/** Kopfzeile: letzter CI-Lauf auf develop, Hinweis von gh, Neu laden. */
function GitHubBar({ gh, reload }: { gh: GitHubData | null; reload: () => Promise<void> }) {
  const tone = { grün: 'ok', rot: 'error', läuft: 'warn', '–': 'neutral' } as const;
  return (
    <Flex className="pl-ghbar" align="center" gap="2" wrap="wrap">
      <Text size="1" color="gray">GitHub</Text>
      {gh?.develop && (
        <button type="button" className="pl-link" onClick={() => backend.openUrl(gh.develop!.url)} title={gh.develop.created}>
          <StatusBadge tone={tone[gh.develop.ci]}>develop · CI {gh.develop.ci}</StatusBadge>
        </button>
      )}
      {gh?.develop && <Text size="1" color="gray" className="pl-ellipsis">{gh.develop.title}</Text>}
      {gh?.error && <StatusBadge tone="warn">{gh.error}</StatusBadge>}
      {gh === null && <Text size="1" color="gray">lädt …</Text>}
      <ActionButton size="1" variant="ghost" color="gray" className="pl-count" onClick={reload}>Neu laden</ActionButton>
    </Flex>
  );
}

function FilterBar({ data, filter, setFilter, hits }:
  { data: PlanningData; filter: PlanFilter; setFilter: (f: PlanFilter) => void; hits: number }) {
  const chip = (key: string, label: string, on: boolean, click: () => void) => (
    <Button key={key} size="1" radius="full" variant={on ? 'solid' : 'soft'} color={on ? undefined : 'gray'} onClick={click}>
      {label}
    </Button>
  );
  const any = filter.q !== '' || filter.doms.length > 0 || filter.quick.length > 0;
  return (
    <Flex className="pl-bar" wrap="wrap" align="center" gap="2">
      <TextField.Root size="1" className="pl-search" placeholder="Suche: Nr., Titel, Ticket …" value={filter.q}
        onChange={(e) => setFilter({ ...filter, q: e.target.value })} />
      {QUICK.map((f) => chip(f.id, f.label, filter.quick.includes(f.id), () => setFilter({ ...filter, quick: toggle(filter.quick, f.id) })))}
      <span className="pl-sep" />
      {domains(data).map((d) => chip(d, d, filter.doms.includes(d), () => setFilter({ ...filter, doms: toggle(filter.doms, d) })))}
      {any && (
        <Button size="1" variant="ghost" color="gray" onClick={() => setFilter({ ...filter, q: '', doms: [], quick: [] })}>
          × zurücksetzen
        </Button>
      )}
      <span className="pl-sep" />
      <Text as="label" size="1" color="gray">
        <Flex gap="1" align="center">
          <Checkbox size="1" checked={filter.done} onCheckedChange={(v) => setFilter({ ...filter, done: v === true })} />
          Erledigte zeigen ({data.done})
        </Flex>
      </Text>
      <Text size="1" color="gray" className="pl-count">{hits} Treffer</Text>
    </Flex>
  );
}

/** Alle Projekte und Sprints auf einmal ein- oder ausklappen (B-364). */
function FoldAll({ ids }: { ids: string[] }) {
  return (
    <span className="pl-foldall">
      <button type="button" className="pl-textlink" onClick={() => foldAll(ids, true)}>alle einklappen</button>
      <button type="button" className="pl-textlink" onClick={() => foldAll(ids, false)}>alle aufklappen</button>
    </span>
  );
}

/** Kopf der linken Spalte: mit Projekten „Projekte“, sonst „Sprints“ (B-364). */
function ColumnHead({ data, shown }: { data: PlanningData; shown: PlanSprint[] }) {
  const count = <>{shown.length} von {data.sprints.length} · {data.done} erledigt</>;
  if (!data.projects?.length) return <h2 className="pl-h">Sprints <small>{count}</small></h2>;
  return (
    <h2 className="pl-h">Projekte <small>{rankedCount(data.projects)} nach Rang · Sprints {count}</small>
      <FoldAll ids={['area:ABN', 'area:ohne', ...data.projects.map((p) => p.id), ...shown.map((s) => s.id)]} />
    </h2>
  );
}
