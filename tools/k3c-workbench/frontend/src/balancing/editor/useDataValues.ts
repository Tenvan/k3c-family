import { useEffect, useMemo, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { backend, type BalanceDataFileValue } from '../../api';
import { buildIndex, type ValueIndex } from './variantCheck';

export interface DataValues {
  /** Alle Zahlenwerte der variierbaren Dateien; null beim Laden oder nach einem Fehler. */
  values: BalanceDataFileValue[] | null;
  index: ValueIndex | null;
  error: string;
}

/** Lädt die Werte aller variierbaren Dateien einmal (B-662) für Daten-Browser und Prüfung der Tabelle. */
export function useDataValues(): DataValues {
  const [state, setState] = useState<{ values: BalanceDataFileValue[] | null; error: string }>({ values: null, error: '' });
  useEffect(() => {
    let alive = true;
    backend.balanceDataValuesAll().then(
      (values) => alive && setState({ values, error: '' }),
      (e) => alive && setState({ values: null, error: errorText(e) }),
    );
    return () => { alive = false; };
  }, []);
  const index = useMemo(() => (state.values ? buildIndex(state.values) : null), [state.values]);
  return { ...state, index };
}
