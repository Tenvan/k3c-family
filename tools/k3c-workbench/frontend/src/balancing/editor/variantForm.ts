import type { BalanceBaseline } from '../../api';
import { EXAMPLES, formFromExample } from '../help';
import { EMPTY_FORM, formFromBaseline, specFromForm, type BalanceForm, type SpecResult } from '../spec';
import { EMPTY_TUNE } from '../tune';
import { countKinds, lineOf, parseLine, type VariantRow } from './variants';

// Zeilen ↔ BalanceForm (B-661): feste Zeilen sind `form.vary`, Bereiche `form.tune.params`. Den Auftrag baut weiter
// `specFromForm`, damit der Editor dieselbe BalanceSpec erzeugt wie das alte Formular.

/** Formular samt Option „je Variante“ (`perVariant`). */
export type EditorForm = BalanceForm;

const lines = (text: string) => text.split('\n').map((l) => l.trim()).filter(Boolean);

/** Formular → Zeilen: erst die Varianten, dann die Parameter (nur bei gewähltem Abstimm-Modus, wie `specFromForm`). */
export function rowsFromForm(form: BalanceForm): VariantRow[] {
  const params = form.tune.mode === '' ? [] : lines(form.tune.params);
  return [...lines(form.vary), ...params].map(parseLine);
}

/** Zeilen → Formular: feste Zeilen in `vary`, Bereiche in `tune.params` (Modus Suche als Vorgabe); ohne Bereich kein Abstimmen. */
export function formWithRows(form: BalanceForm, rows: readonly VariantRow[]): BalanceForm {
  const ranges = rows.filter((r) => r.kind === 'bereich');
  const vary = rows.filter((r) => r.kind === 'fest').map(lineOf).join('\n');
  if (ranges.length === 0) return { ...form, vary, tune: EMPTY_TUNE };
  const mode = form.tune.mode || 'search';
  // Budget gilt nur für die Suche, Schritte nur für den Sweep.
  const tune = { mode, params: ranges.map(lineOf).join('\n'), steps: mode === 'sweep' ? form.tune.steps : '', budget: mode === 'search' ? form.tune.budget : '' };
  return { ...form, vary, tune };
}

/** „Je Variante“ geht nur mit mindestens einer festen Variante und ohne Bereich (Go: checkPerVariant). */
export function canPerVariant(rows: readonly VariantRow[]): boolean {
  const { fest, bereich } = countKinds(rows);
  return fest > 0 && bereich === 0;
}

/** Auftrag aus Formular, Zeilen und Option; Fehler wie `specFromForm`. */
export function buildSpec(form: BalanceForm, rows: readonly VariantRow[], perVariant: boolean): SpecResult {
  const result = specFromForm(formWithRows(form, rows));
  if (result.spec && perVariant && canPerVariant(rows)) result.spec.perVariant = true;
  return result;
}

/** Formular aus Speicher oder Voreinstellung: fehlende Felder alter Voreinstellungen kommen aus dem leeren Formular. */
export function normalizeForm(form: Partial<EditorForm> | undefined): EditorForm {
  return { ...EMPTY_FORM, ...form, tune: { ...EMPTY_TUNE, ...form?.tune } };
}

export type OriginKind = 'leer' | 'beispiel' | 'basis' | 'voreinstellung' | 'bericht';

/** Woher der Lauf kommt; `label` nennt Beispiel, Basis-Lauf, Voreinstellung oder Bericht. */
export interface Origin {
  kind: OriginKind;
  label: string;
}

export const exampleForm = (name: string): EditorForm | undefined => {
  const hit = EXAMPLES.find((e) => e.name === name);
  return hit ? formFromExample(hit, EMPTY_FORM) : undefined;
};

export const baselineForm = (b: BalanceBaseline): EditorForm => ({ ...formFromBaseline(b), perVariant: b.spec?.perVariant });
