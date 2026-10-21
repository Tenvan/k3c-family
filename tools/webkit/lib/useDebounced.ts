import { useEffect, useState } from 'react';

/** Wert, der erst nach ms Ruhe nachzieht (Textfelder der Filter, B-064: 300 ms). */
export function useDebounced<T>(value: T, ms = 300): T {
  const [debounced, setDebounced] = useState(value);
  useEffect(() => {
    const t = setTimeout(() => setDebounced(value), ms);
    return () => clearTimeout(t);
  }, [value, ms]);
  return debounced;
}
