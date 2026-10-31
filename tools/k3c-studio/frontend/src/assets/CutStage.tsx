import { useEffect, useRef, useState, type KeyboardEvent, type PointerEvent } from 'react';
import type { AssetRect } from '../api';
import { clampRect, fitScale, nudgeRect, type CutView } from './assets';

/** Höchste Höhe der Kachel; die Breite folgt dem Platz (Splitter), die Grafik wird eingepasst statt gescrollt. */
export const STAGE_MAX_H = 360;
const HANDLE = 10;

interface Props {
  src: string;
  cut: AssetRect;
  view: CutView;
  /** Maße der ganzen Grafik aus dem Katalog; sonst die geladenen */
  size?: { w: number; h: number };
  onCut: (cut: AssetRect) => void;
}

/** Breite des Elements, folgt Größenänderungen (Splitter, Fenster). */
function useWidth(ref: React.RefObject<HTMLElement | null>): number {
  const [w, setW] = useState(0);
  useEffect(() => {
    const el = ref.current;
    if (!el) return;
    setW(el.clientWidth);
    if (typeof ResizeObserver === 'undefined') return;
    const ro = new ResizeObserver((entries) => setW(entries[0]?.contentRect.width ?? el.clientWidth));
    ro.observe(el);
    return () => ro.disconnect();
  }, [ref]);
  return w;
}

/** Zeichnet die Grafik (Rahmen: ganz mit Maske, Rahmen und Anfasser; Ausschnitt: nur das Ausgeschnittene) in Quellpixeln. */
function draw(c: HTMLCanvasElement, g: CanvasRenderingContext2D, img: HTMLImageElement, cut: AssetRect, view: CutView, shown: { w: number; h: number }, scale: number): void {
  c.width = shown.w;
  c.height = shown.h;
  g.imageSmoothingEnabled = false;
  if (view === 'ausschnitt') {
    g.drawImage(img, cut.x, cut.y, cut.w, cut.h, 0, 0, cut.w, cut.h);
    return;
  }
  g.drawImage(img, 0, 0);
  g.fillStyle = 'rgba(8, 10, 22, 0.62)';
  g.fillRect(0, 0, c.width, cut.y);
  g.fillRect(0, cut.y + cut.h, c.width, c.height - cut.y - cut.h);
  g.fillRect(0, cut.y, cut.x, cut.h);
  g.fillRect(cut.x + cut.w, cut.y, c.width - cut.x - cut.w, cut.h);
  const line = scale > 0 ? Math.max(1, 2 / scale) : 1;
  g.lineWidth = line;
  g.strokeStyle = '#ffd24a';
  g.strokeRect(cut.x + line / 2, cut.y + line / 2, cut.w - line, cut.h - line);
  g.fillStyle = '#ffd24a';
  const hs = scale > 0 ? HANDLE / scale : HANDLE;
  g.fillRect(cut.x + cut.w - hs, cut.y + cut.h - hs, hs, hs);
}

type Grab = { mode: 'move' | 'size'; dx: number; dy: number };

/** Was ein Klick auf Bildpunkt `p` greift: Anfasser unten rechts (Größe), Rahmeninneres (Verschieben) oder daneben (Rahmen springt dorthin). */
function grab(p: { x: number; y: number }, cut: AssetRect, scale: number): Grab {
  const hs = (HANDLE * 1.5) / scale;
  const inX = (lo: number, hi: number) => p.x >= lo && p.x <= hi;
  const inY = (lo: number, hi: number) => p.y >= lo && p.y <= hi;
  if (inX(cut.x + cut.w - hs, cut.x + cut.w + hs) && inY(cut.y + cut.h - hs, cut.y + cut.h + hs)) return { mode: 'size', dx: 0, dy: 0 };
  if (inX(cut.x, cut.x + cut.w) && inY(cut.y, cut.y + cut.h)) return { mode: 'move', dx: p.x - cut.x, dy: p.y - cut.y };
  return { mode: 'move', dx: cut.w / 2, dy: cut.h / 2 };
}

