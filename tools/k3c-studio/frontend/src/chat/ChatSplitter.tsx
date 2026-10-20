import { useEffect, useRef, useState } from 'react';
import { loadText, savePref } from '../lib/prefs';

// Trennlinie zwischen Seite und Chat: Ziehen (Zeiger), Pfeiltasten (Tastatur) und Doppelklick (zurücksetzen) ändern die
// Breite des Chats. Die Breite wird in Pixeln gemerkt, `chat.width`.

export const CHAT_WIDTH_DEFAULT = 360;
export const CHAT_WIDTH_MIN = 280;
const KEY = 'chat.width';
const STEP = 16;

/** Breite des Fensters; ohne Fenster (Render auf dem Server, Test) eine übliche Breite. */
const viewportWidth = (): number => (typeof window === 'undefined' ? 1920 : window.innerWidth);

/** Größte Breite des Chats: 60 % des Fensters, nie unter dem Minimum. */
export const chatWidthMax = (windowWidth: number): number => Math.max(CHAT_WIDTH_MIN, Math.floor(windowWidth * 0.6));

/** Klemmt die Breite auf Minimum … 60 % des Fensters, ganze Pixel. */
export function clampChatWidth(px: number, windowWidth: number): number {
  return Math.round(Math.min(chatWidthMax(windowWidth), Math.max(CHAT_WIDTH_MIN, px)));
}

/** Gemerkte Breite als Text: unlesbar oder leer gilt die Vorgabe. */
export function parseChatWidth(raw: string, windowWidth: number): number {
  const n = Number(raw);
  return raw.trim() !== '' && Number.isFinite(n) ? clampChatWidth(n, windowWidth) : CHAT_WIDTH_DEFAULT;
}

/** Gemerkte Breite des Chats; liefert Wert, Setzen (klemmt und merkt) und Zurücksetzen auf die Vorgabe. Ändert sich die
 *  Fenstergröße, wird nur die angezeigte Breite neu geklemmt, die gemerkte bleibt (ein größeres Fenster stellt sie wieder her). */
export function useChatWidth(): [number, (px: number) => void, () => void] {
  const [wanted, setWanted] = useState(() => parseChatWidth(loadText(KEY, ''), viewportWidth()));
  const [viewport, setViewport] = useState(viewportWidth);
  useEffect(() => {
    const onResize = () => setViewport(viewportWidth());
    window.addEventListener('resize', onResize);
    return () => window.removeEventListener('resize', onResize);
  }, []);
  const set = (px: number) => {
    const n = clampChatWidth(px, viewportWidth());
    setWanted(n);
    savePref(KEY, String(n));
  };
  return [clampChatWidth(wanted, viewport), set, () => set(CHAT_WIDTH_DEFAULT)];
}

/** Breite nach einer Taste: Pfeil links macht den Chat breiter (die Linie wandert nach links), Pfeil rechts schmaler,
 *  Umschalt wählt große Schritte, Pos1/Ende springen an die Grenzen; null bei anderen Tasten. */
export function keyWidth(key: string, shift: boolean, width: number, windowWidth: number): number | null {
  const step = shift ? STEP * 4 : STEP;
  if (key === 'ArrowLeft') return clampChatWidth(width + step, windowWidth);
  if (key === 'ArrowRight') return clampChatWidth(width - step, windowWidth);
  if (key === 'Home') return CHAT_WIDTH_MIN;
  if (key === 'End') return chatWidthMax(windowWidth);
  return null;
}

export function ChatSplitter({ width, onChange, onReset }: { width: number; onChange: (px: number) => void; onReset: () => void }) {
  const dragging = useRef(false);
  // Der Zeiger bleibt mit der Linie verbunden (setPointerCapture), auch über dem Chat oder der Seite; es gibt keine Listener am Fenster.
  const start = (e: React.PointerEvent) => {
    e.preventDefault();
    try {
      e.currentTarget.setPointerCapture(e.pointerId);
    } catch {
      // ohne Capture (alte Umgebung) zieht die Linie nur über sich selbst
    }
    dragging.current = true;
  };
  const stop = () => { dragging.current = false; };
  const press = (e: React.KeyboardEvent) => {
    const next = keyWidth(e.key, e.shiftKey, width, viewportWidth());
    if (next === null) return;
    e.preventDefault();
    onChange(next);
  };
  return (
    <div className="chat-split" role="separator" aria-orientation="vertical" aria-label="Breite des Chats" tabIndex={0}
      aria-valuenow={width} aria-valuemin={CHAT_WIDTH_MIN} aria-valuemax={chatWidthMax(viewportWidth())}
      title="Ziehen oder Pfeiltasten ändern die Breite, Doppelklick setzt zurück"
      onPointerDown={start} onPointerMove={(e) => { if (dragging.current) onChange(viewportWidth() - e.clientX); }}
      onPointerUp={stop} onPointerCancel={stop} onLostPointerCapture={stop}
      onKeyDown={press} onDoubleClick={onReset} />
  );
}
