import { useState } from 'react';
import { Button, Code, Flex, Heading, StatusBadge, Text, TextField } from '@webkit/atoms';
import { ActionMenu, ChipSwitch } from '@webkit/molecules';
import { Dialog } from '@webkit/organisms';
import { errorText } from '@webkit/lib/errors';
import { backend, type BalanceReportInfo, type BalanceSpec, type BalanceSummary, type ReportRating } from '../api';
import { Confirm, type ConfirmRequest } from '../git/Confirm';
import { entryKind, entryLabel, entryLine2, entryMeta, type ListEntry } from './entries';
import { downloadText, summaryJSON, summaryMarkdown } from './export';

/** Was der Kopf und die Reiter eines Berichts auslösen; die Seite führt es aus. */
export interface DetailActions {
  onRestart: (spec: BalanceSpec) => void;
  /** Formular im Editor öffnen, mit dem Auftrag und optional einer zusätzlichen Variante `datei.json:pfad=wert`. */
  onEdit: (spec: BalanceSpec | undefined, variant?: string) => void;
  onCompareWith: (path: string) => void;
  onRate: (path: string, rating: ReportRating) => void;
  onDelete: (paths: string[]) => void;
  onSaveBaseline: (path: string, name: string) => Promise<boolean>;
}

const RATINGS: { value: ReportRating; label: string }[] = [
  { value: 'gut', label: 'gut' }, { value: 'schlecht', label: 'schlecht' }, { value: 'unklar', label: 'unklar' }, { value: 'referenz', label: 'Referenz' },
];

/** Name der Baseline abfragen (Buchstaben, Ziffern, _ und -, wie Go). */
function BaselineDialog({ open, onClose, onSave }: { open: boolean; onClose: () => void; onSave: (name: string) => Promise<boolean> }) {
  const [name, setName] = useState('');
  const save = async () => {
    if (await onSave(name.trim())) onClose();
  };
  return (
    <Dialog.Root open={open} onOpenChange={(o) => !o && onClose()}>
      <Dialog.Content maxWidth="420px" aria-describedby={undefined}>
        <Dialog.Title>Als Baseline sichern</Dialog.Title>
        <TextField.Root aria-label="Name der Baseline" placeholder="z. B. M5" value={name} onChange={(e) => setName(e.target.value)} />
        <Flex gap="3" mt="4" justify="end">
          <Button variant="soft" color="gray" onClick={onClose}>Abbrechen</Button>
          <Button disabled={name.trim() === ''} onClick={() => void save()}>Sichern</Button>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}

interface Props {
  entry: Extract<ListEntry, { type: 'report' }>;
  info: BalanceReportInfo;
  summary?: BalanceSummary;
  actions: DetailActions;
}

/** Kopf eines Berichts: Name, Status, Auftrag in einer Zeile, Pfad und die Aktionen (B-660). */
export function DetailHead({ entry, info, summary, actions }: Props) {
  const [note, setNote] = useState('');
  const [confirm, setConfirm] = useState<ConfirmRequest | null>(null);
  const [baseline, setBaseline] = useState(false);
  const now = new Date();
  const name = info.path.replace(/\.json$/, '').split('/').pop() || 'balancing';
  const html = () => backend.balanceReportHTML(info.path).then((p) => setNote(`Geöffnet: ${p}`), (e) => setNote(`Fehlgeschlagen: ${errorText(e)}`));
  const ask = () =>
    setConfirm({ title: 'Bericht löschen?', text: `${info.path} wird gelöscht (samt Markdown und Bewertung).`, action: 'Löschen', danger: true, run: () => actions.onDelete([info.path]) });
  return (
    <header className="bal-dhead">
      <Flex gap="2" align="center" wrap="wrap">
        <Heading as="h2" size="4" className="bal-dtitle">{entryLabel(entry, now)}</Heading>
        <StatusBadge tone={info.pass ? 'ok' : 'error'}>{info.pass ? 'Pass' : 'Fail'}</StatusBadge>
        {entryKind(entry) !== 'Balance' && <Text size="1" color="gray">{entryKind(entry)}{entry.baseline ? ` ${entry.baseline}` : ''}</Text>}
      </Flex>
      <Text size="2" color="gray">{[entryMeta(entry), entryLine2(entry, now)].filter(Boolean).join(' · ')}</Text>
      <Code size="1" variant="ghost">{info.path}</Code>
      <Flex gap="2" align="center" wrap="wrap">
        <Button size="1" disabled={!info.spec} title={info.spec ? undefined : 'Älterer Bericht ohne Auftrag'} onClick={() => info.spec && actions.onRestart(info.spec)}>Erneut starten</Button>
        <Button size="1" variant="soft" disabled={!info.spec} title={info.spec ? undefined : 'Älterer Bericht ohne Auftrag'} onClick={() => actions.onEdit(info.spec)}>Bearbeiten und starten</Button>
        <Button size="1" variant="soft" onClick={() => actions.onCompareWith(info.path)}>Vergleichen mit …</Button>
        <ActionMenu label="Export" aria-label="Export" actions={[
          { label: 'Markdown', disabled: !summary, onSelect: () => summary && downloadText(`${name}.md`, summaryMarkdown(summary, name), 'text/markdown') },
          { label: 'JSON', disabled: !summary, onSelect: () => summary && downloadText(`${name}.json`, summaryJSON(summary), 'application/json') },
          { label: 'HTML-Bericht öffnen', onSelect: () => void html() },
        ]} />
        <ActionMenu label="Mehr" aria-label="Weitere Aktionen" actions={[
          { label: 'Als Baseline sichern …', disabled: info.kind !== 'balance' || !info.pass, onSelect: () => setBaseline(true) },
          { label: 'Löschen', danger: true, onSelect: ask },
        ]} />
        <ChipSwitch label="Bewertung" options={RATINGS} value={info.rating ?? ''} onChange={(v) => actions.onRate(info.path, v === info.rating ? '' : v)} />
      </Flex>
      {note && <Text size="1" color="gray" role="status">{note}</Text>}
      <BaselineDialog open={baseline} onClose={() => setBaseline(false)} onSave={(n) => actions.onSaveBaseline(info.path, n)} />
      <Confirm request={confirm} onClose={() => setConfirm(null)} />
    </header>
  );
}
