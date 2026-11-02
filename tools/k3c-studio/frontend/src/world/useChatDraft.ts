import { useEffect, useRef } from 'react';
import { backend } from '../api';
import { registerDraftLoader, registerWorldDraft } from '../chat/chatBridge';
import type { WorldDraftState } from './useWorldDraft';

export const OVERWRITE_QUESTION = 'Der Welt-Entwurf hat ungespeicherte Änderungen. Durch die Antwort aus dem Chat ersetzen?';

/** Der Welten Editor nimmt Antworten des Chats an (WZ19.3, B-488/AC-08): Das Backend prüft sie wie beim Laden und ergänzt
 *  einen fehlenden Hafen; nur ein gültiger Entwurf wird geladen (B-686), Ungültiges und Unlesbares nur als Meldung. Ein
 *  ungespeicherter Entwurf wird nie still überschrieben (Rückfrage). Geschrieben wird nichts. Der ungespeicherte Entwurf
 *  geht mit jeder Nachricht an den Chat (B-686), damit er auf ihm aufsetzt. */
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
    if (!answer.draft || !answer.ok) { w.report(answer.draft ? `Antwort nicht geladen, sie besteht die Prüfung nicht: ${answer.error}` : answer.error); loaded.current?.(); return; }
    if (w.dirty && !window.confirm(OVERWRITE_QUESTION)) return;
    w.load(answer.draft, '', answer.hint);
    loaded.current?.(); // der Entwurf steht im Modus Welt-Struktur (B-664/AC-06)
  }), []);
  useEffect(() => registerWorldDraft(() => (ref.current.dirty ? ref.current.draft ?? undefined : undefined)), []);
}
