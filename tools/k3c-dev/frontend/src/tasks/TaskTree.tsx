import { Text } from '@radix-ui/themes';
import type { TaskGateState, TaskInfo, TaskNamespace, TaskRun } from '../api';
import { StatusBadge } from '../ui/parts';
import { describeGate, describeRun, taskTooltip } from './tasks';

interface TaskTreeProps {
  visible: TaskNamespace[];
  favorites: TaskInfo[];
  runs: TaskRun[];
  gates: Record<string, TaskGateState>;
  pinned: string[];
  selected: string;
  collapsed: string[];
  onToggleNs: (ns: string) => void;
  onSelect: (name: string) => void;
  onGate: (name: string) => void;
  onPin: (name: string) => void;
}

/** Baum der Tasks-Seite: ★ Favoriten als Sicht auf dieselben Knoten, darunter die Namensräume (einklappbar). */
export function TaskTree(p: TaskTreeProps) {
  const row = (t: TaskInfo, key: string) => (
    <TaskRow key={key} task={t} run={p.runs.find((r) => r.name === t.name)} gate={p.gates[t.name]} pinned={p.pinned.includes(t.name)}
      active={t.name === p.selected} onSelect={() => p.onSelect(t.name)} onGate={() => p.onGate(t.name)} onPin={() => p.onPin(t.name)} />
  );
  return (
    <div className="tk-tree">
      {p.favorites.length > 0 && (
        <div>
          <div className="tk-ns tk-ns-fav">★ Favoriten <span className="tk-ns-count">{p.favorites.length}</span></div>
          {p.favorites.map((t) => row(t, `fav:${t.name}`))}
        </div>
      )}
      {p.visible.map((ns) => {
        const open = !p.collapsed.includes(ns.name);
        return (
          <div key={ns.name}>
            <button type="button" className="tk-ns" onClick={() => p.onToggleNs(ns.name)} aria-expanded={open}
              aria-label={`${ns.name} ${open ? 'einklappen' : 'aufklappen'}`}>
              {open ? '▾' : '▸'} {ns.name} <span className="tk-ns-count">{ns.tasks.length}</span>
            </button>
            {open && ns.tasks.map((t) => row(t, t.name))}
          </div>
        );
      })}
      {p.visible.length === 0 && <Text color="gray" size="2">Kein Task passt zum Filter.</Text>}
    </div>
  );
}

interface TaskRowProps {
  task: TaskInfo;
  run?: TaskRun;
  gate?: TaskGateState;
  pinned: boolean;
  active: boolean;
  onSelect: () => void;
  onGate: () => void;
  onPin: () => void;
}

function TaskRow({ task, run, gate, pinned, active, onSelect, onGate, onPin }: TaskRowProps) {
  const badge = describeRun(run);
  const lock = describeGate(gate);
  return (
    <div className={active ? 'tk-row tk-row-on' : 'tk-row'}>
      <button type="button" className={lock.temp ? 'tk-icon tk-lock-temp' : 'tk-icon'} title={`${lock.label} · Klick schaltet bis zum Beenden um`}
        aria-label={`Schloss ${task.name}: ${lock.label}`} onClick={onGate}>{lock.icon}</button>
      <button type="button" className={pinned ? 'tk-icon tk-pin-on' : 'tk-icon tk-pin'} title={pinned ? 'Aus Favoriten nehmen' : 'Zu Favoriten'}
        aria-pressed={pinned} aria-label={`Favorit ${task.name}`} onClick={onPin}>📌</button>
      <button type="button" className="tk-row-main" title={taskTooltip(task)} onClick={onSelect}>
        <span className="tk-row-name">{task.leaf}</span>
        <span className="tk-row-desc">{task.desc}</span>
      </button>
      {run && <StatusBadge tone={badge.tone}>{badge.label}</StatusBadge>}
    </div>
  );
}
