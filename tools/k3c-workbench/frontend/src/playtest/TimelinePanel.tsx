import { useCallback, useEffect, useState } from 'react';
import { ActionButton, CardButton, Flex, Heading, StatusBadge, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { frameUrl, playtestApi } from './api';
import { clock } from './RecordBar';
import type { PlaytestCapability, PlaytestJob, PlaytestSession, PlaytestTimeline } from './types';

const message = (e: unknown) => (e instanceof Error ? e.message : String(e));
const NO_JOB: PlaytestJob = { running: false, id: '', step: '', done: 0, total: 0, message: '', error: '' };

interface Props {
  session: PlaytestSession;
  /** Springt im Player an die Videozeit (ms). */
  onSeek: (videoMs: number) => void;
  /** Die Auswertung ist zu Ende (Liste neu laden). */
  onDone: () => void;
}

/** Auswertung (B-1223): Knopf Auswerten (Transkript aus voice.wav, Bild je Marker) mit Fortschritt und Abbrechen, dazu
 *  die Zeitleiste aus Sätzen, Markern und deren Bildern; ein Klick springt im Player an die Stelle. */
export function TimelinePanel({ session: s, onSeek, onDone }: Props) {
  const [cap, setCap] = useState<PlaytestCapability | null>(null);
  const [job, setJob] = useState(NO_JOB);
  const [timeline, setTimeline] = useState<PlaytestTimeline | null>(null);
  const [error, setError] = useState('');
  const mine = job.id === s.id;
  const busy = job.running;

  const loadTimeline = useCallback(() => {
    if (s.running) return;
    playtestApi.timeline(s.id).then(setTimeline, (e) => setError(message(e)));
  }, [s.id, s.running]);

  useEffect(() => {
    playtestApi.check().then(setCap, (e) => setError(message(e)));
    playtestApi.transcribeStatus().then(setJob, () => undefined);
  }, []);
  useEffect(() => {
    setTimeline(null);
    loadTimeline();
  }, [loadTimeline, s.stage]);
  useEffect(() => {
    if (!busy) return undefined;
    const t = setInterval(() => void playtestApi.transcribeStatus().then((j) => {
      setJob(j);
      if (!j.running) {
        loadTimeline();
        onDone();
      }
    }, () => undefined), 1000);
    return () => clearInterval(t);
  }, [busy, loadTimeline, onDone]);

  const start = async () => {
    setError('');
    try {
      setJob(await playtestApi.transcribe(s.id));
      onDone();
      loadTimeline();
    } catch (e) {
      setError(message(e));
    }
  };
  const cancel = () => void playtestApi.transcribeCancel().catch((e) => setError(message(e)));

  const blocked = !s.hasVoice || s.running || (busy && !mine) || cap?.ready === false;
  return (
    <Flex direction="column" gap="2" data-timeline={s.id}>
      <Flex align="center" justify="between" gap="3" wrap="wrap">
        <Heading size="3">Auswertung</Heading>
        <Flex gap="2" align="center">
          {busy && mine && (
            <Text size="2" role="status">
              {job.step}{job.total > 0 ? ` ${job.done}/${job.total}` : ' …'}
            </Text>
          )}
          {busy && mine
            ? <ActionButton variant="soft" color="red" onClick={cancel}>Abbrechen</ActionButton>
            : <ActionButton disabled={blocked} onClick={start}>Auswerten</ActionButton>}
        </Flex>
      </Flex>
      {cap && !cap.ready && <NoticeCard title="Auswerten gesperrt" tone="warn">{cap.message}</NoticeCard>}
      {mine && !busy && job.error && <NoticeCard title="Auswertung" tone="error">{job.error}</NoticeCard>}
      {mine && !busy && !job.error && job.message && <Text size="2" color="gray">{job.message}</Text>}
      {error && <NoticeCard title="Auswertung" tone="error">{error}</NoticeCard>}
      <Entries id={s.id} timeline={timeline} onSeek={onSeek} />
    </Flex>
  );
}

function Entries({ id, timeline, onSeek }: { id: string; timeline: PlaytestTimeline | null; onSeek: (ms: number) => void }) {
  if (!timeline) return null;
  if (timeline.entries.length === 0) return <Text size="2" color="gray">Noch kein Transkript und keine Marker.</Text>;
  return (
    <Flex direction="column" gap="1" role="list" aria-label="Zeitleiste">
      {!timeline.hasTranscript && <Text size="2" color="gray">Noch kein Transkript, nur Marker.</Text>}
      {timeline.entries.map((e) => {
        const img = e.frame && e.nr ? frameUrl(id, e.nr) : null;
        return (
          <CardButton key={`${e.kind}-${e.ms}-${e.nr ?? 0}`} role="listitem" data-entry={e.kind} variant="ghost"
            onClick={() => onSeek(e.videoMs)} title="Im Player hierhin springen">
            <Flex gap="3" align="start">
              <Text size="2" color="gray" style={{ minWidth: 56, fontVariantNumeric: 'tabular-nums' }}>{clock(e.videoMs)}</Text>
              <Flex direction="column" gap="1" align="start">
                {e.kind === 'marker'
                  ? <StatusBadge tone="warn">{e.text}</StatusBadge>
                  : <Text size="2">{e.text}</Text>}
                {img && <img src={img} alt={`Bild zu Marker ${e.nr}`} loading="lazy" style={{ maxWidth: 240, borderRadius: 4 }} />}
              </Flex>
            </Flex>
          </CardButton>
        );
      })}
    </Flex>
  );
}
