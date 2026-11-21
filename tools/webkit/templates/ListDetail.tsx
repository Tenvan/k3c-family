import type { CSSProperties, ReactNode } from 'react';
import { Splitter } from '../atoms';

interface Props {
  /** Kopf über beiden Spalten (Titel, Filter, Hinweise); scrollt nicht mit. */
  header?: ReactNode;
  list: ReactNode;
  /** Das Detail rechts. */
  children: ReactNode;
  /** Breite der Liste in Pixeln (Standard 320). */
  listWidth?: number;
  className?: string;
  /** Verschiebbare Trennlinie: Anteil der Liste in Prozent (20–80); ersetzt `listWidth`. */
  split?: { value: number; onChange: (v: number) => void; onReset: () => void; label?: string };
}

/** Seite mit Liste | Detail in voller Höhe (WB2): beide Spalten scrollen je für sich, der Kopf bleibt stehen.
 *  Aussehen in theme.css (`wk-listdetail`). */
export function ListDetail({ header, list, children, listWidth = 320, className, split }: Props) {
  const style = { '--wk-list-w': split ? `${split.value}%` : `${listWidth}px` } as CSSProperties;
  return (
    <div className={['wk-listdetail', split && 'has-split', className].filter(Boolean).join(' ')} style={style}>
      {header && <div className="wk-listdetail-head">{header}</div>}
      <div className="wk-listdetail-body">
        <div className="wk-listdetail-list" data-scroll="list">{list}</div>
        {split && <Splitter className="wk-listdetail-split" label={split.label ?? 'Breite der Liste'} value={split.value} onChange={split.onChange} onReset={split.onReset} />}
        <div className="wk-listdetail-detail" data-scroll="detail">{children}</div>
      </div>
    </div>
  );
}
