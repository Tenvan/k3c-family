import { Button, Flex, Text, TextField } from '@webkit/atoms';
import { useEffect, useState } from 'react';
import { text } from './fehlertext';
import { fillPrompt } from '../prompts/fill';
import { reviewWerte } from './prompt';
import { assetsPrompts, type ReviewAsset, type Vorlage } from '../api';

/** Prompt-Buttons am Review-Eintrag (Vorlagen mit Ort `review`): füllen den Prompt und kopieren ihn in die Zwischenablage. */
export function PromptButtons({ asset, basis }: { asset: ReviewAsset; basis: string }) {
  const [vorlagen, setVorlagen] = useState<Vorlage[]>([]);
  const [eingabe, setEingabe] = useState('');
  const [meldung, setMeldung] = useState<{ text: string; fehler: boolean }>();
  useEffect(() => {
    let aktiv = true;
    assetsPrompts().then(
      (v) => aktiv && setVorlagen(v.filter((x) => x.ort === 'review')),
      (e) => aktiv && setMeldung({ text: text(e), fehler: true }),
    );
    return () => {
      aktiv = false;
    };
  }, []);
  const kopiere = async (v: Vorlage) => {
    try {
      await navigator.clipboard.writeText(fillPrompt(v, reviewWerte(asset, basis, eingabe)));
      setMeldung({ text: 'Kopiert.', fehler: false });
    } catch (e) {
      setMeldung({ text: text(e), fehler: true });
    }
  };
  return (
    <Flex direction="column" gap="2">
      <TextField.Root size="1" aria-label="Auftrag an Claude" placeholder="Was soll Claude ändern?" value={eingabe} onChange={(e) => setEingabe(e.target.value)} />
      <Flex gap="2" wrap="wrap" align="center">
        {vorlagen.map((v) => (
          <Button key={v.id} size="1" variant="soft" disabled={!eingabe.trim()} onClick={() => void kopiere(v)}>
            {v.titel}
          </Button>
        ))}
        {meldung && <Text size="1" color={meldung.fehler ? 'red' : 'green'}>{meldung.text}</Text>}
      </Flex>
    </Flex>
  );
}
