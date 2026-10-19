import { useEffect, useState } from 'react';
import { backend, type WorldsView } from '../api';
import type { Notice } from './planSave';

/** Welten des Welten Editors (jede eine JSON-Datei mit allen Inseln und Anpassungen): Liste und gewählte Welt vom Backend.
 *  Wählen, Anlegen und Entfernen melden Fehler als Hinweis und rufen danach `onChanged` (Planung der gewählten Welt neu laden). */
export function useWorlds(onChanged: () => Promise<void>, setNotice: (n: Notice) => void) {
  const [worlds, setWorlds] = useState<WorldsView | null>(null);
  useEffect(() => { void backend.planWorlds().then(setWorlds); }, []);
  const apply = async (call: () => Promise<WorldsView>) => {
    const next = await call();
    setWorlds(next);
    if (next.error) { setNotice({ kind: 'error', text: next.error }); return; }
    await onChanged();
  };
  return {
    worlds,
    selectWorld: (id: string) => apply(() => backend.planWorldSelect(id)),
    addWorld: (name: string) => apply(() => backend.planWorldAdd(name)),
    removeWorld: (id: string) => apply(() => backend.planWorldRemove(id)),
  };
}
