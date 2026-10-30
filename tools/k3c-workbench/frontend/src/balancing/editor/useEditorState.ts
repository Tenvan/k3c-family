import { useCallback, useState } from 'react';
import type { BalanceForm } from '../spec';
import { normalizeForm, rowsFromForm, type EditorForm } from './variantForm';
import type { VariantRow } from './variants';

/** Zustand des Lauf-Editors: Felder, Varianten-Zeilen (statt `vary`/`tune.params`), Option „je Variante“. */
export interface EditorState {
  form: BalanceForm;
  rows: VariantRow[];
  perVariant: boolean;
  /** „Liste …“ bei den Seeds gewählt (auch mit leerer Liste). */
  listMode: boolean;
}

export function stateOf(ef: Partial<EditorForm> | undefined): EditorState {
  const { perVariant, ...form } = normalizeForm(ef);
  return { form, rows: rowsFromForm(form), perVariant: perVariant === true, listMode: false };
}

export function useEditorState(initial: EditorForm) {
  const [state, setState] = useState<EditorState>(() => stateOf(initial));
  const patch = useCallback((p: Partial<BalanceForm>) => setState((s) => ({ ...s, form: { ...s.form, ...p } })), []);
  const setRows = useCallback((rows: VariantRow[]) => setState((s) => ({ ...s, rows })), []);
  const setPerVariant = useCallback((perVariant: boolean) => setState((s) => ({ ...s, perVariant })), []);
  const setListMode = useCallback((listMode: boolean) => setState((s) => ({ ...s, listMode })), []);
  const load = useCallback((ef: Partial<EditorForm> | undefined) => setState(stateOf(ef)), []);
  return { state, patch, setRows, setPerVariant, setListMode, load };
}
