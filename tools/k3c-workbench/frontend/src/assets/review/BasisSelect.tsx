import { Button, Flex, TextField } from '@webkit/atoms';
import { useState } from 'react';

const FEST = ['HEAD', 'HEAD~1', 'origin/main'] as const;

/** Basis des Reviews: HEAD (Standard), HEAD~1, origin/main oder eine Commit-SHA; der Server prüft den Wert. */
export function BasisSelect({ basis, onChange }: { basis: string; onChange: (basis: string) => void }) {
  const [sha, setSha] = useState('');
  return (
    <Flex gap="2" align="center" wrap="wrap">
      {FEST.map((b) => (
        <Button key={b} size="1" variant={basis === b ? 'solid' : 'soft'} aria-pressed={basis === b} onClick={() => onChange(b)}>
          {b}
        </Button>
      ))}
      <TextField.Root size="1" aria-label="Commit-SHA" placeholder="Commit-SHA" value={sha} onChange={(e) => setSha(e.target.value)} />
      <Button size="1" variant="soft" disabled={!sha.trim()} onClick={() => onChange(sha.trim())}>
        Vergleichen
      </Button>
    </Flex>
  );
}
