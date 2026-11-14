import type { CSSProperties, ReactNode } from 'react';
import { Splitter } from '../atoms';
import { useSplit, type SplitStore } from '../lib/split';

export interface SplitStackProps {
  /** Ablage der App (z. B. `prefsStore` aus lib/prefs); der Anteil steht unter `<app>.<prefKey>`. */
  store: SplitStore;
  prefKey: string;
  /** Vorgabe für den Anteil des unteren Bereichs in Prozent (20–80). */
  dflt?: number;
  top: ReactNode;
  bottom: ReactNode;
  /** Name des Splitters für Screenreader. */
  label?: string;
  className?: string;
}

/** Zwei Bereiche übereinander mit waagerechtem Splitter (B-875): Ziehen, Pfeiltasten und Doppelklick wie `Splitter`,
 *  der Anteil des unteren Bereichs (20–80 %) wird gemerkt. Der obere Bereich scrollt in sich, der untere füllt seine
 *  Höhe; beide brauchen einen Elternteil mit fester Höhe. */
export function SplitStack({ store, prefKey, dflt = 50, top, bottom, label, className }: SplitStackProps) {
  const [share, setShare, reset] = useSplit(store, prefKey, dflt);
  return (
    <div className={['wk-stack', className].filter(Boolean).join(' ')} style={{ '--wk-stack-bottom': share } as CSSProperties}>
      <div className="wk-stack-top">{top}</div>
      <Splitter horizontal value={share} onChange={setShare} onReset={reset} label={label} />
      <div className="wk-stack-bottom">{bottom}</div>
    </div>
  );
}
