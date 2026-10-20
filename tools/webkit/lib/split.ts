import { useState } from 'react';

// Rechnung der Splitter (atoms/Splitter, B-403/B-445): Anteil in Prozent, geklemmt auf 20–80 %.

/** Anteil in Prozent, den der Zeiger im Kasten einnimmt: senkrechte Spalte von links, waagerechter Balken von unten. */
export function splitShare(horizontal: boolean, box: { left: number; bottom: number; width: number; height: number }, x: number, y: number): number {
  return horizontal ? ((box.bottom - y) / box.height) * 100 : ((x - box.left) / box.width) * 100;
}

/** Klemmt den Anteil auf 20–80 % und rundet auf ganze Prozent. */
export function clampSplit(share: number): number {
  return Math.round(Math.min(80, Math.max(20, share)));
}

/** Gemerkter Anteil als Text: 20–80 %, unlesbar oder leer gilt die Vorgabe. */
export function parseSplit(raw: string, dflt: number): number {
  const n = Number(raw);
  return raw.trim() !== '' && Number.isFinite(n) ? clampSplit(n) : clampSplit(dflt);
}

/** Ablage des Anteils: die an das Präfix der App gebundenen Funktionen aus lib/prefs. */
export interface SplitStore {
  loadText: (key: string, fallback: string) => string;
  savePref: (key: string, value: string) => void;
}

/** Gemerkter Anteil eines Splitters unter `<app>.<key>`; liefert Wert, Setzen und Zurücksetzen auf die Vorgabe. */
export function useSplit(store: SplitStore, key: string, dflt: number): [number, (v: number) => void, () => void] {
  const [split, setState] = useState(() => parseSplit(store.loadText(key, ''), dflt));
  const set = (v: number) => {
    const n = clampSplit(v);
    setState(n);
    store.savePref(key, String(n));
  };
  return [split, set, () => set(dflt)];
}
