import { de } from '@game/core/texts.de';
import type { HubCell, HubRow } from '../api';

// Anzeige der Matrix des Gebäude-Planers (B-462, WZ9.2): Namen, Platz und Tiefe als Text. Das Matrix-Modell und die
// Prüfung kommen aus dem Go-Backend; hier wird nur formatiert, nichts geprüft.

/** Nur im Planer: der Händler ist kein Bauplatz und hat keinen Spieltext. */
const MERCHANT_LABELS: Record<string, string> = { merchantBuy: 'Händler Kaufen', merchantSell: 'Händler Verkaufen' };

/** Spieltext `site.<kind>` (texts.de.ts), falls es ihn gibt. */
function siteText(kind: string): string | undefined {
  return (de as Record<string, string>)[`site.${kind}`];
}

/** Name der Zeile: Art mit dem Spieltext, Mauerlinie und Tor mit ihrer Linie; unbekannte Arten bleiben, wie sie sind. */
export function rowLabel(row: Pick<HubRow, 'kind' | 'line'>): string {
  if (row.kind === 'wall') return `Mauerlinie ${row.line}`;
  if (row.kind === 'gate') return `${de['site.gate']} Linie ${row.line}`;
  return MERCHANT_LABELS[row.kind] ?? siteText(row.kind) ?? row.kind;
}

/** Betrag der Zahl ohne überflüssige Nachkommastellen; das Vorzeichen setzt der Aufrufer (± oder echtes Minus). */
function num(n: number): string {
  return String(Math.abs(n));
}

/** Platz zur Hub-Mitte: je Seite als ±, sonst mit Vorzeichen. */
export function offsetText(offset: number, bothSides: boolean): string {
  if (bothSides) return `±${num(offset)}`;
  return `${offset < 0 ? '−' : '+'}${num(offset)}`;
}

/** Tiefe einer Zelle als Text; leer = jede Tiefe ohne Bedingung. */
export function depthText(cell: Pick<HubCell, 'fromDepth' | 'needsDeeper' | 'onlyDepth'>): string {
  const parts: string[] = [];
  if (cell.fromDepth > 0) parts.push(`ab Tiefe ${cell.fromDepth}`);
  if (cell.needsDeeper) parts.push('nur mit tieferer Stufe');
  if (cell.onlyDepth >= 0) parts.push(`nur Tiefe ${cell.onlyDepth}`);
  return parts.join(', ');
}

/** Zusatz der Zeile: Turm innen bei der Mauerlinie, Streuung je Seed bei Linie und Tor, Insel-Platz. */
export function rowExtra(row: Pick<HubRow, 'source' | 'towerUnits' | 'jitterUnits' | 'kind'>): string {
  const parts: string[] = [];
  if (row.kind === 'wall') parts.push(`${de['site.tower']} ±${num(row.towerUnits)}`);
  if (row.jitterUnits > 0) parts.push(`0…+${num(row.jitterUnits)} je Seed`);
  if (row.source === 'islandSites') parts.push('je Insel');
  return parts.join(', ');
}

/** Spalten der Matrix: Hub-Stufe 1 bis levels (beliebig viele). */
export function levelColumns(levels: number): number[] {
  return Array.from({ length: Math.max(0, levels) }, (_, i) => i + 1);
}
