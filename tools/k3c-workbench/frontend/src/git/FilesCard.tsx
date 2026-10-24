import { Code, Text } from '@webkit/atoms';
import { Table } from '@webkit/organisms';
import type { GitChange, GitDiffFile } from '../api';

const CODE: Record<string, string> = { M: 'geändert', A: 'neu', D: 'gelöscht', R: 'umbenannt', C: 'kopiert', U: 'Konflikt', '?': 'ungetrackt', T: 'Typ' };

/** Anzahl Dateien; eine Datei mit gestagten und ungestagten Änderungen zählt einmal. */
export const fileCount = (changes: GitChange[]) => new Set(changes.map((c) => c.path)).size;

/** Erster Schritt des Ablaufs: was im Index liegt (geht in den nächsten Commit) und was noch nicht. */
export function FilesCard({ changes }: { changes: GitChange[] }) {
  const staged = changes.filter((c) => c.staged);
  const unstaged = changes.filter((c) => !c.staged);
  return (
    <section className="git-card" aria-label="Geänderte Dateien">
      <header className="git-head">
        <Text weight="bold">Geänderte Dateien</Text>
        <Text size="1" color="gray">{staged.length} gestaged · {unstaged.length} nicht gestaged</Text>
      </header>
      {changes.length === 0 && <Text size="2" color="gray">Working Tree sauber.</Text>}
      <ChangeList title="Gestaged (im nächsten Commit)" list={staged} />
      <ChangeList title="Nicht gestaged" list={unstaged} />
    </section>
  );
}

function ChangeList({ title, list }: { title: string; list: GitChange[] }) {
  if (list.length === 0) return null;
  return (
    <>
      <Text size="1" color="gray" as="p" mt="2">{title}</Text>
      <Table.Root size="1" className="git-table">
        <Table.Body>
          {list.map((c) => (
            <Table.Row key={c.path}>
              <Table.Cell width="110px"><Text size="1" color={c.code === 'U' ? 'red' : c.staged ? 'green' : 'gray'}>{CODE[c.code] ?? c.code}</Text></Table.Cell>
              <Table.Cell><Code size="1" variant="ghost">{c.path}</Code></Table.Cell>
            </Table.Row>
          ))}
        </Table.Body>
      </Table.Root>
    </>
  );
}

/** Dateien der Commits seit develop (das, was ein PR zeigen würde); Teil des Commit-Status. */
export function DiffList({ base, files }: { base: string; files: GitDiffFile[] }) {
  const added = files.reduce((n, f) => n + f.added, 0);
  const deleted = files.reduce((n, f) => n + f.deleted, 0);
  return (
    <>
      <Text size="1" color="gray" as="p" mt="2">Commits seit {base}: {files.length} Dateien, +{added} −{deleted}</Text>
      {files.length > 0 && (
        <Table.Root size="1" className="git-table">
          <Table.Body>
            {files.map((f) => (
              <Table.Row key={f.path}>
                <Table.Cell width="110px"><Text size="1" color="gray">{CODE[f.code] ?? f.code}</Text></Table.Cell>
                <Table.Cell><Code size="1" variant="ghost">{f.path}</Code></Table.Cell>
                <Table.Cell width="110px" justify="end">
                  <Text size="1" color="gray">{f.binary ? 'binär' : <><Text color="green">+{f.added}</Text> <Text color="red">−{f.deleted}</Text></>}</Text>
                </Table.Cell>
              </Table.Row>
            ))}
          </Table.Body>
        </Table.Root>
      )}
    </>
  );
}