/** Maus (Ziehen, Anfasser) und Tastatur (Pfeile, Shift = Größe) für den Rahmen. */
function useFrameInput(canvas: React.RefObject<HTMLCanvasElement | null>, o: { cut: AssetRect; view: CutView; scale: number; full: { w: number; h: number }; onCut: (c: AssetRect) => void }) {
  const drag = useRef<Grab | null>(null);
  const { cut, view, scale, full, onCut } = o;
  const move = (e: PointerEvent) => {
    const d = drag.current;
    const r = canvas.current?.getBoundingClientRect();
    if (!d || !r || scale <= 0) return;
    const p = { x: (e.clientX - r.left) / scale, y: (e.clientY - r.top) / scale };
    onCut(clampRect(d.mode === 'move' ? { ...cut, x: p.x - d.dx, y: p.y - d.dy } : { ...cut, w: p.x - cut.x, h: p.y - cut.y }, full));
  };
  const down = (e: PointerEvent) => {
    const r = canvas.current?.getBoundingClientRect();
    if (view !== 'rahmen' || !r || scale <= 0) return;
    drag.current = grab({ x: (e.clientX - r.left) / scale, y: (e.clientY - r.top) / scale }, cut, scale);
    canvas.current?.setPointerCapture?.(e.pointerId);
    move(e);
  };
  const key = (e: KeyboardEvent) => {
    const next = view === 'rahmen' ? nudgeRect(cut, e.key, e.shiftKey, full) : undefined;
    if (!next) return;
    e.preventDefault();
    onCut(next);
  };
  return { onPointerDown: down, onPointerMove: move, onPointerUp: () => { drag.current = null; }, onKeyDown: key };
}

/** Zeichnet die Grafik eingepasst: „rahmen“ ganz mit Maske und Rahmen (Maus, Tastatur), „ausschnitt“ nur das Ausgeschnittene. */
export function CutStage({ src, cut, view, size, onCut }: Props) {
  const host = useRef<HTMLDivElement>(null);
  const canvas = useRef<HTMLCanvasElement>(null);
  const [img, setImg] = useState<HTMLImageElement>();
  const [failed, setFailed] = useState(false);
  const boxW = useWidth(host);
  const full = size ?? (img ? { w: img.naturalWidth, h: img.naturalHeight } : { w: 0, h: 0 });
  const shown = view === 'rahmen' ? full : { w: cut.w, h: cut.h };
  const scale = fitScale({ w: boxW, h: STAGE_MAX_H }, shown);

  useEffect(() => {
    const i = new Image();
    i.onload = () => { setImg(i); setFailed(false); };
    i.onerror = () => setFailed(true);
    i.src = src;
  }, [src]);

  useEffect(() => {
    const c = canvas.current;
    const g = c?.getContext('2d');
    if (!c || !g || !img || shown.w <= 0 || shown.h <= 0) return;
    draw(c, g, img, cut, view, shown, scale);
  }, [img, cut, view, shown.w, shown.h, scale]);

  const input = useFrameInput(canvas, { cut, view, scale, full, onCut });

  if (failed) return <p className="as-warn">Bild lädt nicht: {src}</p>;
  const css = scale > 0 ? { width: shown.w * scale, height: shown.h * scale } : { width: 0, height: 0 };
  return (
    <div ref={host} className="as-stage" data-view={view} data-scale={scale}>
      <canvas
        ref={canvas}
        style={css}
        tabIndex={view === 'rahmen' ? 0 : -1}
        aria-label={view === 'rahmen' ? 'Ausschnitt-Rahmen: Pfeiltasten verschieben, Shift mit Pfeiltasten ändert die Größe' : 'Ausgeschnittenes Bild'}
        {...input}
      />
    </div>
  );
}
