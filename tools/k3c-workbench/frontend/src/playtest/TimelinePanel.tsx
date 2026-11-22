import { useCallback, useEffect, useState } from 'react';
import { ActionButton, CardButton, Checkbox, Flex, Heading, SelectField, StatusBadge, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { DescribeBar, useDescribe } from './DescribeBar';
import { FindingsPanel } from './FindingsPanel';
import { frameUrl, playtestApi } from './api';
import { statsLines } from './format';
import { clock } from './RecordBar';
import { loadText, savePref } from '../lib/prefs';
import type { PlaytestCapability, PlaytestJob, PlaytestModel, PlaytestSession, PlaytestTimeline } from './types';

const message = (e: unknown) => (e instanceof Error ? e.message : String(e));
const LANGS = ['de', 'en', 'auto'];
const NO_JOB: PlaytestJob = { running: false, id: '', step: '', done: 0, total: 0, message: '', error: '' };

interface Props {
  session: PlaytestSession;
  /** Springt im Player an die Videozeit (ms). */
  onSeek: (videoMs: number) => void;
  /** Die Auswertung ist zu Ende (Liste neu laden). */
  onDone: () => void;
  /** Zähler: bei Änderung werden Modelle und Voraussetzungen neu gelesen (neue Modelldatei in models/). */
  refresh?: number;
}

/** Auswertung (B-1223): Knopf Auswerten (Transkript aus voice.wav, Bild je Marker) mit Fortschritt und Abbrechen, dazu
 *  die Zeitleiste aus Sätzen, Markern und deren Bildern; ein Klick springt im Player an die Stelle. */
export function TimelinePanel({ session: s, onSeek, onDone, refresh = 0 }: Props) {
  const [cap, setCap] = useState<PlaytestCapability | null>(null);
  const [job, setJob] = useState(NO_JOB);
  const [timeline, setTimeline] = useState<PlaytestTimeline | null>(null);
  const [error, setError] = useState('');
  const [models, setModels] = useState<PlaytestModel[]>([]);
  const [model, setModel] = useState(() => loadText('playtest.model', ''));
  const [lang, setLang] = useState(() => loadText('playtest.language', 'de'));
  const mine = job.id === s.id;
  const busy = job.running;

  const loadTimeline = useCallback(() => {
    if (s.running) return;
    playtestApi.timeline(s.id).then(setTimeline, (e) => setError(message(e)));
  }, [s.id, s.running]);

  useEffect(() => {
    playtestApi.check().then(setCap, (e) => setError(message(e)));
    playtestApi.transcribeStatus().then(setJob, () => undefined);
  }, [refresh]);
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

  useEffect(() => {
    playtestApi.models().then(setModels, () => undefined);
  }, [refresh]);
  const chosen = models.find((m) => m.id === model) ?? models.find((m) => m.default) ?? models[0];
  const pickModel = (v: string) => { setModel(v); savePref('playtest.model', v); };
  const pickLang = (v: string) => { setLang(v); savePref('playtest.language', v); };

  const start = async () => {
    setError('');
    try {
      setJob(await playtestApi.transcribe(s.id, { model: chosen?.id ?? '', language: lang }));
      onDone();
      loadTimeline();
    } catch (e) {
      setError(message(e));
    }
  };
  const describe = useDescribe(s.id, refresh, loadTimeline);
  const toggleSkip = (nr: number, skip: boolean) =>
    void playtestApi.skipFrame(s.id, nr, skip).then(() => loadTimeline(), (e) => setError(message(e)));
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
          {models.length > 0 && (
            <SelectField aria-label="Whisper-Modell" value={chosen?.id ?? ''} disabled={busy}
              options={models.map((m) => ({ value: m.id, label: `${m.name} (${m.sizeMb} MB)` }))} onChange={pickModel} />
          )}
          <SelectField aria-label="Sprache" value={LANGS.includes(lang) ? lang : 'de'} disabled={busy}
            options={LANGS.map((v) => ({ value: v, label: v }))} onChange={pickLang} />
          {busy && mine
            ?<ActionButton variant="soft" color="red" onClick={cancel}>Abbrechen</ActionButton>
            : <ActionButton disabled={blocked} onClick={start}>Auswerten</ActionButton>}
        </Flex>
      </Flex>
      {cap && !cap.ready && <NoticeCard title="Auswerten gesperrt" tone="warn">{cap.message}</NoticeCard>}
      {mine && !busy && job.error && <NoticeCard title="Auswertung" tone="error">{job.error}</NoticeCard>}
      {mine && !busy && !job.error && job.message && <Text size="2" color="gray">{job.message}</Text>}
      {error && <NoticeCard title="Auswertung" tone="error">{error}</NoticeCard>}
      {timeline?.stats && (
        <Text size="2" color="gray" data-stats>{statsLines(timeline.stats).join(' · ')}</Text>
      )}
      {timeline?.entries.some((e) => e.frame) && <DescribeBar d={describe} sessionId={s.id} />}
      <Entries id={s.id} timeline={timeline} onSeek={onSeek} onSkipFrame={toggleSkip} onDescribe={(nr) => describe.start([nr])} describeBusy={describe.busy} describeReady={describe.info?.ready ?? false} />
      {!s.running && <FindingsPanel sessionId={s.id} hasTranscript={timeline?.hasTranscript ?? false} captureMs={timeline?.captureMs ?? s.startMs} onSeek={onSeek} />}
    </Flex>
  );
}

