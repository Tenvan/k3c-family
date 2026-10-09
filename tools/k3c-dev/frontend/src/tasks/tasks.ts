import type { TaskGateState, TaskInfo, TaskNamespace, TaskRun, TaskRunState } from '../api';
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

/** Schloss eines Tasks als Symbol und Text; `-temp` gilt nur bis zum Beenden von k3c-dev (gestrichelter Rahmen). */
export function describeGate(state: TaskGateState | undefined): { icon: string; label: string; temp: boolean } {
  switch (state) {
    case 'open':
      return { icon: '🔓', label: 'frei für Agenten (Datei)', temp: false };
    case 'open-temp':
      return { icon: '🔓', label: 'frei für Agenten bis zum Beenden', temp: true };
    case 'closed-temp':
      return { icon: '🔒', label: 'gesperrt bis zum Beenden', temp: true };
    default:
      return { icon: '🔒', label: 'gesperrt (Datei)', temp: false };
  }
}

/** Nur Tasks, deren Schloss offen ist (🔑-Schalter). */
export function onlyAllowed(namespaces: TaskNamespace[], gates: Record<string, TaskGateState>): TaskNamespace[] {
  return namespaces
    .map((ns) => ({ name: ns.name, tasks: ns.tasks.filter((t) => gates[t.name]?.startsWith('open')) }))
    .filter((ns) => ns.tasks.length > 0);
}

/** Favoriten als Sicht auf dieselben Knoten, in der Reihenfolge des Baums. */
export function favoriteTasks(namespaces: TaskNamespace[], favorites: string[]): TaskInfo[] {
  return namespaces.flatMap((ns) => ns.tasks).filter((t) => favorites.includes(t.name));
}

/** Höchstens so viele beendete Läufe merkt sich die Seite je Sitzung. */
export const HISTORY_LIMIT = 50;

/** Hängt einen beendeten Lauf vorn an die Liste der letzten Läufe; laufende zählen nicht. */
export function pushHistory(history: TaskRun[], run: TaskRun): TaskRun[] {
  if (run.state === 'running') return history;
  return [run, ...history.filter((r) => !(r.name === run.name && r.startedAt === run.startedAt))].slice(0, HISTORY_LIMIT);
}
