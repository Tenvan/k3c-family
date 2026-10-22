import { useEffect, useState } from 'react';
import { backend, type Notice } from '../../api';
import { CardButton, Flex, Text, TONE_COLOR, type Tone } from '@webkit/atoms';

// Hinweise eines Agenten (notify_ui) unten rechts: höchstens vier, neueste unten; Fehler bleiben bis zum Klick,
// alle anderen verschwinden nach AUTO_CLOSE_MS.

const AUTO_CLOSE_MS = 8000;
const MAX = 4;
const TONES: Record<Notice['level'], Tone> = { info: 'info', success: 'ok', warn: 'warn', error: 'error' };

interface Shown extends Notice {
  id: number;
}

export function Notices() {
  const [list, setList] = useState<Shown[]>([]);
  useEffect(() => {
    let next = 0;
    return backend.on('ui:notify', (n) => {
      const id = ++next;
      setList((l) => [...l, { ...n, id }].slice(-MAX));
      if (n.level !== 'error') setTimeout(() => setList((l) => l.filter((x) => x.id !== id)), AUTO_CLOSE_MS);
    });
  }, []);
  if (list.length === 0) return null;
  return (
    <div className="notices" role="status" aria-live="polite">
      {list.map((n) => (
        <CardButton key={n.id} variant="classic" className="notice-toast" data-tone={TONES[n.level]} title="Klick schließt"
          onClick={() => setList((l) => l.filter((x) => x.id !== n.id))}>
          <Flex direction="column" gap="1" align="start">
            <Text weight="bold" color={TONE_COLOR[TONES[n.level]]}>{n.title}</Text>
            {n.text && <Text size="2" color="gray">{n.text}</Text>}
          </Flex>
        </CardButton>
      ))}
    </div>
  );
}
