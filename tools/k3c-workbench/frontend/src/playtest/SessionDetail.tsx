import { useRef } from 'react';
import { ActionButton, Flex, Heading, Text } from '@webkit/atoms';
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
  /** Zähler: steigt bei „Neu laden“, die Auswertung liest dann Modelle und Voraussetzungen neu. */
  refresh?: number;
}

/** Titel und die zwei Aufräum-Knöpfe. */
function SessionHead({ session: s, onClean }: Pick<Props, 'session' | 'onClean'>) {
  return (
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
  );
}

/** Dateien der Session mit Größe. */
function FilesTable({ files }: { files: PlaytestSession['files'] }) {
  return (
    <Table.Root size="1" variant="surface">
      <Table.Header>
        <Table.Row>
          <Table.ColumnHeaderCell>Datei</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell justify="end">Größe</Table.ColumnHeaderCell>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {files.map((f) => (
          <Table.Row key={f.name}>
            <Table.RowHeaderCell>{f.name}</Table.RowHeaderCell>
            <Table.Cell justify="end">{formatBytes(f.size)}</Table.Cell>
          </Table.Row>
        ))}
      </Table.Body>
    </Table.Root>
  );
}

/** Detail einer Session: Kennzahlen, Audio- und Video-Player, Dateien, die zwei Aufräum-Knöpfe. */
export function SessionDetail({ session: s, onClean, onChanged, refresh = 0 }: Props) {
  const videoEl = useRef<HTMLVideoElement>(null);
  const audioEl = useRef<HTMLAudioElement>(null);
  const seek = (ms: number) => {
    for (const el of [videoEl.current, audioEl.current]) if (el) el.currentTime = Math.max(0, ms) / 1000;
  };
  // video.mkv hat keinen Ton (der steckt in voice.wav): Der Ton läuft mit dem Bild, Start, Pause und Springen gelten für beide.
  const follow = (play: boolean) => {
    const v = videoEl.current;
    const a = audioEl.current;
    if (!v || !a) return;
    if (Math.abs(a.currentTime - v.currentTime) > 0.3) a.currentTime = v.currentTime;
    if (play) void a.play().catch(() => undefined);
    else a.pause();
  };
  const voice = mediaUrl(s.id, 'voice.wav');
  const video = mediaUrl(s.id, 'video.mkv');
  return (
    <Flex direction="column" gap="4" p="3" data-detail={s.id}>
      <SessionHead session={s} onClean={onClean} />
      {s.running && <Text size="2" color="gray">Die Aufnahme läuft, Löschen ist gesperrt.</Text>}
      {s.hasVideo && (
        <Flex direction="column" gap="1" data-player="video">
          <Text size="2" weight="bold">Bild (video.mkv, ohne Ton)</Text>
          <video ref={videoEl} controls preload="metadata" src={video} aria-label="Bildschirmaufnahme" style={{ width: '100%', maxHeight: '50vh' }}
            onPlay={() => follow(true)} onPause={() => follow(false)} onSeeked={() => follow(!videoEl.current?.paused)} />
        </Flex>
      )}
      {s.hasVoice && (
        <Flex direction="column" gap="1" data-player="voice">
          <Text size="2" weight="bold">Ton (voice.wav){s.hasVideo ? ', läuft mit dem Bild' : ''}</Text>
          <audio ref={audioEl} controls preload="metadata" src={voice} aria-label="Sprachaufnahme" style={{ width: '100%' }} />
        </Flex>
      )}
      <Text size="2" color="gray" data-facts>
        {[germanDate(s.date), sessionLength(s.durationMs), `${s.markers} Marker`, formatBytes(s.size), s.stage].join(' · ')}
      </Text>
      <TimelinePanel session={s} onSeek={seek} onDone={onChanged} refresh={refresh} />
      <FilesTable files={s.files} />
    </Flex>
  );
}

