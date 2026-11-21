import { useState } from 'react';
import { Button, Flex, SelectField, Text, TextField } from '@webkit/atoms';
import type { WorldInfo } from '../api';

// „Welt anlegen“ (B-954): ID, Name und optional die Eltern-Welt; das Backend legt data/worlds/<id>.json an (eine Insel,
// Tiefen 0–4) und lehnt eine vorhandene oder ungültige Welt mit Grund ab. Weiter ausbauen: MCP-Tools world_set & Co.

const ID_PATTERN = /^[a-z0-9][a-z0-9-]{0,31}$/;

interface Props {
  worlds: WorldInfo[];
  /** Antwort ist der Grund der Ablehnung, leer bei Erfolg */
  onCreate: (id: string, name: string, extendsId: string) => Promise<string>;
}

export function NewWorld({ worlds, onCreate }: Props) {
  const [open, setOpen] = useState(false);
  const [id, setId] = useState('');
  const [name, setName] = useState('');
  const [parent, setParent] = useState('');
  const [error, setError] = useState('');
  const valid = ID_PATTERN.test(id) && name.trim() !== '';
  const submit = async () => {
    const err = await onCreate(id, name, parent);
    setError(err);
    if (!err) {
      setOpen(false);
      setId('');
      setName('');
      setParent('');
    }
  };
  if (!open) return <Button size="1" variant="soft" aria-label="Welt anlegen" onClick={() => setOpen(true)}>Welt anlegen</Button>;
  return (
    <Flex direction="column" gap="1" role="form" aria-label="Neue Welt">
      <TextField.Root size="1" aria-label="Welt-ID" placeholder="ID, z. B. hanse" value={id} onChange={(e) => setId(e.target.value)} />
      <TextField.Root size="1" aria-label="Welt-Name" placeholder="Name" value={name} onChange={(e) => setName(e.target.value)}
        onKeyDown={(e) => e.key === 'Enter' && valid && void submit()} />
      <SelectField size="1" aria-label="Eltern-Welt der neuen Welt" value={parent} onChange={setParent}
        options={[{ value: '', label: 'erbt von: Basis' }, ...worlds.filter((w) => !w.missing).map((w) => ({ value: w.id, label: `erbt von: ${w.name}` }))]} />
      {id !== '' && !ID_PATTERN.test(id) && <Text size="1" color="orange">ID: a–z, 0–9, -, höchstens 32 Zeichen</Text>}
      {error && <Text size="1" color="red" role="alert">{error}</Text>}
      <Flex gap="1">
        <Button size="1" aria-label="Anlegen" disabled={!valid} onClick={() => void submit()}>Anlegen</Button>
        <Button size="1" variant="soft" color="gray" aria-label="Abbrechen" onClick={() => { setOpen(false); setError(''); }}>Abbrechen</Button>
      </Flex>
    </Flex>
  );
}
