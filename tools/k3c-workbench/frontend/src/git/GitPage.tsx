import { useCallback, useEffect, useState } from 'react';
import { ActionButton, Flex, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import { errorText } from '@webkit/lib/errors';
import { backend, type GitAction, type GitView } from '../api';
import { BranchesCard } from './BranchesCard';
import { Confirm, type ConfirmRequest } from './Confirm';
import { FilesCard } from './FilesCard';
import { PrsCard } from './PrsCard';
import { StatusCard } from './StatusCard';

interface Result {
  ok: boolean;
  text: string;
}

/** Reiter `Git` (B-580): Stand des Repos und die täglichen Handgriffe. Baut sich bei jedem Wechsel neu auf und lädt
 *  dabei frisch; nach jeder Aktion lädt sie neu, das Ergebnis steht oben. */
export function GitPage() {
  const [view, setView] = useState<GitView | null>(null);
  const [error, setError] = useState('');
  const [result, setResult] = useState<Result | null>(null);
  const [ask, setAsk] = useState<ConfirmRequest | null>(null);
  const load = useCallback(() => backend.gitOverview().then((v) => {
    setView(v);
    setError('');
  }, (e: unknown) => setError(errorText(e))), []);
  useEffect(() => void load(), [load]);

  const act = async (task: Promise<string>) => {
    try {
      setResult({ ok: true, text: await task });
    } catch (e) {
      setResult({ ok: false, text: errorText(e) });
    }
    await load();
  };
  return (
    <div className="git-page">
      <Flex gap="3" align="center">
        <ActionButton size="1" variant="soft" onClick={load}>Neu laden</ActionButton>
        {!view && !error && <Text size="2" color="gray">Lese das Repo …</Text>}
        {result && <Text size="2" color={result.ok ? 'green' : 'red'} role="status">{result.text}</Text>}
      </Flex>
      {error && <NoticeCard title="Repo nicht gelesen" tone="error">{error}</NoticeCard>}
      {view && (
        <>
          <StatusCard status={view.status} run={(a: GitAction) => act(backend.gitAction(a))} confirm={setAsk} />
          <BranchesCard branches={view.branches} remove={(names, remote) => act(backend.gitDeleteBranches(names, remote))} confirm={setAsk} />
          <FilesCard status={view.status} files={view.files} />
          <PrsCard branch={view.status.branch} act={act} confirm={setAsk} />
        </>
      )}
      <Confirm request={ask} onClose={() => setAsk(null)} />
    </div>
  );
}
