import { useEffect, useRef } from 'react';
import { backend } from '../api';
import { registerDraftLoader } from '../chat/chatBridge';
import type { WorldDraftState } from './useWorldDraft';

export const OVERWRITE_QUESTION = 'Der Welt-Entwurf hat ungespeicherte Änderungen. Durch die Antwort aus dem Chat ersetzen?';

/** Der Welten Editor nimmt Antworten des Chats an (WZ19.3, B-488/AC-08): Das Backend prüft sie wie beim Laden; ein lesbarer
 *  Entwurf wird geladen (Prüffehler stehen am Entwurf), Unlesbares nur als Meldung. Ein ungespeicherter Entwurf wird nie still
 *  überschrieben (Rückfrage). Geschrieben wird nichts. */
export function useChatDraft(world: WorldDraftState, onLoaded?: () => void, blocked?: () => string | undefined) {
  const ref = useRef(world);
  ref.current = world;
  const loaded = useRef(onLoaded);
  loaded.current = onLoaded;
  const block = useRef(blocked);
  block.current = blocked;
  useEffect(() => registerDraftLoader(async (text) => {
    const w = ref.current;
    const why = block.current?.(); // ein offener Entwurf der Planung wird nie vom Welt-Entwurf überdeckt
    if (why) { w.report(why); return; }
    let answer;
    try { answer = await backend.planWorldAnswer(text); } catch (e) { w.report(`Antwort nicht geprüft: ${e instanceof Error ? e.message : String(e)}`); loaded.current?.(); return; }
    if (!answer.draft) { w.report(answer.error); loaded.current?.(); return; }
    if (w.dirty && !window.confirm(OVERWRITE_QUESTION)) return;
    w.load(answer.draft, answer.error);
    loaded.current?.(); // der Entwurf steht im Modus Welt-Struktur (B-664/AC-06)
  }), []);
}
