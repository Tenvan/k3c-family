// Chat des Studios (B-488, WZ19): Ereignisse des Antwort-Stroms von POST /api/studio/chat (Go: backend/chat_http.go).

/** Ausgang einer Antwort: `fertig`, `abgebrochen` (Zeitlimit oder Abbruch, der Text bis dahin bleibt) oder `fehler`. */
export type ChatStatus = 'fertig' | 'abgebrochen' | 'fehler';

export type ChatEvent =
  | { kind: 'start'; chat: string }
  | { kind: 'chunk'; text: string }
  | { kind: 'done'; status: ChatStatus; error?: string; notice?: string };

/** Schickt `text` an den Chat `chat` (leer = neuer Chat) und liefert die Ereignisse der Antwort; `signal` bricht ab. */
export type ChatStream = (chat: string, text: string, signal: AbortSignal) => AsyncGenerator<ChatEvent, void>;
