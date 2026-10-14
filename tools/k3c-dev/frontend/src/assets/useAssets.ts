import { useCallback, useEffect, useState } from 'react';
import { backend, type AssetChoice, type AssetsView } from '../api';
import { errorText } from '../lib/errors';

/** Slot-Register der Seite Assets: lädt frisch, speichert eine Zuordnung sofort und übernimmt die Antwort aus Go. */
export function useAssets() {
  const [view, setView] = useState<AssetsView | null>(null);
  const [error, setError] = useState('');

  const reload = useCallback(async () => {
    try {
      setView(await backend.assetsList());
      setError('');
    } catch (e) {
      setError(errorText(e));
    }
  }, []);

  useEffect(() => {
    void reload();
  }, [reload]);

  /** Speichert die Zuordnungen eines Slots; eine Ablehnung aus Go (fehlende Datei, Ausschnitt außerhalb) steht in error. */
  const save = useCallback(async (id: string, wahl: AssetChoice[]) => {
    try {
      const slot = await backend.assetsSet(id, wahl);
      setView((v) => (v ? { ...v, slots: v.slots.map((s) => (s.id === id ? slot : s)) } : v));
      setError('');
      return true;
    } catch (e) {
      setError(errorText(e));
      return false;
    }
  }, []);

  /** Vite-Dienst des Spiels starten (Bilder und Mini-Szene), danach die Vorschau-Adresse neu holen */
  const startVite = useCallback(async () => {
    try {
      await backend.serviceStart('Vite');
    } catch (e) {
      setError(errorText(e));
    }
    await reload();
  }, [reload]);

  return { view, error, reload, save, startVite };
}
