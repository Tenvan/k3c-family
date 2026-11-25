import { useState } from 'react';
import { ActionButton, Flex, Heading, SelectField, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { DescribeBar, useDescribe, type Describe } from './DescribeBar';
import { FindingsPanel } from './FindingsPanel';
import { statsLines } from './format';
import { Entries } from './TimelineEntries';
import { LANGS, useEvalSettings, useTimeline, useTranscribe } from './useTimeline';
import type { PlaytestSession, PlaytestTimeline } from './types';

interface Props {
  session: PlaytestSession;
  /** Springt im Player an die Videozeit (ms). */
  onSeek: (videoMs: number) => void;
  /** Die Auswertung ist zu Ende (Liste neu laden). */
  onDone: () => void;
  /** Zähler: bei Änderung werden Modelle und Voraussetzungen neu gelesen (neue Modelldatei in models/). */
  refresh?: number;
}

type Transcribe = ReturnType<typeof useTranscribe>;
type Settings = ReturnType<typeof useEvalSettings>;

/** Der Auswerten-Knopf ist gesperrt ohne Sprachspur, bei laufender Aufnahme, fremdem Lauf oder fehlenden Voraussetzungen. */
const evalBlocked = (s: PlaytestSession, tr: Transcribe) =>
  !s.hasVoice || s.running || (tr.busy && !tr.mine) || tr.cap?.ready === false;

/** Fortschritt, Modell, Sprache und Knopf (Auswerten bzw. Abbrechen). */
function EvalControls({ s, tr, settings }: { s: PlaytestSession; tr: Transcribe; settings: Settings }) {
  const { job, busy } = tr;
  const { models, chosen, lang } = settings;
  const running = busy && tr.mine;
  return (
    <Flex gap="2" align="center">
      {running && (
        <Text size="2" role="status">
          {job.step}{job.total > 0 ? ` ${job.done}/${job.total}` : ' …'}
        </Text>
      )}
      {models.length > 0 && (
        <SelectField aria-label="Whisper-Modell" value={chosen?.id ?? ''} disabled={busy}
          options={models.map((m) => ({ value: m.id, label: `${m.name} (${m.sizeMb} MB)` }))} onChange={settings.pickModel} />
      )}
      <SelectField aria-label="Sprache" value={LANGS.includes(lang) ? lang : 'de'} disabled={busy}
        options={LANGS.map((v) => ({ value: v, label: v }))} onChange={settings.pickLang} />
      {running
        ? <ActionButton variant="soft" color="red" onClick={tr.cancel}>Abbrechen</ActionButton>
        : <ActionButton disabled={evalBlocked(s, tr)} onClick={() => void tr.start(chosen?.id ?? '', lang)}>Auswerten</ActionButton>}
    </Flex>
  );
}

/** Hinweise zur Auswertung: gesperrt, Fehler oder Meldung des letzten Laufs, letzter Fehler der Seite. */
function EvalNotices({ tr, error }: { tr: Transcribe; error: string }) {
  const { cap, job } = tr;
  const idle = tr.mine && !tr.busy;
  return (
    <>
      {cap && !cap.ready && <NoticeCard title="Auswerten gesperrt" tone="warn">{cap.message}</NoticeCard>}
      {idle && job.error && <NoticeCard title="Auswertung" tone="error">{job.error}</NoticeCard>}
      {idle && !job.error && job.message && <Text size="2" color="gray">{job.message}</Text>}
      {error && <NoticeCard title="Auswertung" tone="error">{error}</NoticeCard>}
    </>
  );
}

interface FrameToolsProps {
  timeline: PlaytestTimeline | null;
  describe: Describe;
  sessionId: string;
  setAll: (skip: boolean) => Promise<void>;
}

/** Bildbeschreibung und „Alle an- oder abwählen“; nur sichtbar, wenn die Zeitleiste Marker-Bilder hat. */
function FrameTools({ timeline, describe, sessionId, setAll }: FrameToolsProps) {
  if (!timeline?.entries.some((e) => e.frame)) return null;
  return (
    <>
      <DescribeBar d={describe} sessionId={sessionId} />
      <Flex gap="2" align="center" data-pick-all>
        <Text size="1" color="gray">Bilder:</Text>
        <ActionButton size="1" variant="ghost" onClick={() => void setAll(false)}>Alle anwählen</ActionButton>
        <ActionButton size="1" variant="ghost" onClick={() => void setAll(true)}>Alle abwählen</ActionButton>
      </Flex>
    </>
  );
}

/** Auswertung (B-1223): Knopf Auswerten (Transkript aus voice.wav, Bild je Marker) mit Fortschritt und Abbrechen, dazu
 *  die Zeitleiste aus Sätzen, Markern und deren Bildern; ein Klick springt im Player an die Stelle. */
export function TimelinePanel({ session: s, onSeek, onDone, refresh = 0 }: Props) {
  const [error, setError] = useState('');
  const { timeline, loadTimeline, setAll, toggleSkip } = useTimeline(s, setError);
  const tr = useTranscribe(s, refresh, onDone, loadTimeline, setError);
  const settings = useEvalSettings(refresh);
  const describe = useDescribe(s.id, refresh, loadTimeline);
  return (
    <Flex direction="column" gap="2" data-timeline={s.id}>
      <Flex align="center" justify="between" gap="3" wrap="wrap">
        <Heading size="3">Auswertung</Heading>
        <EvalControls s={s} tr={tr} settings={settings} />
      </Flex>
      <EvalNotices tr={tr} error={error} />
      {timeline?.stats && (
        <Text size="2" color="gray" data-stats>{statsLines(timeline.stats).join(' · ')}</Text>
      )}
      <FrameTools timeline={timeline} describe={describe} sessionId={s.id} setAll={setAll} />
      <Entries id={s.id} timeline={timeline} onSeek={onSeek} onSkipFrame={toggleSkip} onDescribe={(nr) => describe.start([nr])}
        describeBusy={describe.busy} describeReady={describe.info?.ready ?? false} />
      {!s.running && <FindingsPanel sessionId={s.id} hasTranscript={timeline?.hasTranscript ?? false} captureMs={timeline?.captureMs ?? s.startMs} onSeek={onSeek} />}
    </Flex>
  );
}
