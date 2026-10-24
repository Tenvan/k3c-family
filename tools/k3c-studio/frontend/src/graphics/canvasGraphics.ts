import type Phaser from 'phaser';

/**
 * Minimale `Phaser.GameObjects.Graphics`-Fläche über einem 2D-Kontext: damit zeichnet das Studio mit denselben Funktionen
 * wie das Spiel (`drawBuildSign`, `drawBar`, `drawWarn`). Nur die Aufrufe, die diese Funktionen brauchen.
 */
const css = (color: number, alpha: number) => `rgba(${(color >> 16) & 255},${(color >> 8) & 255},${color & 255},${alpha})`;

export function canvasGraphics(ctx: CanvasRenderingContext2D): Phaser.GameObjects.Graphics {
  const g = {
    fillStyle(color: number, alpha = 1) {
      ctx.fillStyle = css(color, alpha);
      return g;
    },
    lineStyle(width: number, color: number, alpha = 1) {
      ctx.lineWidth = width;
      ctx.strokeStyle = css(color, alpha);
      return g;
    },
    fillRect(x: number, y: number, w: number, h: number) {
      ctx.fillRect(x, y, w, h);
      return g;
    },
    strokeRect(x: number, y: number, w: number, h: number) {
      ctx.strokeRect(x, y, w, h);
      return g;
    },
    fillCircle(x: number, y: number, r: number) {
      ctx.beginPath();
      ctx.arc(x, y, r, 0, Math.PI * 2);
      ctx.fill();
      return g;
    },
    strokeCircle(x: number, y: number, r: number) {
      ctx.beginPath();
      ctx.arc(x, y, r, 0, Math.PI * 2);
      ctx.stroke();
      return g;
    },
  };
  return g as unknown as Phaser.GameObjects.Graphics;
}
