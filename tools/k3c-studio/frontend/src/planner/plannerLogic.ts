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

/** Name der Zeile: Art mit dem Spieltext, der Baugrund mit seiner Zone; unbekannte Arten bleiben, wie sie sind. */
export function rowLabel(row: Pick<HubRow, 'kind' | 'line'>): string {
  if (row.kind === 'wall') return `Baugrund Zone ${row.line}`;
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

/** Zusatz der Zeile: Turm innen beim Baugrund, Insel-Platz. */
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

/** Platz der Zeile in der Hub-Stufe: Zelle der Stufe, sonst die erste verfügbare; null = in keiner Stufe verfügbar. */
export function rowOffset(row: Pick<HubRow, 'cells'>, hubLevel: number): number | null {
  const cell = row.cells[hubLevel - 1]?.available ? row.cells[hubLevel - 1] : row.cells.find((c) => c.available);
  return cell ? cell.offsetUnits : null;
}

/** Liste bzw. Block der Zeile in hub.json, kurz für die Zeile unter dem Namen. */
const SOURCE_LABELS: Record<HubRow['source'], string> = { sites: 'sites', islandSites: 'islandSites', sitePlacement: 'Baugrund', merchant: 'Händler' };

/** Platz und Liste klein unter dem Namen (B-670), z. B. „+40 · sites“. */
export function rowPlace(row: Pick<HubRow, 'cells' | 'bothSides' | 'source'>, hubLevel: number): string {
  const offset = rowOffset(row, hubLevel);
  return [offset === null ? '' : offsetText(offset, row.bothSides), SOURCE_LABELS[row.source]].filter(Boolean).join(' · ');
}

/** Zeile zu einem Bauplatz der Bühne (B-670): gleiche Art (ein Turm gehört zum Baugrund mit seinem Turm-Abstand), in der
 *  Hub-Stufe verfügbar, Abstand zur Hub-Mitte wie der Platz der Zeile (Toleranz 0,5 plus Streuung je Seed); die nächste gewinnt. */
export function rowAt<R extends Pick<HubRow, 'kind' | 'cells' | 'bothSides' | 'towerUnits' | 'jitterUnits'>>(
  rows: readonly R[], hubLevel: number, site: { kind: string; x: number }, hubX: number,
): R | undefined {
  let best: { row: R; d: number } | undefined;
  for (const row of rows) {
    const tower = site.kind === 'tower' && row.kind === 'wall';
    const cell = row.cells[hubLevel - 1];
    if ((row.kind !== site.kind && !tower) || !cell?.available) continue;
    const want = tower ? row.towerUnits : cell.offsetUnits;
    const d = row.bothSides ? Math.abs(Math.abs(site.x - hubX) - Math.abs(want)) : Math.abs(site.x - hubX - want);
    if (d <= 0.5 + row.jitterUnits && (!best || d < best.d)) best = { row, d };
  }
  return best?.row;
}
