import { Button, Flex, Heading, Text, TextField } from '@radix-ui/themes';
import { useEffect, useMemo, useState } from 'react';
import { backend, type TaskCatalog, type TaskInfo, type TaskRun } from '../api';
import { errorText } from '../lib/errors';
import { formatDuration } from '../lib/format';
import { loadText, savePref } from '../lib/prefs';
import { ConsoleView } from '../logs/ConsoleView';
import { ActionButton, NoticeCard, StatusBadge } from '../ui/parts';
import { countTasks, describeRun, filterNamespaces, findTask, splitArgs, taskTooltip, upsertRun } from './tasks';

/** Reiter `Tasks`: links der Katalog nach Namensräumen mit Filter, rechts Start/Stopp und Ausgabe des gewählten Tasks. */
export function TasksPage() {
  const [catalog, setCatalog] = useState<TaskCatalog | null>(null);
  const [loadError, setLoadError] = useState('');
  const [runs, setRuns] = useState<TaskRun[]>([]);
  const [query, setQuery] = useState('');
  const [selected, setSelected] = useState(() => loadText('task', ''));
  const [collapsed, setCollapsed] = useState<Set<string>>(new Set());
  const [reloading, setReloading] = useState(false);

  useEffect(() => {
    const off = backend.on('task:state', (run) => setRuns((list) => upsertRun(list, run)));
    backend.taskRuns().then((r) => setRuns((cur) => r.reduce(upsertRun, cur)), () => undefined);
    backend.tasks().then(setCatalog, (e) => setLoadError(errorText(e)));
    return off;
  }, []);

  const reload = async () => {
    setReloading(true);
    try {
      setCatalog(await backend.tasksReload());
    } catch (e) {
      setLoadError(errorText(e));
    } finally {
      setReloading(false);
    }
  };
  const choose = (name: string) => {
    setSelected(name);
    savePref('task', name);
  };
  const toggle = (ns: string) =>
    setCollapsed((s) => {
      const next = new Set(s);
      if (!next.delete(ns)) next.add(ns);
      return next;
    });

  const visible = useMemo(() => filterNamespaces(catalog?.namespaces ?? [], query), [catalog, query]);

  if (loadError) return <NoticeCard title="Task-Katalog nicht abrufbar" tone="error">{loadError}</NoticeCard>;
  if (!catalog) return <Text color="gray">Lade Task-Katalog …</Text>;
  if (catalog.error && catalog.namespaces.length === 0) {
    return <NoticeCard title="Task-Katalog nicht ladbar" tone="error">task --list-all --json --no-status: {catalog.error}</NoticeCard>;
  }
  const task = findTask(catalog.namespaces, selected);
  return (
    <div className="tk-page">
      <TaskList catalog={catalog} visible={visible} runs={runs} selected={selected} query={query} collapsed={collapsed}
        reloading={reloading} onQuery={setQuery} onReload={() => void reload()} onToggle={toggle} onSelect={choose} />
      <section className="tk-detail">
        {task ? <TaskDetail key={task.name} task={task} run={runs.find((r) => r.name === task.name)} /> : (
          <NoticeCard title="Kein Task gewählt" tone="neutral">Links einen Task wählen: Start, Stopp und Ausgabe erscheinen hier.</NoticeCard>
        )}
      </section>
    </div>
  );
}

interface TaskListProps {
  catalog: TaskCatalog;
  visible: TaskCatalog['namespaces'];
  runs: TaskRun[];
  selected: string;
  query: string;
  collapsed: Set<string>;
  reloading: boolean;
  onQuery: (q: string) => void;
  onReload: () => void;
  onToggle: (ns: string) => void;
  onSelect: (name: string) => void;
}

