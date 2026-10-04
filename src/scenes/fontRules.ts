import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import type { Cell } from './layout';

/**
 * Schrift des Spiels (S4.2, Beschluss Q03, `docs/rules/bedienung.md` § 2): ein Katalog aller Schriftrollen und die
 * Mindestgrößen je Layout. Reine Daten und Rechnung, ohne Phaser; HUD und Renderer lesen ihre Größen von hier.
 */

/** `info` = Pflicht-Info (Gold, Vorrat, HP, Warnungen, Meldungen), `side` = Nebeninfo (Hinweise, Tastenhilfen) */
export type FontArt = 'info' | 'side';

export interface FontRole {
  px: number;
  art: FontArt;
  /** `hud`: bildschirmfest, nicht skaliert; `world`: im Szenenraum, mit dem Zoom der Zellen-Kamera verkleinert */
  space: 'hud' | 'world';
  color: string;
}

/** Konturfarbe aller Texte: trennt Text und Hintergrund, Grundlage des Kontrasttests */
export const OUTLINE_COLOR = '#000000';

/**
 * Welt-Texte stehen im Szenenraum und schrumpfen mit `Zelle.h / GAME_HEIGHT` (Zoom 0,5 im Streifen und Viertel):
 * 56 px ergeben dort 28 px (Pflicht-Info im Streifen, `ceil(28 / 0,5)`).
 */
export const FONTS = {
  playerValue: { px: 28, art: 'info', space: 'hud', color: '#ffffff' },
  shared: { px: 28, art: 'info', space: 'hud', color: '#ffffff' },
  fight: { px: 28, art: 'info', space: 'hud', color: '#ff8fa3' },
  banner: { px: 56, art: 'info', space: 'hud', color: '#ffffff' },
  travel: { px: 40, art: 'info', space: 'hud', color: '#ffd166' },
  joinCenter: { px: 44, art: 'info', space: 'hud', color: '#ffffff' },
  joinCorner: { px: 26, art: 'side', space: 'hud', color: '#ffffff' },
  controlsHint: { px: 24, art: 'side', space: 'hud', color: '#ffffff' },
  roomInfo: { px: 24, art: 'side', space: 'hud', color: '#ffffff' },
  exitSign: { px: 56, art: 'info', space: 'world', color: '#ffffff' },
  priceTag: { px: 56, art: 'info', space: 'world', color: '#ffffff' },
  purse: { px: 56, art: 'info', space: 'world', color: '#ffd166' },
} as const satisfies Record<string, FontRole>;

export type FontName = keyof typeof FONTS;

/** Phaser-Stil-Anteil einer Rolle: Größe und Farbe */
export function fontStyle(name: FontName): { fontSize: string; color: string } {
  return { fontSize: `${FONTS[name].px}px`, color: FONTS[name].color };
}

/**
 * Mindestgröße in px (bezogen auf 1920 × 1080, nach der Skalierung der Zelle) laut `docs/rules/bedienung.md` § 2.
 * Pflicht-Info: 28 px in Zellen über die ganze Breite (Vollbild, Streifen, breiter Streifen bei 3), 24 px in den Vierteln.
 * Nebeninfo: 24 px bei 1 und 2 Spielern, 20 px bei 3 und 4. Die Mitspieler-Zelle (`partner`) nennt die Regel nicht: `null`.
 */
export function minFontPx(seats: number, cell: Pick<Cell, 'w' | 'kind'>, art: FontArt): number | null {
  if (cell.kind === 'partner') return null;
  if (art === 'info') return cell.w >= GAME_WIDTH ? 28 : 24;
  return seats <= 2 ? 24 : 20;
}

/** Größe, in der der Text auf dem Bildschirm erscheint: HUD unverändert, Welt mit dem Kamera-Zoom der Zelle (`GameScene.layoutCameras`) */
export function effectiveFontPx(font: Pick<FontRole, 'px' | 'space'>, cell: Pick<Cell, 'h'>): number {
  return font.space === 'hud' ? font.px : (font.px * cell.h) / GAME_HEIGHT;
}

function luminance(hex: string): number {
  const channel = (i: number) => {
    const c = parseInt(hex.slice(1 + i * 2, 3 + i * 2), 16) / 255;
    return c <= 0.03928 ? c / 12.92 : ((c + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(0) + 0.7152 * channel(1) + 0.0722 * channel(2);
}

/** Kontrastverhältnis nach WCAG, 1 bis 21; Farben als `#rrggbb` */
export function contrastRatio(a: string, b: string): number {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x);
  return (hi! + 0.05) / (lo! + 0.05);
}
