import { useEffect, useState } from 'react';
import { ActionButton, Button, Flex, StatusBadge, Text, type Tone } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import type { ListFilterConfig } from '@webkit/lib/listFilter';
import { ListFilter } from '@webkit/molecules';
import { backend } from '../../api';
import type { SoundEintrag } from '../api';
import { PromptButtons } from '../prompts/PromptButtons';
import { useVorlagen, type VorlagenJeOrt } from '../prompts/useVorlagen';

const TONE: Record<SoundEintrag['status'], Tone> = { zugeordnet: 'ok', luecke: 'warn', fehlt: 'error' };
const LABEL: Record<SoundEintrag['status'], string> = { zugeordnet: 'zugeordnet', luecke: 'Lücke', fehlt: 'fehlt' };

/** Filter nach Text, Kategorie und Status (B-728); gruppiert nach Kategorie wie bisher. */
const FILTER: ListFilterConfig<SoundEintrag> = {
  searchText: (s) => `${s.id} ${s.beschreibung} ${s.kategorie} ${s.dateien.join(' ')}`,
  dimensions: [
    { key: 'kategorie', label: 'Kategorie', values: (s) => s.kategorie },
    { key: 'status', label: 'Status', values: (s) => s.status, valueLabel: (v) => LABEL[v as SoundEintrag['status']] ?? v, order: ['zugeordnet', 'luecke', 'fehlt'] },
  ],
  sorts: [{ key: 'id', label: 'Slot', compare: (a, b) => a.id.localeCompare(b.id) }],
};

/** Seite Sound im Bereich Assets (WB1.4, D-03): Sound-Slots aus data/sounds.json mit Status und Hörprobe, Filter und je
 *  Slot Prompt-Buttons (B-728). Zuordnen, Recherche und Import laufen über die MCP-Tools assets_*. */
export function SoundPage() {
  const [list, setList] = useState<SoundEintrag[]>([]);
  const [error, setError] = useState('');
  const [wahl, setWahl] = useState('');
  const vorlagen = useVorlagen();
  useEffect(() => {
    let live = true;
    backend.studioSounds().then((l) => live && setList(l), (e) => live && setError(errorText(e)));
    return () => {
      live = false;
    };
  }, []);
  return (
    <Flex direction="column" gap="4" p="4">
      {error && <Text color="red">{error}</Text>}
      {vorlagen.error && <Text color="red">{vorlagen.error}</Text>}
      <ListFilter {...FILTER} items={list} defaultGroup="kategorie" emptyHint="Keine Sounds" itemKey={(s) => s.id}
        renderItem={(s) => <SoundRow sound={s} an={wahl === s.id} waehle={() => setWahl(wahl === s.id ? '' : s.id)} jeOrt={vorlagen.jeOrt} />} />
    </Flex>
  );
}

interface RowProps {
  sound: SoundEintrag;
  /** Gewählter Slot zeigt seine Prompt-Buttons. */
  an: boolean;
  waehle: () => void;
  jeOrt: VorlagenJeOrt;
}

function SoundRow({ sound, an, waehle, jeOrt }: RowProps) {
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
        <Button size="1" variant={an ? 'solid' : 'ghost'} aria-pressed={an} data-wahl="sound" onClick={waehle}>{sound.id}</Button>
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
      {an && (
        <PromptButtons caption={`Slot: ${sound.id}`} vorlagen={jeOrt('sound')} leer="Keine Prompt-Vorlagen für Sounds"
          werte={{ slot: sound.id, datei: sound.dateien.join(', ') || 'keine Datei' }} />
      )}
    </Flex>
  );
}
