import bossesJson from '../../../data/bosses.json';
import eventsJson from '../../../data/events.json';
import { de } from '../../core/texts.de';
import type { ResourceKind } from '../../model/types';
import type { DevMessage } from '../../online/clientProtocol';

/** Dev-Aktionen im Cheat-Dialog (B-179, B-231): nur Daten und Abbildung auf die Nachricht `dev`, keine Spielregel. */

export type DevActionGroup = 'gold' | 'material' | 'vorrat' | 'ausbau' | 'zeit' | 'grad' | 'kampf';

/** Gruppen in Anzeige-Reihenfolge (eine Zeile je Gruppe im Dialog). */
export const DEV_GROUPS: readonly DevActionGroup[] = ['gold', 'material', 'vorrat', 'ausbau', 'zeit', 'grad', 'kampf'];

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
/** Grade des Raums in Anzeige-Reihenfolge (K5.3, B-107); der Server prüft und lehnt ohne Dev-Mode ab */
export const GRADES: [string, string][] = [
  ['dev', 'Dev'],
  ['easy', 'Leicht'],
  ['normal', 'Normal'],
  ['hard', 'Hart'],
  ['ultra', 'Ultra'],
];
/** Bosse der Gruppe „Kampf“ (B-603): Mini- und Endbosse aus data/bosses.json mit ihrem Namen. */
export const BOSSES: [string, string][] = [...bossesJson.bosses, ...bossesJson.endBosses].map((b) => [b.id, b.name]);
/** Events der Gruppe „Kampf“: Ids aus data/events.json, Namen aus den Texten (ohne Text die Id). */
export const EVENTS: [string, string][] = eventsJson.events.map((e) => {
  const key = `event.${e.id}`;
  return [e.id, key in de ? de[key as keyof typeof de] : e.id];
});
const RESOURCES: [ResourceKind, string][] = [
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
  { group: 'ausbau', label: 'Burgfall', key: 'castlefall' },
  ...FACTORS.map((f): DevAction => ({ group: 'zeit', label: `Zeit ${f}×`, key: `timescale:${f}` })),
  { group: 'zeit', label: 'Zeitraffer → 1× bei Tageszeitwechsel', key: 'autoslow', toggle: 'timescaleAuto' },
  { group: 'zeit', label: 'Nächste Tageszeit', key: 'nextphase' },
  ...GRADES.map(([g, name]): DevAction => ({ group: 'grad', label: `Grad ${name}`, key: `grade:${g}` })),
  ...BOSSES.map(([id, name]): DevAction => ({ group: 'kampf', label: `Boss ${name}`, key: `boss:${id}` })),
  { group: 'kampf', label: 'Boss: nächste Phase', key: 'bossphase' },
  ...EVENTS.map(([id, name]): DevAction => ({ group: 'kampf', label: `Event ${name}`, key: `event:${id}` })),
  { group: 'kampf', label: 'Nacht sofort', key: 'night' },
];

/** Sichtbar: die Gruppe „Grad“ nur im Dev-Mode des Raums (B-107/AC-02); die übrigen zeigt der Dialog mit Hinweis. */
export const actionVisible = (a: DevAction, devMode: boolean): boolean => a.group !== 'grad' || devMode;

/** Aktuell gesetzter Grad: die Zeile ist markiert (Grad aus dem Zustand, `s.grade`). */
export const isCurrentGrade = (a: DevAction, grade: string | undefined): boolean => grade !== undefined && a.key === `grade:${grade}`;

/** Beschriftung einer Zeile; Umschalter zeigen „an“ bzw. „aus“. */
export const actionLabel = (a: DevAction, toggles: DevToggles): string => (a.toggle ? `${a.label}: ${toggles[a.toggle] ? 'an' : 'aus'}` : a.label);

/** Nachricht der Gruppe „Kampf“ (B-603): Boss an der Stufe des lokalen Slots, Phase, Event, Nacht; sonst `null`. */
function combatMessage(action: string, value: string, slot: number): DevMessage | null {
  switch (action) {
    case 'boss':
      return { t: 'dev', action: 'boss', slot, boss: value };
    case 'bossphase':
      return { t: 'dev', action: 'bossPhase' };
    case 'event':
      return { t: 'dev', action: 'event', event: value };
    case 'night':
      return { t: 'dev', action: 'night' };
  }
  return null;
}

/**
 * Nachricht `dev` für eine Auswahl der Liste, den lokalen Slot (gold, material, castlefall, boss) und den Zustand der
 * Umschalter (die Nachricht setzt den umgekehrten Wert); unbekannte Auswahl → `null`.
 */
export function devMessage(key: string, slot: number, toggles: DevToggles = NO_TOGGLES): DevMessage | null {
  if (!DEV_ACTIONS.some((a) => a.key === key)) return null;
  const [action, value] = key.split(':') as [string, string];
  const combat = combatMessage(action, value, slot);
  if (combat) return combat;
  switch (action) {
    case 'gold':
      return { t: 'dev', action: 'gold', slot, amount: Number(value) };
    case 'material':
      return { t: 'dev', action: 'material', slot, resource: value as ResourceKind, amount: MATERIAL_AMOUNT };
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
    case 'castlefall':
      return { t: 'dev', action: 'castleFall', slot };
    case 'grade':
      return { t: 'dev', action: 'grade', grade: value };
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
