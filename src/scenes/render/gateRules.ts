import type { Site } from '../../model/types';

type LineSite = Pick<Site, 'kind' | 'x' | 'state' | 'level'>;

/**
 * Stufe, in der ein Tor gezeichnet wird (GEB4 AC-05): die Stufe der Mauer seiner Linie. Das Tor steht
 * `wallLines.gateOutsetUnits` außen vor seiner Mauer (`data/hub.json`), seine Mauer ist also die nächste Mauer weiter
 * innen auf derselben Seite relativ zu `hubX`. Ohne Mauer oder ist sie nicht gebaut: Stufe 1; `level` fehlt = 1.
 * Reine Auswahl ohne Phaser, nur Darstellung.
 */
export function gateStage(gate: Pick<Site, 'x'>, sites: readonly LineSite[], hubX: number): number {
  const side = Math.sign(gate.x - hubX);
  const reach = Math.abs(gate.x - hubX);
  let wall: LineSite | undefined;
  for (const s of sites) {
    if (s.kind !== 'wall' || Math.sign(s.x - hubX) !== side) continue;
    const d = Math.abs(s.x - hubX);
    if (d < reach && (!wall || d > Math.abs(wall.x - hubX))) wall = s;
  }
  return wall?.state === 'built' ? (wall.level ?? 1) : 1;
}
