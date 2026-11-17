import { useMemo, useState } from 'react';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { Splitter, Text } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { NoticeCard } from '@webkit/molecules';
import { isStringList, loadJSON, loadText, prefsStore, saveJSON, savePref } from '../lib/prefs';
import { TasksFoot, TasksHead } from './TaskBars';
import { TaskDetail } from './TaskDetail';
import { TaskTree } from './TaskTree';
import { countTasks, favoriteTasks, filterNamespaces, findTask } from './tasks';
import { useTasks } from './useTasks';

const SPLIT_DEFAULT = 40;
const toggled = (list: string[], x: string) => (list.includes(x) ? list.filter((y) => y !== x) : [...list, x]);

/** Gemerkter Zustand der Seite: Auswahl, Eingeklapptes, Favoriten, Splitter (Freitext nicht). */
function useTaskPrefs() {
  const [selected, setSelected] = useState(() => loadText('tasks.selected', ''));
  const [collapsed, setCollapsed] = useState(() => loadJSON('tasks.collapsed', isStringList, []));
  const [pinned, setPinned] = useState(() => loadJSON('tasks.favorites', isStringList, []));
  const [split, setSplit, resetSplit] = useSplit(prefsStore, 'tasks.split', SPLIT_DEFAULT);
  const keep = <T,>(set: (v: T) => void, key: string) => (v: T) => {
    set(v);
    saveJSON(key, v);
  };
  return {
    selected, collapsed, pinned, split, setSplit, resetSplit,
    select: (n: string) => { setSelected(n); savePref('tasks.selected', n); },
    toggleNs: (ns: string) => keep(setCollapsed, 'tasks.collapsed')(toggled(collapsed, ns)),
    togglePin: (n: string) => keep(setPinned, 'tasks.favorites')(toggled(pinned, n)),
  };
}

/** Reiter `Tasks` (Workbench-Spec § 4): Kopf mit Filter und Schaltern, Baum | Splitter | Ausgabe, Fußzeile. */
export function TasksPage({ active = true }: { active?: boolean }) {
  const data = useTasks();
  const prefs = useTaskPrefs();
  useTicketPage('tasks', prefs.selected, active);
  const [query, setQuery] = useState('');
  const { catalog } = data;
  const visible = useMemo(() => filterNamespaces(catalog?.namespaces ?? [], query), [catalog, query]);

  if (data.loadError && !catalog) return <NoticeCard title="Task-Katalog nicht abrufbar" tone="error">{data.loadError}</NoticeCard>;
  if (!catalog) return <Text color="gray">Lade Task-Katalog …</Text>;
  const task = findTask(catalog.namespaces, prefs.selected);
  const filter = query.trim() && `„${query.trim()}“`;
  return (
    <div className="tk-wrap">
      <div className="tk-page">
        <TasksHead query={query} reloading={data.reloading} onQuery={setQuery} onReload={data.reload} />
        <div className="tk-body" style={{ '--tk-split': `${prefs.split}%` } as React.CSSProperties}>
          <TaskTree visible={visible} favorites={favoriteTasks(visible, prefs.pinned)} runs={data.runs}
            pinned={prefs.pinned} selected={prefs.selected} collapsed={prefs.collapsed} onToggleNs={prefs.toggleNs}
            onSelect={prefs.select} onPin={prefs.togglePin} />
          <Splitter className="tk-split" value={prefs.split} onChange={prefs.setSplit} onReset={prefs.resetSplit} />
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
