import { BUILDINGS } from '../../model/data';
import type { Site, World } from '../../model/types';
import { PRICE_TAG_RANGE } from './viewRules';

/**
 * Welches Preisschild steht (B-395, „ein Element je Weltposition“): Jeder lebende Spieler zeigt nur das nächste
 * Preisschild in Reichweite, die übrigen entfallen. Reine Anzeige-Regel ohne Simulation.
 */

/** Zeigt dieser Bauplatz ein Preisschild: unbezahlt und vom Server als bezahlbar gemeldet (`payable`, B-653), oder gebaute Werkstatt mit freiem Bogen-Regal */
export function offersPrice(s: Site): boolean {
  if (s.state === 'unpaid') return s.payable === true;
  return s.state === 'built' && s.kind === 'workshop' && s.bows < (BUILDINGS.workshop.bowRack ?? 0);
}

/**
 * Nimmt die Burg Münzen für den Hub-Ausbau (B-591): Der Server meldet `hubUpgrade`, solange eine nächste Stufe
 * offen ist; ohne `state` ist das Gold noch offen. Mit `state` läuft der Ausbau schon (Wartegrund statt Angebot).
 */
export function offersHubUpgrade(world: Partial<Pick<World, 'hubUpgrade'>>): boolean {
  const up = world.hubUpgrade;
  return !!up && !up.state && up.gold > up.paid;
}

type Candidate = Pick<Site, 'id' | 'x'>;

/**
 * Nummern der Bauplätze, deren Preisschild steht; bei gleichem Abstand gewinnt die kleinere Nummer. Die Burg steht
 * mit ihrer Nummer (`castle.id`, wie der Zahlplatz des Hub-Ausbaus im Server) unter den Kandidaten (B-591).
 */
export function priceTagSites(world: Pick<World, 'sites' | 'players'> & Partial<Pick<World, 'castle' | 'hubUpgrade'>>): Set<number> {
  const candidates: Candidate[] = world.sites.filter(offersPrice);
  if (world.castle && offersHubUpgrade(world)) candidates.push(world.castle);
  const shown = new Set<number>();
  for (const p of world.players) {
    if (p.respawnIn > 0) continue;
    let best: Candidate | undefined;
    for (const s of candidates) {
      const d = Math.abs(s.x - p.x);
      if (d >= PRICE_TAG_RANGE) continue;
      if (!best || d < Math.abs(best.x - p.x) || (d === Math.abs(best.x - p.x) && s.id < best.id)) best = s;
    }
    if (best) shown.add(best.id);
  }
  return shown;
}
