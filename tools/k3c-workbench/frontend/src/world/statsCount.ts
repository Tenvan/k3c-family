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
  /** Optional: nur die Zahl zeigen, nichts fehlt (Info) */
  optional?: boolean;
}

/** Eigene Assets einer Welt: Gruppen (Katalog-Gruppen der exklusiven Assets) und Zahl. */
export interface OwnAssets {
  groups: readonly string[];
  count: number;
}

/** Mindestausstattung einer komplett neuen Welt (B-1217): je Gruppe mindestens ein eigenes Asset (Vorschlag, offen). */
export const MIN_GROUPS = ['figuren', 'gegner', 'gebaeude', 'hintergruende', 'reittiere'] as const;
const GROUP_ALIAS: Readonly<Record<string, string>> = { 'gebaeude-paper': 'gebaeude', reittiere2: 'reittiere' };
const groupOf = (g: string) => GROUP_ALIAS[g] ?? g;

/** Art der Welt: das Feld der Datei, sonst spinoff mit `extends`, sonst new (wie die Engine). */
export const kindOf = (d: WorldDetail): 'spinoff' | 'new' => d.kind ?? (d.extends ? 'spinoff' : 'new');

const filled = (l?: readonly unknown[]) => (l?.length ?? 0) > 0;

/** `own`: eigene Assets der Welt; null = unbekannt oder Basis-Welt (keine Zeile). */
export function worldStats(d: WorldDetail, own: OwnAssets | null = null): StatLine[] {
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
    { key: 'chunks', label: 'Inseln mit geplanten Chunks (optional, sonst Zufall)', have: planned, need: n, missing: '', optional: true },
  ];
  if (own) {
    const have = new Set(own.groups.map(groupOf));
    const lacking = MIN_GROUPS.filter((g) => !have.has(g));
    lines.push(kindOf(d) === 'new'
      ? { key: 'assets', label: 'Eigene Assets (neue Welt)', have: MIN_GROUPS.length - lacking.length, need: MIN_GROUPS.length, missing: lacking.join(', ') }
      : { key: 'assets', label: 'Eigene Assets (Ableger, optional)', have: own.count, need: own.count, missing: '', optional: true });
  }
  if (d.error) lines.unshift({ key: 'datei', label: 'Welt-Datei', have: 0, need: 1, missing: 'Fehler in der Datei' });
  return lines;
}

/** Anteil des Fertigen an allen Zeilen in Prozent. */
export function worldProgress(lines: readonly StatLine[]): number {
  const need = lines.reduce((s, l) => s + l.need, 0);
  return need === 0 ? 0 : Math.round((lines.reduce((s, l) => s + Math.min(l.have, l.need), 0) / need) * 100);
}
