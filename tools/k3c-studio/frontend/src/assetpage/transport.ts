import { stepVolume } from '@game/tools/soundtestLogic';
import { useSyncExternalStore } from 'react';
import type { AssetChoice, AssetSlot } from '../api';
import { core, playChoice, stopAll } from '../sound/audio';
import { playStatus } from '../sound/playback';

// Zustand der einen Abspielleiste der Assets-Seite (B-677): was spielt, Meldung, Lautstärke. Alle ▶ der Seite laufen über
// playOn; es klingt immer nur ein Ton (Musik blendet über, alles andere stoppt vorher).

export interface Transport {
  playing: string;
  status: string;
  volume: number;
}

let state: Transport | null = null;
const listeners = new Set<() => void>();

function current(): Transport {
  state ??= { playing: '', status: '', volume: core().getVolume('music') };
  return state;
}
function set(next: Partial<Transport>): void {
  state = { ...current(), ...next };
  listeners.forEach((fn) => fn());
}

const subscribe = (fn: () => void) => {
  listeners.add(fn);
  return () => listeners.delete(fn);
};

export const useTransport = (): Transport => useSyncExternalStore(subscribe, current, current);

/** Spielt eine Zuordnung des Slots. `far` dämpft einen Atlas-Ton wie in der anderen Bildhälfte. */
export async function playOn(slot: Pick<AssetSlot, 'id' | 'beschreibung' | 'bus'>, choice: AssetChoice, far = false): Promise<boolean> {
  if (slot.bus === 'music') {
    core().stop('sfx');
    core().stop('ambient');
  } else stopAll();
  const ok = await playChoice(slot, choice, far);
  set({ playing: ok ? `${slot.id} · ${choice.datei.split('/').pop()}${choice.sprite ? `#${choice.sprite}` : ''}` : '', status: playStatus(slot.beschreibung, choice, ok) });
  return ok;
}

/** Spielt eine Datei ohne Slot (Vorrat, Netz-Vorschau). */
export async function playFile(label: string, datei: string): Promise<boolean> {
  stopAll();
  const ok = await playChoice({}, { datei });
  set({ playing: ok ? label : '', status: ok ? '' : `Datei fehlt oder ist nicht abspielbar: ${datei}` });
  return ok;
}

export function stopPlayback(): void {
  stopAll();
  set({ playing: '', status: '' });
}

/** Lautstärke ±10 % (Mixer des Audio-Kerns, gemerkt), wie die Hörprobe. */
export async function changeVolume(dir: -1 | 1): Promise<void> {
  const k = core();
  await k.onInput();
  if (!k.running) return;
  k.setVolume('music', stepVolume(k.getVolume('music'), dir));
  k.setVolume('sfx', k.getVolume('music'));
  set({ volume: k.getVolume('music') });
}
