import { BUILDINGS } from '../model/data';
import type { Site, World } from '../model/types';
import { PRICE_TAG_RANGE } from './viewRules';

/**
 * Welches Preisschild steht (B-395, „ein Element je Weltposition“): Jeder lebende Spieler zeigt nur das nächste
 * Preisschild in Reichweite, die übrigen entfallen. Reine Anzeige-Regel ohne Simulation.
 */

/** Zeigt dieser Bauplatz ein Preisschild: unbezahlt oder gebaute Werkstatt mit freiem Bogen-Regal */
export function offersPrice(s: Site): boolean {
  if (s.state === 'unpaid') return true;
  return s.state === 'built' && s.kind === 'workshop' && s.bows < (BUILDINGS.workshop.bowRack ?? 0);
}

/** Nummern der Bauplätze, deren Preisschild steht; bei gleichem Abstand gewinnt die kleinere Nummer */
export function priceTagSites(world: Pick<World, 'sites' | 'players'>): Set<number> {
  const candidates = world.sites.filter(offersPrice);
  const shown = new Set<number>();
  for (const p of world.players) {
    if (p.respawnIn > 0) continue;
    let best: Site | undefined;
    for (const s of candidates) {
      const d = Math.abs(s.x - p.x);
      if (d >= PRICE_TAG_RANGE) continue;
      if (!best || d < Math.abs(best.x - p.x) || (d === Math.abs(best.x - p.x) && s.id < best.id)) best = s;
    }
    if (best) shown.add(best.id);
  }
  return shown;
}
