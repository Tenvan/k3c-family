import { DataList } from '@webkit/molecules';
import { Code } from '@webkit/atoms';
import type { BalanceSpec } from '../api';

const DEPTH = ['Wald', 'Höhle', 'Mine', 'Eisenstollen', 'Kristallhöhle'];

const seedsText = (s: BalanceSpec) => (s.seedList?.length ? `${s.seedList.length} · ${s.seedList.join(', ')}` : s.seeds ? `${s.seeds} (die ersten der Ziele)` : 'alle der Ziele');
const tuneText = (s: BalanceSpec) => (s.tune ? `${s.tune.mode}: ${s.tune.params.join(', ')}` : '');
const depthText = (s: BalanceSpec) => (s.depth === undefined ? 'je Ziel' : `${s.depth} (${DEPTH[s.depth] ?? '?'})`);
const or = (v: string | number | undefined, fallback: string) => String(v ?? fallback);

/** Zeilen des Auftrags: fehlende Werte heißen „je Ziel“ bzw. der Standard. */
export function specRows(spec: BalanceSpec): [string, string][] {
  const rows: [string, string][] = [
    ['Welt', spec.world ? `${spec.world} · ganze Insel 1` : 'keine (eine Stufe)'],
    ['Tiefe', depthText(spec)],
    ['Spieler · Bot', `${or(spec.players, 'je Ziel')} · ${or(spec.bot, 'je Ziel')}`],
    ['Grad · Tage', `${or(spec.grade, 'normal')} · ${or(spec.days, 'je Ziel')}`],
    ['Hilfen', or(spec.helps, 'keine')],
    ['Seeds', seedsText(spec)],
    ['Varianten', spec.vary?.length ? spec.vary.join(', ') : 'keine'],
    ['Baseline', or(spec.baseline, 'Baseline des Repos')],
  ];
  return spec.tune ? [...rows, ['Abstimmen', tuneText(spec)]] : rows;
}

/** Auftrag eines Laufs als Liste. */
export function SpecList({ spec }: { spec: BalanceSpec }) {
  return (
    <DataList.Root size="2" aria-label="Auftrag des Laufs">
      {specRows(spec).map(([k, v]) => (
        <DataList.Item key={k}>
          <DataList.Label minWidth="120px">{k}</DataList.Label>
          <DataList.Value><Code variant="ghost">{v}</Code></DataList.Value>
        </DataList.Item>
      ))}
    </DataList.Root>
  );
}
