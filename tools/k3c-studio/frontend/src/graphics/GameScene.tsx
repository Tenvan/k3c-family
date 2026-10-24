import { CombatScene } from './CombatScene';
import { FigureScene } from './FigureScenes';
import { SignScene } from './SignScene';
import { SkillScene } from './SkillScene';
import type { GameSceneId, Licht } from './gameScenesLogic';
import type { GebaeudeEintrag } from './scenesLogic';

/** Verteiler der Spiel-Szenen (B-598, B-602) auf ihre Bausteine */
export function GameScene({ szene, biom, licht, index }: { szene: GameSceneId; biom: string; licht: Licht; index: readonly GebaeudeEintrag[] }) {
  switch (szene) {
    case 'skills': return <SkillScene biom={biom} licht={licht} />;
    case 'schilder': return <SignScene biom={biom} licht={licht} index={index} />;
    case 'kampf': return <CombatScene biom={biom} licht={licht} />;
    default: return <FigureScene szene={szene} biom={biom} />;
  }
}
