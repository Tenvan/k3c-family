import { useState } from 'react';
import { ActionButton, Flex, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import type { Vorlage } from '../api';
import { fillPrompt, type Werte } from './fill';

interface Props {
  vorlagen: Vorlage[];
  werte: Werte;
  caption: string;
  /** Hinweis statt leerer Fläche, wenn es für den Ort keine Vorlage gibt; ohne Hinweis zeichnet nichts. */
  leer?: string;
}

/** Prompt-Buttons (Entscheidung 005 › 5): der Titel kopiert den gefüllten Prompt in die Zwischenablage. Kein schreibendes Control: geändert wird erst über MCP. */
export function PromptButtons({ vorlagen, werte, caption, leer }: Props) {
  const [note, setNote] = useState<{ ok: boolean; text: string } | null>(null);
  if (!vorlagen.length) return leer ? <Text size="1" color="gray">{leer}</Text> : null;
  const run = async (v: Vorlage) => {
    try {
      const prompt = fillPrompt(v, werte);
      await navigator.clipboard.writeText(prompt);
      setNote({ ok: true, text: `„${v.titel}“ kopiert` });
    } catch (e) {
      setNote({ ok: false, text: errorText(e) });
    }
  };
  return (
    <Flex direction="column" gap="1">
      <Text size="1" color="gray">{caption}</Text>
      <Flex gap="2" wrap="wrap">
        {vorlagen.map((v) => (
          <ActionButton key={v.id} size="1" variant="soft" onClick={() => run(v)}>{v.titel}</ActionButton>
        ))}
      </Flex>
      {note && <Text size="1" color={note.ok ? 'green' : 'red'}>{note.text}</Text>}
    </Flex>
  );
}
