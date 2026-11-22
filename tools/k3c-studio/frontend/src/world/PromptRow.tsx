import { useState } from 'react';
import { ActionButton, Flex, IconButton, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend } from '../api';

// Prompt-Zeile des Welten-Editors (B-879, B-1216): kopiert den Auftrag oder öffnet ihn in Claude; die Welt-Dateien
// ändert Claude über die MCP-Tools (world_set, world_story_set, …), nie die Oberfläche selbst.

export function PromptRow({ prompt, label = 'Prompt kopieren', title = 'In Claude ändern' }: { prompt: string; label?: string; title?: string }) {
  const [note, setNote] = useState<{ ok: boolean; text: string } | null>(null);
  const run = async (open: boolean) => {
    try {
      await (open ? backend.openInClaude(prompt) : navigator.clipboard.writeText(prompt));
      setNote({ ok: true, text: open ? 'In Claude geöffnet' : 'Prompt kopiert' });
    } catch (e) {
      setNote({ ok: false, text: errorText(e) });
    }
  };
  return (
    <Flex gap="2" align="center" wrap="wrap">
      <ActionButton size="1" variant="soft" onClick={() => run(false)}>{label}</ActionButton>
      <IconButton size="1" variant="ghost" aria-label={title} title="In Claude öffnen" onClick={() => run(true)}>✦</IconButton>
      {note && <Text size="1" color={note.ok ? 'green' : 'red'}>{note.text}</Text>}
    </Flex>
  );
}
