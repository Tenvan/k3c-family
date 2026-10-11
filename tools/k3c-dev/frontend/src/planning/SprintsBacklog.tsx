import { Button, Checkbox, Flex, Text, TextField } from '@radix-ui/themes';
import { useEffect, useState, type CSSProperties } from 'react';
import { backend, type GitHubData, type PlanningData, type PlanSprint } from '../api';
import { ProjectsView } from './ProjectsView';
import { rankedCount, visibleSprints } from './projects';
import { foldAll } from './fold';
import { errorText } from '../lib/errors';
import { loadText, savePref } from '../lib/prefs';
import { Splitter } from '../tasks/TaskBars';
import { ActionButton, NoticeCard, StatusBadge } from '../ui/parts';
import { BacklogList } from './Backlog';
import { PlanDetail } from './Detail';
import { unfold } from './fold';
import { findSession } from './planning';
import { SprintCard } from './SprintCard';
import { CopyPrompt } from './PromptParts';
import { openSessions, promptSessions } from './prompts';
import { domains, filterProjects, filterSprints, filterTickets, parseFilter, parseSplit, PROJECT_STATES, PQUICK, QUICK, SPLIT_DEFAULT, sortSprints, toggle, type PlanFilter } from './planning';

const PREF = 'planning.filter';
const SPLIT_PREF = 'planning.split';
const PRIOS = ['hoch', 'mittel', 'niedrig'];
const ENVS = ['offline', 'live'];

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
  // Der Projekt-Filter wählt die Projekte, der Backlog-Filter die Sprints darin und den Backlog (B-401).
  const shown: PlanningData = { ...data, projects: filterProjects(data, filter) };
  const select = (nr: string) => setFilter({ ...filter, sel: filter.sel === nr ? '' : nr });
  const pick = (id: string) => setFilter({ ...filter, sel: id });
  const reveal = (id: string) => { pick(id); revealIn(data, id); };
  const check = (nr: string, on: boolean) => setChecked((cur) => { const n = new Set(cur); if (on) n.add(nr); else n.delete(nr); return n; });
  // in Planungs-Reihenfolge; fertige oder verschwundene fallen heraus
  const picked = sprints.flatMap(openSessions).filter((it) => checked.has(it.session.nr));
  const card = (s: PlanSprint) => (
    <SprintCard key={s.id} sprint={s} gh={gh?.sprints[s.id.toUpperCase()]} sel={filter.sel} onSelect={select} onPick={pick} checked={checked}
      onCheck={check} />
  );
  return (
    <div className="pl-board">
      <GitHubBar gh={gh} reload={reloadGh} />
      <div className="pl-grid">
        <section className="pl-col">
          <ColumnHead data={shown} all={visibleSprints(data, shown.projects)} shown={sprints} />
          {data.projects?.length ? <ProjectBar data={data} filter={filter} setFilter={setFilter} hits={shown.projects?.length ?? 0} /> : null}
          {picked.length > 0 && (
            <div className="pl-picked">
              <Text size="1" weight="medium">{picked.length} Session{picked.length > 1 ? 's' : ''} markiert</Text>
              <CopyPrompt prompt={promptSessions(picked, data.projects)} what={`${picked.length} markierte Sessions`} />
              <button type="button" className="pl-textlink" onClick={() => setChecked(new Set())}>Auswahl aufheben</button>
            </div>
          )}
          {data.projects?.length
            ? <ProjectsView data={shown} sprints={sprints} tickets={data.tickets} sel={filter.sel} onPick={pick} renderSprint={card} reload={load} />
            : sprints.map(card)}
          {sprints.length === 0 && <Text color="gray">Kein Sprint passt zum Filter.</Text>}
        </section>
        <BacklogColumn data={data} tickets={tickets} filter={filter} setFilter={setFilter} onReveal={reveal} />
      </div>
    </div>
  );
}

/** Anteil des Detailbereichs (B-400): gemerkt, immer zwischen 20 und 80 %. */
function useSplit(): [number, (v: number) => void] {
  const [split, setState] = useState(() => parseSplit(loadText(SPLIT_PREF, '')));
  return [split, (v) => {
    const n = parseSplit(String(v));
    setState(n);
    savePref(SPLIT_PREF, String(n));
  }];
}

interface ColumnProps {
  data: PlanningData;
  tickets: PlanningData['tickets'];
  filter: PlanFilter;
  setFilter: (f: PlanFilter) => void;
  onReveal: (id: string) => void;
}

/** Rechte Spalte: Backlog mit eigenem Filter, darunter Splitter und Detail der Auswahl. */
function BacklogColumn({ data, tickets, filter, setFilter, onReveal }: ColumnProps) {
  const [split, setSplit] = useSplit();
  return (
    <section className="pl-col pl-right" style={{ '--pl-detail': `${split}%` } as CSSProperties}>
      <div className="pl-scroll">
        <h2 className="pl-h">Backlog <small>{tickets.length} von {data.tickets.length} offenen Tickets</small></h2>
        <BacklogBar data={data} filter={filter} setFilter={setFilter} />
        <BacklogList data={data} tickets={tickets} sel={filter.sel} />
      </div>
      {filter.sel && <Splitter horizontal className="pl-split" value={split} onChange={setSplit} onReset={() => setSplit(SPLIT_DEFAULT)} />}
      <PlanDetail data={data} sel={filter.sel} onSelect={onReveal} onClose={() => setFilter({ ...filter, sel: '' })} />
    </section>
  );
}

