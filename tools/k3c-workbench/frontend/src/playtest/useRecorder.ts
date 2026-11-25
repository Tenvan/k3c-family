import { useEffect, useState } from 'react';
import { loadText, savePref } from '../lib/prefs';
import { playtestApi } from './api';
import type { PlaytestDevices, PlaytestStatus } from './types';

const IDLE: PlaytestStatus = { running: false, id: '', startMs: 0, encoder: '' };

const message = (e: unknown) => (e instanceof Error ? e.message : String(e));

/** Zustand der Aufnahmeleiste: Geräte, gemerkte Wahl, Status im Sekundentakt, Start und Stopp. Endet die Aufnahme,
 *  ruft `onChange` die Liste neu auf. */
export function useRecorder(active: boolean, onChange: () => void) {
  const [devices, setDevices] = useState<PlaytestDevices | null>(null);
  const [mic, setMic] = useState(() => loadText('playtest.mic', ''));
  const [monitor, setMonitor] = useState(() => loadText('playtest.monitor', '0'));
  const [status, setStatus] = useState(IDLE);
  const [error, setError] = useState('');

  useEffect(() => {
    if (!active) return undefined;
    playtestApi.devices().then(setDevices, (e) => setError(message(e)));
    let was = false;
    const poll = () => playtestApi.status().then((s) => {
      setStatus(s);
      if (was && !s.running) onChange();
      was = s.running;
    }, () => undefined);
    void poll();
    const t = setInterval(() => void poll(), 1000);
    return () => clearInterval(t);
  }, [active, onChange]);

  const choose = (m: string) => { setMic(m); savePref('playtest.mic', m); };
  const screen = (m: string) => { setMonitor(m); savePref('playtest.monitor', m); };
  const refresh = () => { setError(''); playtestApi.devices().then(setDevices, (e) => setError(message(e))); };
  const start = async (shown: string, voiceOnly = false) => {
    setError('');
    try {
      setStatus(await playtestApi.start({ mic, monitor: Number(shown), gameArgs: [], voiceOnly }));
      onChange();
    } catch (e) {
      setError(message(e));
    }
  };
  const stop = async () => {
    try {
      await playtestApi.stop();
    } catch (e) {
      setError(message(e));
    }
  };
  return { devices, mic, monitor, status, error, choose, screen, refresh, start, stop };
}

export type Recorder = ReturnType<typeof useRecorder>;
