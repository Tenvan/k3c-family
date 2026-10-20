import { useCallback, useEffect, useRef, useState } from 'react';
import { backend } from '../api';
import type { ChatChoice, ChatChoiceValues, ChatOptionsView } from '../api/chatTypes';
import { loadText, savePref } from '../lib/prefs';

// Wahl von Anbieter, Modell und Denkstufe des Chats: Werte kommen vom Backend (chatOptions), die Wahl wird gemerkt
// (`chat.provider`, `chat.model`, `chat.effort`) und geht mit jeder Nachricht mit. Leer heißt Standard.

const STANDARD: ChatChoice = { id: '', label: 'Standard' };
const FIELDS = ['provider', 'model', 'effort'] as const;
const NONE: ChatOptionsView = { providers: [STANDARD], models: [STANDARD], efforts: [STANDARD] };

/** Liste mit dem Eintrag „Standard“ (id leer) an erster Stelle, auch wenn das Backend ihn nicht liefert. */
export const withStandard = (list: readonly ChatChoice[]): ChatChoice[] => (list.some((c) => c.id === '') ? [...list] : [STANDARD, ...list]);

export const choiceList = (view: ChatOptionsView, field: keyof ChatChoiceValues): ChatChoice[] =>
  withStandard(field === 'provider' ? view.providers : field === 'model' ? view.models : view.efforts);

export interface ChatChoices {
  view: ChatOptionsView;
  values: ChatChoiceValues;
  /** aktuelle Wahl, für den Adapter (immer der neueste Stand) */
  current: () => ChatChoiceValues;
  set: (field: keyof ChatChoiceValues, value: string) => void;
  /** übernimmt Anbieter, Modell und Denkstufe eines geladenen Chats (fehlend = Standard, Unbekanntes = Standard) */
  apply: (from: Partial<ChatChoiceValues>) => void;
}

export function useChatChoices(): ChatChoices {
  const [view, setView] = useState<ChatOptionsView>(NONE);
  const [values, setValues] = useState<ChatChoiceValues>(() => ({
    provider: loadText('chat.provider', ''), model: loadText('chat.model', ''), effort: loadText('chat.effort', ''),
  }));
  const ref = useRef(values);
  const viewRef = useRef<ChatOptionsView | null>(null);
  const update = useCallback((next: ChatChoiceValues) => {
    ref.current = next;
    setValues(next);
    FIELDS.forEach((f) => savePref(`chat.${f}`, next[f]));
  }, []);
  /** Werte, die es in den Auswahlen nicht gibt, gelten als Standard; solange die Liste fehlt, bleibt alles stehen. */
  const known = useCallback((v: ChatChoiceValues): ChatChoiceValues => {
    const options = viewRef.current;
    if (!options) return v;
    const ok = (f: keyof ChatChoiceValues) => (choiceList(options, f).some((c) => c.id === v[f]) ? v[f] : '');
    return { provider: ok('provider'), model: ok('model'), effort: ok('effort') };
  }, []);
  useEffect(() => {
    let alive = true;
    backend.chatOptions().then((v) => {
      if (!alive) return;
      viewRef.current = v;
      setView(v);
      update(known(ref.current));
    }).catch(() => {}); // ohne Liste bleibt es beim Standard
    return () => { alive = false; };
  }, [update, known]);
  const set = useCallback((field: keyof ChatChoiceValues, value: string) => update({ ...ref.current, [field]: value }), [update]);
  // „Standard“ (leer) ist eine bewusste Wahl des gespeicherten Chats, deshalb gelten alle drei Felder
  const apply = useCallback((from: Partial<ChatChoiceValues>) => {
    update(known({ provider: from.provider ?? '', model: from.model ?? '', effort: from.effort ?? '' }));
  }, [update, known]);
  return { view, values, current: () => ref.current, set, apply };
}