/** Auswahl von außen (Detail, Abhängigkeits-Link): Projekt und Sprint klappen auf, die Karte scrollt in Sicht. */
function revealIn(data: PlanningData, id: string) {
  const sprint = findSession(data, id)?.sprint ?? data.sprints.find((s) => s.id === id);
  unfold([sprint?.project ?? '', sprint?.id ?? '', id].filter(Boolean));
  setTimeout(() => document.getElementById(`pl-sp-${sprint?.id ?? id}`)?.scrollIntoView({ block: 'nearest', behavior: 'smooth' }), 0);
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

/** Schalter-Chip der Filterleisten. */
function Chip({ label, on, click }: { label: string; on: boolean; click: () => void }) {
  return <Button size="1" radius="full" variant={on ? 'solid' : 'soft'} color={on ? undefined : 'gray'} onClick={click}>{label}</Button>;
}

/** Backlog-Filter (B-401): Suche, Schnellfilter, Prio, Umgebung und Projekt; wirkt auf Backlog und Sprint-Liste. */
function BacklogBar({ data, filter, setFilter }: { data: PlanningData; filter: PlanFilter; setFilter: (f: PlanFilter) => void }) {
  const projects = (data.projects ?? []).map((p) => p.id);
  const pick = (key: 'prios' | 'envs' | 'projs', v: string) => setFilter({ ...filter, [key]: toggle(filter[key], v) });
  const any = filter.q !== '' || filter.quick.length + filter.prios.length + filter.envs.length + filter.projs.length > 0;
  return (
    <Flex className="pl-bar" wrap="wrap" align="center" gap="2" aria-label="Backlog-Filter">
      <TextField.Root size="1" className="pl-search" placeholder="Suche: Nr., Titel, Ticket …" value={filter.q}
        onChange={(e) => setFilter({ ...filter, q: e.target.value })} />
      {QUICK.map((f) => <Chip key={f.id} label={f.label} on={filter.quick.includes(f.id)}
        click={() => setFilter({ ...filter, quick: toggle(filter.quick, f.id) })} />)}
      <span className="pl-sep" />
      {PRIOS.map((v) => <Chip key={v} label={`Prio ${v}`} on={filter.prios.includes(v)} click={() => pick('prios', v)} />)}
      {ENVS.map((v) => <Chip key={v} label={v} on={filter.envs.includes(v)} click={() => pick('envs', v)} />)}
      {projects.length > 0 && <span className="pl-sep" />}
      {projects.map((v) => <Chip key={v} label={v} on={filter.projs.includes(v)} click={() => pick('projs', v)} />)}
      {any && (
        <Button size="1" variant="ghost" color="gray"
          onClick={() => setFilter({ ...filter, q: '', quick: [], prios: [], envs: [], projs: [] })}>× zurücksetzen</Button>
      )}
      <Text as="label" size="1" color="gray">
        <Flex gap="1" align="center">
          <Checkbox size="1" checked={filter.done} onCheckedChange={(v) => setFilter({ ...filter, done: v === true })} />
          Erledigte zeigen ({data.done})
        </Flex>
      </Text>
    </Flex>
  );
}

/** Projekt-Filter (B-401): Status und Domäne; wirkt nur auf die Projekte. */
function ProjectBar({ data, filter, setFilter, hits }:
  { data: PlanningData; filter: PlanFilter; setFilter: (f: PlanFilter) => void; hits: number }) {
  const any = filter.pstatus.length + filter.pquick.length + filter.doms.length > 0;
  return (
    <Flex className="pl-bar" wrap="wrap" align="center" gap="2" aria-label="Projekt-Filter">
      {PROJECT_STATES.map((v) => <Chip key={v} label={v} on={filter.pstatus.includes(v)}
        click={() => setFilter({ ...filter, pstatus: toggle(filter.pstatus, v) })} />)}
      <span className="pl-sep" />
      {PQUICK.map((f) => <Chip key={f.id} label={f.label} on={filter.pquick.includes(f.id)}
        click={() => setFilter({ ...filter, pquick: toggle(filter.pquick, f.id) })} />)}
      <span className="pl-sep" />
      {domains(data).map((d) => <Chip key={d} label={d} on={filter.doms.includes(d)}
        click={() => setFilter({ ...filter, doms: toggle(filter.doms, d) })} />)}
      {any && <Button size="1" variant="ghost" color="gray" onClick={() => setFilter({ ...filter, pstatus: [], pquick: [], doms: [] })}>× zurücksetzen</Button>}
      <Text size="1" color="gray" className="pl-count">{hits} Projekte</Text>
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
function ColumnHead({ data, all, shown }: { data: PlanningData; all: PlanSprint[]; shown: PlanSprint[] }) {
  const count = <>{shown.length} von {all.length} · {all.filter((s) => s.status === 'erledigt').length} erledigt</>;
  if (!data.projects?.length) return <h2 className="pl-h">Sprints <small>{count}</small></h2>;
  return (
    <h2 className="pl-h">Projekte <small>{rankedCount(data.projects)} nach Rang · Sprints {count}</small>
      <FoldAll ids={['area:ABN', 'area:ohne', ...data.projects.map((p) => p.id), ...shown.map((s) => s.id)]} />
    </h2>
  );
}
