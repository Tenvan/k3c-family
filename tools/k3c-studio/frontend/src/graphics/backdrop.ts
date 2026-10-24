import { biomOf, type Licht } from './gameScenesLogic';

/** Himmel, zwei Hügelbänder und Boden in den Farben des Bioms (`palette`); Nacht legt einen dunkelblauen Schleier darüber. */
export function zeichneHintergrund(ctx: CanvasRenderingContext2D, w: number, h: number, groundY: number, biomId: string, licht: Licht): void {
  const p = biomOf(biomId).palette;
  ctx.fillStyle = p.sky;
  ctx.fillRect(0, 0, w, h);
  ctx.fillStyle = p.far;
  ctx.fillRect(0, groundY - h * 0.35, w, h * 0.35);
  ctx.fillStyle = p.near;
  ctx.fillRect(0, groundY - h * 0.15, w, h * 0.15);
  ctx.fillStyle = p.ground;
  ctx.fillRect(0, groundY, w, h - groundY);
  if (licht === 'nacht') {
    ctx.fillStyle = 'rgba(8, 10, 40, 0.55)';
    ctx.fillRect(0, 0, w, h);
  }
}

/** Platzhalter-Monarch: Körper und Krone, Fuß bei (x, groundY); die Figur selbst zeigt die Szene Figuren. */
export function zeichneMonarch(ctx: CanvasRenderingContext2D, x: number, groundY: number): void {
  ctx.fillStyle = '#d9d9d9';
  ctx.fillRect(x - 7, groundY - 32, 14, 32);
  ctx.fillStyle = '#ffd23c';
  ctx.fillRect(x - 7, groundY - 38, 14, 6);
}
