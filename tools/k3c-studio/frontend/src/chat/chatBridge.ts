import { useSyncExternalStore } from 'react';
import { loadPref, savePref } from '../lib/prefs';

// Zustand des Chats außerhalb der Oberfläche: ein- und ausgeblendet (gemerkt) und das Eingabefeld von außen füllbar,
// damit Seiten (Welten Editor, WZ19.3) einen Prompt in den Chat legen, ohne ihn zu kennen.
const STATES = ['offen', 'zu'] as const;
const listeners = new Set<() => void>();
let open = loadPref('chat.state', STATES, 'offen') === 'offen';
let filler: ((text: string) => void) | null = null;

export function setChatOpen(next: boolean) {
  open = next;
  savePref('chat.state', next ? 'offen' : 'zu');
  listeners.forEach((fn) => fn());
}

export function useChatOpen(): boolean {
  return useSyncExternalStore((fn) => { listeners.add(fn); return () => listeners.delete(fn); }, () => open, () => open);
}

/** Der Chat meldet hier an, wie sein Eingabefeld gefüllt wird (nur ein Chat zugleich). */
export function registerChatFill(fn: (text: string) => void): () => void {
  filler = fn;
  return () => { if (filler === fn) filler = null; };
}

/** Legt `text` ins Eingabefeld des Chats und blendet ihn ein; false, wenn kein Chat da ist. Sendet nichts. */
export function fillChat(text: string): boolean {
  if (!filler) return false;
  setChatOpen(true);
  filler(text);
  return true;
}
