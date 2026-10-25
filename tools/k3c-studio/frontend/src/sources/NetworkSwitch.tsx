import { SelectField, type SelectOption } from '@webkit/atoms';
import { useEffect, useState } from 'react';
import { backend, type NetworkMode, type NetworkView } from '../api';
import { loadPref, savePref } from '../lib/prefs';

/** Meldet einen geänderten Netz-Modus; die Seite Quellen lädt dann neu. */
export const NETWORK_CHANGED = 'studio:network-changed';

const MODES = ['auto', 'online', 'offline'] as const;

/** Schalter in der Kopfzeile: Netz automatisch erkennen (Probe) oder fest online/offline. Der Modus bleibt gemerkt. */
export function NetworkSwitch() {
  const [view, setView] = useState<NetworkView>({ mode: 'auto', offline: false });
  const apply = async (mode: NetworkMode) => {
    try {
      setView(await backend.setNetwork(mode));
      savePref('app.network', mode);
      window.dispatchEvent(new Event(NETWORK_CHANGED));
    } catch {
      // Backend ohne Netz-Modus (älterer Stand): Schalter bleibt wirkungslos, die Probe gilt weiter
    }
  };
  useEffect(() => void apply(loadPref('app.network', MODES, 'auto')), []);
  const options: SelectOption<NetworkMode>[] = [
    { value: 'auto', label: `Netz: automatisch (${view.offline ? 'offline' : 'online'})` },
    { value: 'online', label: 'Netz: online' },
    { value: 'offline', label: 'Netz: offline' },
  ];
  return <SelectField size="1" aria-label="Netz-Modus" value={view.mode} options={options} onChange={(m) => void apply(m)} />;
}
