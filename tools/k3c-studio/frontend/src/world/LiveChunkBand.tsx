import type { CSSProperties } from 'react';
import { createPortal } from 'react-dom';
import type { PlannedChunk } from '../api';
import type { SlotHint } from './chunkMarks';
import type { LevelModel } from './levelView';
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

/** Chunk-Infos im Bodenstreifen der Live-Vorschau (B-485, WZ17.4): ein Overlay über der Bühne zwischen Boden-Oberfläche
 *  und Fluss, je Chunk Farbe (Hinweis, Hub, besonders, neutral), Emoji, deutscher Name, Platz, Seed und feste Objekte.
 *  Lage und Breite folgen Weltposition und Zoom der Live-Vorschau (`view`, Bühne `stage`); nur im Welten Editor, die Bühne
 *  selbst bleibt unverändert. Fängt keine Maus (Ziehen und Rad gehen an die Bühne). */
export function LiveChunkBand({ stage, view, model, draft, hints }: {
  stage: Box; view: View; model: LevelModel | null; draft: readonly PlannedChunk[]; hints: readonly SlotHint[];
}) {
  if (!stage.el || !model || stage.h <= 0) return null;
  const band = bandBox(stage.h, view.scale);
  if (band.height < 2) return null;
  return createPortal(
    <div className="wd-band" aria-label="Chunk-Infos" style={{ top: band.top, height: band.height, fontSize: Math.min(12, Math.max(9, band.height * 0.7)), lineHeight: `${LINE_PX}px` }}>
      {bandChunks(model.chunks, view, stage.w).map(({ chunk, x, w }) => (
        <BandCell key={chunk.index} info={chunkInfo(model.chunks, chunk, draft, hints)} w={w} h={band.height} style={{ left: x, width: w }} />
      ))}
    </div>,
    stage.el,
  );
}
