import type { BalanceFigures, NightFigures } from '../api';

export interface Point {
  x: number;
  y: number;
}
export interface Series {
  name: string;
  points: Point[];
}
export interface Bar {
  label: string;
  value: number;
}

const grades = (f: BalanceFigures) => [...new Set(f.nights.map((n) => n.grade))];
const bySeries = (f: BalanceFigures, pick: (n: NightFigures) => number): Series[] =>
  grades(f).map((g) => ({ name: g, points: f.nights.filter((n) => n.grade === g).map((n) => ({ x: n.night, y: pick(n) })) }));

/** Überleben je Nacht, eine Linie je Grad. */
export const survivalSeries = (f: BalanceFigures) => bySeries(f, (n) => n.survival);

/** Wartezeit-Anteil je Nacht in Prozent, eine Linie je Grad. */
export const idleSeries = (f: BalanceFigures) => bySeries(f, (n) => n.idleShare * 100);

/** Gold-Bilanz (Einnahmen − Ausgaben) je Nacht: Tag und Nacht, über alle Grade gemittelt. */
export function goldSeries(f: BalanceFigures): Series[] {
  const nights = [...new Set(f.nights.map((n) => n.night))].sort((a, b) => a - b);
  const mean = (night: number, pick: (n: NightFigures) => number) => {
    const rows = f.nights.filter((n) => n.night === night);
    return rows.reduce((sum, n) => sum + pick(n), 0) / rows.length;
  };
  return [
    { name: 'Tag', points: nights.map((x) => ({ x, y: mean(x, (n) => n.dayIncome - n.dayExpense) })) },
    { name: 'Nacht', points: nights.map((x) => ({ x, y: mean(x, (n) => n.nightIncome - n.nightExpense) })) },
  ];
}

const RESOURCES = [['wood', 'Holz'], ['stone', 'Stein'], ['copper', 'Kupfer'], ['iron', 'Eisen'], ['crystal', 'Kristall']] as const;

/** Rohstoff-Fluss: Zufluss − Abfluss je Rohstoff, über alle Nächte summiert. */
export const resourceBars = (f: BalanceFigures): Bar[] =>
  RESOURCES.map(([key, label]) => ({ label, value: f.nights.reduce((sum, n) => sum + n.resourceIn[key] - n.resourceOut[key], 0) }));
