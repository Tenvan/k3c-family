import { useEffect, useState } from 'react';
import { ActionButton, Flex, SelectField, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { loadText, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { playtestApi } from './api';
import type { PlaytestDevices, PlaytestStatus } from './types';

const MONITORS = ['0', '1', '2', '3'] as const;
const IDLE: PlaytestStatus = { running: false, id: '', startMs: 0, encoder: '' };

const message = (e: unknown) => (e instanceof Error ? e.message : String(e));

/** Laufzeit „mm:ss“ bzw. „h:mm:ss“. */
export function clock(ms: number): string {
  const s = Math.max(0, Math.floor(ms / 1000));
  const mm = String(Math.floor((s % 3600) / 60)).padStart(2, '0');
  const ss = String(s % 60).padStart(2, '0');
  return s >= 3600 ? `${Math.floor(s / 3600)}:${mm}:${ss}` : `${mm}:${ss}`;
}

/** Spieltest starten (B-1219): Mikrofon und Monitor (gemerkt), Start, Stopp und Laufzeit. Endet das Spiel, endet die
 *  Aufnahme von selbst; `onChange` lädt nach Start und Ende die Liste neu. Der Status wird im Sekundentakt gelesen. */
export function RecordBar({ active, onChange }: { active: boolean; onChange: () => void }) {
  const [devices, setDevices] = useState<PlaytestDevices | null>(null);
  const [mic, setMic] = useState(() => loadText('playtest.mic', ''));
  const [monitor, setMonitor] = useState(() => loadText('playtest.monitor', '0'));
  const [status, setStatus] = useState(IDLE);
  const [error, setError] = useState('');
  const now = useNow(1000);

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
  const start = async () => {
    setError('');
    try {
      setStatus(await playtestApi.start({ mic, monitor: Number(monitor), gameArgs: [] }));
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

  const mics = devices?.mics ?? [];
  return (
    <Flex direction="column" gap="2" px="3" pb="3" data-record={status.running ? 'running' : 'idle'}>
      <Flex gap="2" align="center" wrap="wrap">
        <SelectField aria-label="Mikrofon" placeholder="Mikrofon wählen" value={mic} disabled={status.running}
          options={mics.map((m) => ({ value: m, label: m }))} onChange={choose} />
        <SelectField aria-label="Monitor" value={MONITORS.includes(monitor as (typeof MONITORS)[number]) ? monitor : '0'}
          disabled={status.running} options={MONITORS.map((m) => ({ value: m, label: `Monitor ${Number(m) + 1}` }))} onChange={screen} />
        <ActionButton disabled={status.running || !mic || devices?.ffmpeg === false} onClick={start}>Spieltest starten</ActionButton>
        <ActionButton color="red" variant="soft" disabled={!status.running} onClick={stop}>Stopp</ActionButton>
        {status.running && (
          <Text size="2" weight="bold" color="red" role="status">
            ● Aufnahme {status.id} · {clock(now - status.startMs)} · {status.encoder}
          </Text>
        )}
      </Flex>
      {devices?.ffmpeg === false && <NoticeCard title="ffmpeg fehlt" tone="warn">{devices.message}</NoticeCard>}
      {error && <NoticeCard title="Spieltest" tone="error">{error}</NoticeCard>}
    </Flex>
  );
}
