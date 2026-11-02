import { keyAction } from '@game/tools/soundtestLogic';
import { useEffect, useRef } from 'react';
import { changeVolume, stopPlayback } from './transport';

interface Keys {
  /** Auswahl in der Slot-Liste verschieben (↑ = −1, ↓ = +1) */
  onMove: (delta: -1 | 1) => void;
  /** Enter, Leertaste oder X: die aktive Zuordnung des gewählten Slots abspielen bzw. überblenden */
  onPlay: () => void;
}

/** Eingabefelder und Bedienelemente mit eigener Tastatur (Dialoge, Menüs, Auswahllisten, Splitter, Regler, Reiter) behalten ihre Tasten. */
const OWN_KEYS = '[role="dialog"],[role="alertdialog"],[role="menu"],[role="listbox"]:not([aria-label="Asset-Slots"]),[role="separator"],[role="slider"],[role="tablist"]';
const typing = (t: EventTarget | null): boolean =>
  t instanceof HTMLElement && (t.isContentEditable || ['INPUT', 'TEXTAREA', 'SELECT'].includes(t.tagName) || t.closest(OWN_KEYS) !== null);

/** Tastatur der Hörprobe auf der ganzen Seite (↑↓, Enter/Leertaste, X, S, +/−), nicht während der Eingabe in ein Feld; Enter und
 *  Leertaste auf einem Knopf bleiben dessen eigener Klick. */
export function usePageKeys(keys: Keys): void {
  const ref = useRef(keys);
  ref.current = keys;
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const a = keyAction(e.code);
      if (!a || e.defaultPrevented || typing(e.target) || e.ctrlKey || e.metaKey || e.altKey) return;
      if (a === 'play' && e.target instanceof HTMLButtonElement) return;
      e.preventDefault();
      if (a === 'up') ref.current.onMove(-1);
      else if (a === 'down') ref.current.onMove(1);
      else if (a === 'play' || a === 'fade') ref.current.onPlay();
      else if (a === 'stop') stopPlayback();
      else void changeVolume(a === 'volUp' ? 1 : -1);
    };
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, []);
}
