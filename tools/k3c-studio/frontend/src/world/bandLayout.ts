import { hudPx } from '../stage/hudBar';
import type { PlannedChunk } from '../api';
import type { MarkKind, SlotHint } from './chunkMarks';
import { iconOf, nameOf } from './kindIcons';
import { SPECIAL_CHUNKS, type ModelChunk } from './levelView';
import { slotLabel } from './plan';
import type { View } from './viewState';

// Chunk-Infos im Bodenstreifen der Live-Vorschau (B-485, WZ17.4): reine Logik, kein DOM. Die Bühne (PreviewStage.applyView)
// zeigt die Welt mit Zoom = Maßstab / UNIT_PX, die Unterkante der Bühne ist GAME_HEIGHT, links beginnt sie bei `view.left`.

/** Lage der Chunk-Infos auf der Bühne in Pixeln: fest oben auf der Leiste des globalen HUDs (stage/hudBar.ts), mindestens eine
 *  gut lesbare Zeile hoch */
export function bandBox(scale: number): { top: number; height: number } {
  return { top: 0, height: Math.max(MIN_BAND_PX, hudPx(scale)) };
}

const MIN_BAND_PX = 20;

export interface BandChunk {
  chunk: ModelChunk;
  /** sichtbarer Teil auf der Bühne in Pixeln (linke Kante, Breite); so bleibt der Text im Bild, auch am Rand */
  x: number;
  w: number;
}

/** Sichtbare Chunks mit ihrem Pixelbereich auf der Bühne (gleiche Abbildung wie die Kamera der Live-Vorschau:
 *  x = (Anfang − linke Kante) · Maßstab), auf die Bühne `stageW` beschnitten. */
export function bandChunks(chunks: readonly ModelChunk[], view: View, stageW: number): BandChunk[] {
  return chunks
    .map((chunk) => {
      const from = Math.max(0, (chunk.from - view.left) * view.scale);
      const to = Math.min(stageW, (chunk.to - view.left) * view.scale);
      return { chunk, x: from, w: to - from };
    })
    .filter((b) => b.w > 0);
}

/** Darstellung je Platz: nur Farbe, kompakt (Emoji, Platz), eine Zeile oder mehrzeilig */
export type BandMode = 'color' | 'compact' | 'line' | 'full';

/** Höhe einer Zeile in Pixeln bei voller Schrift (12 px, Zeilenhöhe 1,15) */
export const LINE_PX = 14;

/** Darstellung für einen Pixelbereich w × h mit `lines` Zeilen Inhalt: mehrzeilig nur, wenn alle Zeilen hineinpassen. */
export function bandMode(w: number, h: number, lines = 1): BandMode {
  if (h < 8 || w < 28) return 'color';
  if (w < 70 || h < 13) return 'compact';
  return lines > 1 && h >= lines * LINE_PX && w >= 140 ? 'full' : 'line';
}

/** Platz eines Abschnitts relativ zu den Hub-Abschnitten (+n rechts, −n links, 0 = Hub oder ohne Hub) */
export function slotOfIndex(chunks: readonly ModelChunk[], index: number): number {
  const hub = chunks.filter((c) => c.kind === 'hub').map((c) => c.index);
  if (hub.length === 0) return 0;
  const [first, last] = [Math.min(...hub), Math.max(...hub)];
  return index > last ? index - last : index < first ? index - first : 0;
}

/** Farbe der Markierung: Hinweis (geplant, Fehler), sonst Hub, besonderer Abschnitt oder neutral */
export type BandColor = MarkKind | 'hub' | 'special' | 'none';

export interface ChunkInfo {
  color: BandColor;
  icon: string;
  name: string;
  /** „+1“, „−2“; leer bei Hub, Ausgang und Rand */
  slot: string;
  seed: string;
  objects: number;
}

const NO_SLOT = new Set(['hub', 'exit', 'edge']);

/** Inhalt der Info je Chunk aus dem Level, dem Entwurf (`draft`, Seed und feste Objekte) und den Hinweisen */
export function chunkInfo(chunks: readonly ModelChunk[], chunk: ModelChunk, draft: readonly PlannedChunk[], hints: readonly SlotHint[]): ChunkInfo {
  const slot = NO_SLOT.has(chunk.kind) ? 0 : slotOfIndex(chunks, chunk.index);
  const planned = slot === 0 ? undefined : draft.find((c) => c.slot === slot);
  const hint = slot === 0 ? undefined : hints.find((h) => h.slot === slot);
  const color: BandColor = hint?.kind ?? (chunk.kind === 'hub' ? 'hub' : SPECIAL_CHUNKS.has(chunk.kind) ? 'special' : 'none');
  return {
    color, icon: iconOf(chunk.kind), name: nameOf(chunk.kind), slot: slot === 0 ? '' : slotLabel(slot).replace('-', '−'),
    seed: planned?.seed ?? '', objects: planned?.objects?.length ?? 0,
  };
}
