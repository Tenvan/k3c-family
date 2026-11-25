import { ActionButton, Flex, SelectField, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { useNow } from '../lib/useNow';
import { RecordMonitor } from './RecordMonitor';
import { useRecorder, type Recorder } from './useRecorder';
import type { PlaytestStatus } from './types';

/** Ohne Geräteprobe (kein Desktop, ffmpeg fehlt) bleibt eine feste Auswahl, damit die Seite nutzbar bleibt. */
const FALLBACK_MONITORS = [0, 1, 2, 3].map((index) => ({ index, width: 0, height: 0, label: `Monitor ${index + 1}` }));

/** Laufzeit „mm:ss“ bzw. „h:mm:ss“. */
export function clock(ms: number): string {
  const s = Math.max(0, Math.floor(ms / 1000));
  const mm = String(Math.floor((s % 3600) / 60)).padStart(2, '0');
  const ss = String(s % 60).padStart(2, '0');
  return s >= 3600 ? `${Math.floor(s / 3600)}:${mm}:${ss}` : `${mm}:${ss}`;
}

/** Mikrofone, Monitore und was davon gerade benutzbar ist. */
function deviceView(r: Recorder) {
  const { devices, mic, monitor } = r;
  const mics = devices?.mics ?? [];
  const monitors = devices?.monitors?.length ? devices.monitors : FALLBACK_MONITORS;
  // Ein gemerktes Gerät, das es nicht mehr gibt (Mikro abgezogen, Monitor weg), wird nie stillschweigend benutzt.
  const micKnown = mics.includes(mic);
  const shownMonitor = monitors.some((m) => String(m.index) === monitor) ? monitor : String(monitors[0].index);
  const cannotStart = r.status.running || !micKnown || devices?.ffmpeg === false;
  return { mics, monitors, micKnown, shownMonitor, cannotStart };
}

/** Kopfzeile „● Aufnahme <id> · mm:ss“ während der Aufnahme. */
function RunningBadge({ status, now }: { status: PlaytestStatus; now: number }) {
  const kind = status.voiceOnly ? 'Mikro-Session' : 'Aufnahme';
  return (
    <Text size="2" weight="bold" color="red" role="status">
      ● {status.starting ? 'Start' : kind} {status.id} · {clock(now - status.startMs)}{status.encoder ? ` · ${status.encoder}` : ''}
    </Text>
  );
}

/** Hinweise zu fehlendem ffmpeg oder Mikrofon und der letzte Fehler. */
function RecordNotices({ r, mics, micKnown }: { r: Recorder; mics: string[]; micKnown: boolean }) {
  const { devices, mic, error } = r;
  return (
    <>
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
    </>
  );
}

/** Spieltest starten (B-1219): Mikrofon und Monitor (gemerkt), Start, Stopp und Laufzeit. Endet das Spiel, endet die
 *  Aufnahme von selbst; `onChange` lädt nach Start und Ende die Liste neu. Der Status wird im Sekundentakt gelesen. */
export function RecordBar({ active, onChange }: { active: boolean; onChange: () => void }) {
  const r = useRecorder(active, onChange);
  const now = useNow(1000);
  const { devices, status } = r;
  const { mics, monitors, micKnown, shownMonitor, cannotStart } = deviceView(r);
  return (
    <Flex direction="column" gap="2" px="3" pb="3" data-record={status.running ? 'running' : 'idle'}>
      <Flex gap="2" align="center" wrap="wrap">
        <SelectField aria-label="Mikrofon" placeholder="Mikrofon wählen" value={r.mic} disabled={status.running}
          options={mics.map((m) => ({ value: m, label: m }))} onChange={r.choose} />
        <SelectField aria-label="Monitor" value={shownMonitor}
          disabled={status.running} options={monitors.map((m) => ({ value: String(m.index), label: m.label }))} onChange={r.screen} />
        <ActionButton variant="soft" disabled={status.running} onClick={r.refresh}>Geräte neu suchen</ActionButton>
        {devices?.encoder && !status.running && <Text size="1" color="gray">Encoder: {devices.encoder}</Text>}
        <ActionButton disabled={cannotStart} onClick={() => r.start(shownMonitor)}>Spieltest starten</ActionButton>
        <ActionButton variant="soft" disabled={cannotStart} onClick={() => r.start(shownMonitor, true)}>Nur Mikro starten</ActionButton>
        <ActionButton color="red" variant="soft" disabled={!status.running} onClick={r.stop}>Stopp</ActionButton>
        {status.running && <RunningBadge status={status} now={now} />}
      </Flex>
      {status.starting && <Text size="2" color="gray" role="status">Das Spiel startet, die Aufnahme beginnt mit seinem ersten Bild …</Text>}
      {status.running && !status.starting && status.stats && <RecordMonitor stats={status.stats} />}
      <RecordNotices r={r} mics={mics} micKnown={micKnown} />
    </Flex>
  );
}
