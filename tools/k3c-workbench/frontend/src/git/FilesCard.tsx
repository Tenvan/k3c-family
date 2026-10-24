import { Code, Text } from '@webkit/atoms';
import { Table } from '@webkit/organisms';
import type { GitChange, GitDiffFile, GitStatus, GitView } from '../api';

const CODE: Record<string, string> = { M: 'geändert', A: 'neu', D: 'gelöscht', R: 'umbenannt', C: 'kopiert', U: 'Konflikt', '?': 'ungetrackt', T: 'Typ' };

/** Working Tree (noch nicht committet) und Diff des Branches zu origin/develop (das, was ein PR zeigen würde). */
export function FilesCard({ status, files }: { status: GitStatus; files: GitView['files'] }) {
  const added = files.files.reduce((n, f) => n + f.added, 0);
  const deleted = files.files.reduce((n, f) => n + f.deleted, 0);
  return (
    <section className="git-card" aria-label="Geänderte Dateien">
      <header className="git-head">
        <Text weight="bold">Geänderte Dateien</Text>
        <Text size="1" color="gray">Working Tree: {status.changes.length} · Branch zu {files.base}: {files.files.length} Dateien, +{added} −{deleted}</Text>
      </header>
      {status.changes.length > 0 && (
        <>
          <Text size="1" color="gray" as="p" mt="2">Nicht committet</Text>
          <Table.Root size="1" className="git-table">
            <Table.Body>{status.changes.map((c) => <ChangeRow key={c.path} c={c} />)}</Table.Body>
          </Table.Root>
        </>
      )}
      <Text size="1" color="gray" as="p" mt="2">Im Branch seit develop</Text>
      {files.files.length === 0
        ? <Text size="2" color="gray">Keine Commits über develop hinaus.</Text>
        : (
          <Table.Root size="1" className="git-table">
            <Table.Body>{files.files.map((f) => <DiffRow key={f.path} f={f} />)}</Table.Body>
          </Table.Root>
        )}
    </section>
  );
}

function ChangeRow({ c }: { c: GitChange }) {
  return (
    <Table.Row>
      <Table.Cell width="110px"><Text size="1" color={c.code === 'U' ? 'red' : 'gray'}>{CODE[c.code] ?? c.code}{c.staged ? ' · Index' : ''}</Text></Table.Cell>
      <Table.Cell><Code size="1" variant="ghost">{c.path}</Code></Table.Cell>
    </Table.Row>
  );
}

function DiffRow({ f }: { f: GitDiffFile }) {
  return (
    <Table.Row>
      <Table.Cell width="110px"><Text size="1" color="gray">{CODE[f.code] ?? f.code}</Text></Table.Cell>
      <Table.Cell><Code size="1" variant="ghost">{f.path}</Code></Table.Cell>
      <Table.Cell width="110px" justify="end">
        <Text size="1" color="gray">{f.binary ? 'binär' : <><Text color="green">+{f.added}</Text> <Text color="red">−{f.deleted}</Text></>}</Text>
      </Table.Cell>
    </Table.Row>
  );
}
