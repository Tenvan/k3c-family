import { useRef } from 'react';
import { ActionButton, Flex, Heading, Text } from '@webkit/atoms';
import { DataList } from '@webkit/molecules';
import { Table } from '@webkit/organisms';
import { formatBytes } from '../lib/format';
import { mediaUrl } from './api';
import { germanDate, sessionLength } from './format';
import { TimelinePanel } from './TimelinePanel';
import type { PlaytestAction, PlaytestSession } from './types';

interface Props {
  session: PlaytestSession;
  /** Startet die Aufräum-Aktion (Plan lesen, Dialog, Bestätigung) – die Seite hält den Dialog. */
  onClean: (action: PlaytestAction, id: string) => Promise<void>;
  /** Die Auswertung hat den Stand der Session geändert (Liste neu laden). */
  onChanged: () => void;
}

/** Detail einer Session: Kennzahlen, Audio- und Video-Player, Dateien, die zwei Aufräum-Knöpfe. */
export function SessionDetail({ session: s, onClean, onChanged }: Props) {
  const videoEl = useRef<HTMLVideoElement>(null);
  const audioEl = useRef<HTMLAudioElement>(null);
  const seek = (ms: number) => {
    const el = videoEl.current ?? audioEl.current;
    if (el) el.currentTime = Math.max(0, ms) / 1000;
  };
  const voice = mediaUrl(s.id, 'voice.wav');
  const video = mediaUrl(s.id, 'video.mkv');
  return (
    <Flex direction="column" gap="4" p="3" data-detail={s.id}>
      <Flex align="center" justify="between" gap="3" wrap="wrap">
        <Heading size="4">Session {s.id}</Heading>
        <Flex gap="2">
          <ActionButton variant="soft" color="red" disabled={s.running || !s.hasVideo} onClick={() => onClean('video', s.id)}>
            Video löschen
          </ActionButton>
          <ActionButton variant="soft" color="red" disabled={s.running} onClick={() => onClean('session', s.id)}>
            Session löschen
          </ActionButton>
        </Flex>
      </Flex>
      {s.running && <Text size="2" color="gray">Die Aufnahme läuft, Löschen ist gesperrt.</Text>}
      {s.hasVoice && (
        <Flex direction="column" gap="1" data-player="voice">
          <Text size="2" weight="bold">Ton (voice.wav)</Text>
          <audio ref={audioEl} controls preload="metadata" src={voice} aria-label="Sprachaufnahme" style={{ width: '100%' }} />
        </Flex>
      )}
      {s.hasVideo && (
        <Flex direction="column" gap="1" data-player="video">
          <Text size="2" weight="bold">Bild (video.mkv)</Text>
          <video ref={videoEl} controls preload="metadata" src={video} aria-label="Bildschirmaufnahme" style={{ width: '100%', maxHeight: '50vh' }} />
        </Flex>
      )}
      <DataList.Root size="2">
        <Item label="Datum">{germanDate(s.date)}</Item>
        <Item label="Dauer">{sessionLength(s.durationMs)}</Item>
        <Item label="Marker">{s.markers}</Item>
        <Item label="Größe">{formatBytes(s.size)}</Item>
        <Item label="Stand">{s.stage}</Item>
      </DataList.Root>
      <TimelinePanel session={s} onSeek={seek} onDone={onChanged} />
      <Table.Root size="1" variant="surface">
        <Table.Header>
          <Table.Row>
            <Table.ColumnHeaderCell>Datei</Table.ColumnHeaderCell>
            <Table.ColumnHeaderCell justify="end">Größe</Table.ColumnHeaderCell>
          </Table.Row>
        </Table.Header>
        <Table.Body>
          {s.files.map((f) => (
            <Table.Row key={f.name}>
              <Table.RowHeaderCell>{f.name}</Table.RowHeaderCell>
              <Table.Cell justify="end">{formatBytes(f.size)}</Table.Cell>
            </Table.Row>
          ))}
        </Table.Body>
      </Table.Root>
    </Flex>
  );
}

function Item({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <DataList.Item>
      <DataList.Label minWidth="88px">{label}</DataList.Label>
      <DataList.Value>{children}</DataList.Value>
    </DataList.Item>
  );
}
