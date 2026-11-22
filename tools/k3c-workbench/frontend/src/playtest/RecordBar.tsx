import { useEffect, useState } from 'react';
import { ActionButton, Flex, SelectField, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { loadText, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { playtestApi } from './api';
import { RecordMonitor } from './RecordMonitor';
import type { PlaytestDevices, PlaytestStatus } from './types';

/** Ohne Geräteprobe (kein Desktop, ffmpeg fehlt) bleibt eine feste Auswahl, damit die Seite nutzbar bleibt. */
const FALLBACK_MONITORS = [0, 1, 2, 3].map((index) => ({ index, width: 0, height: 0, label: `Monitor ${index + 1}` }));
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
  const refresh = () => { setError(''); playtestApi.devices().then(setDevices, (e) => setError(message(e))); };
  const start = async (shown: string) => {
    setError('');
    try {
      setStatus(await playtestApi.start({ mic, monitor: Number(shown), gameArgs: [] }));
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
  const monitors = devices?.monitors?.length ? devices.monitors : FALLBACK_MONITORS;
  // Ein gemerktes Gerät, das es nicht mehr gibt (Mikro abgezogen, Monitor weg), wird nie stillschweigend benutzt.
  const micKnown = mics.includes(mic);
  const shownMonitor = monitors.some((m) => String(m.index) === monitor) ? monitor : String(monitors[0].index);
  return (
    <Flex direction="column" gap="2" px="3" pb="3" data-record={status.running ? 'running' : 'idle'}>
      <Flex gap="2" align="center" wrap="wrap">
        <SelectField aria-label="Mikrofon" placeholder="Mikrofon wählen" value={mic} disabled={status.running}
          options={mics.map((m) => ({ value: m, label: m }))} onChange={choose} />
        <SelectField aria-label="Monitor" value={shownMonitor}
          disabled={status.running} options={monitors.map((m) => ({ value: String(m.index), label: m.label }))} onChange={screen} />
        <ActionButton variant="soft" disabled={status.running} onClick={refresh}>Geräte neu suchen</ActionButton>
        {devices?.encoder && !status.running && <Text size="1" color="gray">Encoder: {devices.encoder}</Text>}
        <ActionButton disabled={status.running || !micKnown || devices?.ffmpeg === false}
          onClick={() => start(shownMonitor)}>Spieltest starten</ActionButton>
        <ActionButton color="red" variant="soft" disabled={!status.running} onClick={stop}>Stopp</ActionButton>
        {status.running && (
          <Text size="2" weight="bold" color="red" role="status">
            ● Aufnahme {status.id} · {clock(now - status.startMs)} · {status.encoder}
          </Text>
        )}
      </Flex>
      {status.running && status.stats && <RecordMonitor stats={status.stats} />}
      {devices?.ffmpeg === false && <NoticeCard title="ffmpeg fehlt" tone="warn">{devices.message}</NoticeCard>}
      {devices?.ffmpeg && mics.length === 0 && (
        <NoticeCard title="Kein Mikrofon gefunden" tone="warn">
          ffmpeg sieht kein Aufnahmegerät (DirectShow). Raummikro anschließen und „Geräte neu suchen“ drücken.
        </NoticeCard>
      )}
      {devices?.ffmpeg && mic && mics.length > 0 && !micKnown && (
        <NoticeCard title="Mikrofon nicht gefunden" tone="warn">„{mic}“ ist nicht angeschlossen. Anderes Mikrofon wählen oder „Geräte neu suchen“.</NoticeCard>
      )}
      {error && <NoticeCard title="Spieltest" tone="error">{error}</NoticeCard>}
    </Flex>
  );
}
