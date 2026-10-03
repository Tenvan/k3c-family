import { useEffect, useRef, useState } from 'react';
import { backend, type Source } from '../api';
import { errorText } from '../lib/errors';
import { upsertSource } from './lines';

/** Alle Quellen (Dienste, Log-Dateien, Läufe, Konsolen), live über `source:state`. */
export function useSources() {
  const [sources, setSources] = useState<Source[] | null>(null);
  const [error, setError] = useState('');
  const pending = useRef<Source[]>([]); // Meldungen, die vor der Liste kamen

  useEffect(() => {
    const off = backend.on('source:state', (src) =>
      setSources((list) => {
        if (!list) pending.current.push(src);
        return list && upsertSource(list, src);
      }),
    );
    backend.sources().then(
      (list) => setSources(pending.current.reduce(upsertSource, list)),
      (e) => setError(errorText(e)),
    );
    return off;
  }, []);
  return { sources, error };
}
