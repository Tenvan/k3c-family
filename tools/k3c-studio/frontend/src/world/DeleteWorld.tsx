import { useState } from 'react';
import { Button, Flex, LinkButton, Text, TextField } from '@webkit/atoms';

// „Welt löschen“ (B-1172, WZ62.6): Bestätigung mit dem Namen der Welt; das Backend lehnt feste Welten und Welten mit Erben
// mit Grund ab und löscht sonst die Datei endgültig (sie liegt in Git). Spielstände der Welt bleiben.

interface Props {
  id: string;
  name: string;
  /** Antwort ist der Grund der Ablehnung, leer bei Erfolg */
  onDelete: (id: string) => Promise<string>;
}

export function DeleteWorld({ id, name, onDelete }: Props) {
  const [open, setOpen] = useState(false);
  const [typed, setTyped] = useState('');
  const [error, setError] = useState('');
  const confirmed = typed.trim() === name;
  const submit = async () => setError(await onDelete(id));
  if (!open) return <LinkButton size="1" color="red" aria-label="Welt löschen" onClick={() => setOpen(true)}>Welt löschen …</LinkButton>;
  return (
    <Flex direction="column" gap="1" role="form" aria-label="Welt löschen">
      <Text size="1">Die Datei data/worlds/{id}.json wird endgültig gelöscht (in Git wiederherstellbar). Zur Bestätigung den Namen der Welt eingeben: <b>{name}</b></Text>
      <Flex gap="1" align="center" wrap="wrap">
        <TextField.Root size="1" aria-label="Name zur Bestätigung" placeholder={name} value={typed} onChange={(e) => setTyped(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && confirmed && void submit()} />
        <Button size="1" color="red" aria-label="Endgültig löschen" disabled={!confirmed} onClick={() => void submit()}>Endgültig löschen</Button>
        <Button size="1" variant="soft" color="gray" aria-label="Abbrechen" onClick={() => { setOpen(false); setTyped(''); setError(''); }}>Abbrechen</Button>
      </Flex>
      {error && <Text size="1" color="red" role="alert">{error}</Text>}
    </Flex>
  );
}
