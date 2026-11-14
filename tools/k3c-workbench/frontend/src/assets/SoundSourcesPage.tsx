import { useState } from 'react';
import { Flex } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import { loadPref, savePref } from '../lib/prefs';
import { SoundPage } from './sound/SoundPage';
import { SourcesPage } from './sources/SourcesPage';

const TEILE = ['sound', 'quellen'] as const;
type Teil = (typeof TEILE)[number];

/** Seite „Sound & Quellen“ im Bereich Studio (B-892): Sound-Slots und Quellen gehören zusammen und teilen sich eine
 *  Seite; der Umschalter oben wählt den Teil, gemerkt unter `studio.soundTeil`. */
export function SoundSourcesPage() {
  const [teil, setTeil] = useState<Teil>(() => loadPref('studio.soundTeil', TEILE, 'sound'));
  const waehle = (t: Teil) => {
    setTeil(t);
    savePref('studio.soundTeil', t);
  };
  return (
    <Flex direction="column">
      <Flex px="4" pt="3">
        <ChipSwitch label="Teil" options={[{ value: 'sound', label: 'Sound' }, { value: 'quellen', label: 'Quellen' }]} value={teil} onChange={waehle} />
      </Flex>
      {teil === 'sound' ? <SoundPage /> : <SourcesPage />}
    </Flex>
  );
}
