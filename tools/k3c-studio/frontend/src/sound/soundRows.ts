/**
 * Zeilen der Sound-Szenen Skills, Gegner und Bosse, Bauen, Reiten, UI und Warnungen (WZ30.2, B-597): reine Aufbereitung ohne
 * Audio. Je Zeile Slot, Datei, Lizenz und Herkunft des Tons; Lücken (kein Ton) bleiben sichtbar. Ereignis → Slot kommt aus
 * dem Audio-Kern (`cueFor`, `UI_SLOTS`), nichts davon wird hier kopiert.
 */
import bosses from '@data/bosses.json';
import monarch from '@data/monarch.json';
import { cueFor } from '@game/audio/events';
import { SKILL_FALLBACK, skillSlot as gameSkillSlot } from '@game/audio/skills';
import { UI_SLOTS } from '@game/audio/ui';
import type { GameEvent } from '@game/model/types';
import type { ListFilterConfig } from '@webkit/lib/listFilter';
import type { AssetChoice, AssetSlot } from '../api';
import { licenseFlag, type LicenseEntry } from '../graphics/scenesLogic';

export const ROW_SCENES = ['skills', 'gegner', 'bauen', 'reiten', 'ui', 'warnungen'] as const;
export type RowScene = (typeof ROW_SCENES)[number];

export const ROW_TITLE: Record<RowScene, string> = {
  skills: 'Skills', gegner: 'Gegner und Bosse', bauen: 'Bauen', reiten: 'Reiten', ui: 'UI', warnungen: 'Warnungen',
};

/** Woher der Ton einer Zeile kommt: Slot des Skills, Linien-Slot, Rückfall `sfx:skill`, eigener Slot, noch ohne Zuordnung oder gar kein Slot. */
export type Herkunft = 'skill' | 'linie' | 'rueckfall' | 'slot' | 'luecke' | 'still';

export const HERKUNFT_LABEL: Record<Herkunft, string> = {
  skill: 'Skill-Slot', linie: 'Linien-Slot', rueckfall: 'Rückfall sfx:skill', slot: 'Slot', luecke: 'Lücke (Slot ohne Zuordnung)', still: 'still (kein Slot)',
};

export interface SoundRow {
  key: string;
  szene: RowScene;
  titel: string;
  gruppe: string;
  /** nur Skills */
  linie?: string;
  art?: string;
  herkunft: Herkunft;
  /** Slot, aus dem der Ton kommt; ohne Slot (still) leer */
  slot?: AssetSlot;
  /** Präferenz 1 des Slots */
  choice?: AssetChoice;
  /** Warnung: im Spiel überall gleich laut, senkt die Musik (Ducking) */
  global: boolean;
}

const withTone = (s: AssetSlot | undefined): s is AssetSlot => !!s && s.wahl.length > 0;

/**
 * Skill → Slot mit der Abbildung des Spiels (`@game/audio/skills`, SO8.2): Slot des Skills vor Linien-Slot vor `sfx:skill`, passiv ohne
 * eigenen Slot still. Grundlage ist das Register des Studios (ein Slot zählt nur mit Zuordnung); spielt das Spiel nichts, ist die Zeile still.
 */
export function skillTone(skillId: string, slots: readonly AssetSlot[]): { herkunft: 'skill' | 'linie' | 'rueckfall' | 'still'; slot?: AssetSlot } {
  const files = Object.fromEntries(slots.filter(withTone).map((s) => [s.id, s.wahl[0]!.datei]));
  const id = gameSkillSlot(skillId, files);
  const slot = slots.find((s) => s.id === id);
  if (!id || !withTone(slot)) return { herkunft: 'still' };
  if (id === SKILL_FALLBACK) return { herkunft: 'rueckfall', slot };
  return { herkunft: id === `sfx:skill:${skillId}` ? 'skill' : 'linie', slot };
}

interface SkillDef {
  line: string;
  kind: string;
}
const SKILLS = monarch.skills as unknown as Record<string, SkillDef>;
const LINES = monarch.lines as unknown as Record<string, { name: string }>;

/** Alle Skills nach Linie (Reihenfolge der Daten) mit Art aktiv/passiv und dem Ton nach `skillTone`. */
export function skillRows(slots: readonly AssetSlot[]): SoundRow[] {
  return Object.entries(SKILLS).map(([id, def]) => {
    const r = skillTone(id, slots);
    const art = def.kind === 'passive' ? 'passiv' : 'aktiv';
    return { key: `skill:${id}`, szene: 'skills', titel: id, gruppe: LINES[def.line]?.name ?? def.line, linie: def.line, art, herkunft: r.herkunft, slot: r.slot, choice: r.slot?.wahl[0], global: false };
  });
}

const eventSlot = (type: GameEvent['type']) => cueFor({ type } as GameEvent);

interface Wanted {
  titel: string;
  gruppe: string;
  /** Ereignis-Typ des Spiels (Slot und `global` aus `cueFor`) oder fester Slot */
  event?: GameEvent['type'];
  slot?: string;
  global?: boolean;
}

