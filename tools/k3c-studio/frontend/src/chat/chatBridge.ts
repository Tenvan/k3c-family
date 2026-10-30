import { useEffect, useSyncExternalStore } from 'react';
import type { AssignContext, ChatTask } from '../api/chatTypes';
import { loadPref, savePref } from '../lib/prefs';

// Zustand des Chats außerhalb der Oberfläche: ein- und ausgeblendet (gemerkt) und der Entwurfs-Lader einer Seite
// (Welten Editor, WZ19.3); solange er angemeldet ist, sind Nachrichten Welt-Aufträge (B-516). Sound- und Grafik-Seite
// melden ihren Kontext an (B-600); solange er steht, sind Nachrichten Zuordnungs-Aufträge.
const STATES = ['offen', 'zu'] as const;
const listeners = new Set<() => void>();
let open = loadPref('chat.state', STATES, 'offen') === 'offen';
let loader: ((text: string) => Promise<void>) | null = null;
let assign: AssignContext | null = null;
let sources = false; // Seite Quellen hat einen Such-Auftrag gestartet (B-673), bis sie verlassen wird
let prefill = ''; // vorbereiteter Text für die Eingabe, bis der Chat ihn übernommen hat
const notify = () => listeners.forEach((fn) => fn());
const subscribe = (fn: () => void) => { listeners.add(fn); return () => { listeners.delete(fn); }; };

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

/** Auftragsart der nächsten Nachricht: `welt`, solange der Welten Editor einen Entwurfs-Lader angemeldet hat (B-516),
 *  `zuordnung`, solange Sound- oder Grafik-Seite ihren Kontext angemeldet haben (B-600), `quellen` nach „Claude suchen
 *  lassen“ auf der Seite Quellen (B-673). */
export function chatTask(): ChatTask | undefined {
  return loader ? 'welt' : assign ? 'zuordnung' : sources ? 'quellen' : undefined;
}

/** Seite Quellen: öffnet den Chat mit vorbereitetem Auftrag (Text liegt in der Eingabe, 🧑 schickt ihn ab). */
export function startSourcesChat(text: string): void {
  sources = true;
  prefill = text;
  notify();
  setChatOpen(true);
}

/** Seite Quellen verlassen: der Chat ist wieder frei. */
export function endSourcesChat(): void {
  if (!sources) return;
  sources = false;
  notify();
}

/** Läuft ein Quellen-Auftrag (für den Hinweis im Chat)? */
export function useSourcesTask(): boolean {
  return useSyncExternalStore(subscribe, () => sources, () => false);
}

/** Vorbereiteter Text für die Eingabe ("" = keiner). */
export function usePrefill(): string {
  return useSyncExternalStore(subscribe, () => prefill, () => '');
}

/** Der Chat hat den vorbereiteten Text übernommen. */
export function clearPrefill(): void {
  prefill = '';
  notify();
}

/** Seitenkontext der nächsten Nachricht (nur bei `zuordnung`). */
export function chatContext(): AssignContext | undefined {
  return assign ?? undefined;
}

/** Sound- oder Grafik-Seite meldet ihren Kontext (null = keiner); beim Aushängen fällt er weg. */
export function useAssignContext(ctx: AssignContext | null): void {
  const key = ctx ? JSON.stringify(ctx) : '';
  useEffect(() => {
    if (!key) return undefined;
    const mine = JSON.parse(key) as AssignContext;
    assign = mine;
    notify();
    return () => { if (assign === mine) { assign = null; notify(); } };
  }, [key]);
}

/** Aktueller Seitenkontext für die Anzeige (Vorschlagskarten, Hinweis im Chat). */
export function useAssign(): AssignContext | null {
  return useSyncExternalStore(subscribe, () => assign, () => null);
}

/** Lädt die Antwort `text` als Entwurf (prüft sie vorher); false, wenn keine Seite sie annimmt. */
export async function loadDraft(text: string): Promise<boolean> {
  if (!loader) return false;
  await loader(text);
  return true;
}
