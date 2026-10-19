import { useEffect, useRef, type CSSProperties } from 'react';
import { createPortal } from 'react-dom';
import type { PlannedChunk } from '../api';
import type { SlotHint } from './chunkMarks';
import type { LevelModel, ModelChunk } from './levelView';
import { LINE_PX, bandBox, bandChunks, bandMode, chunkInfo, type ChunkInfo } from './bandLayout';
import type { Box } from './viewHooks';
import type { View } from './viewState';

/** Eine Zelle je Chunk; der Inhalt passt sich dem Platz an und läuft nie über den Chunk-Rand. */
function BandCell({ info, w, h, style }: { info: ChunkInfo; w: number; h: number; style: CSSProperties }) {
  const slot = info.slot ? ` ${info.slot}` : '';
  const extra = [info.seed ? `Seed „${info.seed}“` : '', info.objects > 0 ? `${info.objects} feste Obj.` : ''].filter(Boolean);
  const mode = bandMode(w, h, 1 + extra.length);
  return (
    <div className={`wd-band-chunk wd-band-${info.color}`} data-mode={mode} style={style}>
      {mode === 'compact' && <span className="wd-band-line">{`${info.icon}${slot}`}</span>}
      {mode === 'line' && <span className="wd-band-line">{[`${info.icon} ${info.name}${slot}`, ...extra].join(' · ')}</span>}
      {mode === 'full' && [`${info.icon} ${info.name}${slot}`, ...extra].map((t) => <span key={t} className="wd-band-line">{t}</span>)}
    </div>
  );
}

const TAP_PX = 5; // weiter bewegt = Ziehen, kein Klick

/** Kurzer Klick (ohne Verschieben) auf das Element: meldet die Stelle in Pixeln ab der Innenkante. Das Band selbst fängt
 *  keine Maus, damit Ziehen und Rad bei der Bühne bleiben; deshalb hört der Klick an der Bühne mit. */
function useTap(el: HTMLElement | null, onTap: (x: number, y: number) => void) {
  const latest = useRef(onTap);
  latest.current = onTap;
  useEffect(() => {
    if (!el) return;
    let down: { x: number; y: number } | null = null;
    const onDown = (e: PointerEvent) => { down = e.button === 0 ? { x: e.clientX, y: e.clientY } : null; };
    const onUp = (e: PointerEvent) => {
      if (down && Math.abs(e.clientX - down.x) < TAP_PX && Math.abs(e.clientY - down.y) < TAP_PX) {
        const r = el.getBoundingClientRect();
        latest.current(e.clientX - r.left - el.clientLeft, e.clientY - r.top - el.clientTop);
      }
      down = null;
    };
    el.addEventListener('pointerdown', onDown);
    el.addEventListener('pointerup', onUp);
    return () => { el.removeEventListener('pointerdown', onDown); el.removeEventListener('pointerup', onUp); };
  }, [el]);
}

/** Chunk-Infos der Live-Vorschau (B-485, WZ17.4): ein Overlay fest am oberen Rand der Bühne, wo im Spiel die HUD-Leiste liegt, je Chunk Farbe (Hinweis, Hub, besonders, neutral), Emoji, deutscher Name, Platz, Seed und feste Objekte.
 *  Lage und Breite folgen Weltposition und Zoom der Live-Vorschau (`view`, Bühne `stage`); nur im Welten Editor, die Bühne
 *  selbst bleibt unverändert. Fängt keine Maus (Ziehen und Rad gehen an die Bühne); ein kurzer Klick ins Overlay
 *  meldet den Chunk darunter an `onChunk` (Sprung dorthin). */
export function LiveChunkBand({ stage, view, model, draft, hints, onChunk }: {
  stage: Box; view: View; model: LevelModel | null; draft: readonly PlannedChunk[]; hints: readonly SlotHint[]; onChunk: (chunk: ModelChunk) => void;
}) {
  const band = bandBox();
  useTap(stage.el, (x, y) => {
    if (!model || y < band.top || y > band.top + band.height) return;
    const units = view.left + x / view.scale;
    const hit = model.chunks.find((c) => c.from <= units && units < c.to);
    if (hit) onChunk(hit);
  });
  if (!stage.el || !model || stage.h <= 0) return null;
  return createPortal(
    <div className="wd-band" aria-label="Chunk-Infos" style={{ top: band.top, height: band.height, fontSize: Math.min(12, Math.max(9, band.height * 0.7)), lineHeight: `${LINE_PX}px` }}>
      {bandChunks(model.chunks, view, stage.w).map(({ chunk, x, w }) => (
        <BandCell key={chunk.index} info={chunkInfo(model.chunks, chunk, draft, hints)} w={w} h={band.height} style={{ left: x, width: w }} />
      ))}
    </div>,
    stage.el,
  );
}
