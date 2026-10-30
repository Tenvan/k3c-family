import { useEffect, useRef, useState } from 'react';
import type { FigureInfo, PreviewStage } from '../stage/PreviewStage';
import type { KampfWorld } from '@game/scenes/kampfAnzeige';
import type { Control } from './figure';
import type { FigureItem } from './figureView';

/** Figuren auf der Bühne des Studios (B-463, WZ10.1): eine Phaser-Instanz je Seite, nicht je Karte. Phaser wird beim ersten
 *  Zeichnen geladen (eigener Baustein, groß). `items` ersetzt die gezeigten Figuren (null = leer), `control` steuert die
 *  Wiedergabe; `onInfo` meldet Lücke und Bildzahl je Figur, sobald sie gezeichnet sind. Eine neuere `items`-Liste macht
 *  eine ältere Antwort ungültig. */
export function FigureStage({ items, control, zoom, kampf, selected, onFigure, onInfo, onLoadError }: {
  items: readonly FigureItem[] | null;
  control: Control;
  /** Zoom der Ansicht (1 = alle Figuren im Bild); ohne Angabe bleibt die Ansicht, wie sie ist */
  zoom?: number;
  /** Beispielzustand für die Boss-Anzeigen des Spiels (Boss-Leiste, Phasen-Text, Warnkreis); ohne Angabe keine */
  kampf?: KampfWorld | null;
  /** ID der markierten Figur (B-672); ohne Angabe keine Markierung */
  selected?: string | null;
  /** Klick auf eine Figur meldet ihre ID (null = daneben); ohne Angabe kein Treffer-Test */
  onFigure?: (id: string | null) => void;
  onInfo?: (info: FigureInfo[]) => void;
  onLoadError?: (files: string[]) => void;
}) {
  const host = useRef<HTMLDivElement>(null);
  const stage = useRef<PreviewStage | null>(null);
  const latest = useRef({ items, control, onInfo, onLoadError, onFigure });
  latest.current = { items, control, onInfo, onLoadError, onFigure };
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
  // Boss-Anzeigen: nach dem Zeigen der Figuren (die Anzeige braucht sie) und bei jedem neuen Zustand
  useEffect(() => { if (ready && kampf !== undefined) stage.current?.setKampf(kampf); }, [ready, kampf, items]);
  // Auswahl (B-672): Klick meldet die ID der Figur, die gewählte wird markiert (nach dem Zeigen neuer Figuren erneut)
  const clickable = onFigure !== undefined;
  useEffect(() => {
    if (!ready || !clickable) return;
    stage.current?.setFigureClick((i) => latest.current.onFigure?.(i === null ? null : latest.current.items?.[i]?.id ?? null));
  }, [ready, clickable]);
  useEffect(() => {
    if (!ready || selected === undefined) return;
    const index = items?.findIndex((i) => i.id === selected) ?? -1;
    stage.current?.setFigureMark(index < 0 ? null : index);
  }, [ready, selected, items]); // die Bühne merkt sich die Markierung und zeichnet sie mit den neuen Figuren
  // nach dem Zeigen neuer Figuren erneut anwenden: das Einpassen setzt den Zoom neu
  useEffect(() => { if (ready && zoom !== undefined) stage.current?.setFigureZoom(zoom); }, [ready, zoom, items]);

  return <div className="wd-stage" ref={host} aria-label="Figuren-Bühne" />;
}
