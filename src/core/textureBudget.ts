/**
 * Texturbudget der Xbox (B-772): Auf der Xbox-Edge verliert das Spiel den WebGL-Kontext bei zu viel Texturspeicher. Das Budget steht in
 * `data/render.json` (Datenwert, nicht im Code). Hier nur reine Funktionen für Erkennung, Budget und Bewertung; gemessen wird in
 * `renderInfo.ts` (`textureMiB`), geloggt in `renderLog.ts`. Phaser fordert den Kontext mit `getContext('webgl')` an, das ist WebGL 1.
 */
import renderData from '../../data/render.json';

export interface RenderBudgetData {
  xbox: { textureMiB: number; near: number };
}

/** Ampel des Texturspeichers gegen das Budget */
export type BudgetState = 'ok' | 'near' | 'over';

/** Läuft der Client auf einer Xbox? Edge auf der Xbox nennt sich im User-Agent „Xbox“. */
export const isXbox = (userAgent: string): boolean => /\bXbox\b/i.test(userAgent);

/** Budget in MiB für dieses Gerät; `null` ohne Budget (PC, Handy: keine Begrenzung). */
export function textureBudgetMiB(userAgent: string, data: RenderBudgetData = renderData): number | null {
  return isXbox(userAgent) ? data.xbox.textureMiB : null;
}

/** Bewertung: `over` ab dem Budget, `near` ab dem Anteil `near` (Standard 0,85), sonst `ok`. */
export function budgetState(usedMiB: number, budgetMiB: number, near = renderData.xbox.near): BudgetState {
  if (usedMiB >= budgetMiB) return 'over';
  return usedMiB >= budgetMiB * near ? 'near' : 'ok';
}

/** Text für das Log: „Texturen 96 von 130 MiB (74 %)“ */
export function budgetText(usedMiB: number, budgetMiB: number): string {
  return `Texturen ${Math.round(usedMiB)} von ${budgetMiB} MiB (${Math.round((usedMiB / budgetMiB) * 100)} %)`;
}
