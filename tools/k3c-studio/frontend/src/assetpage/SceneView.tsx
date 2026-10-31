import { Flex, SegmentedControl } from '@webkit/atoms';
import { useState } from 'react';
import { ScenesView } from '../graphics/ScenesView';
import { loadPref, savePref } from '../lib/prefs';
import { SoundScenes } from '../sound/SoundScenes';
import '../styles/assetTools.css';

// Sicht „Szene“ der Assets-Seite (B-677): die Mini-Szenen im Spielmaßstab. Bei Fokus Grafik oder Sound ist die Art
// vorgegeben; bei Fokus Alles wählt ein Schalter und merkt sich die Art.

type Art = 'grafik' | 'sound';
const ARTEN: readonly Art[] = ['grafik', 'sound'];
const KEY = 'assets.szenenart';

export function SceneView({ art }: { art: '' | Art }) {
  const [own, setOwn] = useState<Art>(() => loadPref(KEY, ARTEN, 'grafik'));
  const shown = art || own;
  const pick = (v: string) => { setOwn(v as Art); savePref(KEY, v); };
  return (
    <Flex direction="column" gap="2">
      {art === '' && (
        <SegmentedControl.Root className="ap-art" value={own} onValueChange={pick} aria-label="Szenenart">
          <SegmentedControl.Item value="grafik">Grafik</SegmentedControl.Item>
          <SegmentedControl.Item value="sound">Sound</SegmentedControl.Item>
        </SegmentedControl.Root>
      )}
      {shown === 'grafik' ? <ScenesView /> : <SoundScenes />}
    </Flex>
  );
}
