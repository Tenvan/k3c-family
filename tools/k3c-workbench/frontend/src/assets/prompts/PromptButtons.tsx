import { useState } from 'react';
import { backend } from '../../api';
import { ActionButton, Flex, IconButton, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import type { Vorlage } from '../api';
import { fillPrompt, type Werte } from './fill';

interface Props {
  vorlagen: Vorlage[];
  werte: Werte;
  caption: string;
}

/** Prompt-Buttons (Entscheidung 005 › 5): der Titel kopiert den gefüllten Prompt in die Zwischenablage, ✦ öffnet ihn als
 *  neue Code-Session in Claude Desktop (B-701 D-12). Kein schreibendes Control: geändert wird erst über MCP. */
export function PromptButtons({ vorlagen, werte, caption }: Props) {
  const [note, setNote] = useState<{ ok: boolean; text: string } | null>(null);
  if (!vorlagen.length) return null;
  const run = async (v: Vorlage, open: boolean) => {
    try {
      const prompt = fillPrompt(v, werte);
      await (open ? backend.openInClaude(prompt) : navigator.clipboard.writeText(prompt));
      setNote({ ok: true, text: open ? `„${v.titel}“ in Claude geöffnet` : `„${v.titel}“ kopiert` });
    } catch (e) {
      setNote({ ok: false, text: errorText(e) });
    }
  };
  return (
    <Flex direction="column" gap="1">
      <Text size="1" color="gray">{caption}</Text>
      <Flex gap="2" wrap="wrap">
        {vorlagen.map((v) => (
          <Flex key={v.id} gap="0" align="center">
            <ActionButton size="1" variant="soft" onClick={() => run(v, false)}>{v.titel}</ActionButton>
            <IconButton size="1" variant="ghost" aria-label={`In Claude öffnen: ${v.titel}`} title="In Claude öffnen" onClick={() => run(v, true)}>✦</IconButton>
          </Flex>
        ))}
      </Flex>
      {note && <Text size="1" color={note.ok ? 'green' : 'red'}>{note.text}</Text>}
    </Flex>
  );
}
