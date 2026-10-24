/**
 * Spiel-Szenen der Grafik-Seite (B-598, B-602): Skill-Effekte, Schilder und Gerüste, Gegner je Biom, Reittiere, Bosse und
 * Kampf-Anzeigen. Reine Aufbereitung; gezeichnet wird mit den Funktionen des Spiels (`@game/scenes/...`), keine zweite Darstellung.
 */
import spritesJson from '@data/sprites.json';
import { MONARCH } from '@game/model/data';
import { BIOMES, type BiomeConfig } from '@game/model/biome';
import type { Site } from '@game/model/types';
import { SKILL_LOOKS, skillLook } from '@game/scenes/effects/skillLooks';
import type { SkillLook } from '@game/scenes/effects/effects';
import { signVariant, type SignVariant } from '@game/scenes/render/buildSignView';
import { HUB_SCAFFOLD } from '@game/scenes/render/hubGrowth';
import type { Card } from '@game/tools/spriteReference';
import { bossBar } from '@game/scenes/kampfAnzeige';
import { BOSSES, mockWorld, MOCK_START, type BossInfo } from './combatLogic';

/** Szenen dieser Datei neben den Slot-Szenen aus `scenesLogic` */
export const GAME_SCENES = ['skills', 'schilder', 'gegner', 'reittiere', 'bosse', 'kampf'] as const;
export type GameSceneId = (typeof GAME_SCENES)[number];
export const isGameScene = (v: string): v is GameSceneId => (GAME_SCENES as readonly string[]).includes(v);

export const LICHT = ['tag', 'nacht'] as const;
export type Licht = (typeof LICHT)[number];

export const biomOf = (id: string): BiomeConfig => BIOMES.find((b) => b.id === id) ?? BIOMES[0]!;

/** Zeile der Szene „Skill-Effekte“ */
export interface SkillRow {
  id: string;
  line: string;
  kind: string;
  tier: number;
  /** Aussehen im Spiel: eigener Look oder Ring der Linie; `undefined` = Linie ohne Farbe, nichts zu zeigen */
  look: SkillLook | undefined;
  /** kein eigener Look: der Ring der Linie (SK2.3) zeigt den Skill */
  fallback: boolean;
}

const LINE_ORDER = Object.keys(MONARCH.lines);

/** Alle Skills nach Linie und Art (aktiv, passiv, …), je Skill sein Effekt oder der Rückfall. */
export function skillRows(): SkillRow[] {
  const skills = MONARCH.skills as Record<string, { line: string; kind: string; tier: number }>;
  const rank = (l: string) => (LINE_ORDER.includes(l) ? LINE_ORDER.indexOf(l) : LINE_ORDER.length);
  return Object.entries(skills)
    .map(([id, s]) => ({ id, line: s.line, kind: s.kind, tier: s.tier, look: skillLook(id), fallback: !(id in SKILL_LOOKS) }))
    .sort((a, b) => rank(a.line) - rank(b.line) || a.kind.localeCompare(b.kind) || a.tier - b.tier);
}

/** Vergrößerungen der Skill-Vorschau: Spielgröße und ×2/×3 */
export const SKILL_SCALES = [1, 2, 3] as const;

/** Beispiel-Plätze je Schild-Variante (`signVariant` aus dem Spiel) */
export const SIGN_SAMPLES: { kind: Site['kind']; variant: SignVariant }[] = (['wall', 'farm', 'stairsUp'] as const).map((kind) => ({ kind, variant: signVariant(kind) }));

/** Plätze, für die das Schild gilt (Beschriftung) */
export const SIGN_KINDS = (v: SignVariant): string => (['wall', 'tower', 'gate', 'workshop', 'storage', 'farm', 'barracks', 'stairsUp', 'stairsDown', 'tavern', 'healer', 'smithy', 'armory'] as const).filter((k) => signVariant(k) === v).join(', ');

/** Datei des Baugerüsts (Index der Gebäude-Grafik) */
export const GERUEST_ID = HUB_SCAFFOLD;

interface Spec {
  sheet: string;
  tint?: string;
  scale?: number;
}
const SPRITES = spritesJson as unknown as { players: Spec[]; enemies: Record<string, Spec>; mounts: Record<string, { label: string }> };

/** Gegner, die das Biom spawnen lässt (Portal, dann Nacht), als Figuren-Karten; Gegner ohne Eintrag in `sprites.json` entfallen. */
export function gegnerKarten(biomId: string): Card[] {
  const e = biomOf(biomId).enemies;
  return [...e.portal.map((id) => [id, 'Portal'] as const), ...e.night.map((id) => [id, 'Nacht'] as const)].flatMap(([id, wann]) =>
    SPRITES.enemies[id] ? [{ title: id, note: `${wann} · ${biomId}`, spec: SPRITES.enemies[id]! }] : [],
  );
}

/** Reittiere mit dem Monarchen als Reiter (wie im Spiel, Sattelpunkte aus `sprites.json`) */
export function reittierKarten(): Card[] {
  const rider = SPRITES.players[0]!;
  return Object.entries(SPRITES.mounts).map(([key, m]) => ({ title: m.label, note: key, spec: rider, mount: key }));
}

/** Phasen-Text je Boss für die Karte: Fähigkeit, bei Endbossen je Phase mit Schwelle */
export function phasenText(b: BossInfo): string {
  if (b.phases.length === 0) return b.ability ? `Fähigkeit: ${b.ability}` : 'ohne Phasen';
  return b.phases.map((p, i) => `Phase ${i + 1}${p.belowHpPercent !== undefined ? ` ab < ${p.belowHpPercent} % Leben` : ''}: ${[p.ability, p.tempo && `Tempo ×${p.tempo}`].filter(Boolean).join(', ') || 'wie vorher'}`).join(' · ');
}

/** Bosse als Karten: Sprite des Standardgegners (Boss-Sprites gibt es nicht), Name aus den Texten, Phasen in der Notiz */
export function bossKarten(): Card[] {
  return BOSSES.flatMap((b) => {
    const spec = SPRITES.enemies[b.base];
    const name = bossBar(mockWorld({ ...MOCK_START, boss: b.id })!)?.name ?? b.id;
    return spec ? [{ title: `${name} (${b.kind === 'end' ? 'Endboss' : 'Miniboss'})`, note: `${phasenText(b)} · Sprite: ${b.base}`, spec: { ...spec, scale: (spec.scale ?? 1) * 1.5 } }] : [];
  });
}
