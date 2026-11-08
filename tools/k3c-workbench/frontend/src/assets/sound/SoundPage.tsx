import { useEffect, useState } from 'react';
import { ActionButton, Flex, Heading, StatusBadge, Text, type Tone } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend } from '../../api';
import type { SoundEintrag } from '../api';

const TONE: Record<SoundEintrag['status'], Tone> = { zugeordnet: 'ok', luecke: 'warn', fehlt: 'error' };
const LABEL: Record<SoundEintrag['status'], string> = { zugeordnet: 'zugeordnet', luecke: 'Lücke', fehlt: 'fehlt' };

/** Seite Sound im Bereich Assets (WB1.4, D-03): Sound-Slots aus data/sounds.json mit Status und Hörprobe. Zuordnen,
 *  Recherche und Import laufen über die MCP-Tools assets_*. */
export function SoundPage() {
  const [list, setList] = useState<SoundEintrag[]>([]);
  const [error, setError] = useState('');
  useEffect(() => {
    let live = true;
    backend.studioSounds().then((l) => live && setList(l), (e) => live && setError(errorText(e)));
    return () => {
      live = false;
    };
  }, []);
  const kategorien = [...new Set(list.map((s) => s.kategorie))];
  return (
    <Flex direction="column" gap="4" p="4">
      {error && <Text color="red">{error}</Text>}
      {kategorien.map((k) => (
        <Flex key={k} direction="column" gap="2">
          <Heading size="3">{k}</Heading>
          {list.filter((s) => s.kategorie === k).map((s) => <SoundRow key={s.id} sound={s} />)}
        </Flex>
      ))}
    </Flex>
  );
}

function SoundRow({ sound }: { sound: SoundEintrag }) {
  const [src, setSrc] = useState('');
  const [error, setError] = useState('');
  const listen = async (pfad: string) => {
    try {
      setSrc(await backend.studioSoundData(pfad));
      setError('');
    } catch (e) {
      setSrc('');
      setError(errorText(e));
    }
  };
  return (
    <Flex direction="column" gap="1" data-sound={sound.id} className="as-material">
      <Flex gap="2" align="center" wrap="wrap">
        <Text size="2" weight="bold">{sound.id}</Text>
        <Text size="1" color="gray">{sound.beschreibung}</Text>
        <StatusBadge tone={TONE[sound.status]}>{LABEL[sound.status]}</StatusBadge>
      </Flex>
      {sound.dateien.map((d) => (
        <Flex key={d} gap="2" align="center">
          <ActionButton size="1" variant="soft" onClick={() => listen(d)}>Anhören</ActionButton>
          <Text size="1">{d}</Text>
        </Flex>
      ))}
      {src && <audio controls autoPlay src={src} className="as-audio" />}
      {error && <Text size="1" color="red">{error}</Text>}
    </Flex>
  );
}
