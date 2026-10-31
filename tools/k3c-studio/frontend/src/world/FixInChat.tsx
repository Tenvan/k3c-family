import { Button, Text } from '@webkit/atoms';
import { askChat } from '../chat/chatBridge';
import type { Notice } from './planSave';
import type { PlanState } from './usePlan';
import type { WorldDraftState } from './useWorldDraft';

/** Alle Fehler, die der Welten Editor gerade kennt: Prüfung des offenen Entwurfs, Fehlermeldung und die Fehlerliste der
 *  gespeicherten Datei (B-686), ohne Doppelte. */
export function worldErrors(welt: boolean, plan: PlanState, world: WorldDraftState, notice: Notice): string[] {
  const state = welt ? world : plan;
  const list = [
    ...(state.dirty && !state.check.ok ? [state.check.error] : []),
    ...(notice?.kind === 'error' ? [notice.text] : []),
    ...(plan.view?.problems ?? []),
  ];
  return [...new Set(list.filter(Boolean))];
}

/** Text für den Chat: der Auftrag mit der vollständigen Fehlerliste. */
export function fixText(errors: string[]): string {
  return `Korrigiere die Fehler der Welt. Die Prüfung meldet:\n${errors.map((e) => `- ${e}`).join('\n')}`;
}

/** Fehlerbanner des Welten Editors (B-686): die Fehlerliste der gespeicherten Datei und „Im Chat korrigieren“, das alle
 *  bekannten Fehler als Auftrag in die Eingabe des Chats legt (🧑 schickt ab). */
export function FixInChat({ errors, problems }: { errors: string[]; problems: string[] }) {
  if (errors.length === 0) return null;
  return (
    <div className="wd-bar wd-fix" role="alert" aria-label="Fehler der Welt">
      {problems.length > 0 && (
        <div>
          <Text as="p" size="2" color="red">Die gespeicherte Welt ist ungültig:</Text>
          <ul className="wd-fix-list">{problems.map((p) => <li key={p}><Text size="1" color="red">{p}</Text></li>)}</ul>
        </div>
      )}
      <span className="wd-grow" />
      <Button variant="soft" color="red" title="Fehlerliste als Auftrag in den Chat übernehmen" onClick={() => askChat(fixText(errors))}>Im Chat korrigieren</Button>
    </div>
  );
}
