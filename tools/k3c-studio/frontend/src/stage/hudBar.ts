import { GAME_HEIGHT, UNIT_PX } from '@game/core/constants';
import { BAR_HEIGHT, WORLD_TOP, viewZoom } from '@game/scenes/layout';

// Aufteilung der Bühne wie im Spiel: oben die Leiste des globalen HUDs (BAR_HEIGHT von GAME_HEIGHT), darunter der Weltausschnitt
// von WORLD_TOP bis GAME_HEIGHT. Im Spiel skaliert die Welt mit viewZoom(Zellenhöhe); die Leiste ist dort BAR_HEIGHT Pixel hoch,
// in Welt-Pixeln also BAR_HEIGHT / viewZoom. Die Bühne rechnet beides mit dem Kamera-Zoom (Maßstab / UNIT_PX) um.

const WORLD_PX = GAME_HEIGHT - WORLD_TOP;
const HUD_WORLD_PX = BAR_HEIGHT / viewZoom(GAME_HEIGHT - BAR_HEIGHT);

const zoomOf = (scale: number): number => scale / UNIT_PX;

/** Höhe der HUD-Leiste oben in Bildschirm-Pixeln bei Maßstab `scale` (Pixel je Unit) */
export const hudPx = (scale: number): number => Math.round(HUD_WORLD_PX * zoomOf(scale));

/** Höhe des Weltausschnitts in Bildschirm-Pixeln */
export const worldPx = (scale: number): number => Math.round(WORLD_PX * zoomOf(scale));

/** Gesamthöhe der Bühne: Leiste plus Weltausschnitt */
export const stagePx = (scale: number): number => hudPx(scale) + worldPx(scale);
