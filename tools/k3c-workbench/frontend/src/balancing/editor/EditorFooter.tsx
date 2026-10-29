import { useState } from 'react';
import { ActionButton, Button, Flex, Text, TextField } from '@webkit/atoms';

interface Props {
  summary: string;
  startLabel: string;
  /** Grund, warum Starten aus ist (erste falsche Zeile oder Eingabe); leer = bereit. */
  problem: string;
  /** Fehler des letzten Startversuchs. */
  error: string;
  /** Name für „Als Voreinstellung speichern“ vorgeschlagen (Label des Laufs). */
  presetName: string;
  onSavePreset: (name: string) => void;
  onReset: () => void;
  onStart: () => Promise<unknown> | void;
}

/** Feste Fußleiste (B-661): Zusammenfassung, Voreinstellung speichern, Zurücksetzen, Starten. */
export function EditorFooter({ summary, startLabel, problem, error, presetName, onSavePreset, onReset, onStart }: Props) {
  const [naming, setNaming] = useState(false);
  const [name, setName] = useState('');
  const save = () => { onSavePreset(name); setNaming(false); };
  return (
    <footer className="bal-ed-foot" aria-label="Fußleiste">
      <div className="bal-ed-footsum">
        <Text size="2" role="status" aria-label="Zusammenfassung">{summary}</Text>
        {(problem || error) && <Text size="1" color="red" role="alert">{error || problem}</Text>}
      </div>
      {naming ? (
        <Flex gap="2" align="center">
          <TextField.Root aria-label="Name der Voreinstellung" placeholder="Name" value={name} onChange={(e) => setName(e.target.value)} />
          <Button size="2" variant="soft" disabled={name.trim() === ''} onClick={save}>Speichern</Button>
          <Button size="2" variant="ghost" color="gray" onClick={() => setNaming(false)}>Abbrechen</Button>
        </Flex>
      ) : (
        <Button variant="soft" onClick={() => { setName(presetName); setNaming(true); }}>Als Voreinstellung speichern</Button>
      )}
      <Button variant="ghost" color="gray" onClick={onReset}>Zurücksetzen</Button>
      <ActionButton disabled={problem !== ''} onClick={onStart}>{startLabel}</ActionButton>
    </footer>
  );
}
