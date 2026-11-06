import { useState, type MouseEvent } from 'react';
import type { Box } from '../api';
import { bildPunkt, teilAn } from './hit';

interface Props {
  teile: Record<string, Box>;
  reihenfolge: string[];
  selected: string;
  onSelect: (teil: string) => void;
}

interface Size {
  width: number;
  height: number;
}

/** Klickfläche genau über dem Vorschaubild: wählt das Teil unter dem Zeiger (D-05) und rahmt es. */
export function TeilLayer({ teile, reihenfolge, selected, onSelect }: Props) {
  const [natural, setNatural] = useState<Size>({ width: 0, height: 0 });
  const pick = (e: MouseEvent<HTMLDivElement>) => {
    const layer = e.currentTarget;
    const img = layer.parentElement?.querySelector('img');
    const rect = layer.getBoundingClientRect();
    const size = { width: img?.naturalWidth ?? 0, height: img?.naturalHeight ?? 0 };
    setNatural(size);
    const [x, y] = bildPunkt(e.clientX - rect.left, e.clientY - rect.top, rect, size);
    onSelect(teilAn(teile, reihenfolge, x, y));
  };
  const box = selected ? teile[selected] : undefined;
  return (
    <div data-hit-layer className="as-hit" onClick={pick} title="Teil wählen">
      {box && <Mark id={selected} box={box} natural={natural} />}
    </div>
  );
}

/** Rahmen in Prozent der Bildgröße (folgt der Skalierung); ohne bekannte Größe in Bildpixeln. */
function Mark({ id, box: [x, y, b, h], natural }: { id: string; box: Box; natural: Size }) {
  const unit = (v: number, total: number) => (total > 0 ? `${(v / total) * 100}%` : `${v}px`);
  return (
    <div data-teil-mark={id} className="as-mark"
      style={{ left: unit(x, natural.width), top: unit(y, natural.height), width: unit(b, natural.width), height: unit(h, natural.height) }} />
  );
}