function TaskList({ catalog, visible, runs, selected, query, collapsed, reloading, onQuery, onReload, onToggle, onSelect }: TaskListProps) {
  const running = runs.filter((r) => r.state === 'running').length;
  return (
    <section className="tk-list">
      <Flex gap="2" align="center">
        <TextField.Root className="tk-filter" size="2" placeholder="Filter: Name oder Beschreibung" value={query}
          onChange={(e) => onQuery(e.target.value)} onKeyDown={(e) => e.key === 'Escape' && onQuery('')} />
        <Button variant="soft" loading={reloading} onClick={onReload}>Neu laden</Button>
      </Flex>
      <div className="tk-tree">
        {visible.map((ns) => (
          <div key={ns.name}>
            <button type="button" className="tk-ns" onClick={() => onToggle(ns.name)} aria-expanded={!collapsed.has(ns.name)}>
              {collapsed.has(ns.name) ? '▸' : '▾'} {ns.name} <span className="tk-ns-count">{ns.tasks.length}</span>
            </button>
            {!collapsed.has(ns.name) &&
              ns.tasks.map((t) => (
                <TaskRow key={t.name} task={t} run={runs.find((r) => r.name === t.name)} active={t.name === selected}
                  onSelect={() => onSelect(t.name)} />
              ))}
          </div>
        ))}
        {visible.length === 0 && <Text color="gray" size="2">Kein Task passt zum Filter.</Text>}
      </div>
      <Text size="1" color="gray">
        {query.trim() ? `${countTasks(visible)} von ${catalog.count}` : catalog.count} Tasks
        {running > 0 && ` · ${running} ${running === 1 ? 'läuft' : 'laufen'}`}
        {catalog.error && ` · veraltet: ${catalog.error}`}
      </Text>
    </section>
  );
}

function TaskRow({ task, run, active, onSelect }: { task: TaskInfo; run?: TaskRun; active: boolean; onSelect: () => void }) {
  const badge = describeRun(run);
  return (
    <button type="button" className={active ? 'tk-row tk-row-on' : 'tk-row'} title={taskTooltip(task)} onClick={onSelect}>
      <span className="tk-row-name">{task.leaf}</span>
      <span className="tk-row-desc">{task.desc}</span>
      {run && <StatusBadge tone={badge.tone}>{badge.label}</StatusBadge>}
    </button>
  );
}

function TaskDetail({ task, run }: { task: TaskInfo; run?: TaskRun }) {
  const [args, setArgs] = useState('');
  const [error, setError] = useState('');
  const running = run?.state === 'running';
  const badge = describeRun(run);
  const act = (fn: () => Promise<unknown>) => async () => {
    setError('');
    try {
      await fn();
    } catch (e) {
      setError(errorText(e));
    }
  };
  return (
    <div className="tk-detail-body">
      <Flex align="center" gap="3" wrap="wrap">
        <Heading size="4">{task.name}</Heading>
        <StatusBadge tone={badge.tone}>{badge.label}</StatusBadge>
        {run && !running && run.durationMs > 0 && <Text size="2" color="gray">{formatDuration(run.durationMs)}</Text>}
        {running && <Text size="2" color="gray">PID {run?.pid}</Text>}
      </Flex>
      {task.desc && <Text as="p" size="2" color="gray">{task.desc} · {task.file}:{task.line}</Text>}
      <Flex gap="2" align="center">
        <TextField.Root className="tk-args" size="2" placeholder="Argumente nach -- (z. B. -run Foo)" value={args}
          disabled={running} onChange={(e) => setArgs(e.target.value)} />
        {running
          ? <ActionButton color="red" onClick={act(() => backend.taskStop(task.name))}>Stoppen</ActionButton>
          : <ActionButton onClick={act(() => backend.taskStart(task.name, splitArgs(args)))}>Starten</ActionButton>}
      </Flex>
      {error && <p className="svc-error">{error}</p>}
      {run
        ? <ConsoleView name={`task:${task.name}`} />
        : <NoticeCard title="Noch nicht gelaufen" tone="neutral">Die Ausgabe erscheint hier live, sobald der Task gestartet ist.</NoticeCard>}
    </div>
  );
}
