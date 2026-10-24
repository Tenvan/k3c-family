import { HUB } from '../../model/data';
import type { Site, World } from '../../model/types';

/**
 * Welche unbezahlten Bauplätze dürfen jetzt gebaut werden (B-584): Spiegel von `sitePayable`/`lineOpen` im Server
 * (`engine/sim/economy.go`, `world_lines.go`), nur für die Anzeige des Bauschilds. Der Server entscheidet weiter allein.
 * Hub-Plätze ab ihrer Hub-Stufe (`data/hub.json`), Mauer und Turm der Linie k ab Hub-Stufe k, wenn alle Mauern innen auf
 * derselben Seite stehen, das Tor nur an der äußersten gebauten Mauer seiner Seite.
 */

const LINE_KINDS = ['wall', 'tower', 'gate'] as const;
type View = Pick<World, 'sites' | 'hubX'> & { hubLevel?: number };

/** Hub-Stufe, ab der ein Platz frei ist: Eintrag aus `hub.json` an seiner Stelle, bei Ruinen die kleinste seiner Art, sonst 1 */
export function siteHubLevel(s: Pick<Site, 'kind' | 'x' | 'ruin'>, hubX: number): number {
  const all = [...HUB.sites, ...HUB.islandSites].filter((h) => h.kind === s.kind);
  if (s.ruin) return all.length ? Math.max(1, Math.min(...all.map((h) => h.hubLevel))) : 1;
  const at = all.find((h) => Math.abs(hubX + h.offsetUnits - s.x) < 0.001);
  return Math.max(1, at?.hubLevel ?? 1);
}

/** Linie (1 = innerste) eines Mauer-, Turm- oder Tor-Platzes auf seiner Seite des Hubs, 0 für andere Plätze */
function lineIndex(sites: Site[], s: Site, hubX: number): number {
  if (!(LINE_KINDS as readonly string[]).includes(s.kind)) return 0;
  const side = Math.sign(s.x - hubX);
  const same = sites.filter((o) => o.kind === s.kind && Math.sign(o.x - hubX) === side).sort((a, b) => Math.abs(a.x - hubX) - Math.abs(b.x - hubX));
  return same.indexOf(s) + 1;
}

/** Nummern der Bauplätze, die jetzt Münzen annehmen würden */
export function buildableSites(world: View): Set<number> {
  const level = world.hubLevel ?? 1;
  const wallBuilt = (side: number, k: number): boolean =>
    world.sites.some((w) => w.kind === 'wall' && Math.sign(w.x - world.hubX) === side && lineIndex(world.sites, w, world.hubX) === k && w.state === 'built');
  const out = new Set<number>();
  for (const s of world.sites) {
    if (s.state !== 'unpaid') continue;
    const k = lineIndex(world.sites, s, world.hubX);
    if (k === 0) {
      if (level >= siteHubLevel(s, world.hubX)) out.add(s.id);
      continue;
    }
    const side = Math.sign(s.x - world.hubX);
    if (s.kind === 'gate') {
      if (level >= Math.max(HUB.wallLines.gateHubLevel, k) && wallBuilt(side, k) && !wallBuilt(side, k + 1)) out.add(s.id);
      continue;
    }
    let open = level >= k;
    for (let j = 1; j < k && open; j++) open = wallBuilt(side, j);
    if (open) out.add(s.id);
  }
  return out;
}
