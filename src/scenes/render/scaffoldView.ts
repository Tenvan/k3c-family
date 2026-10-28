import type Phaser from 'phaser';
import type { Site, SiteKind } from '../../model/types';
import { showsBuildSign } from './buildSignView';

/**
 * Gerüst im Bau (B-586, GEB16): Ein bezahlter, noch nicht fertiger Platz zeigt ein Gerüst in Gebäudeform statt des Bauschilds.
 * Größenklasse = Breite des Gebäude-Sprites in Kacheln (`geruest-<breite>.json`, `docs/standards/gebaeude-grafik.md` § 1);
 * fehlt die Grafik, zeichnet `drawScaffoldShape` eine einfache Form. Zeichnung nur, keine Spiel-Logik.
 */

export type ScaffoldClass = 1 | 2 | 3 | 4 | 6;

/** Größenklasse je Platzart: Mauer 1 × 5, Tor 2 × 4, Turm 3 × 8, Baumschule und Treppen 4 × 3, Hub-Gebäude 6 × 4 */
export const SCAFFOLD_CLASS: Record<SiteKind, ScaffoldClass> = {
  wall: 1,
  tower: 3,
  nursery: 4,
  stairsUp: 4,
  stairsDown: 4,
  workshop: 6,
  storage: 6,
  barracks: 6,
  tavern: 6,
  healer: 6,
  smithy: 6,
  armory: 6,
};

export const SCAFFOLD_CLASSES: readonly ScaffoldClass[] = [1, 2, 3, 4, 6];

/** Textur-Schlüssel des Gerüsts einer Größenklasse (`BUILDING_TEXTURES` lädt `gebaeude/geruest-<klasse>-32.png`) */
export const scaffoldKey = (c: ScaffoldClass): string => `grafik:geruest-${c}`;

/** Textur des Gerüsts für eine Platzart */
export const scaffoldTexture = (kind: SiteKind): string => scaffoldKey(SCAFFOLD_CLASS[kind]);

/** Im Bau: bezahlt, aber nicht fertig (wartet auf Material oder Bauer, auch mit Fortschritt) */
export const showsScaffold = (s: Pick<Site, 'state'>): boolean => s.state === 'waitingMaterial' || s.state === 'waitingWorker';

/** Was am ungebauten Platz steht: Gerüst im Bau, sonst Bauschild bei `payable`, nie beides (B-586/AC-02) */
export function siteMarker(s: Pick<Site, 'state' | 'payable'>): 'scaffold' | 'sign' | null {
  if (showsScaffold(s)) return 'scaffold';
  return showsBuildSign(s) ? 'sign' : null;
}

/** Rückfall ohne Grafik: Stangen und Laufbretter in Platzhalter-Größe `w` × `h`, Fuß bei (0, 0) */
export function drawScaffoldShape(g: Phaser.GameObjects.Graphics, w: number, h: number): void {
  g.fillStyle(0x663931);
  for (const x of [-w / 2, -2, w / 2 - 4]) g.fillRect(x, -h, 4, h);
  g.fillStyle(0x8f563b);
  for (let y = -h + 8; y < 0; y += 32) g.fillRect(-w / 2, y, w, 4);
}
