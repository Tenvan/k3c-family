import type { WorldDetail } from '../api';

// Statistik einer Welt für den Kopf des Welten-Editors: was ist fertig, was ist vorhanden, was fehlt noch für eine
// funktionierende Welt. Reine Zählung aus den Welt-Daten, kein Netz.

export interface StatLine {
  key: string;
  label: string;
  /** vorhanden / nötig */
  have: number;
  need: number;
  /** Was fehlt, kurz (leer, wenn nichts fehlt) */
  missing: string;
}

const filled = (l?: readonly unknown[]) => (l?.length ?? 0) > 0;

export function worldStats(d: WorldDetail): StatLine[] {
  const n = d.islands.length;
  const stages = d.islands.reduce((s, i) => s + i.depths.length, 0);
  const withBiome = d.islands.reduce((s, i) => s + i.depths.filter((_, k) => !!i.biomes[k]).length, 0);
  const s = d.story;
  const islandEntries = d.islands.flatMap((i) => [i.story?.arrival, i.story?.boss, i.story?.done]);
  const storyHave = [s?.start, s?.end, s?.bottles, ...islandEntries].filter(filled).length;
  const storyNeed = 3 + 3 * n;
  const etappen = new Set(d.islands.map((i, k) => i.etappe ?? k + 1)).size;
  const planned = d.islands.filter((i) => Object.values(i.plannedChunks ?? {}).some((p) => p.length > 0)).length;
  const lines: StatLine[] = [
    { key: 'inseln', label: 'Inseln', have: n, need: Math.max(n, 1), missing: n === 0 ? 'keine Insel' : '' },
    { key: 'stufen', label: 'Stufen mit Biom', have: withBiome, need: Math.max(stages, 1), missing: stages === 0 ? 'keine Stufen' : stages - withBiome > 0 ? `${stages - withBiome} ohne Biom` : '' },
    { key: 'etappen', label: 'Etappen', have: etappen, need: Math.max(etappen, 1), missing: '' },
    { key: 'geschichte', label: 'Geschichte', have: storyHave, need: storyNeed, missing: storyNeed - storyHave > 0 ? `${storyNeed - storyHave} Einträge` : '' },
    { key: 'chunks', label: 'Inseln mit geplanten Chunks', have: planned, need: Math.max(n, 1), missing: n - planned > 0 ? `${n - planned} Inseln (reiner Zufall)` : '' },
  ];
  if (d.error) lines.unshift({ key: 'datei', label: 'Welt-Datei', have: 0, need: 1, missing: 'Fehler in der Datei' });
  return lines;
}

/** Anteil des Fertigen an allen Zeilen in Prozent. */
export function worldProgress(lines: readonly StatLine[]): number {
  const need = lines.reduce((s, l) => s + l.need, 0);
  return need === 0 ? 0 : Math.round((lines.reduce((s, l) => s + Math.min(l.have, l.need), 0) / need) * 100);
}
