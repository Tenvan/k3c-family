import { GAME_WIDTH } from '@game/core/constants';
import { BAR_HEIGHT } from '@game/scenes/layout';
import { BAR, drawBar, drawWarn } from '@game/scenes/kampfView';
import { fontStyle } from '@game/scenes/render/fontRules';
import { zeichneHintergrund, zeichneMonarch } from './backdrop';
import { canvasGraphics } from './canvasGraphics';
import { TOP_STRIP, type KampfBild } from './combatLogic';
import type { Licht } from './gameScenesLogic';

// Zeichnet die Szene „Kampf-Anzeigen“ in Spielkoordinaten (1920 px breit) mit `drawBar`/`drawWarn` aus `kampfView.ts` des Spiels;
// Texte mit den Schriftregeln des Spiels (`fontStyle`), Phasen-Text und Banner kommen fertig aus `kampfAnzeige.ts`.

export const KAMPF_W = GAME_WIDTH;
/** Höhe des gezeigten Ausschnitts: Leiste, Boss-Leiste, Banner und der Boden darunter */
export const KAMPF_H = 640;
/** Boden der Welt in der Szene (im Spiel `GROUND_Y`, hier nach oben gerückt) */
const GROUND = KAMPF_H - 50;

export interface KampfOptionen {
  biom: string;
  licht: Licht;
  spieler: 1 | 2;
  /** Deckkraft von Leiste und Banner (Auftritt, Ein-/Ausblenden) */
  leiste: number;
  banner: number;
}

function text(ctx: CanvasRenderingContext2D, name: 'bossBar' | 'banner', s: string, y: number): number {
  const f = fontStyle(name);
  const px = parseInt(f.fontSize, 10);
  ctx.font = `bold ${f.fontSize} sans-serif`;
  ctx.textAlign = 'center';
  ctx.textBaseline = 'top';
  ctx.lineJoin = 'round';
  ctx.strokeStyle = '#000000';
  let top = y;
  for (const line of s.split('\n')) {
    ctx.lineWidth = name === 'bossBar' ? 6 : 4;
    ctx.strokeText(line, GAME_WIDTH / 2, top);
    ctx.fillStyle = f.color;
    ctx.fillText(line, GAME_WIDTH / 2, top);
    top += Math.ceil(px * 1.2);
  }
  return top - y;
}

export function zeichneKampf(ctx: CanvasRenderingContext2D, bild: KampfBild, o: KampfOptionen): void {
  const g = canvasGraphics(ctx);
  zeichneHintergrund(ctx, KAMPF_W, KAMPF_H, GROUND, o.biom, o.licht);
  // Welt: Warnkreise am Boden (Pixel des Szenenraums), Monarchen
  drawWarn(g, bild.circles, GROUND);
  zeichneMonarch(ctx, o.spieler === 1 ? 400 : 440, GROUND);
  if (o.spieler === 2) {
    zeichneMonarch(ctx, 1480, GROUND);
    ctx.fillStyle = '#ffffff';
    ctx.fillRect(KAMPF_W / 2 - 2, BAR_HEIGHT, 4, KAMPF_H - BAR_HEIGHT);
  }
  // HUD: Spieler-Leiste, darunter Boss-Leiste mit Name und Phase, dann die Events (wie `KampfHud.draw`)
  ctx.fillStyle = 'rgba(0,0,0,0.55)';
  ctx.fillRect(0, 0, KAMPF_W, BAR_HEIGHT);
  let y = BAR_HEIGHT + BAR.gap;
  if (bild.boss) {
    ctx.globalAlpha = o.leiste;
    y += text(ctx, 'bossBar', [bild.boss.name, bild.boss.phase].filter(Boolean).join('  ·  '), y) + 4;
    drawBar(g, (GAME_WIDTH - BAR.w) / 2, y, bild.boss.share);
    y += BAR.h + BAR.gap;
  }
  ctx.globalAlpha = o.banner;
  if (bild.banner) text(ctx, 'banner', bild.banner, y);
  ctx.globalAlpha = 1;
  // Hilfslinie: oben bleiben 70 px für den Home-Button frei
  ctx.setLineDash([12, 8]);
  ctx.strokeStyle = '#ff8fa3';
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(0, TOP_STRIP);
  ctx.lineTo(KAMPF_W, TOP_STRIP);
  ctx.stroke();
  ctx.setLineDash([]);
  ctx.font = '20px sans-serif';
  ctx.textAlign = 'left';
  ctx.fillStyle = '#ff8fa3';
  ctx.fillText(`Home-Button: oben ${TOP_STRIP} px frei`, 8, TOP_STRIP - 24);
}
