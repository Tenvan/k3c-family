import type { DevMessage, DevResource } from '../online/clientProtocol';

/** Dev-Aktionen im Cheat-Dialog (B-179, B-231): nur Daten und Abbildung auf die Nachricht `dev`, keine Spielregel. */

export type DevActionGroup = 'gold' | 'material' | 'vorrat' | 'ausbau' | 'zeit';

/** Gruppen in Anzeige-Reihenfolge (eine Zeile je Gruppe im Dialog). */
export const DEV_GROUPS: readonly DevActionGroup[] = ['gold', 'material', 'vorrat', 'ausbau', 'zeit'];

/** Umschalter des Dialogs: Der Zustand steht lokal im Dialog, die Nachricht trägt `on`. */
export type DevToggle = 'cheapBuild' | 'timescaleAuto';
export type DevToggles = Record<DevToggle, boolean>;
export const NO_TOGGLES: DevToggles = { cheapBuild: false, timescaleAuto: false };

/** Eine Zeile der Aktionsliste; `key` ist die Auswahl, die `devMessage` versteht. */
export interface DevAction {
  group: DevActionGroup;
  label: string;
  key: string;
  /** Umschalter: die Beschriftung zeigt den Zustand */
  toggle?: DevToggle;
}

const GOLD_AMOUNTS = [10, 50, 100];
const MATERIAL_AMOUNT = 50;
const FACTORS = [1, 2, 4, 8];
/** Schritt von „Max +100“ */
export const MAX_STEP = 100;
/** Wählbare Stufen von „Alles ausbauen“ */
export const UPGRADE_LEVELS = [1, 2, 3, 4, 5];
const RESOURCES: [DevResource, string][] = [
  ['wood', 'Holz'],
  ['stone', 'Stein'],
  ['copper', 'Kupfer'],
  ['iron', 'Eisen'],
  ['crystal', 'Kristall'],
];

/** Aktionsliste in Anzeige-Reihenfolge (nach `DEV_GROUPS`). */
export const DEV_ACTIONS: readonly DevAction[] = [
  ...GOLD_AMOUNTS.map((n): DevAction => ({ group: 'gold', label: `Gold ${n}`, key: `gold:${n}` })),
  ...RESOURCES.map(([r, name]): DevAction => ({ group: 'material', label: `${name} ${MATERIAL_AMOUNT}`, key: `material:${r}` })),
  { group: 'vorrat', label: `Max +${MAX_STEP}`, key: 'maxplus' },
  { group: 'vorrat', label: 'Alle Ressourcen füllen', key: 'fillall' },
  { group: 'vorrat', label: 'Baukosten 1', key: 'buildcost1', toggle: 'cheapBuild' },
  ...UPGRADE_LEVELS.map((n): DevAction => ({ group: 'ausbau', label: `Alles ausbauen: Stufe ${n}`, key: `upgrade:${n}` })),
  ...FACTORS.map((f): DevAction => ({ group: 'zeit', label: `Zeit ${f}×`, key: `timescale:${f}` })),
  { group: 'zeit', label: 'Zeitraffer → 1× bei Tageszeitwechsel', key: 'autoslow', toggle: 'timescaleAuto' },
  { group: 'zeit', label: 'Nächste Tageszeit', key: 'nextphase' },
];

/** Beschriftung einer Zeile; Umschalter zeigen „an“ bzw. „aus“. */
export const actionLabel = (a: DevAction, toggles: DevToggles): string => (a.toggle ? `${a.label}: ${toggles[a.toggle] ? 'an' : 'aus'}` : a.label);

/**
 * Nachricht `dev` für eine Auswahl der Liste, den lokalen Slot (gold, material) und den Zustand der Umschalter
 * (die Nachricht setzt den umgekehrten Wert); unbekannte Auswahl → `null`.
 */
export function devMessage(key: string, slot: number, toggles: DevToggles = NO_TOGGLES): DevMessage | null {
  if (!DEV_ACTIONS.some((a) => a.key === key)) return null;
  const [action, value] = key.split(':') as [string, string];
  switch (action) {
    case 'gold':
      return { t: 'dev', action: 'gold', slot, amount: Number(value) };
    case 'material':
      return { t: 'dev', action: 'material', slot, resource: value as DevResource, amount: MATERIAL_AMOUNT };
    case 'timescale':
      return { t: 'dev', action: 'timescale', factor: Number(value) };
    case 'maxplus':
      return { t: 'dev', action: 'maxStock', amount: MAX_STEP };
    case 'fillall':
      return { t: 'dev', action: 'fillStock' };
    case 'buildcost1':
      return { t: 'dev', action: 'cheapBuild', on: !toggles.cheapBuild };
    case 'upgrade':
      return { t: 'dev', action: 'buildAll', level: Number(value) };
    case 'autoslow':
      return { t: 'dev', action: 'timescaleAuto', on: !toggles.timescaleAuto };
    case 'nextphase':
      return { t: 'dev', action: 'nextPhase' };
  }
  return null;
}

/** Zustand der Umschalter nach dem Auslösen von `key` (lokale Anzeige). */
export function nextToggles(key: string, toggles: DevToggles): DevToggles {
  const t = DEV_ACTIONS.find((a) => a.key === key)?.toggle;
  return t ? { ...toggles, [t]: !toggles[t] } : toggles;
}

/** Dev-Mode des Raums: nur dann steht `devTimescale` im Zustand (docs/protocol.md › Dev-Aktionen). */
export const roomDevMode = (state: { devTimescale?: number } | null): boolean => state?.devTimescale !== undefined;

/** Raum anhalten bzw. weiterlaufen lassen (Cheat-Dialog, B-231). */
export const pauseMessage = (paused: boolean): DevMessage => ({ t: 'dev', action: 'pause', paused });
