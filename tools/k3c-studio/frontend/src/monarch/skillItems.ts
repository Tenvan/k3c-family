import { t } from '@game/core/texts';
import { cardOf } from '../enemies/enemyLogic';
import type { FigureItem } from '../playback/figureView';
import type { SkillPlay } from './actions';
import type { Target } from './skillTarget';

// Bühne beim Abspielen eines Skills (B-671, B-676): ohne Ziel die vier Monarchen mit dem Effekt; mit Ziel-Gegner Monarch 1
// links und der Gegner `distance` Units rechts davon, Flächenzauber und Kontrolleffekte am Gegner. Ohne Ziel in Reichweite
// „Skill ohne Ziel“ wie im Spiel (`castFailed`).

const NO_TARGET_EFFECT = { kind: 'noTarget' as const, text: t('cast.noTarget') };

/** Figuren für den gespielten Skill; `monarchs` sind die Monarchen der Seite (Index 0 = Spieler 1). */
export function skillItems(monarchs: readonly FigureItem[], play: SkillPlay, target: Target | null): FigureItem[] {
  const own = (f: FigureItem): FigureItem => ({ ...f, timed: play.timed, ...(play.noTarget ? { effect: NO_TARGET_EFFECT } : {}) });
  if (!target) return monarchs.map(own);
  const card = cardOf(target.kind);
  const enemy: FigureItem = {
    id: `ziel-${target.kind}`, x: target.distance / 2, spec: card.spec, anim: 'idle', facing: -1,
    ...(play.onTarget.length > 0 ? { timed: play.onTarget } : {}),
  };
  return [{ ...own(monarchs[0]!), x: -target.distance / 2 }, enemy];
}
