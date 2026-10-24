import type { BalanceTune, TuneMode, Tuning } from '../api';
import type { Bar, Series } from './figures';

// Abstimmen (WZ28.6): Prüfung der Eingaben wie in Go (simrun/spec.go) und Aufbereitung des Ergebnisses fürs Diagramm.

/** Gleiche Form wie in Go (simrun/spec.go: tuneParam). */
const PARAM = /^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_.[\]-]+=-?[0-9]+(\.[0-9]+)?\.\.-?[0-9]+(\.[0-9]+)?$/;
const MAX_PARAMS = 2;
const MAX_STEPS = 21;
const MAX_BUDGET = 1000;

/** Eingaben des Bereichs „Abstimmen“; `mode` leer = kein Abstimmen. */
export interface TuneForm {
  mode: '' | TuneMode;
  /** Ein Parameter je Zeile: `datei.json:pfad=min..max`. */
  params: string;
  steps: string;
  budget: string;
}

export const EMPTY_TUNE: TuneForm = { mode: '', params: '', steps: '', budget: '' };

export interface TuneResult {
  tune?: BalanceTune;
  error?: string;
}

function intField(raw: string, label: string, min: number, max: number): { n?: number; error?: string } {
  if (raw.trim() === '') return {};
  const n = Number(raw);
  if (!Number.isInteger(n) || n < min || n > max) return { error: `${label}: ganze Zahl ${min} bis ${max} erwartet` };
  return { n };
}

/** Prüft einen Parameter wie Go: Form, kein „..“ im Pfad, min < max. */
export function paramError(p: string): string | undefined {
  const [path, range = ''] = p.split('=');
  const [lo, hi] = range.split('..').map(Number);
  if (!PARAM.test(p) || path.includes('..') || !(lo < hi)) return `Parameter „${p}“: Form datei.json:pfad=min..max mit min < max, z. B. buildings.json:wall.cost.gold=2..6`;
  return undefined;
}

/** Formular → Abstimm-Auftrag; leerer Modus liefert weder Auftrag noch Fehler. */
export function tuneFromForm(form: TuneForm): TuneResult {
  if (form.mode === '') return {};
  const params = form.params.split('\n').map((l) => l.trim()).filter(Boolean);
  if (params.length < 1 || params.length > MAX_PARAMS) return { error: `Abstimmen: 1 bis ${MAX_PARAMS} Parameter (eine je Zeile)` };
  const bad = params.map(paramError).find(Boolean);
  if (bad) return { error: bad };
  const steps = intField(form.steps, 'Schritte', 2, MAX_STEPS);
  const budget = intField(form.budget, 'Budget', 1, MAX_BUDGET);
  if (steps.error || budget.error) return { error: steps.error ?? budget.error };
  const tune: BalanceTune = { mode: form.mode, params };
  if (steps.n !== undefined) tune.steps = steps.n;
  if (budget.n !== undefined) tune.budget = budget.n;
  return { tune };
}

/** Auftrag → Formular. */
export function formFromTune(t: BalanceTune | undefined): TuneForm {
  if (!t) return EMPTY_TUNE;
  return { mode: t.mode, params: t.params.join('\n'), steps: t.steps === undefined ? '' : String(t.steps), budget: t.budget === undefined ? '' : String(t.budget) };
}

/** Beschriftung eines Parameters: `datei:pfad`. */
export const paramName = (p: { file: string; path: string }) => `${p.file}:${p.path}`;

/** Ein Parameter: Score über Wert, nach Wert sortiert; sonst keine Linie. */
export function scoreSeries(t: Tuning): Series[] {
  if (t.params.length !== 1) return [];
  const points = t.points.map((p) => ({ x: p.values[0], y: p.score })).sort((a, b) => a.x - b.x);
  return [{ name: 'Score', points }];
}

/** Zwei Parameter: ein Balken je Punkt (Score), Pass grün, sonst Warnung; Beschriftung `wert1 / wert2`. */
export function scoreBars(t: Tuning): (Bar & { tone: 'ok' | 'warn' })[] {
  return t.points.map((p) => ({ label: p.values.join(' / '), value: p.score, tone: p.pass ? 'ok' : 'warn' }));
}

/** Index des besten Punkts in `points` (Werte gleich `best.values`), −1 wenn keiner. */
export const bestIndex = (t: Tuning) =>
  t.points.findIndex((p) => p.values.length === t.best.values.length && p.values.every((v, i) => v === t.best.values[i]));
