import { useCallback, useEffect, useState } from 'react';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { ActionButton, CardButton, Flex, Heading, StatusBadge, Text } from '@webkit/atoms';
import { LoadingNote, NoticeCard } from '@webkit/molecules';
import { Confirm, type ConfirmRequest } from '@webkit/organisms';
import { ListDetail } from '@webkit/templates';
import { formatBytes } from '../lib/format';
import { playtestApi } from './api';
import { clockTime, confirmTexts, germanDate, sessionLength, STAGE_TONE } from './format';
import { RecordBar } from './RecordBar';
import { SessionDetail } from './SessionDetail';
import type { PlaytestAction, PlaytestSession } from './types';

const LIST_POLL_MS = 5000;
const message = (e: unknown) => (e instanceof Error ? e.message : String(e));

/** Sessions der Liste: laden, bei Aktivität nachlesen (auch Änderungen von außen) und der letzte Fehler. */
function useSessions(active: boolean) {
  const [sessions, setSessions] = useState<PlaytestSession[] | null>(null);
  const [error, setError] = useState('');
  const reload = useCallback(async () => {
    try {
      setSessions(await playtestApi.list());
      setError('');
    } catch (e) {
      setError(message(e));
    }
  }, []);
  useEffect(() => {
    if (active) void reload();
  }, [active, reload]);
  // Befunde, Transkript und Bilder ändern sich auch von außen (Claude über MCP): die Liste liest sich still nach,
  // damit der Stand (ausgewertet) stimmt; nur bei einer Änderung rendert die Seite neu.
  useEffect(() => {
    if (!active) return undefined;
    const t = setInterval(() => void playtestApi.list().then((l) => setSessions((old) => (JSON.stringify(old) === JSON.stringify(l) ? old : l)), () => undefined), LIST_POLL_MS);
    return () => clearInterval(t);
  }, [active]);
  return { sessions, error, setError, reload };
}

interface HeaderProps {
  active: boolean;
  evaluated: number;
  onClean: (action: PlaytestAction, id: string) => Promise<void>;
  /** Stabile Funktion: die Aufnahmeleiste hängt ihren Status-Takt daran. */
  reload: () => void;
  onRefresh: () => void;
}

/** Kopf: Titel, Aufräumen der ausgewerteten Videos, Neu laden und die Aufnahmeleiste. */
function PageHeader({ active, evaluated, onClean, reload, onRefresh }: HeaderProps) {
  return (
    <>
      <Flex align="center" justify="between" gap="3" p="3" wrap="wrap">
        <Heading size="4">Spieltest</Heading>
        <Flex gap="2" align="center">
          <ActionButton variant="soft" color="red" disabled={evaluated === 0} onClick={() => onClean('ausgewertet', '')}>
            Videos ausgewerteter Sessions löschen
          </ActionButton>
          <ActionButton variant="soft" color="gray" onClick={onRefresh}>Neu laden</ActionButton>
        </Flex>
      </Flex>
      <RecordBar active={active} onChange={reload} />
    </>
  );
}

interface ListProps {
  sessions: PlaytestSession[] | null;
  error: string;
  currentId?: string;
  onSelect: (id: string) => void;
}

/** Liste der Sessions als Karten, davor Fehler- und Ladehinweis. */
function SessionList({ sessions, error, currentId, onSelect }: ListProps) {
  return (
    <Flex direction="column" gap="2" p="2">
      {error && <NoticeCard title="Spieltest" tone="error">{error}</NoticeCard>}
      {!sessions && !error && <LoadingNote id="list" text="Lade Sessions …" />}
      {sessions?.length === 0 && <Text size="2" color="gray">Noch keine Aufnahmen in playtests/.</Text>}
      {sessions?.map((s) => (
        <CardButton key={s.id} data-session={s.id} aria-pressed={s.id === currentId} onClick={() => onSelect(s.id)}>
          <Flex direction="column" gap="1" align="start">
            <Flex justify="between" align="center" gap="2" width="100%">
              <Text weight="bold">{germanDate(s.date)} · {s.startMs ? clockTime(s.startMs) : s.id.slice(11)}</Text>
              <StatusBadge tone={STAGE_TONE[s.stage]}>{s.stage}</StatusBadge>
            </Flex>
            <Text size="1" color="gray">{sessionLength(s.durationMs)} · {s.markers} Marker · {formatBytes(s.size)}</Text>
          </Flex>
        </CardButton>
      ))}
    </Flex>
  );
}

/** Seite `Spieltest` (B-1218): Sessions aus `playtests/` neueste zuerst, Player und Aufräumen. Jeder Löschknopf liest
 *  erst den Plan (was wird gelöscht) und löscht erst nach der Bestätigung. */
export function PlaytestPage({ active = true }: { active?: boolean }) {
  const { sessions, error, setError, reload } = useSessions(active);
  const [selected, setSelected] = useState('');
  const [confirm, setConfirm] = useState<ConfirmRequest | null>(null);
  const [refresh, setRefresh] = useState(0); // „Neu laden“ liest auch die Modelle und die Auswertung neu
  const current = sessions?.find((s) => s.id === selected) ?? sessions?.[0] ?? null;
  useTicketPage('spieltest', current?.id ?? '', active);

  const clean = async (action: PlaytestAction, id: string) => {
    try {
      const plan = await playtestApi.plan(action, id);
      if (plan.files.length === 0) {
        setError('Nichts zu löschen.');
        return;
      }
      const t = confirmTexts(action, sessions?.find((s) => s.id === id) ?? null, plan);
      setConfirm({ ...t, danger: true, run: () => void playtestApi.remove(action, id, true).then(reload, (e) => setError(message(e))) });
    } catch (e) {
      setError(message(e));
    }
  };

  const evaluated = sessions?.filter((s) => s.stage === 'ausgewertet' && s.hasVideo).length ?? 0;
  const header = <PageHeader active={active} evaluated={evaluated} onClean={clean} reload={reload} onRefresh={() => { setRefresh((n) => n + 1); void reload(); }} />;
  const list = <SessionList sessions={sessions} error={error} currentId={current?.id} onSelect={setSelected} />;
  return (
    <>
      <ListDetail header={header} list={list} listWidth={340}>
        {current ? <SessionDetail session={current} onClean={clean} onChanged={reload} refresh={refresh} /> : <Text size="2" color="gray" as="p" m="3">Keine Session gewählt.</Text>}
      </ListDetail>
      <Confirm request={confirm} onClose={() => setConfirm(null)} />
    </>
  );
}
