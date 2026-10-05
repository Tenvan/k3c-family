/** Zeichnet einen Verlauf der Monitoring-Seite auf Canvas (B-282), ohne Chart-Bibliothek. */

export interface ChartLine {
  label: string;
  color: string;
  points: { t: number; v: number }[];
}

export interface ChartMark {
  t: number;
  color: string;
  /** Gestrichelt mit Beschriftung, z. B. „Neustart“. */
  label?: string;
}

export interface ChartSpec {
  lines: ChartLine[];
  from: number;
  to: number;
  unit: string;
  /** Werte darüber werden als Ausreißer rot markiert. */
  outlier?: number;
  marks: ChartMark[];
  /** Gewählte Stelle (Klick auf ein Ereignis). */
  cursor?: number;
}

export const PALETTE = ['#4cc9f0', '#ffd166', '#06d6a0', '#ef476f', '#b388ff', '#f78c6b', '#90be6d', '#e0e0e0'];
const HEIGHT = 160;
const PAD = { top: 18, bottom: 4 };
const OUTLIER = '#ff4d4d';

const fmt = (v: number) => v.toLocaleString('de-DE', { maximumFractionDigits: 1 });

function prepare(canvas: HTMLCanvasElement): CanvasRenderingContext2D | null {
  const dpr = window.devicePixelRatio || 1;
  const w = Math.max(1, canvas.clientWidth);
  canvas.width = Math.round(w * dpr);
  canvas.height = Math.round(HEIGHT * dpr);
  const ctx = canvas.getContext('2d');
  ctx?.setTransform(dpr, 0, 0, dpr, 0, 0);
  ctx?.clearRect(0, 0, w, HEIGHT);
  return ctx;
}

function drawMark(ctx: CanvasRenderingContext2D, x: number, mark: ChartMark): void {
  ctx.strokeStyle = mark.color;
  ctx.setLineDash(mark.label ? [4, 3] : []);
  ctx.beginPath();
  ctx.moveTo(x, mark.label ? 0 : HEIGHT - 10);
  ctx.lineTo(x, HEIGHT);
  ctx.stroke();
  ctx.setLineDash([]);
  if (mark.label) {
    ctx.fillStyle = mark.color;
    ctx.fillText(mark.label, x + 3, HEIGHT - 6);
  }
}

function drawLine(ctx: CanvasRenderingContext2D, line: ChartLine, x: (t: number) => number, y: (v: number) => number, outlier: number): void {
  ctx.strokeStyle = line.color;
  ctx.beginPath();
  line.points.forEach((p, i) => (i === 0 ? ctx.moveTo(x(p.t), y(p.v)) : ctx.lineTo(x(p.t), y(p.v))));
  ctx.stroke();
  ctx.fillStyle = OUTLIER;
  for (const p of line.points) {
    if (p.v > outlier) ctx.fillRect(x(p.t) - 2.5, y(p.v) - 2.5, 5, 5);
  }
}

export function drawChart(canvas: HTMLCanvasElement, spec: ChartSpec): void {
  const ctx = prepare(canvas);
  if (!ctx) return;
  const w = canvas.clientWidth;
  const top = spec.lines.reduce((m, l) => l.points.reduce((a, p) => Math.max(a, p.v), m), 1) * 1.1;
  const x = (t: number) => ((t - spec.from) / Math.max(1, spec.to - spec.from)) * w;
  const y = (v: number) => PAD.top + (1 - v / top) * (HEIGHT - PAD.top - PAD.bottom);
  ctx.font = '12px system-ui, sans-serif';
  ctx.lineWidth = 1;
  ctx.strokeStyle = '#2e3a4d';
  ctx.strokeRect(0.5, 0.5, w - 1, HEIGHT - 1);
  for (const m of spec.marks) drawMark(ctx, x(m.t), m);
  ctx.lineWidth = 1.5;
  for (const line of spec.lines) drawLine(ctx, line, x, y, spec.outlier ?? Infinity);
  if (spec.cursor !== undefined) drawMark(ctx, x(spec.cursor), { t: spec.cursor, color: '#ffd166', label: '▼' });
  ctx.fillStyle = '#8a97aa';
  ctx.fillText(`max ${fmt(top / 1.1)} ${spec.unit}`, 4, 13);
  let lx = w;
  for (const line of [...spec.lines].reverse()) {
    lx -= ctx.measureText(line.label).width + 10;
    ctx.fillStyle = line.color;
    ctx.fillText(line.label, lx, 13);
  }
}
