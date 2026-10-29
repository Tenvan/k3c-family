import type { Guard } from './WorldGuard';
import type { Mode } from './WorldHeader';
import type { PlanState } from './usePlan';
import type { WorldDraftState } from './useWorldDraft';
import { WorldDraftEditor } from './WorldDraftEditor';

/** Modus Welt-Struktur des Welten Editors (B-664): Welt (Name, Notizen), Inseln mit Reihenfolge, Stufen und Skalierung ohne
 *  Live-Vorschau. Welt wählen, anlegen, kopieren und löschen steht im Kopf (`WorldHeader`). „Stufe bearbeiten →“ wählt die
 *  Stufe der gespeicherten Insel und wechselt in den anderen Modus (mit Rückfrage bei offenem Entwurf). */
export function WorldStructure({ plan, world, guard, onMode }: { plan: PlanState; world: WorldDraftState; guard: Guard; onMode: (mode: Mode) => void }) {
  const edit = (islandId: string, stage: number) => guard.run(() => { plan.selectIsland(islandId); plan.selectStage(stage); onMode('stufe'); });
  return <WorldDraftEditor world={world} plan={plan} onEditStage={edit} />;
}
