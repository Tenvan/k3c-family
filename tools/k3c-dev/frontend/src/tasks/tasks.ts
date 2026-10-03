import type { TaskInfo, TaskNamespace, TaskRun, TaskRunState } from '../api';
import type { Tone } from '../ui/parts';

// Reine Funktionen der Tasks-Seite: Filter (dieselbe Regel wie taskcat.Filter in Go, lokal, damit der Baum beim
// Tippen ohne Roundtrip reagiert), Zustand → Badge, Argumentzeile, Listenpflege.

/** Namensräume mit Tasks, deren Name oder Beschreibung den Begriff enthält; ohne Treffer fällt der Namensraum weg. */
export function filterNamespaces(namespaces: TaskNamespace[], query: string): TaskNamespace[] {
  const q = query.trim().toLowerCase();
  if (q === '') return namespaces;
  const out: TaskNamespace[] = [];
  for (const ns of namespaces) {
    const tasks = ns.tasks.filter((t) => t.name.toLowerCase().includes(q) || t.desc.toLowerCase().includes(q));
    if (tasks.length > 0) out.push({ name: ns.name, tasks });
  }
  return out;
}

export const countTasks = (namespaces: TaskNamespace[]): number =>
  namespaces.reduce((sum, ns) => sum + ns.tasks.length, 0);

export function findTask(namespaces: TaskNamespace[], name: string): TaskInfo | undefined {
  for (const ns of namespaces) {
    const task = ns.tasks.find((t) => t.name === name);
    if (task) return task;
  }
  return undefined;
}

/** Ersetzt den Lauf gleichen Namens oder hängt ihn an; nach Name sortiert wie TaskRuns in Go. */
export function upsertRun(list: TaskRun[], run: TaskRun): TaskRun[] {
  return [...list.filter((r) => r.name !== run.name), run].sort((a, b) => (a.name < b.name ? -1 : a.name > b.name ? 1 : 0));
}

export interface RunBadge {
  tone: Tone;
  label: string;
}

/** Zustand der Zeile: ohne Lauf bereit, laufend blau, erfolgreich grün, gescheitert rot mit Exit-Code. */
export function describeRun(run: TaskRun | undefined): RunBadge {
  if (!run) return { tone: 'neutral', label: 'bereit' };
  return describeState(run.state, run.exitCode, run.reason);
}

export function describeState(state: TaskRunState, exitCode: number, reason = ''): RunBadge {
  switch (state) {
    case 'running':
      return { tone: 'info', label: 'läuft' };
    case 'succeeded':
      return { tone: 'ok', label: 'erfolgreich' };
    case 'failed':
      return { tone: 'error', label: reason ? `fehlgeschlagen: ${reason}` : `fehlgeschlagen (Exit ${exitCode})` };
    case 'cancelled':
      return { tone: 'warn', label: 'abgebrochen' };
    default:
      return { tone: 'neutral', label: state };
  }
}

/**
 * Zusatzargumente aus dem Eingabefeld: an Leerzeichen getrennt, Anführungszeichen halten zusammen. Die
 * Zeichenprüfung macht Go (taskrun.ValidateArgs); hier wird nur getrennt, nie bereinigt.
 */
export function splitArgs(input: string): string[] {
  const out: string[] = [];
  let cur = '';
  let quote: string | null = null;
  let has = false;
  for (const ch of input) {
    if (quote) {
      if (ch === quote) quote = null;
      else cur += ch;
    } else if (ch === '"' || ch === "'") {
      quote = ch;
      has = true;
    } else if (/\s/.test(ch)) {
      if (has || cur) out.push(cur);
      cur = '';
      has = false;
    } else {
      cur += ch;
    }
  }
  if (has || cur) out.push(cur);
  return out;
}

/** Tooltip der Zeile: Beschreibung, Zusammenfassung, Herkunft, Aliase; leere Teile fallen weg. */
export function taskTooltip(t: TaskInfo): string {
  const lines = [t.desc, t.summary, t.file && `${t.file}:${t.line}`, t.aliases.length > 0 && `Aliase: ${t.aliases.join(', ')}`];
  return lines.filter(Boolean).join('\n');
}
