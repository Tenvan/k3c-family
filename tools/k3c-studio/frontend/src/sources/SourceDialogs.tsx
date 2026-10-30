import { ActionButton, Button, Flex, Text } from '@webkit/atoms';
import { Dialog } from '@webkit/organisms';
import { useEffect, useState } from 'react';
import { backend, type SourceAddView, type SourceCleanupView } from '../api';
import { formatBytes, type Selection } from './sourcesLogic';

interface DownloadProps {
  open: boolean;
  selection: Selection;
  onClose: () => void;
  onConfirm: () => Promise<void>;
}

/** Bestätigung vor dem Download (B-601): Gesamtgröße und Quellen stehen im Dialog; ohne „Herunterladen“ wird nichts geladen. */
export function DownloadDialog({ open, selection, onClose, onConfirm }: DownloadProps) {
  return (
    <Dialog.Root open={open} onOpenChange={(o) => !o && onClose()}>
      <Dialog.Content maxWidth="520px" aria-describedby={undefined}>
        <Dialog.Title>Herunterladen bestätigen</Dialog.Title>
        <Flex direction="column" gap="2">
          <Text>{selection.count} {selection.count === 1 ? 'Datei' : 'Dateien'}, zusammen {formatBytes(selection.bytes)}, in den Vorrat <code>.work/vorrat/</code>.</Text>
          <Text size="2" color="gray">Quellen, von denen geladen wird:</Text>
          <ul data-testid="download-sources">
            {selection.quellen.map((q) => <li key={q.name}>{q.name}: {q.count} {q.count === 1 ? 'Datei' : 'Dateien'}, {formatBytes(q.bytes)}</li>)}
          </ul>
          <Text size="2" color="gray">Die Dateien gelten als untrusted: nur Bild und Ton mit erlaubter Endung, nichts wird ausgeführt.</Text>
        </Flex>
        <Flex gap="3" justify="end" mt="4">
          <Button variant="soft" color="gray" onClick={onClose}>Abbrechen</Button>
          <ActionButton disabled={selection.count === 0} onClick={onConfirm}>Herunterladen</ActionButton>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}

interface CleanupProps {
  open: boolean;
  onClose: () => void;
  onDone: (view: SourceCleanupView) => void;
}

/** Aufräumen (B-601, wie B-554): nennt die unverwendeten Dateien im Vorrat und entfernt sie erst nach Bestätigung. */
export function CleanupDialog({ open, onClose, onDone }: CleanupProps) {
  const [view, setView] = useState<SourceCleanupView | null>(null);
  const [error, setError] = useState('');
  useEffect(() => {
    if (!open) return;
    setView(null);
    setError('');
    backend.sourceCleanupPreview().then(setView, (e: Error) => setError(e.message));
  }, [open]);
  const run = async () => {
    try {
      onDone(await backend.sourceCleanup());
    } catch (e) {
      setError((e as Error).message);
    }
  };
  return (
    <Dialog.Root open={open} onOpenChange={(o) => !o && onClose()}>
      <Dialog.Content maxWidth="520px" aria-describedby={undefined}>
        <Dialog.Title>Vorrat aufräumen</Dialog.Title>
        <Flex direction="column" gap="2">
          {error && <Text color="red" role="alert">{error}</Text>}
          {!view && !error && <Text color="gray">Lade …</Text>}
          {view && view.dateien.length === 0 && <Text color="gray">Nichts zu entfernen: Jede geladene Datei ist einem Slot zugeordnet.</Text>}
          {view && view.dateien.length > 0 && (
            <>
              <Text>{view.dateien.length} unverwendete {view.dateien.length === 1 ? 'Datei' : 'Dateien'} ({formatBytes(view.groesse)}) werden aus dem Vorrat entfernt; die Einträge werden wieder „offen“.</Text>
              <ul data-testid="cleanup-files">
                {view.dateien.map((d) => <li key={d.id}>{d.dateiname} · {formatBytes(d.groesse)}</li>)}
              </ul>
            </>
          )}
        </Flex>
        <Flex gap="3" justify="end" mt="4">
          <Button variant="soft" color="gray" onClick={onClose}>Abbrechen</Button>
          <ActionButton color="red" disabled={!view || view.dateien.length === 0} onClick={run}>Entfernen</ActionButton>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}

interface ResearchProps {
  open: boolean;
  /** Quelle des Reiters, "" = alle */
  source: string;
  onClose: () => void;
  onDone: (view: SourceAddView) => void;
}

/** Übernahme aus der Recherche (B-673): zeigt, was als Vorschlag „offen“ angehängt wird (Anzahl, Größe, Liste) und was mit
 *  Grund übersprungen wird; geschrieben wird erst nach „Übernehmen“. Geladen wird nichts. */
export function ResearchDialog({ open, source, onClose, onDone }: ResearchProps) {
  const [view, setView] = useState<SourceAddView | null>(null);
  const [error, setError] = useState('');
  useEffect(() => {
    if (!open) return;
    setView(null);
    setError('');
    backend.sourceResearchPreview(source).then(setView, (e: Error) => setError(e.message));
  }, [open, source]);
  const run = async () => {
    if (!view) return;
    try {
      onDone(await backend.sourceResearchImport(source, view.uebernommen.map((n) => n.dateiUrl)));
    } catch (e) {
      setError((e as Error).message);
    }
  };
  const n = view?.uebernommen.length ?? 0;
  return (
    <Dialog.Root open={open} onOpenChange={(o) => !o && onClose()}>
      <Dialog.Content maxWidth="640px" aria-describedby={undefined}>
        <Dialog.Title>Aus Recherche übernehmen{source ? `: ${source}` : ''}</Dialog.Title>
        <Flex direction="column" gap="2">
          {error && <Text color="red" role="alert">{error}</Text>}
          {!view && !error && <Text color="gray">Prüfe Treffer und frage Größen ab …</Text>}
          {view && (
            <>
              <Text data-testid="research-sum">{n} {n === 1 ? 'Treffer wird' : 'Treffer werden'} als Vorschlag „offen“ übernommen, zusammen {formatBytes(view.bytes)}; {view.uebersprungen.length} übersprungen.</Text>
              <ul data-testid="research-added">
                {view.uebernommen.map((a) => <li key={a.dateiUrl}>{a.titel} · {a.slot} · {a.quelle} · {a.lizenz} · {formatBytes(a.groesse)}</li>)}
              </ul>
              {view.uebersprungen.length > 0 && (
                <>
                  <Text size="2" color="gray">Übersprungen:</Text>
                  <ul data-testid="research-skipped">
                    {view.uebersprungen.map((s, i) => <li key={`${s.dateiUrl}${i}`}>{s.titel}: {s.grund}</li>)}
                  </ul>
                </>
              )}
              <Text size="2" color="gray">Es wird nichts geladen; die Treffer bleiben in der Recherche stehen.</Text>
            </>
          )}
        </Flex>
        <Flex gap="3" justify="end" mt="4">
          <Button variant="soft" color="gray" onClick={onClose}>Abbrechen</Button>
          <ActionButton disabled={n === 0} onClick={run}>Übernehmen</ActionButton>
        </Flex>
      </Dialog.Content>
    </Dialog.Root>
  );
}
