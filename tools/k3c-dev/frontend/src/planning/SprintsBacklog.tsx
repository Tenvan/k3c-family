import { Text } from '@radix-ui/themes';
import { useEffect, useState, type CSSProperties } from 'react';
import { backend, type PlanningData, type PlanSprint } from '../api';
import { errorText } from '../lib/errors';
import { loadText, savePref } from '../lib/prefs';
import { Splitter } from '../tasks/TaskBars';
import { NoticeCard } from '../ui/parts';
import { BacklogView, useTarget } from './Backlog';
import { PlanDetail } from './Detail';
import { foldAll, unfold } from './fold';
import { GitHubBar, useGitHub } from './GitHubBar';
import { exists, filterProjects, filterSprints, filterTickets, findSession, parseFilter, parseSplit, projectHits, sortSprints, SPLIT_DEFAULT, type PlanFilter } from './planning';
import { PlanToolbar } from './PlanToolbar';
import { ProjectsView } from './ProjectsView';
import { rankedCount, visibleSprints } from './projects';
import { CopyPrompt } from './PromptParts';
import { NEW_SPRINT, openSessions, promptSessions, type SprintTarget } from './prompts';
import { SprintCard } from './SprintCard';

const PREF = 'planning.filter';

/** Gemerkter Anteil eines Splitters in Prozent, immer zwischen 20 und 80. */
function useSplit(key: string, dflt: number): [number, (v: number) => void, () => void] {
  const [split, setState] = useState(() => parseSplit(loadText(key, String(dflt))));
  const set = (v: number) => {
    const n = parseSplit(String(v));
    setState(n);
    savePref(key, String(n));
  };
  return [split, set, () => set(dflt)];
}

/** Planung aus planning.Data (dieselben Daten wie plan_list) im erp-qs-Look (B-439): GitHub-Leiste, Filterleiste,
 *  links die Projekte, rechts der Backlog und darunter das Detail der Auswahl, beide Grenzen mit Splitter. Lädt bei
 *  `planning:changed` neu; Filter (ohne Freitext), Auswahl und Splitter bleiben gemerkt. */
export function SprintsBacklog() {
  const [data, setData] = useState<PlanningData | null>(null);
  const [error, setError] = useState('');
  const [filter, setFilterState] = useState<PlanFilter>(() => parseFilter(loadText(PREF, '{}')));
  const [gh, reloadGh] = useGitHub();
  const [checked, setChecked] = useState<ReadonlySet<string>>(new Set()); // markierte Sessions (Nr.)
  const [cols, setCols, resetCols] = useSplit('planning.split.cols', 50);
  const [rows, setRows, resetRows] = useSplit('planning.split', SPLIT_DEFAULT);
  const [, goal, choose] = useTarget((data?.sprints ?? []).filter((s) => s.status !== 'erledigt').map((s) => s.id));
  const setFilter = (f: PlanFilter) => {
    setFilterState(f);
    savePref(PREF, JSON.stringify(f));
  };
  const load = () => backend.planningData().then((d) => { setData(d); setError(''); }, (e) => setError(errorText(e)));
  useEffect(() => {
    void load();
    return backend.on('planning:changed', () => void load()); // Wächter in Go: Datei in docs/ geändert
  }, []);
  // Verschwundene Auswahl (archiviert, gelöscht) schließt das Detail.
  useEffect(() => { if (data && filter.sel && !exists(data, filter.sel)) setFilter({ ...filter, sel: '' }); }, [data]);
  if (error) return <NoticeCard title="Planung nicht geladen" tone="error">{error}</NoticeCard>;
  if (data === null) return <Text color="gray">Lade Planung …</Text>;
  const sprints = sortSprints(filterSprints(data, filter), gh?.sprints);
  const tickets = filterTickets(data, filter);
  // Der Projekt-Filter wählt die Projekte, der Backlog-Filter die Sprints darin und den Backlog (B-401).
  const shown: PlanningData = { ...data, projects: projectHits(filterProjects(data, filter), sprints, tickets, filter) };
  const select = (id: string) => setFilter({ ...filter, sel: filter.sel === id ? '' : id });
  const pick = (id: string) => setFilter({ ...filter, sel: id });
  const reveal = (id: string) => { pick(id); revealIn(data, id); };
  const check = (nr: string, on: boolean) => setChecked((cur) => { const n = new Set(cur); if (on) n.add(nr); else n.delete(nr); return n; });
  const card = (s: PlanSprint) => (
    <SprintCard key={s.id} sprint={s} gh={gh?.sprints[s.id.toUpperCase()]} sel={filter.sel} onSelect={select} onPick={pick} checked={checked} onCheck={check} />
  );
  const picked = sprints.flatMap(openSessions).filter((it) => checked.has(it.session.nr)); // fertige oder verschwundene fallen heraus
  return (
    <div className="pl-board">
      <GitHubBar gh={gh} reload={reloadGh} />
      <PlanToolbar data={data} filter={filter} setFilter={setFilter} hits={(shown.projects?.length ?? 0) + tickets.length} />
      <div className="pl-grid" style={{ '--pl-split': `${cols}%`, '--pl-detail': rows } as CSSProperties}>
        <section className="pl-pane pl-projects" aria-label="Projekte">
          <ColumnHead data={shown} all={visibleSprints(data, shown.projects)} shown={sprints} />
          {picked.length > 0 && (
            <div className="pl-picked">
              <Text size="1" weight="medium">{picked.length} Session{picked.length > 1 ? 's' : ''} markiert</Text>
              <CopyPrompt prompt={promptSessions(picked, data.projects)} what={`${picked.length} markierte Sessions`} />
              <button type="button" className="pl-textlink" onClick={() => setChecked(new Set())}>Auswahl aufheben</button>
            </div>
          )}
          <div className="pl-list">
            {data.projects?.length
              ? <ProjectsView data={shown} sprints={sprints} tickets={tickets} sel={filter.sel} onPick={pick} renderSprint={card} reload={load} />
              : sprints.map(card)}
            {shown.projects?.length === 0 && sprints.length === 0 && <Text color="gray" size="2">Nichts passt zum Filter.</Text>}
          </div>
        </section>
        <Splitter className="pl-split-v" value={cols} onChange={setCols} onReset={resetCols} />
        <SidePane data={data} tickets={tickets} sel={filter.sel} onPick={select} onReveal={reveal} onClose={() => setFilter({ ...filter, sel: '' })}
          rows={rows} setRows={setRows} resetRows={resetRows} goal={goal} choose={choose} />
      </div>
    </div>
  );
}

