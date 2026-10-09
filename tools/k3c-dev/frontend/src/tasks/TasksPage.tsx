import { Text } from '@radix-ui/themes';
import { useMemo, useState } from 'react';
import { isStringList, loadJSON, loadText, saveJSON, savePref } from '../lib/prefs';
import { NoticeCard } from '../ui/parts';
import { Splitter, TasksFoot, TasksHead } from './TaskBars';
import { TaskDetail } from './TaskDetail';
import { TaskTree } from './TaskTree';
import { countTasks, favoriteTasks, filterNamespaces, findTask, onlyAllowed } from './tasks';
import { useTasks } from './useTasks';

const SPLIT_DEFAULT = 40;
const isSplit = (v: unknown): v is number => typeof v === 'number' && v >= 20 && v <= 80;
const toggled = (list: string[], x: string) => (list.includes(x) ? list.filter((y) => y !== x) : [...list, x]);

/** Gemerkter Zustand der Seite: Auswahl, 🔑, Eingeklapptes, Favoriten, Splitter (Freitext nicht). */
function useTaskPrefs() {
  const [onlyKey, setOnlyKey] = useState(() => loadText('tasks.onlyAllowed', '') === '1');
  const [selected, setSelected] = useState(() => loadText('tasks.selected', ''));
  const [collapsed, setCollapsed] = useState(() => loadJSON('tasks.collapsed', isStringList, []));
  const [pinned, setPinned] = useState(() => loadJSON('tasks.favorites', isStringList, []));
  const [split, setSplit] = useState(() => loadJSON('tasks.split', isSplit, SPLIT_DEFAULT));
  const keep = <T,>(set: (v: T) => void, key: string) => (v: T) => {
    set(v);
    saveJSON(key, v);
  };
  return {
    onlyKey, selected, collapsed, pinned, split,
    setOnlyKey: (v: boolean) => { setOnlyKey(v); savePref('tasks.onlyAllowed', v ? '1' : ''); },
    select: (n: string) => { setSelected(n); savePref('tasks.selected', n); },
    toggleNs: (ns: string) => keep(setCollapsed, 'tasks.collapsed')(toggled(collapsed, ns)),
    togglePin: (n: string) => keep(setPinned, 'tasks.favorites')(toggled(pinned, n)),
    setSplit: keep(setSplit, 'tasks.split'),
  };
}

/** Reiter `Tasks` (Workbench-Spec § 4): Kopf mit Filter und Schaltern, Baum | Splitter | Ausgabe, Fußzeile. */
export function TasksPage() {
  const data = useTasks();
  const prefs = useTaskPrefs();
  const [query, setQuery] = useState('');
  const { catalog, gates } = data;
  const visible = useMemo(() => {
    const ns = filterNamespaces(catalog?.namespaces ?? [], query);
    return prefs.onlyKey ? onlyAllowed(ns, gates.states) : ns;
  }, [catalog, query, prefs.onlyKey, gates]);

  if (data.loadError && !catalog) return <NoticeCard title="Task-Katalog nicht abrufbar" tone="error">{data.loadError}</NoticeCard>;
  if (!catalog) return <Text color="gray">Lade Task-Katalog …</Text>;
  const task = findTask(catalog.namespaces, prefs.selected);
  const filter = [query.trim() && `„${query.trim()}“`, prefs.onlyKey && '🔑'].filter(Boolean).join(' ');
  return (
    <div className="tk-wrap">
      <div className="tk-page">
        <TasksHead query={query} onlyKey={prefs.onlyKey} pending={gates.pending} reloading={data.reloading} onQuery={setQuery}
          onOnlyKey={prefs.setOnlyKey} onSave={data.saveGates} onReload={data.reload} />
        {data.gateError && <p className="svc-error">{data.gateError}</p>}
        <div className="tk-body" style={{ '--tk-split': `${prefs.split}%` } as React.CSSProperties}>
          <TaskTree visible={visible} favorites={favoriteTasks(visible, prefs.pinned)} runs={data.runs} gates={gates.states}
            pinned={prefs.pinned} selected={prefs.selected} collapsed={prefs.collapsed} onToggleNs={prefs.toggleNs}
            onSelect={prefs.select} onGate={data.toggleGate} onPin={prefs.togglePin} />
          <Splitter value={prefs.split} onChange={prefs.setSplit} onReset={() => prefs.setSplit(SPLIT_DEFAULT)} />
          <section className="tk-detail">
            {task ? <TaskDetail key={task.name} task={task} run={data.runs.find((r) => r.name === task.name)} history={data.history} /> : (
              <NoticeCard title="Kein Task gewählt" tone="neutral">Links einen Task wählen: Start, Stopp und Ausgabe erscheinen hier.</NoticeCard>
            )}
          </section>
        </div>
        <TasksFoot catalog={catalog} shown={countTasks(visible)} filter={filter} running={data.runs.filter((r) => r.state === 'running').length}
          error={catalog.error || data.loadError} />
      </div>
    </div>
  );
}
