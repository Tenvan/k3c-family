import type { CSSProperties, ReactNode } from 'react';
import { Button, Splitter } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { prefsStore } from '../lib/prefs';

/** Anteil der Liste in Prozent, wenn nichts gemerkt ist (etwa 380 px bei 1.300 px Seitenbreite). */
const SPLIT_DEFAULT = 30;

export type BalancingView = 'list' | 'detail';

interface Props {
  /** Werkzeugleiste oben, steht fest. */
  toolbar: ReactNode;
  /** Hinweis unter der Werkzeugleiste (z. B. Fehlertext), steht ebenfalls fest. */
  notice?: ReactNode;
  list: ReactNode;
  detail: ReactNode;
  /** Unter 760 px zeigt die Seite nur diesen Bereich; darüber stehen beide nebeneinander oder untereinander. */
  view: BalancingView;
  /** Knopf „Liste“ im Detail (nur in der schmalen Ansicht sichtbar). */
  onBack: () => void;
}

/** Rahmen der Balancing-Seite (B-659): füllt die Inhaltsfläche, die Seite selbst scrollt nie. Oben die Werkzeugleiste,
 *  darunter Liste (links) und Detail (rechts) mit Splitter und je eigenem Scroll-Container. Unter 1.100 px Breite
 *  stehen beide untereinander, unter 760 px zeigt `view` genau einen der beiden (Container Query auf `.bal-wrap`,
 *  Regeln in styles/balancing.css). */
export function BalancingLayout({ toolbar, notice, list, detail, view, onBack }: Props) {
  const [split, setSplit, resetSplit] = useSplit(prefsStore, 'balancing.split', SPLIT_DEFAULT);
  return (
    <div className="bal-wrap">
      <div className="bal-page">
        <div className="bal-toolbar" role="toolbar" aria-label="Werkzeugleiste">{toolbar}</div>
        {notice}
        <div className="bal-body" data-view={view} style={{ '--bal-split': `${split}%` } as CSSProperties}>
          <section className="bal-pane bal-list" aria-label="Läufe und Berichte">
            <div className="bal-scroll" data-pane="list">{list}</div>
          </section>
          <Splitter className="bal-split" value={split} onChange={setSplit} onReset={resetSplit} />
          <section className="bal-pane bal-detail" aria-label="Detail">
            <div className="bal-back">
              <Button size="1" variant="soft" onClick={onBack}>Liste</Button>
            </div>
            <div className="bal-scroll" data-pane="detail">{detail}</div>
          </section>
        </div>
      </div>
    </div>
  );
}
