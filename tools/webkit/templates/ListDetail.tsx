import type { CSSProperties, ReactNode } from 'react';

interface Props {
  /** Kopf über beiden Spalten (Titel, Filter, Hinweise); scrollt nicht mit. */
  header?: ReactNode;
  list: ReactNode;
  /** Das Detail rechts. */
  children: ReactNode;
  /** Breite der Liste in Pixeln (Standard 320). */
  listWidth?: number;
  className?: string;
}

/** Seite mit Liste | Detail in voller Höhe (WB2): beide Spalten scrollen je für sich, der Kopf bleibt stehen.
 *  Aussehen in theme.css (`wk-listdetail`). */
export function ListDetail({ header, list, children, listWidth = 320, className }: Props) {
  const style = { '--wk-list-w': `${listWidth}px` } as CSSProperties;
  return (
    <div className={['wk-listdetail', className].filter(Boolean).join(' ')} style={style}>
      {header && <div className="wk-listdetail-head">{header}</div>}
      <div className="wk-listdetail-body">
        <div className="wk-listdetail-list" data-scroll="list">{list}</div>
        <div className="wk-listdetail-detail" data-scroll="detail">{children}</div>
      </div>
    </div>
  );
}
