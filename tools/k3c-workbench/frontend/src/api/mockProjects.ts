import type { PlanningData, PlanProject } from './types';

// Erfundene Projekte für den Mock (B-358): drei aktive nach Rang (BET mit laufendem Sprint), ein ruhendes, ein
// erledigtes, ABN mit offener Abnahme am Gerät; M5 und B-011 bleiben ohne Projekt.

const p = (id: string, title: string, status: string, rang: string, sprints: string[]): PlanProject => ({ id, title, status, rang, sprints });

/** Hängt Projekte, den ABN-Sprint X1 und die Projekt-Felder an die Mock-Planung. */
export function withProjects(d: PlanningData): PlanningData {
  d.projects = [
    p('BET', 'Betrieb im Heimnetz', 'aktiv', '1', ['SP11', 'F2']),
    p('SKL', 'Skills', 'aktiv', '2', ['S1']),
    p('RGW', 'Regelwerk', 'aktiv', '3', ['F1']),
    p('ABN', 'Abnahmen am Gerät', 'aktiv', '–', ['X1']),
    p('GRA', 'Grafik', 'ruht', '–', []),
    p('FND', 'Fundament', 'erledigt', '–', ['R1']),
  ];
  d.sprints.push({ id: 'X1', title: 'Xbox-Abnahmen', domain: 'PLAT', status: 'aktiv', reife: 'bereit', spec: 'freigegeben',
    tickets: ['B-006'], sessions: [{ nr: 'X1.1', typ: 'Workshop', agent: 'Mensch', status: 'offen', titel: 'Gamepad am TV', env: 'live', domain: 'PLAT' }] });
  const of = new Map(d.projects.flatMap((x) => x.sprints.map((id) => [id, x.id] as const)));
  for (const s of d.sprints) s.project = of.get(s.id) ?? '–';
  const tickets: Record<string, string> = { 'B-007': 'SKL', 'B-112': 'SKL', 'B-006': 'ABN', 'B-035': 'BET', 'B-134': 'RGW', 'B-137': 'BET' };
  for (const t of d.tickets) t.project = tickets[t.nr] ?? '–';
  return d;
}

/** Wie planning.Set in Go für `Rang`: nur aktive Projekte mit Rang, 1 … n, die anderen rücken lückenlos. */
export function setRank(d: PlanningData, id: string, value: string) {
  const ranked = (d.projects ?? []).filter((x) => x.status === 'aktiv' && x.rang !== '–').sort((a, b) => +a.rang - +b.rang);
  const at = ranked.findIndex((x) => x.id === id);
  const to = Number(value);
  if (at < 0) throw new Error(`${id} hat keinen Rang (ruht, erledigt oder ABN)`);
  if (!Number.isInteger(to) || to < 1 || to > ranked.length) throw new Error(`Rang ${value} außerhalb 1 … ${ranked.length}`);
  const [moved] = ranked.splice(at, 1);
  ranked.splice(to - 1, 0, moved);
  ranked.forEach((x, i) => { x.rang = String(i + 1); });
}
