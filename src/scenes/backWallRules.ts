import type { Site } from '../model/types';

/** Ein Abschnitt Hintergrundmauer (Glossar, B-442): von `from` bis `to` in Units, Stufe der äußeren Mauer; `id` = Bauplatz der äußeren Mauer. */
export interface BackWall {
  id: number;
  from: number;
  to: number;
  stage: number;
}

type WallSite = Pick<Site, 'id' | 'kind' | 'x' | 'state' | 'level'>;

/**
 * Hintergrundmauern zwischen je zwei benachbarten Mauern derselben Seite (GEB3 AC-08). Benachbart heißt: nach Abstand
 * zur Hub-Mitte aufeinanderfolgende Mauer-Plätze; ist einer davon nicht gebaut, entsteht dort kein Abschnitt.
 * Reine Auswahl ohne Phaser, Reihenfolge: linke Seite, dann rechte, je von innen nach außen.
 */
export function backWalls(sites: readonly WallSite[], hubX: number): BackWall[] {
  const out: BackWall[] = [];
  for (const side of [-1, 1]) {
    const walls = sites
      .filter((s) => s.kind === 'wall' && Math.sign(s.x - hubX) === side)
      .sort((a, b) => Math.abs(a.x - hubX) - Math.abs(b.x - hubX));
    for (let i = 1; i < walls.length; i++) {
      const inner = walls[i - 1]!;
      const outer = walls[i]!;
      if (inner.state !== 'built' || outer.state !== 'built') continue;
      out.push({ id: outer.id, from: Math.min(inner.x, outer.x), to: Math.max(inner.x, outer.x), stage: outer.level ?? 1 });
    }
  }
  return out;
}
