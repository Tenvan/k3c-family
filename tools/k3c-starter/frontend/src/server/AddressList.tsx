import { Button, Code, Flex } from '@webkit/atoms';
import { Card } from '@webkit/molecules';
import { useEffect, useRef, useState } from 'react';

/** Wie lange „Kopiert“ bzw. „Nicht möglich“ am Knopf steht. */
const FEEDBACK_MS = 1500;

type Feedback = 'Kopieren' | 'Kopiert' | 'Nicht möglich';

/** Eine Adresse mit Knopf, der sie in die Zwischenablage legt. */
function AddressRow({ url }: { url: string }) {
  const [label, setLabel] = useState<Feedback>('Kopieren');
  const timer = useRef<ReturnType<typeof setTimeout>>(undefined);
  useEffect(() => () => clearTimeout(timer.current), []);
  const copy = async () => {
    try {
      await navigator.clipboard.writeText(url);
      setLabel('Kopiert');
    } catch {
      setLabel('Nicht möglich');
    }
    clearTimeout(timer.current);
    timer.current = setTimeout(() => setLabel('Kopieren'), FEEDBACK_MS);
  };
  return (
    <Card size="1">
      <Flex justify="between" align="center" gap="2">
        <Code variant="ghost" size="2">
          {url}
        </Code>
        <Button size="1" variant="soft" onClick={() => void copy()}>
          {label}
        </Button>
      </Flex>
    </Card>
  );
}

/** Adressen im Heimnetz, jede zum Kopieren. */
export function AddressList({ urls }: { urls: string[] }) {
  return (
    <Flex direction="column" gap="1">
      {urls.map((url) => (
        <AddressRow key={url} url={url} />
      ))}
    </Flex>
  );
}
