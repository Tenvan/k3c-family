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
 * wenn Go Ereignisse verworfen hat (Lücke) oder der Zustand der Quelle wechselt: ein neuer Lauf oder Dienst-Start
 * leert den Puffer in Go, und am Ende eines Laufs fehlen sonst verworfene letzte Zeilen, weil keine mehr folgt.
 */
export function useConsole(name: string): ConsoleState {
  const [state, setState] = useState<ConsoleState>({ lines: [], loaded: false, error: '' });
  const current = useRef<ConsoleLine[]>([]);

  useEffect(() => {
    let alive = true;
    let pending: ConsoleLine[] | null = []; // Live-Zeilen während des laufenden Ladens
    let lastState: string | null = null;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const show = (lines: ConsoleLine[], loaded = true, error = '') => {
      current.current = lines;
      setState({ lines, loaded, error });
    };
    const load = async () => {
      const buf: ConsoleLine[] = [];
      pending = buf; // ein neueres Laden übernimmt; nur das jüngste setzt pending zurück
      try {
        const tail = await backend.consoleTail(name);
        if (alive && pending === buf) show(mergeLines(tail, buf).lines);
      } catch (e) {
        if (alive && pending === buf) show([], true, errorText(e));
      } finally {
        if (pending === buf) pending = null;
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
      if (src.name !== name || src.state === lastState) return; // Dienste melden alle 2 s Metriken ohne Wechsel
      if (lastState !== null) reloadSoon();
      lastState = src.state;
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
