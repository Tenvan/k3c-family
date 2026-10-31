import { Splitter } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import type { CSSProperties, ReactNode } from 'react';
import * as prefs from '../lib/prefs';

interface Props {
  /** Facetten links */
  rail: ReactNode;
  /** Inhalt der Liste (Sichten) */
  list: ReactNode;
  /** rechte Spalte: Inspektor oder Werkzeuge (bringt ihr eigenes `<section className="ap-side">` mit) */
  side: ReactNode;
}

/** Aufbau der Assets-Seite mit zwei einstellbaren Trennlinien (B-677): Facetten | Liste und Liste | Inspektor. Breit stehen die Spalten
 *  nebeneinander (senkrechte Splitter), unter 1100 px Breite untereinander (waagerechte Splitter); Anteile werden gemerkt,
 *  Doppelklick setzt sie zurück. */
export function PageLayout({ rail, list, side }: Props) {
  const [fs, setFs, resetFs] = useSplit(prefs, 'assets.fsplit', 22);
  const [fk, setFk, resetFk] = useSplit(prefs, 'assets.fstack', 70);
  const [sp, setSp, resetSp] = useSplit(prefs, 'assets.split', 60);
  const [st, setSt, resetSt] = useSplit(prefs, 'assets.stack', 55);
  const vars = { '--ap-fs': `${fs}%`, '--ap-fk': fk, '--ap-sp': `${sp}%`, '--ap-st': st } as CSSProperties;
  return (
    <div className="ap-body" style={vars}>
      <div className="ap-rail">{rail}</div>
      <Splitter className="ap-fsplit" value={fs} onChange={setFs} onReset={resetFs} />
      <Splitter horizontal className="ap-fstack" value={fk} onChange={setFk} onReset={resetFk} />
      <div className="ap-main">
        <div className="ap-list">{list}</div>
        <Splitter className="ap-split" value={sp} onChange={setSp} onReset={resetSp} />
        <Splitter horizontal className="ap-stack" value={st} onChange={setSt} onReset={resetSt} />
        {side}
      </div>
    </div>
  );
}
