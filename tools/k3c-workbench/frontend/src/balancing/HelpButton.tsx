import { useState, type ReactNode } from 'react';
import { Button, Flex, IconButton, Text } from '@webkit/atoms';
import { Dialog } from '@webkit/organisms';
import { errorText } from '@webkit/lib/errors';
import { MarkdownView } from '../ui/molecules/MarkdownView';
import { FIELD_HELP, type HelpKey } from './help';

interface Props {
  title: string;
  /** Fester Text (Markdown) oder Lader, z. B. die Beschreibung einer Datei aus docs/data/. */
  source: string | (() => Promise<string>);
}

/** Knopf „?“ mit Popup (WZ28.10): zeigt den Text als Markdown; ein Lader wird beim ersten Öffnen gerufen. */
export function HelpButton({ title, source }: Props) {
  const [text, setText] = useState<string | null>(typeof source === 'string' ? source : null);
  const [error, setError] = useState('');
  const load = async (open: boolean) => {
    if (!open || text !== null || typeof source === 'string') return;
    try {
      setText(await source());
    } catch (e) {
      setError(errorText(e));
    }
  };
  return (
    <Dialog.Root onOpenChange={(open) => void load(open)}>
      <Dialog.Trigger>
        <IconButton size="1" variant="soft" radius="full" aria-label={`Hilfe: ${title}`}>?</IconButton>
      </Dialog.Trigger>
      <Dialog.Content maxWidth="720px" className="bal-help bal-modal" aria-describedby={undefined}>
        <Dialog.Title>{title}</Dialog.Title>
        <div className="bal-modal-body">
          {error && <p className="svc-error" role="alert">{error}</p>}
          {text === null && !error && <Text color="gray">Lade …</Text>}
          {text !== null && <MarkdownView source={text.replace(/^# .*\n/, '')} headingOffset={1} />}
        </div>
        <Flex justify="end" mt="4">
          <Dialog.Close>
            <Button variant="soft" color="gray">Schließen</Button>
          </Dialog.Close>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}

/** Eingabefeld mit Beschriftung, Hilfe-Knopf und Hilfetext darunter (Texte aus FIELD_HELP); Auswahlboxen erklären sich
 *  selbst und zeigen den Hilfetext nur im Popup (`hint={false}`). */
export function HelpField({ name, hint = true, children }: { name: HelpKey; hint?: boolean; children: ReactNode }) {
  const h = FIELD_HELP[name];
  return (
    <div className="bal-field" data-help={name}>
      <Flex align="center" gap="1">
        <Text size="1" color="gray">{h.label}</Text>
        <HelpButton title={h.label} source={h.help} />
      </Flex>
      {children}
      {hint && <Text size="1" color="gray" className="bal-hint">{h.hint}</Text>}
    </div>
  );
}
