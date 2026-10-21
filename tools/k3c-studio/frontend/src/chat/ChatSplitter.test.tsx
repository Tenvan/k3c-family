// @vitest-environment happy-dom
import { act } from 'react';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { setChatOpen } from './chatBridge';
import { CHAT_WIDTH_DEFAULT, CHAT_WIDTH_MIN, chatWidthMax, clampChatWidth, keyWidth, parseChatWidth } from './ChatSplitter';
import { mount, panel, unmount } from './chatTestKit';

// Trennlinie zwischen Seite und Chat: Breite in Pixeln, gemerkt unter `chat.width`, geklemmt auf 280 px … 60 % des Fensters.

const KEY = 'k3c-studio.chat.width';
const W = 1000;

describe('Breite des Chats (rechnerisch)', () => {
  it('klemmt auf Minimum und 60 % des Fensters', () => {
    expect(clampChatWidth(100, W)).toBe(CHAT_WIDTH_MIN);
    expect(clampChatWidth(450.4, W)).toBe(450);
    expect(clampChatWidth(900, W)).toBe(600);
    expect(chatWidthMax(300)).toBe(CHAT_WIDTH_MIN); // kleines Fenster: nie unter dem Minimum
  });

  it('liest Gemerktes, Unlesbares ergibt die Vorgabe', () => {
    expect(parseChatWidth('420', W)).toBe(420);
    expect(parseChatWidth('9999', W)).toBe(600);
    expect(parseChatWidth('', W)).toBe(CHAT_WIDTH_DEFAULT);
    expect(parseChatWidth('breit', W)).toBe(CHAT_WIDTH_DEFAULT);
  });

  it('Pfeil links macht breiter, rechts schmaler; Umschalt vergrößert den Schritt; Pos1/Ende springen', () => {
    expect(keyWidth('ArrowLeft', false, 400, W)).toBe(416);
    expect(keyWidth('ArrowRight', false, 400, W)).toBe(384);
    expect(keyWidth('ArrowLeft', true, 400, W)).toBe(464);
    expect(keyWidth('Home', false, 400, W)).toBe(CHAT_WIDTH_MIN);
    expect(keyWidth('End', false, 400, W)).toBe(600);
    expect(keyWidth('a', false, 400, W)).toBeNull();
  });
});

beforeEach(() => { localStorage.clear(); setChatOpen(true); });
afterEach(unmount);

const bar = (h: HTMLElement) => h.querySelector<HTMLElement>('[role="separator"]')!;
const width = (h: HTMLElement) => Number(bar(h).getAttribute('aria-valuenow'));
const pointer = (type: string, clientX: number, target: EventTarget) => act(async () => { target.dispatchEvent(new MouseEvent(type, { bubbles: true, clientX })); });
const key = (h: HTMLElement, k: string, shiftKey = false) => act(async () => { bar(h).dispatchEvent(new KeyboardEvent('keydown', { key: k, shiftKey, bubbles: true })); });

describe('Trennlinie im DOM: Anzeige und Ziehen', () => {
  it('zeigt die Vorgabe mit den Werten für Hilfsmittel', async () => {
    const h = await mount();
    expect(bar(h).getAttribute('aria-orientation')).toBe('vertical');
    expect(width(h)).toBe(CHAT_WIDTH_DEFAULT);
    expect(bar(h).getAttribute('aria-valuemin')).toBe(String(CHAT_WIDTH_MIN));
    expect(bar(h).getAttribute('aria-valuemax')).toBe(String(chatWidthMax(window.innerWidth)));
    expect(bar(h).tabIndex).toBe(0);
    expect(panel(h).style.flexBasis).toBe(`${CHAT_WIDTH_DEFAULT}px`);
  });

  it('ändert die Breite durch Ziehen und merkt sie', async () => {
    const h = await mount();
    await pointer('pointerdown', window.innerWidth - 360, bar(h));
    await pointer('pointermove', window.innerWidth - 500, bar(h));
    expect(width(h)).toBe(500);
    expect(panel(h).style.flexBasis).toBe('500px');
    await pointer('pointerup', window.innerWidth - 500, bar(h));
    await pointer('pointermove', window.innerWidth - 700, bar(h)); // nach dem Loslassen passiert nichts mehr
    expect(width(h)).toBe(500);
    expect(localStorage.getItem(KEY)).toBe('500');
  });

  it('zieht nur nach einem Druck auf die Linie und endet mit dem Verlust des Zeigers', async () => {
    const h = await mount();
    await pointer('pointermove', window.innerWidth - 500, bar(h));
    expect(width(h)).toBe(CHAT_WIDTH_DEFAULT);
    await pointer('pointerdown', 0, bar(h));
    await pointer('lostpointercapture', 0, bar(h));
    await pointer('pointermove', window.innerWidth - 500, bar(h));
    expect(width(h)).toBe(CHAT_WIDTH_DEFAULT);
  });

  it('klemmt beim Ziehen', async () => {
    const h = await mount();
    await pointer('pointerdown', 0, bar(h));
    await pointer('pointermove', window.innerWidth - 10, bar(h));
    expect(width(h)).toBe(CHAT_WIDTH_MIN);
    await pointer('pointermove', 0, bar(h));
    expect(width(h)).toBe(chatWidthMax(window.innerWidth));
    await pointer('pointerup', 0, bar(h));
  });

  it('klemmt die Anzeige beim Verkleinern des Fensters neu und stellt die gemerkte Breite beim Vergrößern wieder her', async () => {
    localStorage.setItem(KEY, '600');
    const h = await mount();
    const original = window.innerWidth;
    expect(width(h)).toBe(Math.min(600, chatWidthMax(original)));
    const resize = async (w: number) => {
      Object.defineProperty(window, 'innerWidth', { value: w, configurable: true });
      await act(async () => { window.dispatchEvent(new Event('resize')); });
    };
    await resize(700);
    expect(width(h)).toBe(420);
    expect(panel(h).style.flexBasis).toBe('420px');
    expect(bar(h).getAttribute('aria-valuemax')).toBe('420');
    expect(localStorage.getItem(KEY)).toBe('600');
    await resize(2000);
    expect(width(h)).toBe(600);
    await resize(original);
  });
});

describe('Trennlinie im DOM: Tastatur, Merken, Ausblenden', () => {
  it('ändert die Breite mit den Pfeiltasten und setzt sie per Doppelklick zurück', async () => {
    const h = await mount();
    await key(h, 'ArrowLeft');
    expect(width(h)).toBe(CHAT_WIDTH_DEFAULT + 16);
    await key(h, 'ArrowRight', true);
    expect(width(h)).toBe(CHAT_WIDTH_DEFAULT + 16 - 64);
    expect(localStorage.getItem(KEY)).toBe(String(CHAT_WIDTH_DEFAULT - 48));
    await act(async () => { bar(h).dispatchEvent(new MouseEvent('dblclick', { bubbles: true })); });
    expect(width(h)).toBe(CHAT_WIDTH_DEFAULT);
    expect(localStorage.getItem(KEY)).toBe(String(CHAT_WIDTH_DEFAULT));
  });

  it('nimmt die gemerkte Breite beim Start', async () => {
    localStorage.setItem(KEY, '420');
    const h = await mount();
    expect(width(h)).toBe(420);
  });

  it('hat keine Trennlinie, solange der Chat ausgeblendet ist', async () => {
    const h = await mount();
    await act(async () => setChatOpen(false));
    expect(h.querySelector('[role="separator"]')).toBeNull();
    expect(panel(h).hidden).toBe(true);
    await act(async () => setChatOpen(true));
    expect(bar(h)).toBeTruthy();
  });
});
