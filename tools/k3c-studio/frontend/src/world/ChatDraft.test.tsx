// @vitest-environment happy-dom
import { Theme } from '@radix-ui/themes';
import { act } from 'react';
import { createRoot, type Root } from 'react-dom/client';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { backend } from '../api';
import { ChatPanel } from '../chat/ChatPanel';
import { setChatOpen } from '../chat/chatBridge';
import { WorldPage } from './WorldPage';

// WZ19.3 (B-488/AC-08) im DOM mit dem Mock-Backend: „Prompt-Grundgerüst“ füllt das Eingabefeld des Chats; eine Antwort wird
// mit „Als Entwurf laden“ geprüft und als Entwurf geladen, Fehler stehen am Entwurf, gespeichert wird nichts.
vi.mock('../stage/PreviewStage', () => ({ PreviewStage: class { show() {} refresh() {} setLook() {} setPosition() {} destroy() {} } }));
(globalThis as { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;

let root: Root | null = null;
let host: HTMLElement | null = null;

const wait = (ms = 0) => act(async () => { await new Promise((r) => setTimeout(r, ms)); });
const flush = async () => { for (let i = 0; i < 6; i++) await wait(); };
const settle = async () => { await wait(260); await flush(); }; // Prüfung ist um 200 ms verzögert

const button = (h: HTMLElement, label: string) =>
  [...h.querySelectorAll('button')].find((b) => b.textContent === label || b.getAttribute('aria-label') === label)! as HTMLButtonElement;
const field = (h: HTMLElement, label: string) => h.querySelector<HTMLInputElement>(`[aria-label="${label}"]`)!;
const notice = (h: HTMLElement) => h.querySelector('.wd-notice')?.textContent ?? '';
const chatInput = (h: HTMLElement) => h.querySelector<HTMLTextAreaElement>('.chat-input')!;

async function typeInto(el: HTMLInputElement | HTMLTextAreaElement, value: string) {
  const proto = el instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype;
  Object.getOwnPropertyDescriptor(proto, 'value')!.set!.call(el, value);
  await act(async () => el.dispatchEvent(new Event('input', { bubbles: true })));
}

/** Schickt `text` an den Chat und wartet auf die fertige Antwort des Mocks („Antwort auf: <text>“). */
async function ask(h: HTMLElement, text: string) {
  await typeInto(chatInput(h), text);
  await act(async () => button(h, 'Senden').click());
  await wait(80);
  await flush();
}

const world = (islands: object[]) => JSON.stringify({ id: 'nordmeer', name: 'Nordmeer', notes: '', islands });
const island = (id: string, depths: number[]) => ({ id, name: id, depths, depthScaling: { hp: 1.5, damage: 1.3, speed: 1.1 } });

beforeEach(async () => {
  localStorage.clear();
  setChatOpen(true);
  host = document.createElement('div');
  document.body.append(host);
  root = createRoot(host);
  await act(async () => root!.render(<Theme><WorldPage /><ChatPanel /></Theme>));
  await flush();
});

afterEach(() => {
  act(() => root?.unmount());
  host?.remove();
  vi.restoreAllMocks();
  vi.unstubAllGlobals();
});

describe('Welt per Prompt füllen (WZ19.3/AC-08)', () => {
  it('legt das Prompt-Grundgerüst ins Eingabefeld des Chats, ohne zu senden', async () => {
    await act(async () => button(host!, 'Prompt-Grundgerüst').click());
    await flush();
    expect(chatInput(host!).value.endsWith('MEINE WÜNSCHE:\n')).toBe(true);
    expect(host!.querySelector('.chat-msg')).toBeNull();
  });

  it('lädt eine gültige Antwort als Entwurf und speichert nichts', async () => {
    const save = vi.spyOn(backend, 'planWorldSave');
    await ask(host!, world([island('island1', [0, 1, 2, 3, 4]), island('eisinsel', [0, 4])]));
    await act(async () => button(host!, 'Als Entwurf laden').click());
    await settle();
    expect(field(host!, 'Name der Welt').value).toBe('Nordmeer');
    expect(field(host!, 'Name Insel 2').value).toBe('eisinsel');
    expect(notice(host!)).toContain('als Entwurf geladen');
    expect(host!.querySelector('.wd-error')).toBeNull();
    expect(save).not.toHaveBeenCalled();
  });

  it('Entwurf mit Planung aus dem Chat gilt nach dem Speichern als gespeichert', async () => {
    const isl = { ...island('eisinsel', [0, 4]), plannedChunks: { '0': [{ slot: -3, kind: 'clearing' }] }, chunkCounts: { '0': 24 } };
    await ask(host!, world([island('island1', [0, 1, 2, 3, 4]), isl]));
    await act(async () => button(host!, 'Als Entwurf laden').click());
    await settle();
    await act(async () => button(host!, 'Welt speichern').click());
    await settle();
    expect(button(host!, 'Welt speichern').disabled).toBe(true);
    expect(button(host!, 'Entwurf verwerfen').disabled).toBe(true);
  });

  it('zeigt den Grund am Entwurf, wenn die Welt die Prüfung nicht besteht', async () => {
    await ask(host!, world([island('island1', [0, 1, 2, 3, 4]), island('insel3', [0, 1, 2])]));
    await act(async () => button(host!, 'Als Entwurf laden').click());
    await settle();
    expect(notice(host!)).toContain('Insel insel3: kein Endboss in der tiefsten Stufe 2');
    expect(host!.querySelector('.wd-error[data-island="1"][data-field="depths"]')?.textContent).toContain('kein Endboss in der tiefsten Stufe 2');
    expect(button(host!, 'Welt speichern').disabled).toBe(true);
  });

  it('meldet „Antwort nicht lesbar“, wenn kein JSON da ist, und lädt nichts', async () => {
    await ask(host!, 'hallo');
    await act(async () => button(host!, 'Als Entwurf laden').click());
    await flush();
    expect(notice(host!)).toContain('Antwort nicht lesbar');
    expect(button(host!, 'Entwurf verwerfen').disabled).toBe(true);
  });

  it('fragt nach, bevor ein ungespeicherter Entwurf ersetzt wird', async () => {
    await typeInto(field(host!, 'Name der Welt'), 'Mein Entwurf');
    await ask(host!, world([island('island1', [0, 1, 2, 3, 4])]));
    const confirm = vi.fn().mockReturnValueOnce(false).mockReturnValueOnce(true);
    vi.stubGlobal('confirm', confirm);
    window.confirm = confirm;
    await act(async () => button(host!, 'Als Entwurf laden').click());
    await flush();
    expect(confirm).toHaveBeenCalledTimes(1);
    expect(field(host!, 'Name der Welt').value).toBe('Mein Entwurf'); // abgelehnt: bleibt
    await act(async () => button(host!, 'Als Entwurf laden').click());
    await flush();
    expect(field(host!, 'Name der Welt').value).toBe('Nordmeer'); // bestätigt: ersetzt
  });
});
