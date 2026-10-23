import monarchJson from '@data/monarch.json';
import { t } from '@game/core/texts';
import { EFFECT_CONFIG, type EffectKind, type SkillLook } from '@game/scenes/effects/effects';
import { SKILL_LOOKS, skillLook } from '@game/scenes/effects/skillLooks';
import { gap, type Anim, type Gap } from '../playback/figure';

// Aktionen und Skills der Monarch-Seite (B-463, WZ10.3): Schlagen, Münzen, Getroffen, Tod, Respawn und die Skill-Liste mit
// Lücken. Effekte und Deckkraft stammen aus dem Spiel (`EFFECT_CONFIG`, `worldRenderer`); was das Spiel nicht darstellt,
// ist eine Lücke, kein Ersatz.

const DATA = monarchJson as unknown as {
  lines: Record<string, { name: string }>;
  skills: Record<string, { line: string; tier: number; kind: string }>;
  presets: Record<string, { name: string; activeSkills: string[] }>;
};

/** Deckkraft des Monarchen, solange er auf den Respawn wartet (`setAlpha(p.respawnIn > 0 ? 0.25 : 1)` im Spiel); ein Test
 *  prüft, dass der Wert noch in `src/scenes/render/worldRenderer.ts` steht. */
export const RESPAWN_ALPHA = 0.25;

export type ActionId = 'swing' | 'coinGive' | 'coinPickup' | 'hit' | 'down' | 'respawn' | 'noTarget';

export interface Action {
  id: ActionId;
  label: string;
  /** Animation des Monarchen (beritten); das Sheet hat nur idle, run und attack */
  anim: Anim;
  /** Effekt des Spiels (`effectFor`): Art aus `EFFECT_CONFIG`, bei noTarget mit dem Text des Spiels */
  effect?: { kind: EffectKind; text?: string };
  /** Deckkraft: fest oder Wechsel (Respawn: von der Wartezeit zurück zu voll) */
  alpha?: number;
  fade?: { from: number; to: number };
  /** Objekt des Spiels (`objectImage`), das neben dem Monarchen steht (Münze) */
  prop?: string;
  /** Darstellungen, die das Spiel nicht hat */
  gaps: Gap[];
}

const noAnim = (what: string): Gap => gap('anim', `Lücke: ${what}-Animation des Monarchen fehlt (das Sheet hat nur idle, run und attack)`);

/** Alle Aktionen mit ihren Lücken. Der Effekt `swing` und die Animation `attack` sind das Schlagen des Spiels. */
export const ACTIONS: readonly Action[] = [
  { id: 'swing', label: 'Schlagen', anim: 'attack', effect: { kind: 'swing' }, gaps: [] },
  { id: 'coinGive', label: 'Münze geben', anim: 'idle', effect: { kind: 'coinGive' }, prop: 'coin', gaps: [noAnim('Wurf')] },
  { id: 'coinPickup', label: 'Münze aufheben', anim: 'idle', effect: { kind: 'coin' }, prop: 'coin', gaps: [] },
  { id: 'hit', label: 'Getroffen', anim: 'idle', effect: { kind: 'hit' }, gaps: [noAnim('Treffer')] },
  { id: 'down', label: 'Tod', anim: 'idle', effect: { kind: 'down' }, alpha: RESPAWN_ALPHA, gaps: [noAnim('Tod')] },
  { id: 'respawn', label: 'Respawn', anim: 'idle', fade: { from: RESPAWN_ALPHA, to: 1 }, gaps: [noAnim('Respawn')] },
  { id: 'noTarget', label: 'Skill ohne Ziel', anim: 'idle', effect: { kind: 'noTarget', text: t('cast.noTarget') }, gaps: [] },
];

export const actionOf = (id: ActionId): Action | undefined => ACTIONS.find((a) => a.id === id);

/** Dauer des Effekts im Spiel in Millisekunden (`EFFECT_CONFIG`) */
export const effectMs = (kind: EffectKind): number => EFFECT_CONFIG[kind].durationMs;

// ---- Skills --------------------------------------------------------------------------------------------------------

export interface SkillRow {
  id: string;
  /** Name der Linie (Tank, Zauberer, Heiler …) oder leer, wenn der Skill fehlt */
  line: string;
  tier: number | null;
  kind: string;
  /** Effekt des Skills im Spiel (`skillLook`, B-578): eigener Look oder Ring der Linie; fehlt = Lücke */
  look: SkillLook | null;
  /** `true`: eigener Effekt des Skills, `false`: Ring der Linie (Rückfall) */
  own: boolean;
  gap: Gap | null;
}

const NO_DISPLAY = gap('skill', 'Lücke: keine Darstellung im Spiel');
const MISSING = gap('skill', 'Lücke: Skill fehlt in data/monarch.json');

/** Alle Skills aus `data/monarch.json` mit Linie, Stufe, Art und dem Effekt des Spiels (B-562, B-578: eigener Look, sonst
 *  der Ring der Linie aus `LINE_COLORS`); die Lücke „keine Darstellung im Spiel“ nur bei Linien ohne Farbe. Dazu die Skills
 *  der Presets, die in `skills` fehlen (seit B-504 keine; das Laden der Daten lehnt sie ab), mit der Lücke „Skill fehlt“. */
export function skillRows(): SkillRow[] {
  const rows: SkillRow[] = Object.entries(DATA.skills).map(([id, s]) => {
    const look = skillLook(id) ?? null;
    return { id, line: DATA.lines[s.line]?.name ?? s.line, tier: s.tier, kind: s.kind, look, own: id in SKILL_LOOKS, gap: look ? null : NO_DISPLAY };
  });
  const known = new Set(rows.map((r) => r.id));
  for (const preset of Object.values(DATA.presets)) {
    for (const id of preset.activeSkills) {
      if (known.has(id)) continue;
      known.add(id);
      rows.push({ id, line: '', tier: null, kind: '', look: null, own: false, gap: MISSING });
    }
  }
  return rows;
}
