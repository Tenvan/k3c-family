import { ActionButton, Flex, Heading, Text } from '@webkit/atoms';
import { DataList } from '@webkit/molecules';
import { Table } from '@webkit/organisms';
import { formatBytes } from '../lib/format';
import { mediaUrl } from './api';
import { germanDate, sessionLength } from './format';
import type { PlaytestAction, PlaytestSession } from './types';

interface Props {
  session: PlaytestSession;
  /** Startet die Aufräum-Aktion (Plan lesen, Dialog, Bestätigung) – die Seite hält den Dialog. */
  onClean: (action: PlaytestAction, id: string) => Promise<void>;
}

/** Detail einer Session: Kennzahlen, Audio- und Video-Player, Dateien, die zwei Aufräum-Knöpfe. */
export function SessionDetail({ session: s, onClean }: Props) {
  const voice = s.hasVoice ? mediaUrl(s.id, 'voice.wav') : null;
  const video = s.hasVideo ? mediaUrl(s.id, 'video.mkv') : null;
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
      <DataList.Root size="2">
        <Item label="Datum">{germanDate(s.date)}</Item>
        <Item label="Dauer">{sessionLength(s.durationMs)}</Item>
        <Item label="Marker">{s.markers}</Item>
        <Item label="Größe">{formatBytes(s.size)}</Item>
        <Item label="Stand">{s.stage}</Item>
      </DataList.Root>
      {s.hasVoice && (voice ? <audio controls preload="none" src={voice} aria-label="Sprachaufnahme" style={{ width: '100%' }} />
        : <Text size="2" color="gray">Audio-Player nur im Browser-Betrieb (Client/Server), nicht im Fenster.</Text>)}
      {s.hasVideo && video && <video controls preload="none" src={video} aria-label="Bildschirmaufnahme" style={{ width: '100%', maxHeight: '50vh' }} />}
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
