import type { BalanceRun, BalanceSpec } from '../api';
import { EMPTY_TUNE, formFromTune, tuneFromForm, type TuneForm } from './tune';

/** Eingaben des Formulars als Text; leer heißt „weglassen“ (Standard der CLI bzw. Ziel-Tiefe). */
export interface BalanceForm {
  seeds: string;
  depth: string;
  players: string;
  bot: string;
  grade: string;
  days: string;
  /** Eine Variante je Zeile: `datei.json:pfad.zum.wert=wert`. */
  vary: string;
  label: string;
  tune: TuneForm;
}

export const EMPTY_FORM: BalanceForm = { seeds: '', depth: '', players: '', bot: '', grade: '', days: '', vary: '', label: '', tune: EMPTY_TUNE };

/** Gleiche Form wie in Go (simrun/spec.go: varyArg). */
const VARY = /^[A-Za-z0-9_-]+\.json:[A-Za-z0-9_.[\]-]+=[^\s=]{1,64}$/;

const NUMBERS = [['seeds', 'Seeds', 1], ['players', 'Spieler', 1], ['days', 'Tage', 1], ['depth', 'Tiefe', 0]] as const;

export interface SpecResult {
  spec?: BalanceSpec;
  error?: string;
}

/** Formular → Spec; der erste Fehler kommt als deutscher Text, sonst nur gesetzte Felder (Tiefe 0 = Wald bleibt erhalten). */
export function specFromForm(form: BalanceForm): SpecResult {
  const spec: BalanceSpec = {};
  for (const [key, label, min] of NUMBERS) {
    const raw = form[key].trim();
    if (raw === '') continue;
    const n = Number(raw);
    if (!Number.isInteger(n) || n < min) return { error: `${label}: ganze Zahl ab ${min} erwartet` };
    spec[key] = n;
  }
  for (const key of ['bot', 'grade', 'label'] as const) {
    const raw = form[key].trim();
    if (raw !== '') spec[key] = raw;
  }
  const vary = form.vary.split('\n').map((l) => l.trim()).filter(Boolean);
  const bad = vary.find((v) => !VARY.test(v));
  if (bad) return { error: `Variante „${bad}“: Form datei.json:pfad.zum.wert=wert` };
  if (vary.length > 0) spec.vary = vary;
  const { tune, error } = tuneFromForm(form.tune);
  if (error) return { error };
  if (tune) spec.tune = tune;
  return { spec };
}

/** Spec → Formular (z. B. „Aus Lauf übernehmen“). */
export function formFromSpec(spec: BalanceSpec | undefined): BalanceForm {
  const s = spec ?? {};
  const text = (v: number | string | undefined) => (v === undefined ? '' : String(v));
  return {
    seeds: text(s.seeds), depth: text(s.depth), players: text(s.players), bot: s.bot ?? '', grade: s.grade ?? '', days: text(s.days),
    vary: (s.vary ?? []).join('\n'), label: s.label ?? '', tune: formFromTune(s.tune),
  };
}

/** Gespeicherte Voreinstellung. */
export interface Preset {
  name: string;
  form: BalanceForm;
}

export function isPresetList(v: unknown): v is Preset[] {
  return (
    Array.isArray(v) &&
    v.every((p) => {
      const x = p as Preset | null;
      return !!x && typeof x.name === 'string' && !!x.form && Object.keys(EMPTY_FORM).every((k) => (k === 'tune' ? typeof x.form.tune === 'object' && x.form.tune !== null : typeof x.form[k as keyof BalanceForm] === 'string'));
    })
  );
}

/** Voreinstellung unter dem Namen speichern bzw. ersetzen (Reihenfolge bleibt, neue hinten). */
export function savePreset(list: Preset[], name: string, form: BalanceForm): Preset[] {
  const n = name.trim();
  if (n === '') return list;
  const next = { name: n, form };
  return list.some((p) => p.name === n) ? list.map((p) => (p.name === n ? next : p)) : [...list, next];
}

export const dropPreset = (list: Preset[], name: string): Preset[] => list.filter((p) => p.name !== name);

export const isActive = (r: BalanceRun) => r.state === 'wartet' || r.state === 'läuft';

/** Fortschritt in Prozent, 0 bei unbekannter Summe. */
export const progressOf = (r: BalanceRun) => (r.total > 0 ? Math.min(100, Math.round((r.done / r.total) * 100)) : 0);

/** Läufe nach ID ersetzen; neue Einträge vorn. */
export function mergeRuns(list: BalanceRun[], next: BalanceRun[]): BalanceRun[] {
  const byId = new Map(next.map((r) => [r.id, r]));
  return [...next, ...list.filter((r) => !byId.has(r.id))];
}