interface SideProps {
  data: PlanningData;
  tickets: PlanningData['tickets'];
  sel: string;
  onPick: (id: string) => void;
  onReveal: (id: string) => void;
  onClose: () => void;
  rows: number;
  setRows: (v: number) => void;
  resetRows: () => void;
  goal: string;
  choose: (v: string) => void;
}

/** Rechte Spalte: Backlog, bei einer Auswahl darunter Splitter und Detail. */
function SidePane({ data, tickets, sel, onPick, onReveal, onClose, rows, setRows, resetRows, goal, choose }: SideProps) {
  const target: SprintTarget = goal === NEW_SPRINT ? { kind: 'new' } : { kind: 'sprint', id: goal };
  return (
    <section className={`pl-pane pl-side${sel ? ' has-detail' : ''}`} aria-label="Backlog und Detail">
      <div className="pl-side-backlog">
        <BacklogView data={data} tickets={tickets} sel={sel} onPick={onPick} target={target} goal={goal} choose={choose} />
      </div>
      {sel && (
        <>
          <Splitter horizontal className="pl-split-h" value={rows} onChange={setRows} onReset={resetRows} />
          <div className="pl-side-detail">
            <PlanDetail data={data} sel={sel} onSelect={onReveal} onClose={onClose} target={target} />
          </div>
        </>
      )}
    </section>
  );
}

/** Auswahl von außen (Detail, Abhängigkeits-Link): Projekt und Sprint klappen auf, die Karte scrollt in Sicht. */
function revealIn(data: PlanningData, id: string) {
  const sprint = findSession(data, id)?.sprint ?? data.sprints.find((s) => s.id === id);
  unfold([sprint?.project ?? '', sprint?.id ?? '', id].filter(Boolean));
  const el = sprint ? `pl-sp-${sprint.id}` : `pl-pj-${id}`;
  setTimeout(() => document.getElementById(el)?.scrollIntoView({ block: 'nearest', behavior: 'smooth' }), 0);
}

/** Kopf der linken Spalte: Zähler und „alle ein-/aufklappen“ (B-364). */
function ColumnHead({ data, all, shown }: { data: PlanningData; all: PlanSprint[]; shown: PlanSprint[] }) {
  const ids = ['area:ABN', 'area:ohne', ...(data.projects ?? []).map((p) => p.id), ...shown.map((s) => s.id)];
  return (
    <div className="pl-pane-head">
      <Text className="pl-heading">{data.projects?.length ? 'Projekte' : 'Sprints'}</Text>
      <Text size="1" color="gray">
        {data.projects?.length ? `${data.projects.length} · ${rankedCount(data.projects)} nach Rang · ` : ''}
        Sprints {shown.length} von {all.length} · {all.filter((s) => s.status === 'erledigt').length} erledigt
      </Text>
      <span className="pl-foldall">
        <button type="button" className="pl-textlink" onClick={() => foldAll(ids, true)}>alle einklappen</button>
        <button type="button" className="pl-textlink" onClick={() => foldAll(ids, false)}>alle aufklappen</button>
      </span>
    </div>
  );
}
