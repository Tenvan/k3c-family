import { ActionButton, Code, Flex, StatusBadge, Text } from '@webkit/atoms';
import type { GitAction, GitStatus, GitView } from '../api';
import type { ConfirmRequest } from './Confirm';
import { DiffList, fileCount } from './FilesCard';
import { ago } from './format';

interface Props {
  status: GitStatus;
  files: GitView['files'];
  run: (action: GitAction) => Promise<void>;
  confirm: (r: ConfirmRequest) => void;
}

/** Zweiter Schritt des Ablaufs: Commits des Branches gegenüber Remote und develop, dazu Fetch, Pull, Push, Stash. */
export function StatusCard({ status: s, files, run, confirm }: Props) {
  const popStash = () => confirm({
    title: 'Letzten Stash zurückholen?', action: 'Zurückholen',
    text: `${s.stashes[0]?.subject ?? ''} wird auf den Working Tree gelegt; bei Konflikten bleibt der Stash erhalten.`,
    run: () => void run('stash-pop'),
  });
  return (
    <section className="git-card" aria-label="Commit-Status">
      <header className="git-head">
        <div>
          <Text weight="bold">Commit-Status </Text>
          <Code size="2" variant="ghost">{s.branch || 'detached HEAD'}</Code>
          <Text size="1" color="gray"> {s.upstream ? `→ ${s.upstream}` : 'ohne Upstream'} · Fetch {ago(s.fetchedAt)}</Text>
        </div>
        <Flex gap="2" wrap="wrap">
          <ActionButton size="1" variant="soft" onClick={() => run('fetch')}>Fetch</ActionButton>
          <ActionButton size="1" variant="soft" onClick={() => run('pull')} disabled={s.behind === 0 || s.upstreamGone}>Pull</ActionButton>
          <ActionButton size="1" variant="soft" onClick={() => run('push')} disabled={!s.branch || s.branch === 'develop' || s.branch === 'main'}>Push</ActionButton>
          <ActionButton size="1" variant="soft" onClick={() => run('stash')} disabled={s.changes.length === 0}>Stash</ActionButton>
          <ActionButton size="1" variant="soft" onClick={popStash} disabled={s.stashes.length === 0}>Stash zurück</ActionButton>
        </Flex>
      </header>
      <Flex gap="2" wrap="wrap" mt="2">
        {badges(s).map(([tone, text]) => <StatusBadge key={text} tone={tone}>{text}</StatusBadge>)}
      </Flex>
      {s.advice.length > 0 && (
        <ul className="git-advice">
          {s.advice.map((a) => <li key={a}><Text size="2">{a}</Text></li>)}
        </ul>
      )}
      <DiffList base={files.base} files={files.files} />
    </section>
  );
}

type Badge = ['ok' | 'warn' | 'error' | 'info' | 'neutral', string];

/** Pillen des Commit-Status; nur, was etwas sagt. Den Working Tree zeigt die Karte darüber. */
export function badges(s: GitStatus): Badge[] {
  const b: Badge[] = [];
  if (s.upstreamGone) b.push(['error', 'Remote-Branch gelöscht']);
  if (s.ahead > 0) b.push(['warn', `${s.ahead} nicht gepusht`]);
  if (s.behind > 0) b.push(['warn', `${s.behind} auf dem Remote`]);
  if (s.upstream && !s.upstreamGone && s.ahead === 0 && s.behind === 0) b.push(['ok', 'synchron mit Remote']);
  b.push(['neutral', `develop +${s.baseAhead} / −${s.baseBehind}`]);
  if (s.changes.length > 0) b.push(['warn', `${fileCount(s.changes)} Dateien nicht committet`]);
  if (s.stashes.length > 0) b.push(['info', `${s.stashes.length} Stashes`]);
  if (s.worktrees.length > 0) b.push(['info', `${s.worktrees.length} Worktrees`]);
  if (s.mainBehind > 0) b.push(['warn', `main ${s.mainBehind} hinter origin`]);
  return b;
}
