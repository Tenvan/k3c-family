import { useCallback, useEffect, useState } from 'react';
import { ActionButton, Flex, SelectField, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { playtestApi } from './api';
import { loadText, savePref } from '../lib/prefs';
import type { VisionInfo, VisionStatus } from './types';

const message = (e: unknown) => (e instanceof Error ? e.message : String(e));
const NO_STATUS: VisionStatus = { running: false, id: '', done: 0, total: 0, error: '' };

export interface Describe {
  info: VisionInfo | null;
  model: string;
  pickModel: (v: string) => void;
  status: VisionStatus;
  busy: boolean;
  error: string;
  /** Beschreibt die Marker-Bilder nrs (leer: alle ausgewählten). */
  start: (nrs: number[]) => void;
  cancel: () => void;
}

/** Zustand der Bildbeschreibung (B-1241): Bildmodelle, Lauf mit Fortschritt; `onDone` lädt die Zeitleiste neu. */
export function useDescribe(sessionId: string, refresh: number, onDone: () => void): Describe {
  const [info, setInfo] = useState<VisionInfo | null>(null);
  const [model, setModel] = useState(() => loadText('playtest.vision', ''));
  const [status, setStatus] = useState(NO_STATUS);
  const [error, setError] = useState('');
  const busy = status.running;

  useEffect(() => {
    playtestApi.visionInfo().then(setInfo, (e) => setError(message(e)));
    playtestApi.describeStatus().then(setStatus, () => undefined);
  }, [refresh]);
  useEffect(() => {
    if (!busy) return undefined;
    const t = setInterval(() => void playtestApi.describeStatus().then((st) => {
      setStatus(st);
      onDone();
    }, () => undefined), 1500);
    return () => clearInterval(t);
  }, [busy, onDone]);

  const start = useCallback((nrs: number[]) => {
    setError('');
    const chosen = info?.models.find((m) => m.id === model) ?? info?.models[0];
    playtestApi.describe(sessionId, chosen?.id ?? '', nrs).then((st) => { setStatus(st); onDone(); }, (e) => setError(message(e)));
  }, [sessionId, info, model, onDone]);
  const cancel = () => void playtestApi.describeCancel().catch((e) => setError(message(e)));
  const pickModel = (v: string) => { setModel(v); savePref('playtest.vision', v); };
  return { info, model, pickModel, status, busy, error, start, cancel };
}

/** Leiste über der Zeitleiste: Bildmodell wählen und die ausgewählten Marker-Bilder beschreiben lassen. */
export function DescribeBar({ d, sessionId }: { d: Describe; sessionId: string }) {
  const { info } = d;
  if (!info) return null;
  const mine = d.status.id === sessionId;
  const chosen = info.models.find((m) => m.id === d.model) ?? info.models[0];
  return (
    <Flex direction="column" gap="2" data-describe>
      <Flex align="center" justify="between" gap="3" wrap="wrap">
        <Text size="2" weight="bold">Bildbeschreibung</Text>
        <Flex gap="2" align="center">
          {d.busy && mine && <Text size="2" role="status">{d.status.done}/{d.status.total}</Text>}
          {info.models.length > 0 && (
            <SelectField aria-label="Bildmodell" value={chosen?.id ?? ''} disabled={d.busy}
              options={info.models.map((m) => ({ value: m.id, label: `${m.name} (${m.sizeMb} MB)` }))} onChange={d.pickModel} />
          )}
          {d.busy && mine
            ? <ActionButton variant="soft" color="red" onClick={d.cancel}>Abbrechen</ActionButton>
            : <ActionButton variant="soft" disabled={!info.ready || d.busy} onClick={() => d.start([])}
                title="Beschreibt alle ausgewählten Marker-Bilder dieser Session lokal, damit der Chat sie nicht ansehen muss">Markierte Bilder beschreiben</ActionButton>}
        </Flex>
      </Flex>
      {!info.ready && <NoticeCard title="Beschreiben gesperrt" tone="warn">{info.message}</NoticeCard>}
      {mine && !d.busy && d.status.error && <NoticeCard title="Bildbeschreibung" tone="error">{d.status.error}</NoticeCard>}
      {d.error && <NoticeCard title="Bildbeschreibung" tone="error">{d.error}</NoticeCard>}
    </Flex>
  );
}
