// Prompt-Vorlagen der Seiten Styleguide, Sound und Quellen (B-727 bis B-729): einmal laden, nach Ort gruppiert.
import { useEffect, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { assetsPrompts, type Vorlage, type VorlagenOrt } from '../api';

export type VorlagenJeOrt = (ort: VorlagenOrt) => Vorlage[];

/** Lädt `data/asset-prompts.json` und liefert die Vorlagen eines Orts; ein Ladefehler steht in `error`. */
export function useVorlagen(): { jeOrt: VorlagenJeOrt; error: string } {
  const [vorlagen, setVorlagen] = useState<Vorlage[]>([]);
  const [error, setError] = useState('');
  useEffect(() => {
    let live = true;
    assetsPrompts().then((v) => live && setVorlagen(v), (e) => live && setError(errorText(e)));
    return () => {
      live = false;
    };
  }, []);
  return { jeOrt: (ort) => vorlagen.filter((v) => v.ort === ort), error };
}
