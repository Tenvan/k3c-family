import { useEffect, useRef, useState } from 'react';
import type { FigureInfo, PreviewStage } from '../stage/PreviewStage';
import type { Control } from './figure';
import type { FigureItem } from './figureView';

/** Figuren auf der Bühne des Studios (B-463, WZ10.1): eine Phaser-Instanz je Seite, nicht je Karte. Phaser wird beim ersten
 *  Zeichnen geladen (eigener Baustein, groß). `items` ersetzt die gezeigten Figuren (null = leer), `control` steuert die
 *  Wiedergabe; `onInfo` meldet Lücke und Bildzahl je Figur, sobald sie gezeichnet sind. Eine neuere `items`-Liste macht
 *  eine ältere Antwort ungültig. */
export function FigureStage({ items, control, onInfo, onLoadError }: {
  items: readonly FigureItem[] | null;
  control: Control;
  onInfo?: (info: FigureInfo[]) => void;
  onLoadError?: (files: string[]) => void;
}) {
  const host = useRef<HTMLDivElement>(null);
  const stage = useRef<PreviewStage | null>(null);
  const latest = useRef({ items, control, onInfo, onLoadError });
  latest.current = { items, control, onInfo, onLoadError };
  const [ready, setReady] = useState(false);

  useEffect(() => {
    let cancelled = false;
    void import('../stage/PreviewStage').then(({ PreviewStage: Stage }) => {
      if (cancelled || !host.current) return;
      stage.current = new Stage(host.current, (files) => latest.current.onLoadError?.(files));
      setReady(true);
    });
    return () => { cancelled = true; stage.current?.destroy(); stage.current = null; };
  }, []);

  useEffect(() => {
    if (!ready || !stage.current) return;
    let current = true;
    void stage.current.showFigures(items).then((info) => { if (current && info.length > 0) latest.current.onInfo?.(info); });
    return () => { current = false; };
  }, [ready, items]);

  useEffect(() => { if (ready) stage.current?.setControl(control); }, [ready, control, items]);

  return <div className="wd-stage" ref={host} aria-label="Figuren-Bühne" />;
}
