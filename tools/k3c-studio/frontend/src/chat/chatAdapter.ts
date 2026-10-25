import type { ChatModelAdapter, ChatModelRunResult } from '@assistant-ui/react';
import { ChatGoneError, type AssignContext, type ChatChoiceValues, type ChatSendOptions, type ChatStatus, type ChatStream, type ChatTask } from '../api/chatTypes';

/** Text der letzten Nachricht von 🧑 (die CLI kennt den Verlauf nur über den Chat des Servers). */
function lastUserText(messages: readonly { role: string; content: readonly { type: string; text?: string }[] }[]): string {
  const last = [...messages].reverse().find((m) => m.role === 'user');
  return (last?.content ?? []).map((p) => (p.type === 'text' ? p.text ?? '' : '')).join('');
}

const text = (acc: string): ChatModelRunResult => ({ content: [{ type: 'text', text: acc }] });

/** Antworttext eines Fehlers ohne die Fehlermeldung (B-515): Schickt der Server den Text der CLI als Stück und danach
 *  als Fehler, stünde er zweimal da. Ist der Text genau die Meldung, entfällt er; endet er auf die Meldung, wird genau
 *  dieses Ende entfernt; sonst bleibt er unverändert. */
export function answerText(answer: string, error: string | undefined): string {
  const err = (error ?? '').trim();
  const body = answer.trim();
  if (!err) return answer;
  if (body === err) return '';
  return body.endsWith(err) ? body.slice(0, body.length - err.length).trimEnd() : answer;
}

const errorText = (e: unknown): string => (e instanceof Error ? e.message : String(e));
const cancelled = (acc: string): ChatModelRunResult => ({ ...text(acc), status: { type: 'incomplete', reason: 'cancelled' } });
const failed = (acc: string, error: string): ChatModelRunResult => ({ ...text(acc), status: { type: 'incomplete', reason: 'error', error } });

/** Letzter Stand einer Antwort nach dem Ausgang des Servers. */
function outcome(status: ChatStatus, acc: string, error: string | undefined): ChatModelRunResult {
  if (status === 'fertig') return { ...text(acc), status: { type: 'complete', reason: 'stop' } };
  if (status === 'abgebrochen') return cancelled(acc);
  return failed(answerText(acc, error), error ?? 'Unbekannter Fehler');
}

/** Verbindung des Adapters zur Oberfläche; alles ist optional. */
export interface ChatAdapterHooks {
  /** Chat des Servers, mit dem es weitergeht (leer = neuer Chat) */
  chat?: string;
  /** aktuelle Wahl von Anbieter, Modell und Denkstufe, bei jeder Nachricht neu gelesen */
  options?: () => ChatChoiceValues;
  /** Auftragsart der Nachricht (B-516), bei jeder Nachricht neu gelesen; undefined = freier Chat */
  task?: () => ChatTask | undefined;
  /** Seitenkontext des Auftrags `zuordnung` (B-600), bei jeder Nachricht neu gelesen */
  context?: () => AssignContext | undefined;
  /** z. B. „Verlauf nicht gespeichert“ */
  onNotice?: (notice: string) => void;
  /** der Server hat dem Chat eine Kennung gegeben (neuer Chat); leer, wenn der Chat nicht mehr existiert */
  onChat?: (chat: string) => void;
  /** die Antwort ist zu Ende (auch bei Fehler und Abbruch) */
  onEnd?: () => void;
}

const NO_CHOICE: ChatChoiceValues = { provider: '', model: '', effort: '' };

function sendOptions(hooks: ChatAdapterHooks): ChatSendOptions {
  const choice = hooks.options?.() ?? NO_CHOICE;
  const task = hooks.task?.();
  const kontext = task === 'zuordnung' ? hooks.context?.() : undefined;
  if (kontext) return { ...choice, task, kontext };
  return task && task !== 'zuordnung' ? { ...choice, task } : choice;
}

/** Bindet den Antwort-Strom des Studios an assistant-ui: Stücke wachsen zu einer Antwort, `done` setzt den Ausgang
 *  (fertig, abgebrochen, Fehler), `onNotice` meldet z. B. „Verlauf nicht gespeichert“. Der Chat des Servers
 *  (`start`) bleibt über alle Nachrichten derselben Oberfläche erhalten; `hooks.chat` setzt ihn für einen geladenen Chat. */
export function createChatAdapter(stream: ChatStream, hooks: ChatAdapterHooks = {}): ChatModelAdapter {
  let chat = hooks.chat ?? '';
  const notice = hooks.onNotice ?? (() => {});
  return {
    async *run({ messages, abortSignal }) {
      let acc = '';
      let ended = false;
      notice('');
      try {
        for await (const ev of stream(chat, lastUserText(messages), sendOptions(hooks), abortSignal)) {
          if (ev.kind === 'start') {
            chat = ev.chat;
            hooks.onChat?.(chat);
          }
          if (ev.kind === 'chunk') yield text((acc += ev.text));
          if (ev.kind !== 'done') continue;
          ended = true;
          if (ev.notice) notice(ev.notice);
          yield outcome(ev.status, acc, ev.error);
        }
        if (!ended) yield failed(acc, 'Verbindung ohne Abschluss beendet');
      } catch (e) {
        if (e instanceof ChatGoneError) {
          chat = ''; // der Chat ist weg: die nächste Nachricht beginnt einen neuen
          hooks.onChat?.('');
        }
        yield abortSignal.aborted ? cancelled(acc) : failed(acc, errorText(e));
      } finally {
        hooks.onEnd?.();
      }
    },
  };
}
