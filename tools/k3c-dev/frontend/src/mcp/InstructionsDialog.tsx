import { Button, Dialog, Flex } from '@radix-ui/themes';
import { useState } from 'react';
import { backend } from '../api';
import { errorText } from '../lib/errors';
import { MarkdownView } from '../ui/MarkdownView';

/** Knopf `Systemprompt` mit Dialog (MCP-instructions): der Text, den jeder Client beim Verbinden bekommt, als React-Elemente. */
export function InstructionsDialog() {
  const [text, setText] = useState<string | null>(null);
  const [error, setError] = useState('');
  const load = async (open: boolean) => {
    if (!open || text !== null) return;
    try {
      setText(await backend.mcpInstructions());
    } catch (e) {
      setError(errorText(e));
    }
  };
  return (
    <Dialog.Root onOpenChange={(open) => void load(open)}>
      <Dialog.Trigger>
        <Button size="1" variant="soft">Systemprompt</Button>
      </Dialog.Trigger>
      <Dialog.Content maxWidth="760px" className="mcp-instructions">
        <Dialog.Title>Systemprompt für Agenten (MCP-instructions)</Dialog.Title>
        {error && <p className="svc-error">{error}</p>}
        {text !== null && <MarkdownView source={text} headingOffset={1} />}
        <Flex justify="end" mt="4">
          <Dialog.Close>
            <Button variant="soft" color="gray">Schließen</Button>
          </Dialog.Close>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}
