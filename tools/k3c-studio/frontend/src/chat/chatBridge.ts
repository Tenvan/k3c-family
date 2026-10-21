import { useSyncExternalStore } from 'react';
import type { ChatTask } from '../api/chatTypes';
import { loadPref, savePref } from '../lib/prefs';

// Zustand des Chats außerhalb der Oberfläche: ein- und ausgeblendet (gemerkt) und der Entwurfs-Lader einer Seite
// (Welten Editor, WZ19.3); solange er angemeldet ist, sind Nachrichten Welt-Aufträge (B-516).
const STATES = ['offen', 'zu'] as const;
const listeners = new Set<() => void>();
let open = loadPref('chat.state', STATES, 'offen') === 'offen';
let loader: ((text: string) => Promise<void>) | null = null;

export function setChatOpen(next: boolean) {
  open = next;
  savePref('chat.state', next ? 'offen' : 'zu');
  listeners.forEach((fn) => fn());
}

export function useChatOpen(): boolean {
  return useSyncExternalStore((fn) => { listeners.add(fn); return () => listeners.delete(fn); }, () => open, () => open);
}

/** Eine Seite (Welten Editor) meldet an, wie eine Antwort des Modells als Entwurf geladen wird; ohne sie bleibt der Knopf weg. */
export function registerDraftLoader(fn: (text: string) => Promise<void>): () => void {
  loader = fn;
  listeners.forEach((l) => l());
  return () => { if (loader === fn) { loader = null; listeners.forEach((l) => l()); } };
}

export function useCanLoadDraft(): boolean {
  return useSyncExternalStore((fn) => { listeners.add(fn); return () => listeners.delete(fn); }, () => loader !== null, () => false);
}

/** Auftragsart der nächsten Nachricht: `welt`, solange der Welten Editor einen Entwurfs-Lader angemeldet hat (B-516). */
export function chatTask(): ChatTask | undefined {
  return loader ? 'welt' : undefined;
}

/** Lädt die Antwort `text` als Entwurf (prüft sie vorher); false, wenn keine Seite sie annimmt. */
export async function loadDraft(text: string): Promise<boolean> {
  if (!loader) return false;
  await loader(text);
  return true;
}
