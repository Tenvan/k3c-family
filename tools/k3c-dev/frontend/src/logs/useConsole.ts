import { useEffect, useRef, useState } from 'react';
import { backend, type ConsoleLine } from '../api';
import { errorText } from '../lib/errors';
import { mergeLines } from './lines';

export interface ConsoleState {
  lines: ConsoleLine[];
  loaded: boolean;
  error: string;
}

const RELOAD_MS = 300; // eine Lücke lädt den Puffer neu, gebündelt

/**
 * Puffer einer Quelle: lädt ihn, hängt Live-Zeilen an (auch solche aus der Ladezeit, über ihre Nummer) und lädt neu,
 * wenn Go Ereignisse verworfen hat (Lücke) oder ein neuer Lauf der Quelle beginnt.
 */
export function useConsole(name: string): ConsoleState {
  const [state, setState] = useState<ConsoleState>({ lines: [], loaded: false, error: '' });
  const current = useRef<ConsoleLine[]>([]);

  useEffect(() => {
    let alive = true;
    let pending: ConsoleLine[] | null = [];
    let timer: ReturnType<typeof setTimeout> | undefined;
    const show = (lines: ConsoleLine[], loaded = true, error = '') => {
      current.current = lines;
      setState({ lines, loaded, error });
    };
    const load = async () => {
      pending = [];
      try {
        const tail = await backend.consoleTail(name);
        if (alive) show(mergeLines(tail, pending).lines);
      } catch (e) {
        if (alive) show([], true, errorText(e));
      } finally {
        pending = null;
      }
    };
    const reloadSoon = () => {
      clearTimeout(timer);
      timer = setTimeout(() => void load(), RELOAD_MS);
    };
    const offLines = backend.on('console:line', (lines) => {
      const mine = lines.filter((l) => l.source === name);
      if (mine.length === 0) return;
      if (pending) {
        pending.push(...mine);
        return;
      }
      const merged = mergeLines(current.current, mine);
      show(merged.lines);
      if (merged.gap) reloadSoon();
    });
    const offSource = backend.on('source:state', (src) => {
      if (src.name === name && src.state === 'running') reloadSoon(); // Go hat den Puffer für den Lauf geleert
    });
    show([], false);
    void load();
    return () => {
      alive = false;
      clearTimeout(timer);
      offLines();
      offSource();
    };
  }, [name]);

  return state;
}
