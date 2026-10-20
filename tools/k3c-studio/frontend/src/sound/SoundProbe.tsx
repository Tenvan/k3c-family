import { ActionButton, Button, Flex, Text } from '@webkit/atoms';
import { useState } from 'react';
import type { AssetChoice, AssetSlot } from '../api';
import { playChoice, stopAll } from './audio';
import { playStatus } from './playback';

/** Hörprobe am Slot (B-459/AC-02): spielt die gewählte Präferenz wie im Spiel, Atlas-Töne eingeschlossen; „fern“ dämpft
 *  einen Atlas-Ton wie in der anderen Bildhälfte, Musik blendet über, Umgebungsgeräusche laufen als Schleife. */
export function SoundProbe({ slot, choice }: { slot: AssetSlot; choice: AssetChoice }) {
  const [status, setStatus] = useState('');
  const play = async (far: boolean) => setStatus(playStatus(slot.beschreibung, choice, await playChoice(slot, choice, far)));
  return (
    <Flex gap="2" align="center" wrap="wrap" className="sd-probe">
      <Text size="2" weight="bold">Hörprobe</Text>
      <ActionButton size="1" onClick={() => play(false)}>Abspielen</ActionButton>
      {choice.sprite && <ActionButton size="1" variant="soft" onClick={() => play(true)}>fern</ActionButton>}
      <Button size="1" variant="soft" color="gray" onClick={() => { stopAll(); setStatus(''); }}>Stopp</Button>
      {status && <Text size="1" color="gray">{status}</Text>}
    </Flex>
  );
}
