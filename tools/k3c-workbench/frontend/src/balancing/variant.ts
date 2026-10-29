import type { BalanceDataFileValue, Lever } from '../api';

/** Neuer Wert eines Hebels: aktueller Wert ± Schritt (relativ); ganze Zahlen ab 10 bleiben ganz und ändern sich um mindestens 1. */
export function leverValue(current: number, direction: Lever['direction'], step: number): number {
  const sign = direction === '+' ? 1 : -1;
  const next = current * (1 + sign * step);
  if (!Number.isInteger(current) || Math.abs(current) < 10) return Number(next.toFixed(4));
  const rounded = Math.round(next);
  return rounded === current ? current + sign : rounded;
}

/** Variante `datei.json:pfad=wert` für einen Hebel; null, wenn der aktuelle Wert in den Werten fehlt. */
export function leverLine(lever: Lever, values: readonly BalanceDataFileValue[]): string | null {
  const at = lever.path.indexOf(':');
  const [file, path] = [lever.path.slice(0, at), lever.path.slice(at + 1)];
  const hit = values.find((v) => v.file === file && v.path === path);
  return hit ? `${lever.path}=${leverValue(hit.value, lever.direction, lever.step)}` : null;
}

/** „+20 %“ bzw. „−50 %“. */
export const leverText = (l: Lever) => `${l.direction === '+' ? '+' : '−'}${Math.round(l.step * 100)} %`;
