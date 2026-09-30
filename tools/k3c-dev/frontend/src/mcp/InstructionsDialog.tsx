import { Button, Dialog, Flex } from '@radix-ui/themes';
import { useState } from 'react';
import { backend } from '../api';
import { errorText } from '../lib/errors';
import { parseMarkdown, type Block, type Inline } from './markdown';

/** Knopf `Instructions` mit Dialog: der Text, den jeder Client beim Verbinden bekommt, als React-Elemente. */
export function InstructionsDialog() {
  const [blocks, setBlocks] = useState<Block[] | null>(null);
  const [error, setError] = useState('');
  const load = async (open: boolean) => {
    if (!open || blocks) return;
    try {
      setBlocks(parseMarkdown(await backend.mcpInstructions()));
    } catch (e) {
      setError(errorText(e));
    }
  };
  return (
    <Dialog.Root onOpenChange={(open) => void load(open)}>
      <Dialog.Trigger>
        <Button size="1" variant="soft">Instructions</Button>
      </Dialog.Trigger>
      <Dialog.Content maxWidth="760px" className="mcp-instructions">
        <Dialog.Title>Instructions für Agenten</Dialog.Title>
        {error && <p className="svc-error">{error}</p>}
        {blocks?.map((b, i) => <BlockView key={i} b={b} />)}
        <Flex justify="end" mt="4">
          <Dialog.Close>
            <Button variant="soft" color="gray">Schließen</Button>
          </Dialog.Close>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}

function BlockView({ b }: { b: Block }) {
  if (b.kind === 'h') {
    const H = (`h${b.level + 1}`) as 'h2' | 'h3' | 'h4'; // h1 hat der Dialog-Titel
    return <H><Inlines list={b.inline} /></H>;
  }
  if (b.kind === 'ul') {
    return <ul>{b.items.map((item, i) => <li key={i}><Inlines list={item} /></li>)}</ul>;
  }
  if (b.kind === 'pre') return <pre>{b.text}</pre>;
  return <p><Inlines list={b.inline} /></p>;
}

function Inlines({ list }: { list: Inline[] }) {
  return list.map((x, i) => {
    if (x.kind === 'code') return <code key={i}>{x.text}</code>;
    if (x.kind === 'bold') return <strong key={i}>{x.text}</strong>;
    return <span key={i}>{x.text}</span>;
  });
}
