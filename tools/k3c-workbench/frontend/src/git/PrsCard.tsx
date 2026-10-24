import { useState } from 'react';
import { ActionButton, Button, Flex, LinkButton, StatusBadge, Text, TextArea, TextField, type Tone } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { Table } from '@webkit/organisms';
import { backend, type GitPR, type GitPRAction, type GitPRList } from '../api';
import { CopyPrompt } from '../planning/PromptParts';
import type { ConfirmRequest } from './Confirm';
import { ago } from './format';
import { fixPrPrompt, reviewAllPrsPrompt } from './prompts';

const CI_TONE: Record<string, Tone> = { grün: 'ok', rot: 'error', läuft: 'info' };
const MERGE_TONE: Record<string, Tone> = { konfliktfrei: 'ok', Konflikt: 'error' };

interface Props {
  /** Aktueller Branch; von develop/main und ohne Branch gibt es kein „PR anlegen“. */
  branch: string;
  /** null: noch nicht geladen. Die Seite lädt sie mit „Laden“ und nach jeder Aktion. */
  list: GitPRList | null;
  act: (task: Promise<string>) => Promise<void>;
  confirm: (r: ConfirmRequest) => void;
}

/** Dritter Schritt des Ablaufs: offene PRs gegen develop mit CI- und Merge-Stand; anlegen, mergen, schließen. */
export function PrsCard({ branch, list, act, confirm }: Props) {
  const ask = (pr: GitPR, action: GitPRAction) => confirm({
    title: action === 'close' ? `PR #${pr.number} schließen?` : `PR #${pr.number} ${action === 'squash' ? 'per Squash ' : ''}mergen?`,
    text: `„${pr.title}“ (${pr.head} → develop). Den Branch räumt danach die Branch-Liste auf.`,
    action: action === 'close' ? 'Schließen' : action === 'squash' ? 'Squash-Merge' : 'Mergen',
    danger: action === 'close',
    run: () => void act(backend.gitPRAction(pr.number, action)),
  });
  const prs = list?.prs ?? [];
  return (
    <section className="git-card" aria-label="Pull Requests">
      <header className="git-head">
        <Text weight="bold">Pull Requests gegen develop <Text color="gray" size="1">({prs.length} offen
          {list?.fetched && ` · GitHub ${ago(Date.parse(list.fetched))}, aktualisiert sich selbst`})</Text></Text>
        <Flex gap="2" align="center">
          {prs.length > 0 && <><Text size="1" color="gray">Alle prüfen:</Text><CopyPrompt prompt={reviewAllPrsPrompt(prs)} what="Alle PRs prüfen und mergen" /></>}
        </Flex>
      </header>
      {list?.error && <NoticeCard title="GitHub nicht erreichbar" tone="warn">{list.error}</NoticeCard>}
      {!list && <Text size="2" color="gray">Lade PRs …</Text>}
      {prs.length > 0 && (
        <Table.Root size="1" className="git-table">
          <Table.Body>{prs.map((pr) => <PrRow key={pr.number} pr={pr} ask={ask} />)}</Table.Body>
        </Table.Root>
      )}
      {list && !list.error && prs.length === 0 && <Text size="2" color="gray">Keine offenen PRs.</Text>}
      <CreatePr branch={branch} create={(title, body) => act(backend.gitPRCreate(title, body).then((url) => `PR angelegt: ${url}`))} />
    </section>
  );
}

function PrRow({ pr, ask }: { pr: GitPR; ask: (pr: GitPR, a: GitPRAction) => void }) {
  const broken = pr.ci === 'rot' || pr.merge === 'Konflikt';
  const mergeable = pr.state === 'offen' && pr.merge === 'konfliktfrei' && pr.ci !== 'rot';
  return (
    <Table.Row>
      <Table.Cell width="60px"><LinkButton size="1" onClick={() => backend.openUrl(pr.url)}>#{pr.number}</LinkButton></Table.Cell>
      <Table.Cell>
        <Text size="2">{pr.title}</Text> <Text size="1" color="gray">{pr.head}{pr.state === 'Entwurf' ? ' · Entwurf' : ''}</Text>
      </Table.Cell>
      <Table.Cell width="90px"><StatusBadge tone={CI_TONE[pr.ci] ?? 'neutral'}>CI {pr.ci}</StatusBadge></Table.Cell>
      <Table.Cell width="110px"><StatusBadge tone={MERGE_TONE[pr.merge] ?? 'neutral'}>{pr.merge}</StatusBadge></Table.Cell>
      <Table.Cell width="250px" justify="end">
        <Flex gap="1" justify="end" align="center">
          {broken && <CopyPrompt prompt={fixPrPrompt(pr)} what={`PR #${pr.number} beheben`} />}
          <Button size="1" variant="soft" disabled={!mergeable} onClick={() => ask(pr, 'squash')}>Squash</Button>
          <Button size="1" variant="soft" color="gray" disabled={!mergeable} onClick={() => ask(pr, 'merge')}>Merge</Button>
          <Button size="1" variant="soft" color="red" onClick={() => ask(pr, 'close')}>Schließen</Button>
        </Flex>
      </Table.Cell>
    </Table.Row>
  );
}

/** Formular „PR anlegen“ für den aktuellen Branch; aufgeklappt erst auf Klick. */
function CreatePr({ branch, create }: { branch: string; create: (title: string, body: string) => Promise<void> }) {
  const [open, setOpen] = useState(false);
  const [title, setTitle] = useState('');
  const [body, setBody] = useState('');
  if (!branch || branch === 'develop' || branch === 'main') return null;
  if (!open) return <Button size="1" variant="soft" mt="2" onClick={() => setOpen(true)}>PR anlegen für {branch}</Button>;
  const submit = () => create(title, body).then(() => {
    setOpen(false);
    setTitle('');
    setBody('');
  });
  return (
    <Flex direction="column" gap="2" mt="2" className="git-create">
      <TextField.Root size="1" placeholder="K5: Debug-Overlay nur mit ?dev=1" aria-label="Titel des PRs" value={title} onChange={(e) => setTitle(e.target.value)} />
      <TextArea size="1" rows={4} placeholder="Was der PR ändert, Tickets (B-…), Prüfungen" aria-label="Beschreibung des PRs" value={body} onChange={(e) => setBody(e.target.value)} />
      <Flex gap="2">
        <ActionButton size="1" disabled={!title.trim()} onClick={submit}>PR {branch} → develop anlegen</ActionButton>
        <Button size="1" variant="soft" color="gray" onClick={() => setOpen(false)}>Abbrechen</Button>
      </Flex>
    </Flex>
  );
}
