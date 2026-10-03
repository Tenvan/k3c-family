import type { DevMessage, DevResource } from '../online/clientProtocol';

/** Dev-Aktionen im Debug-Overlay (B-179): nur Daten und Abbildung auf die Nachricht `dev`, keine Spielregel. */

export type DevActionGroup = 'gold' | 'material' | 'zeit';

/** Eine Zeile der Aktionsliste; `key` ist die Auswahl, die `devMessage` versteht. */
export interface DevAction {
  group: DevActionGroup;
  label: string;
  key: string;
}

const GOLD_AMOUNTS = [10, 50, 100];
const MATERIAL_AMOUNT = 50;
const FACTORS = [1, 2, 4, 8];
const RESOURCES: [DevResource, string][] = [
  ['wood', 'Holz'],
  ['stone', 'Stein'],
  ['copper', 'Kupfer'],
  ['iron', 'Eisen'],
  ['crystal', 'Kristall'],
];

/** Aktionsliste in Anzeige-Reihenfolge: Gold, Material, Zeitfaktor. */
export const DEV_ACTIONS: readonly DevAction[] = [
  ...GOLD_AMOUNTS.map((n): DevAction => ({ group: 'gold', label: `Gold ${n}`, key: `gold:${n}` })),
  ...RESOURCES.map(([r, name]): DevAction => ({ group: 'material', label: `${name} ${MATERIAL_AMOUNT}`, key: `material:${r}` })),
  ...FACTORS.map((f): DevAction => ({ group: 'zeit', label: `Zeit ${f}×`, key: `timescale:${f}` })),
];

/** Nachricht `dev` für eine Auswahl der Liste und den lokalen Slot (gold, material); unbekannte Auswahl → `null`. */
export function devMessage(key: string, slot: number): DevMessage | null {
  if (!DEV_ACTIONS.some((a) => a.key === key)) return null;
  const [action, value] = key.split(':') as [string, string];
  switch (action) {
    case 'gold':
      return { t: 'dev', action: 'gold', slot, amount: Number(value) };
    case 'material':
      return { t: 'dev', action: 'material', slot, resource: value as DevResource, amount: MATERIAL_AMOUNT };
    case 'timescale':
      return { t: 'dev', action: 'timescale', factor: Number(value) };
  }
  return null;
}

/** Dev-Mode des Raums: nur dann steht `devTimescale` im Zustand (docs/protocol.md › Dev-Aktionen). */
export const roomDevMode = (state: { devTimescale?: number } | null): boolean => state?.devTimescale !== undefined;

/** Die Aktionsliste erscheint nur bei offenem Overlay und im Dev-Mode des Raums. */
export const actionsVisible = (s: { overlayOn: boolean; devMode: boolean }): boolean => s.overlayOn && s.devMode;
