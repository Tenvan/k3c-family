import { useEffect, useState } from 'react';
import { Dialog } from '@webkit/organisms';
import { Badge, Button, Code, Flex, SelectField, Text } from '@webkit/atoms';
import { errorText } from '@webkit/lib/errors';
import { backend, type BalanceDataFile, type BalanceDataValue } from '../api';
import { MarkdownView } from '../ui/molecules/MarkdownView';
import { HelpButton } from './HelpButton';
import { rangeAround } from './help';

interface Props {
  /** Variante `datei.json:pfad=wert` übernehmen. */
  onVary: (line: string) => void;
  /** Abstimm-Parameter `datei.json:pfad=min..max` übernehmen. */
  onParam: (line: string) => void;
}

/** Knopf „Konfigurationsdateien“ mit modalem Popup (WZ28.10, B-610): alle Dateien in data/, die das Balancing
 *  beeinflussen, mit Zweck, wichtigsten Werten, Lesern und Beschreibung (docs/data/); Zahlenwerte als Variante oder
 *  Parameter. Die Liste lädt beim ersten Öffnen. */
export function DataFiles({ onVary, onParam }: Props) {
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
          <Text size="1" color="gray">Dateien in data/, die Simulation, Level-Generator oder Balancing-Tester lesen. „?“ zeigt Aufbau und Werte, „Werte“ übernimmt einen Zahlenwert in die Konfiguration.</Text>
          {error && <p className="svc-error" role="alert">{error}</p>}
          {files === null && !error && <Text color="gray">Lade …</Text>}
          {files?.map((f) => <FileRow key={f.name} file={f} onVary={onVary} onParam={onParam} />)}
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

function FileRow({ file: f, onVary, onParam }: { file: BalanceDataFile } & Props) {
  const [open, setOpen] = useState(false);
  return (
    <div className="bal-file" data-file={f.name}>
      <Flex align="center" gap="2" wrap="wrap">
        <Code>{f.name}</Code>
        {f.readBy.map((r) => <Badge key={r} size="1" color="gray">{r}</Badge>)}
        <HelpButton title={f.name} source={() => backend.balanceDataDoc(f.doc)} />
        {f.vary && <Button size="1" variant="ghost" onClick={() => setOpen(!open)}>{open ? 'Werte ausblenden' : 'Werte'}</Button>}
      </Flex>
      {f.missing ? (
        <Text size="1" color="orange">Beschreibung fehlt (docs/data/{f.doc}.md)</Text>
      ) : (
        <div className="bal-file-text">
          <Text size="1">{f.purpose}</Text>
          {f.key && <MarkdownView source={`Wichtigste Werte: ${f.key}`} />}
        </div>
      )}
      {open && <ValuePicker file={f.name} onVary={onVary} onParam={onParam} />}
    </div>
  );
}

/** Zahlenwerte einer Datei wählen und als Variante (aktueller Wert) oder Parameter (±25 %) übernehmen. */
function ValuePicker({ file, onVary, onParam }: { file: string } & Props) {
  const [values, setValues] = useState<BalanceDataValue[] | null>(null);
  const [error, setError] = useState('');
  const [path, setPath] = useState('');
  const [taken, setTaken] = useState('');
  const take = (fn: (l: string) => void, l: string) => {
    fn(l);
    setTaken(l);
  };
  useEffect(() => {
    backend.balanceDataValues(file).then((v) => { setValues(v); setPath(v[0]?.path ?? ''); }, (e) => setError(errorText(e)));
  }, [file]);
  if (error) return <p className="svc-error" role="alert">{error}</p>;
  if (!values) return <Text size="1" color="gray">Lade Werte …</Text>;
  const chosen = values.find((v) => v.path === path);
  const options = values.map((v) => ({ value: v.path, label: `${v.path} = ${v.value}` }));
  return (
    <Flex gap="2" align="center" wrap="wrap" className="bal-values">
      <SelectField aria-label={`Wert aus ${file}`} value={path} options={options} onChange={setPath} />
      <Button size="1" variant="soft" disabled={!chosen} onClick={() => chosen && take(onVary, `${file}:${chosen.path}=${chosen.value}`)}>Als Variante</Button>
      <Button size="1" variant="soft" disabled={!chosen} onClick={() => chosen && take(onParam, `${file}:${chosen.path}=${rangeAround(chosen.value)}`)}>Als Parameter</Button>
      {taken && <Text size="1" color="green" role="status">Übernommen: {taken}</Text>}
    </Flex>
  );
}
