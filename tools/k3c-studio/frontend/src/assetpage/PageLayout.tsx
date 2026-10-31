import { Splitter } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import type { CSSProperties, ReactNode } from 'react';
import * as prefs from '../lib/prefs';

interface Props {
  /** Facetten links */
  rail: ReactNode;
  /** Inhalt der Liste (Sichten) */
  list: ReactNode;
  /** rechte Leiste: Inspektor oder Werkzeuge (bringt ihr eigenes `<section className="ap-side">` mit) */
  side: ReactNode;
  /** Bühne unter dem Raster (bringt ihr eigenes `<section className="ap-stage">` mit) */
  stage: ReactNode;
}

/** Aufbau der Assets-Seite (B-677, Variante „Bühne“): Facetten als schmale Spalte ohne Mindestbreite; links das Raster über der
 *  Bühne (Splitter dazwischen, Höhe der Bühne gemerkt als `assets.stage`), rechts die Leiste über die volle Höhe (Breite gemerkt als
 *  `assets.lsplit`). Die DOM-Reihenfolge (Raster, Bühne, Leiste) ist die Tab-Reihenfolge. Unter 860 px Breite stehen Facetten,
 *  Raster, Bühne und Leiste untereinander (waagerechte Splitter); Doppelklick oder Pos1 setzt einen Splitter zurück. */
export function PageLayout({ rail, list, side, stage }: Props) {
  const [fk, setFk, resetFk] = useSplit(prefs, 'assets.fstack', 70);
  const [sp, setSp, resetSp] = useSplit(prefs, 'assets.lsplit', 70);
  const [st, setSt, resetSt] = useSplit(prefs, 'assets.stack', 55);
  const [sg, setSg, resetSg] = useSplit(prefs, 'assets.stage', 38);
  const vars = { '--ap-fk': fk, '--ap-sp': `${sp}%`, '--ap-st': st, '--ap-stg': `${sg}%` } as CSSProperties;
  return (
    <div className="ap-body" style={vars}>
      <div className="ap-rail">{rail}</div>
      <Splitter horizontal className="ap-fstack" value={fk} onChange={setFk} onReset={resetFk} />
      <div className="ap-main">
        <div className="ap-list">{list}</div>
        <Splitter horizontal className="ap-stagesplit" label="Höhe der Bühne" value={sg} onChange={setSg} onReset={resetSg} />
        {stage}
        <Splitter className="ap-split" label="Breite der Leiste" value={sp} onChange={setSp} onReset={resetSp} />
        <Splitter horizontal className="ap-stack" value={st} onChange={setSt} onReset={resetSt} />
        {side}
      </div>
    </div>
  );
}
