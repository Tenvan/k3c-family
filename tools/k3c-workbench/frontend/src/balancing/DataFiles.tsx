import { useState } from 'react';
import { Dialog } from '@webkit/organisms';
import { Badge, Button, Code, Flex, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend, type BalanceDataFile } from '../api';
import { MarkdownView } from '../ui/molecules/MarkdownView';
import { HelpButton } from './HelpButton';

/** Knopf „Konfigurationsdateien“ mit modalem Popup (WZ28.10, B-610): alle Dateien in data/, die das Balancing
 *  beeinflussen, mit Zweck, wichtigsten Werten, Lesern und Beschreibung (docs/data/); Werte wählt der Daten-Browser im Lauf-Editor. Die Liste lädt beim ersten Öffnen. */
export function DataFiles() {
  const [files, setFiles] = useState<BalanceDataFile[] | null>(null);
  const [error, setError] = useState('');
  const load = (open: boolean) => {
    if (open && files === null) backend.balanceDataFiles().then(setFiles, (e) => setError(errorText(e)));
  };
  return (
    <Dialog.Root onOpenChange={load}>
      <Dialog.Trigger>
        <Button size="1" variant="soft">Konfigurationsdateien …</Button>
      </Dialog.Trigger>
      <Dialog.Content maxWidth="820px" className="bal-modal" aria-describedby={undefined}>
        <Dialog.Title>Konfigurationsdateien</Dialog.Title>
        <section className="bal-files bal-modal-body" aria-label="Konfigurationsdateien">
          <Text size="1" color="gray">Dateien in data/, die Simulation, Level-Generator oder Balancing-Tester lesen. „?“ zeigt Aufbau und Werte,</Text>
          {error && <p className="svc-error" role="alert">{error}</p>}
          {files === null && !error && <Text color="gray">Lade …</Text>}
          {files?.map((f) => <FileRow key={f.name} file={f} />)}
        </section>
        <Flex justify="end" mt="4">
          <Dialog.Close>
            <Button variant="soft" color="gray">Schließen</Button>
          </Dialog.Close>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}

function FileRow({ file: f }: { file: BalanceDataFile }) {
  return (
    <div className="bal-file" data-file={f.name}>
      <Flex align="center" gap="2" wrap="wrap">
        <Code>{f.name}</Code>
        {f.readBy.map((r) => <Badge key={r} size="1" color="gray">{r}</Badge>)}
        <HelpButton title={f.name} source={() => backend.balanceDataDoc(f.doc)} />
      </Flex>
      {f.missing ? (
        <Text size="1" color="orange">Beschreibung fehlt (docs/data/{f.doc}.md)</Text>
      ) : (
        <div className="bal-file-text">
          <Text size="1">{f.purpose}</Text>
          {f.key && <MarkdownView source={`Wichtigste Werte: ${f.key}`} />}
        </div>
      )}
    </div>
  );
}
