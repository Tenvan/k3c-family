import { useEffect, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { backend } from '../api';
import type { WeltSicht } from './api';

/** Eine Welt der Auswahl (Liste des Welten-Editors, feste Welten auch ohne Datei, WZ62.1). */
export interface WeltAuswahl {
  id: string;
  name: string;
  missing?: boolean;
}

interface Geladen {
  welt: string;
  sicht: WeltSicht | null;
  error: string;
}

/** Weltliste und Sicht der gewählten Welt (B-1169): welche Assets sie sieht, mit Herkunft. Ohne Welt (`welt === ''`) gibt es
 *  keine Sicht, alles ist sichtbar. Eine Antwort für eine inzwischen andere Welt wird verworfen; solange sie fehlt, ist
 *  `sicht` leer und `laedt` wahr. Katalog und Sound nutzen denselben Hook. */
export function useWeltSicht(welt: string) {
  const [welten, setWelten] = useState<WeltAuswahl[]>([]);
  const [geladen, setGeladen] = useState<Geladen | null>(null);
  useEffect(() => {
    let live = true;
    backend.worldList().then((l) => live && setWelten(l), () => live && setWelten([]));
    return () => {
      live = false;
    };
  }, []);
  useEffect(() => {
    if (!welt) return;
    let live = true;
    backend.studioWeltSicht(welt).then(
      (sicht) => live && setGeladen({ welt, sicht, error: '' }),
      (e) => live && setGeladen({ welt, sicht: null, error: errorText(e) }),
    );
    return () => {
      live = false;
    };
  }, [welt]);
  const aktuell = welt && geladen?.welt === welt ? geladen : null;
  return { welten, sicht: aktuell?.sicht ?? null, laedt: Boolean(welt) && !aktuell, error: aktuell?.error ?? '' };
}
