import { useCallback, useEffect, useState } from 'react';
import { Flex, Heading, SelectField, StatusBadge, Text, TextField } from '@webkit/atoms';
import { CheckField, NoticeCard } from '@webkit/molecules';
import { CopyPrompt } from '../planning/PromptParts';
import { frameUrl, playtestApi } from './api';
import { clock } from './RecordBar';
import { createTicketsPrompt, evaluatePrompt } from './prompts';
import type { PlaytestFinding, PlaytestFindingPatch } from './types';

const message = (e: unknown) => (e instanceof Error ? e.message : String(e));
const TYPES = ['Idee', 'Problem', 'Schuld', 'Frage'];
const PRIOS = ['hoch', 'mittel', 'niedrig', '?'];
const POLL_MS = 3000;

interface Props {
  sessionId: string;
  hasTranscript: boolean;
  /** Unix-ms bei 0 s Videozeit (für den Sprung vom Zeitpunkt des Befunds). */
  captureMs: number;
  onSeek: (videoMs: number) => void;
}

/** Befunde (B-1224): Claude schlägt sie per MCP vor, Ralf hakt an und ändert Typ, Prio, Projekt und Sprint; die
 *  Prompt-Knöpfe geben Claude den Auftrag Auswerten bzw. Tickets anlegen. Die Liste lädt sich selbst nach, wenn
 *  findings.json sich ändert. */
export function FindingsPanel({ sessionId, hasTranscript, captureMs, onSeek }: Props) {
  const [list, setList] = useState<PlaytestFinding[]>([]);
  const [error, setError] = useState('');
  const load = useCallback(() => {
    playtestApi.findings(sessionId).then((l) => setList(l), (e) => setError(message(e)));
  }, [sessionId]);
  useEffect(() => {
    setList([]);
    load();
    const t = setInterval(load, POLL_MS);
    return () => clearInterval(t);
  }, [load]);

  const change = (f: PlaytestFinding, patch: PlaytestFindingPatch) => {
    setError('');
    playtestApi.updateFinding(sessionId, f.id, patch).then(load, (e) => setError(message(e)));
  };
  const ready = list.some((f) => f.selected && !f.ticket);
  return (
    <Flex direction="column" gap="2" data-findings={sessionId}>
      <Flex align="center" justify="between" gap="3" wrap="wrap">
        <Heading size="3">Befunde ({list.length})</Heading>
        <Flex gap="3" align="center">
          <Text size="1" color="gray">Prompt:</Text>
          {hasTranscript
            ? <CopyPrompt prompt={evaluatePrompt(sessionId)} what="Auswerten" />
            : <Text size="1" color="gray">Auswerten erst nach dem Transkript</Text>}
          {ready && <CopyPrompt prompt={createTicketsPrompt(sessionId)} what="Tickets anlegen" />}
        </Flex>
      </Flex>
      {error && <NoticeCard title="Befunde" tone="error">{error}</NoticeCard>}
      {list.length === 0 && <Text size="2" color="gray">Noch keine Befunde: Auswerten-Prompt an Claude geben.</Text>}
      {list.map((f) => <Row key={f.id} sessionId={sessionId} f={f} captureMs={captureMs} onSeek={onSeek} onChange={(p) => change(f, p)} />)}
    </Flex>
  );
}

interface RowProps {
  sessionId: string;
  f: PlaytestFinding;
  captureMs: number;
  onSeek: (videoMs: number) => void;
  onChange: (p: PlaytestFindingPatch) => void;
}

function Row({ sessionId, f, captureMs, onSeek, onChange }: RowProps) {
  const nr = /marker-(\d+)\.jpg$/.exec(f.frame ?? '')?.[1];
  const img = nr ? frameUrl(sessionId, Number(nr)) : null;
  const locked = Boolean(f.ticket);
  return (
    <Flex direction="column" gap="1" p="2" data-finding={f.id} style={{ border: '1px solid var(--gray-a5)', borderRadius: 6 }}>
      <Flex gap="3" align="start" wrap="wrap">
        <CheckField label={`„${f.quote}“`} checked={f.selected} disabled={locked}
          onCheckedChange={(c) => onChange({ selected: c === true })} />
        <Text size="1" color="gray" role="button" tabIndex={0} style={{ cursor: 'pointer' }} title="Im Player hierhin springen"
          onClick={() => onSeek(f.ts - captureMs)} onKeyDown={(e) => e.key === 'Enter' && onSeek(f.ts - captureMs)}>
          {clock(f.ts - captureMs)}
        </Text>
        {f.ticket && <StatusBadge tone="ok">{f.ticket}</StatusBadge>}
        {f.duplicate_of && <StatusBadge tone="warn">Duplikat von {f.duplicate_of}</StatusBadge>}
      </Flex>
      {f.context && <Text size="1" color="gray">{f.context}</Text>}
      {f.error && <Text size="1" color="red">{f.error}</Text>}
      {img && <img src={img} alt={`Bild zu Befund ${f.id}`} loading="lazy" style={{ maxWidth: 240, borderRadius: 4 }} />}
      <Flex gap="2" wrap="wrap" align="center">
        <SelectField aria-label="Typ" placeholder="Typ" value={f.type ?? ''} disabled={locked}
          options={TYPES.map((v) => ({ value: v, label: v }))} onChange={(v) => onChange({ type: v })} />
        <SelectField aria-label="Prio" placeholder="Prio" value={f.prio ?? ''} disabled={locked}
          options={PRIOS.map((v) => ({ value: v, label: v }))} onChange={(v) => onChange({ prio: v })} />
        <TextField.Root size="1" aria-label="Projekt" placeholder="Projekt" defaultValue={f.project ?? ''} disabled={locked} style={{ width: 90 }}
          key={`p${f.project ?? ''}`} onBlur={(e) => e.target.value !== (f.project ?? '') && onChange({ project: e.target.value })} />
        <TextField.Root size="1" aria-label="Sprint" placeholder="Sprint" defaultValue={f.sprint ?? ''} disabled={locked} style={{ width: 90 }}
          key={`s${f.sprint ?? ''}`} onBlur={(e) => e.target.value !== (f.sprint ?? '') && onChange({ sprint: e.target.value })} />
      </Flex>
    </Flex>
  );
}