/** Hub-Ausbau und Hufschlag haben kein `cueFor`-Ereignis: der Audio-Kern spielt sie über den Slot selbst (`onHubLevel`, `syncLoops`). */
const WANTED: Record<Exclude<RowScene, 'skills'>, Wanted[]> = {
  gegner: [
    { titel: 'Treffer', gruppe: 'Gegner', event: 'hit' },
    { titel: 'Pfeil', gruppe: 'Gegner', event: 'arrow' },
    { titel: 'Schlag', gruppe: 'Gegner', event: 'strike' },
    { titel: 'Tod', gruppe: 'Gegner', event: 'kill' },
    ...[...bosses.bosses, ...bosses.endBosses].flatMap((b) => [
      { titel: `${b.id} · Auftritt`, gruppe: 'Boss', event: 'bossSpawned' as const },
      { titel: `${b.id} · Tod`, gruppe: 'Boss', event: 'kill' as const },
    ]),
  ],
  bauen: [
    { titel: 'Fortschritt', gruppe: 'Bauen', event: 'buildProgress' },
    { titel: 'fertig', gruppe: 'Bauen', event: 'built' },
    { titel: 'Hub-Ausbau', gruppe: 'Hub', slot: 'sfx:hub', global: true },
    { titel: 'Münze geben', gruppe: 'Bauen', event: 'coinGive' },
  ],
  reiten: [{ titel: 'Hufschlag (Schleife)', gruppe: 'Reiten', slot: 'sfx:ride' }],
  ui: [
    { titel: 'Auswahl', gruppe: 'UI', slot: UI_SLOTS.select },
    { titel: 'Bestätigen', gruppe: 'UI', slot: UI_SLOTS.confirm },
  ],
  warnungen: [
    { titel: 'Abenddämmerung', gruppe: 'Warnung', event: 'dusk' },
    { titel: 'Welle', gruppe: 'Warnung', event: 'wave' },
    { titel: 'Burg gefallen', gruppe: 'Warnung', event: 'castleFallen' },
    { titel: 'Boss erscheint', gruppe: 'Warnung', event: 'bossSpawned' },
  ],
};

/** Zeilen einer Ereignis-Szene; ein Ereignis ohne Slot im Audio-Kern oder im Register ist „still“, ein Slot ohne Zuordnung eine Lücke. */
export function eventRows(szene: Exclude<RowScene, 'skills'>, slots: readonly AssetSlot[]): SoundRow[] {
  return WANTED[szene].map((w, i) => {
    const cue = w.event ? eventSlot(w.event) : null;
    const slot = slots.find((s) => s.id === (w.slot ?? cue?.slot));
    const herkunft: Herkunft = !slot ? 'still' : slot.wahl.length ? 'slot' : 'luecke';
    return { key: `${szene}:${i}`, szene, titel: w.titel, gruppe: w.gruppe, herkunft, slot, choice: slot?.wahl[0], global: w.global ?? cue?.global ?? false };
  });
}

/** Slots, die im Spiel überall gleich laut klingen und die Musik absenken (Warnungen, Hub-Ausbau): Ducking hörbar in der Mischprobe. */
export const GLOBAL_SLOTS: ReadonlySet<string> = new Set(
  Object.values(WANTED).flat().flatMap((w) => {
    const cue = w.event ? eventSlot(w.event) : null;
    const id = w.slot ?? cue?.slot;
    return id && (w.global ?? cue?.global) ? [id] : [];
  }),
);

export function soundRows(szene: RowScene, slots: readonly AssetSlot[]): SoundRow[] {
  return szene === 'skills' ? skillRows(slots) : eventRows(szene, slots);
}

/** Lizenz-Text des Packs einer Datei aus den Credits; unbekannt → „–“ (Dateien unter audio/ sind selbst erzeugt). */
export function licenseText(datei: string, credits: readonly (LicenseEntry & { license: string })[]): string {
  const [root, folder] = datei.split('/');
  return credits.find((e) => e.root === root && e.folders.includes(folder ?? ''))?.license ?? '–';
}

/** Filter der Zeilen-Szenen (B-599, ListFilter): Herkunft, Status, Linie, Art, Gruppe, Lizenz. */
export function rowFilter(rows: readonly SoundRow[], credits: readonly LicenseEntry[]): ListFilterConfig<SoundRow> {
  const order = new Map(rows.map((r, i) => [r.key, i]));
  const byTitle = (a: SoundRow, b: SoundRow) => a.titel.localeCompare(b.titel, 'de');
  const status = (r: SoundRow) => (r.choice ? 'zugeordnet' : 'Lücke');
  return {
    searchText: (r) => `${r.titel} ${r.slot?.id ?? ''} ${r.choice?.datei ?? ''} ${r.gruppe}`,
    dimensions: [
      { key: 'status', label: 'Status', values: status, order: ['Lücke', 'zugeordnet'] },
      { key: 'herkunft', label: 'Herkunft', values: (r) => HERKUNFT_LABEL[r.herkunft] },
      { key: 'linie', label: 'Linie/Klasse', values: (r) => (r.linie ? LINES[r.linie]?.name ?? r.linie : undefined) },
      { key: 'art', label: 'Art', values: (r) => r.art, order: ['aktiv', 'passiv'] },
      { key: 'gruppe', label: 'Gruppe', values: (r) => r.gruppe },
      { key: 'lizenz', label: 'Lizenz', values: (r) => (r.choice ? (licenseFlag(r.choice.datei, credits) === 'NC' ? 'NC' : 'frei') : undefined), order: ['NC', 'frei'] },
    ],
    sorts: [
      { key: 'spiel', label: 'Reihenfolge im Spiel', compare: (a, b) => (order.get(a.key) ?? 0) - (order.get(b.key) ?? 0) },
      { key: 'name', label: 'Name', compare: byTitle },
      { key: 'status', label: 'Status', compare: (a, b) => status(a).localeCompare(status(b), 'de') || byTitle(a, b) },
    ],
  };
}
