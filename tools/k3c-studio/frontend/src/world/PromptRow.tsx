import { useState } from 'react';
import { ActionButton, Flex, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';

// Prompt-Zeile des Welten-Editors (B-879, B-1216): kopiert den Auftrag; die Welt-Dateien
// ändert Claude über die MCP-Tools (world_set, world_story_set, …), nie die Oberfläche selbst.

export function PromptRow({ prompt, label = 'Prompt kopieren', title = 'Prompt in die Zwischenablage kopieren' }: { prompt: string; label?: string; title?: string }) {
  const [note, setNote] = useState<{ ok: boolean; text: string } | null>(null);
  const run = async () => {
    try {
      await navigator.clipboard.writeText(prompt);
      setNote({ ok: true, text: 'Prompt kopiert' });
    } catch (e) {
      setNote({ ok: false, text: errorText(e) });
    }
  };
  return (
    <Flex gap="2" align="center" wrap="wrap">
      <ActionButton size="1" variant="soft" title={title} onClick={() => run()}>{label}</ActionButton>
      {note && <Text size="1" color={note.ok ? 'green' : 'red'}>{note.text}</Text>}
    </Flex>
  );
}
