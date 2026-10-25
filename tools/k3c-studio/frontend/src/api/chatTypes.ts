// Chat des Studios (B-488, WZ19): Ereignisse des Antwort-Stroms von POST /api/studio/chat (Go: backend/chat_http.go)
// und die gespeicherten Chats (RPC ChatList, ChatGet, ChatDelete, ChatOptions).

/** Ausgang einer Antwort: `fertig`, `abgebrochen` (Zeitlimit oder Abbruch, der Text bis dahin bleibt) oder `fehler`. */
export type ChatStatus = 'fertig' | 'abgebrochen' | 'fehler';

export type ChatEvent =
  | { kind: 'start'; chat: string }
  | { kind: 'chunk'; text: string }
  | { kind: 'done'; status: ChatStatus; error?: string; notice?: string };

/** Der Server kennt den Chat nicht mehr (HTTP 410, z. B. gelöscht): der Chat muss ohne Kennung neu beginnen. */
export class ChatGoneError extends Error {}

/** Wahl für eine Antwort; leer heißt Standard der CLI. */
export interface ChatChoiceValues {
  provider: string;
  model: string;
  effort: string;
}

/** Auftragsart einer Nachricht: `welt` schickt das Grundgerüst als Instruktion mit (B-516), `zuordnung` den Seitenkontext
 *  von Sound- oder Grafik-Seite (B-600); fehlt sie, ist es ein freier Chat. */
export type ChatTask = 'welt' | 'zuordnung';

/** Seitenkontext eines Zuordnungs-Auftrags (Go: assignContext; Aufbereitung chat/assignContext.ts). */
export interface AssignContext {
  seite: 'sound' | 'grafik';
  /** gewählter Slot, leer = keiner */
  slot: string;
  /** aktiver Filter als Text */
  filter: string;
}

/** Wahl, Auftragsart und (nur bei `zuordnung`) Seitenkontext einer Nachricht. */
export type ChatSendOptions = ChatChoiceValues & { task?: ChatTask; kontext?: AssignContext };

/** Schickt `text` an den Chat `chat` (leer = neuer Chat) mit der Wahl `options` und liefert die Ereignisse der Antwort; `signal` bricht ab. */
export type ChatStream = (chat: string, text: string, options: ChatSendOptions, signal: AbortSignal) => AsyncGenerator<ChatEvent, void>;

/** Eintrag der Liste gespeicherter Chats (Go: ChatList); `updated` als Zeitstempel (RFC 3339). */
export interface ChatSummary {
  id: string;
  title: string;
  updated: string;
  count: number;
  provider?: string;
  model?: string;
  effort?: string;
}

/** Nachricht eines gespeicherten Chats; `status` und `error` nur an Antworten. */
export interface ChatStoredMessage {
  role: 'user' | 'assistant';
  text: string;
  status?: ChatStatus;
  error?: string;
  time: string;
}

/** Ein gespeicherter Chat mit allen Nachrichten (Go: ChatGet); Anbieter, Modell und Denkstufe, falls gewählt. */
export interface ChatDetail {
  id: string;
  created: string;
  updated: string;
  messages: ChatStoredMessage[];
  provider?: string;
  model?: string;
  effort?: string;
}

export interface ChatChoice {
  id: string;
  label: string;
}

/** Erlaubte Werte der drei Auswahlen im Chat (Go: ChatOptions); id "" ist der Standard. */
export interface ChatOptionsView {
  providers: ChatChoice[];
  models: ChatChoice[];
  efforts: ChatChoice[];
}

/** Die Chat-Methoden des Backends (Teil von Backend). */
export interface ChatApi {
  /** Chat: schickt eine Nachricht an die Claude Code CLI und liefert die Antwort als Strom (WZ19) */
  chatStream: ChatStream;
  /** Chat: gespeicherte Chats, beliebige Reihenfolge */
  chatList(): Promise<ChatSummary[]>;
  /** Chat: ein gespeicherter Chat mit seinen Nachrichten */
  chatGet(id: string): Promise<ChatDetail>;
  /** Chat: löscht einen gespeicherten Chat */
  chatDelete(id: string): Promise<void>;
  /** Chat: Anbieter, Modelle und Denkstufen zur Auswahl */
  chatOptions(): Promise<ChatOptionsView>;
}