/** Vorschau eines Marker-Bilds: klein in der Zeile, beim Daraufzeigen groß darunter. */
function Thumb({ src, alt, dim }: { src: string; alt: string; dim: boolean }) {
  const [hover, setHover] = useState(false);
  return (
    <span style={{ position: 'relative', display: 'inline-block', lineHeight: 0 }} onMouseEnter={() => setHover(true)} onMouseLeave={() => setHover(false)}>
      <img src={src} alt={alt} loading="lazy" style={{ height: 44, borderRadius: 3, opacity: dim ? 0.3 : 1 }} />
      {hover && (
        <img src={src} alt="" data-preview style={{ position: 'absolute', left: 0, top: '100%', marginTop: 4, width: 560, maxWidth: '80vw', zIndex: 50,
          borderRadius: 6, boxShadow: '0 8px 24px rgba(0,0,0,0.5)', pointerEvents: 'none' }} />
      )}
    </span>
  );
}

interface EntriesProps {
  id: string;
  timeline: PlaytestTimeline | null;
  onSeek: (ms: number) => void;
  onSkipFrame: (nr: number, skip: boolean) => void;
  onDescribe: (nr: number) => void;
  describeBusy: boolean;
  describeReady: boolean;
}

function Entries({ id, timeline, onSeek, onSkipFrame, onDescribe, describeBusy, describeReady }: EntriesProps) {
  if (!timeline) return null;
  if (timeline.entries.length === 0) return <Text size="2" color="gray">Noch kein Transkript und keine Marker.</Text>;
  return (
    <Flex direction="column" role="list" aria-label="Zeitleiste">
      {!timeline.hasTranscript && <Text size="2" color="gray">Noch kein Transkript, nur Marker.</Text>}
      {timeline.entries.map((e) => {
        const img = e.frame && e.nr ? frameUrl(id, e.nr) : null;
        const pick = !!img && !!e.nr; // Zeile mit Bild: Klick schaltet den Haken, die Zeit springt im Player
        return (
          <Flex key={`${e.kind}-${e.ms}-${e.nr ?? 0}`} role="listitem" align="center" gap="1">
          <CardButton data-entry={e.kind} variant="ghost" style={{ flex: 1 }}
            onClick={() => (pick ? onSkipFrame(e.nr!, !e.skip) : onSeek(e.videoMs))}
            title={pick ? 'Klick wählt das Bild ab oder an' : 'Im Player hierhin springen'}>
            <Flex gap="3" align="center">
              {pick ? (
                <Text size="2" color="blue" data-seek style={{ minWidth: 56, fontVariantNumeric: 'tabular-nums', textDecoration: 'underline' }}
                  title="Im Player hierhin springen" onClick={(ev) => { ev.stopPropagation(); onSeek(e.videoMs); }}>{clock(e.videoMs)}</Text>
              ) : (
                <Text size="2" color="gray" style={{ minWidth: 56, fontVariantNumeric: 'tabular-nums' }}>{clock(e.videoMs)}</Text>
              )}
              <Flex direction="column" gap="1" align="start">
              <Flex gap="2" align="center" wrap="wrap">
                {e.kind === 'marker'
                  ? <StatusBadge tone="warn">{e.text}</StatusBadge>
                  : <Text size="2">{e.text}</Text>}
                {img && <Thumb src={img} alt={`Bild zu Marker ${e.nr}`} dim={!!e.skip} />}
              </Flex>
              {e.desc && <Text size="1" color="gray" data-desc style={{ whiteSpace: 'normal', textAlign: 'left' }}>{e.desc}</Text>}
              </Flex>
            </Flex>
          </CardButton>
          {img && e.nr && (
            <Checkbox size="1" checked={!e.skip} aria-label={`Bild von Marker ${e.nr} verwenden`}
              title="Markierte Bilder kommen in den Prompt und werden beschrieben; Haken weg: Bild ausblenden (spart Tokens)"
              onCheckedChange={(v) => onSkipFrame(e.nr!, v !== true)} />
          )}
          {img && e.nr && !e.skip && (
            <ActionButton size="1" variant="ghost" disabled={describeBusy || !describeReady} data-describe-one
              title="Dieses Bild mit dem Bildmodell beschreiben" onClick={() => onDescribe(e.nr!)}>Beschreiben</ActionButton>
          )}
          </Flex>
        );
      })}
    </Flex>
  );
}
